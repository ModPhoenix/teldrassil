use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, username: String, password: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            email,
            username,
            password,
            created_at: now,
            updated_at: now,
        }
    }
}
