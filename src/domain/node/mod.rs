pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("invalid id: {0}")]
    InvalidId(String),

    /// Name was not provided.
    #[error("empty name")]
    EmptyName,

    /// Content was not provided.
    #[error("empty content")]
    EmptyContent,

    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    pub id: field::NodeId,
    pub name: field::Name,
    pub content: field::Content,
    pub created_at: field::CreatedAt,
    pub updated_at: field::UpdatedAt,
}
