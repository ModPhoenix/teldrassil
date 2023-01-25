use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Node {
    pub fn new(name: String, content: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            name,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_with_id(id: Uuid, name: String, content: String) -> Self {
        let now = Utc::now();

        Self {
            id,
            name,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeWithEdges {
    pub node: Node,
    pub parents: Vec<Uuid>,
    pub children: Vec<Uuid>,
}

impl NodeWithEdges {
    pub fn new(node: Node, parents: Vec<Uuid>, children: Vec<Uuid>) -> Self {
        Self {
            node,
            parents,
            children,
        }
    }
}
