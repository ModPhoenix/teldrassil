use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct NodeId(DbId);

impl NodeId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for NodeId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl From<String> for NodeId {
    fn from(id: String) -> Self {
        Self(DbId::from(id))
    }
}
