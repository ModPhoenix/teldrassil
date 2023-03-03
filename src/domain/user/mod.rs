pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("invalid id: {0}")]
    InvalidId(String),

    #[error("invalid email: {0}")]
    InvalidEmail(String),

    #[error("invalid username: {0}")]
    InvalidUsername(String),

    #[error("invalid password: {0}")]
    InvalidPassword(String),

    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: field::UserId,
    pub email: field::Email,
    pub username: field::Username,
    pub password: field::Password,
    pub created_at: field::CreatedAt,
    pub updated_at: field::UpdatedAt,
}
