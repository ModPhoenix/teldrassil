use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Internal database ID that can be used for any ID purposes.
#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct Id(String);

impl Id {
    /// Create a new database ID.
    pub fn new(table: &str) -> Id {
        format!("{table}:`{uuid}`", uuid = Uuid::new_v4()).into()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        format!("{}", id.into_inner())
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Id(value.to_owned())
    }
}
