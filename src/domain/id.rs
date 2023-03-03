use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Id {
        Id(Uuid::new_v4())
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        format!("{}", id.0)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Id {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Id(Uuid::parse_str(id)?))
    }
}
