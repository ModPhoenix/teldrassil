use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    data::DbId,
    domain::{self, Time},
    service,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub(in crate::data) id: DbId,
    pub(in crate::data) email: String,
    pub(in crate::data) username: String,
    pub(in crate::data) password: String,
    pub(in crate::data) created_at: DateTime<Utc>,
    pub(in crate::data) updated_at: DateTime<Utc>,
}

impl TryFrom<User> for domain::User {
    type Error = domain::UserError;
    fn try_from(user: User) -> Result<Self, Self::Error> {
        use crate::domain::user::field;

        Ok(Self {
            id: field::UserId::new(user.id.uuid()),
            email: field::Email::new(user.email.as_str())?,
            username: field::Username::new(user.username.as_str())?,
            password: field::Password::new(user.password.as_str())?,
            created_at: field::CreatedAt::new(Time::new(user.created_at)),
            updated_at: field::UpdatedAt::new(Time::new(user.updated_at)),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl From<service::user::NewUser> for NewUser {
    fn from(input: service::user::NewUser) -> Self {
        Self {
            email: input.email.into_inner(),
            username: input.username.into_inner(),
            password: input.password.into_inner(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetUser {
    pub id: Uuid,
}

impl TryFrom<service::GetUser> for GetUser {
    type Error = service::ServiceError;

    fn try_from(input: service::GetUser) -> Result<Self, Self::Error> {
        Ok(Self {
            id: input.id.into_inner(),
        })
    }
}
