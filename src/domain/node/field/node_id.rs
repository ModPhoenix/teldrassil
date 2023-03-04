use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for NodeId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
