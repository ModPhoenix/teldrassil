use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{self, user::field},
    service::ServiceError,
};

#[derive(InputObject)]
pub struct SignUpInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub username: field::Username,
    pub email: field::Email,
    pub password: field::Password,
}

impl TryFrom<SignUpInput> for NewUser {
    type Error = domain::UserError;

    fn try_from(input: SignUpInput) -> Result<Self, Self::Error> {
        Ok(Self {
            username: input.username.try_into()?,
            email: field::Email::new(&input.email.as_str())?,
            password: input.password.try_into()?,
        })
    }
}

#[derive(InputObject, Clone)]
pub struct SignInInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCredentials {
    pub email: field::Email,
    pub password: field::Password,
}

impl TryFrom<SignInInput> for UserCredentials {
    type Error = domain::UserError;

    fn try_from(input: SignInInput) -> Result<Self, Self::Error> {
        Ok(Self {
            email: field::Email::new(&input.email.as_str())?,
            password: input.password.try_into()?,
        })
    }
}

#[derive(InputObject)]
pub struct GetUserInput {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUser {
    pub id: field::UserId,
}

impl TryFrom<GetUserInput> for GetUser {
    type Error = ServiceError;

    fn try_from(input: GetUserInput) -> Result<Self, Self::Error> {
        Ok(Self {
            id: input.id.try_into()?,
        })
    }
}
