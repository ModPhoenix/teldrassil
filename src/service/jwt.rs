use anyhow::Result;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::data::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
}

impl Claims {
    pub(crate) fn new(user: &User, auth_duration_in_hour: u16) -> Self {
        let User { id, email, .. } = user;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            sub: id.to_string(),
            email: email.into(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn encode_jwt(user: &User) -> Result<String> {
    let claims = Claims::new(user, 24);
    let encoded = encode(
        &Header::default(),
        &claims,
        // TODO: get secret key from env
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(encoded)
}

pub fn decode_jwt(token: String) -> Result<Claims> {
    let claims = decode::<Claims>(
        &token,
        // TODO: get secret key from env
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?
    .claims;

    Ok(claims)
}
