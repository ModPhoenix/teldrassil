//! Intermediate layer between the data layer and graphql layer.
pub mod jwt;
pub mod node;
pub use node::*;

use crate::{data::DataError, domain::NodeError};

/// The possible errors that can occur when working with the [`service layer`](crate::service).
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("node error: {0}")]
    Node(#[from] NodeError),
    #[error("database error: {0}")]
    Data(#[from] DataError),
}
