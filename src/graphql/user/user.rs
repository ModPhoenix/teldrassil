use crate::data;
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

impl From<data::user::User> for User {
    fn from(user: data::user::User) -> User {
        User {
            id: user.id,
            email: user.email.clone(),
            username: user.username.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}