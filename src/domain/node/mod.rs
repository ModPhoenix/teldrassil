//! Structures, errors, and implementation for the [`Clip`](crate::Clip) data type.
pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The possible errors that can occur when building a [`Clip`]
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
