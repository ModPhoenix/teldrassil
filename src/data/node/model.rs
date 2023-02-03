use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    data::DbId,
    domain::{self, Time},
    service,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: DbId,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Convert from a database model into a domain Node.
impl TryFrom<Node> for domain::Node {
    type Error = domain::NodeError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        use crate::domain::node::field;

        Ok(Self {
            id: field::NodeId::new(node.id),
            name: field::Name::new(node.name.as_str())?,
            content: field::Content::new(node.content.as_str())?,
            created_at: field::CreatedAt::new(Time::new(node.created_at)),
            updated_at: field::UpdatedAt::new(Time::new(node.updated_at)),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct NodeChildren {
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewNode {
    pub id: Option<DbId>,
    pub name: String,
    pub content: String,
    pub parent_id: Option<DbId>,
}

impl From<service::node::NewNode> for NewNode {
    fn from(input: service::node::NewNode) -> Self {
        Self {
            id: None,
            name: input.name.into_inner(),
            content: input.content.into_inner(),
            parent_id: input.parent_id.map(|id| id.into_inner()),
        }
    }
}

#[derive(Debug)]
pub struct GetNode {
    pub(in crate::data) id: DbId,
}

impl From<DbId> for GetNode {
    fn from(id: DbId) -> Self {
        GetNode { id }
    }
}

impl From<service::node::GetNode> for GetNode {
    fn from(input: service::node::GetNode) -> Self {
        Self {
            id: input.id.into_inner(),
        }
    }
}
