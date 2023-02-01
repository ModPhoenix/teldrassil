//! Database models and queries.

pub mod init;
pub use init::*;

pub mod node;

pub mod utils;

use surrealdb::{engine::local::Db, Surreal};

pub type DatabaseLocal = Surreal<Db>;
pub type Database = DatabaseLocal;

/// The possible errors that may occur when working with a database.
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// Database error.
    #[error("database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("database error: {0}")]
    Db(#[from] surrealdb::error::Db),

    #[error("record not found")]
    NotFound,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// [`Result`] alias for database query functions.
pub type Result<T> = std::result::Result<T, DataError>;

pub type DbId = String;
