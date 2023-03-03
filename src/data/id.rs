use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Display, From)]
pub struct DbId(String);

impl DbId {
    pub fn new(table_name: &str) -> Self {
        let uuid = Uuid::new_v4();

        Self::from_uuid(table_name, uuid)
    }

    pub fn new_nil(table_name: &str) -> Self {
        let uuid = Uuid::nil();

        Self::from_uuid(table_name, uuid)
    }

    pub fn from_uuid(table_name: &str, id: Uuid) -> Self {
        Self(format!("{}:⟨{}⟩", table_name, id))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn uuid(&self) -> Uuid {
        self.0
            .split(':')
            .nth(1)
            .unwrap()
            .trim_start_matches('⟨')
            .trim_end_matches('⟩')
            .to_string()
            .parse()
            .unwrap()
    }
}
