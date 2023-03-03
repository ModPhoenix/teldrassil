use crate::domain;
use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<domain::User> for User {
    fn from(user: domain::User) -> User {
        User {
            id: user.id.into_inner(),
            email: user.email.into_inner(),
            username: user.username.into_inner(),
            created_at: user.created_at.into_inner().into_inner(),
            updated_at: user.updated_at.into_inner().into_inner(),
        }
    }
}
