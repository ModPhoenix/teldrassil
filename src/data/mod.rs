//! Database models and queries.

pub mod init;
use std::fmt::{Display, Formatter};

pub use init::*;

pub mod node;

pub mod utils;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, Surreal};
use uuid::Uuid;

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

// pub type DbId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbId(String);

impl DbId {
    pub fn new(table_name: &str) -> Self {
        let uuid = Uuid::new_v4();

        Self(format!("{}:⟨{}⟩", table_name, uuid))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn id(&self) -> String {
        self.0
            .split(':')
            .nth(1)
            .unwrap()
            .trim_start_matches('⟨')
            .trim_end_matches('⟩')
            .to_string()
    }
}

impl Display for DbId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for DbId {
    fn from(id: String) -> Self {
        Self(id)
    }
}
