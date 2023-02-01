use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::data::DbId;

#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: DbId,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // pub(in crate::data) parent_id: Option<DbId>,
    // pub(in crate::data) context: Vec<DbId>,
    // pub(in crate::data) meanings: Vec<DbId>,
    // pub(in crate::data) children: Vec<DbId>,
}

#[derive(Debug, Deserialize)]
pub struct NodeChildren {
    pub children: Vec<Node>,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct NodeData {
//     pub(in crate::data) id: DbId,
//     pub(in crate::data) name: String,
//     pub(in crate::data) content: String,
//     pub(in crate::data) created_at: DateTime<Utc>,
//     pub(in crate::data) updated_at: DateTime<Utc>,
// }

#[derive(Debug, Clone, Serialize)]
pub struct NewNode {
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub parent_id: Option<DbId>,
}

#[derive(Debug)]
pub struct GetNode {
    pub(in crate::data) id: DbId,
}

impl From<String> for GetNode {
    fn from(id: DbId) -> Self {
        GetNode { id }
    }
}
