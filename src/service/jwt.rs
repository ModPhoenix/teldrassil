use anyhow::Result;
use chrono::{Duration, Local};
use hyper::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain;

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
    pub(crate) fn new(user: &domain::User, auth_duration_in_hour: u16) -> Self {
        let domain::User { id, email, .. } = user;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            sub: id.clone().into_inner().to_string(),
            email: email.clone().into_inner(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn encode_jwt(user: &domain::User) -> Result<String> {
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

pub fn get_claims_from_headers(headers: &HeaderMap) -> Option<Claims> {
    headers
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt = s.split(' ').last().unwrap_or_default();

                let token_data = decode_jwt(jwt.to_owned()).ok();

                token_data
            })
        })
        .flatten()
}
