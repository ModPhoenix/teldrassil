//! Intermediate layer between the data layer and graphql layer.
pub mod jwt;
use std::convert::Infallible;

pub mod node;
pub use node::*;
pub mod user;
pub use user::*;

use crate::{
    data::DataError,
    domain::{NodeError, UserError},
};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("uuid error: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("{0}")]
    Data(#[from] DataError),
    #[error("node error: {0}")]
    Node(#[from] NodeError),
    #[error("user error: {0}")]
    User(#[from] UserError),
}

impl From<Infallible> for ServiceError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
