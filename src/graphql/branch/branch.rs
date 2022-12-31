use crate::data;
use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub parents: Vec<Branch>,
    pub children: Vec<Branch>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl Branch {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn content(&self) -> String {
        self.content.to_string()
    }

    async fn parents(&self) -> Vec<Branch> {
        vec![]
    }

    async fn children(&self) -> Vec<Branch> {
        vec![]
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl From<data::branch::Branch> for Branch {
    fn from(branch: data::branch::Branch) -> Branch {
        Branch {
            id: branch.id,
            name: branch.name.clone(),
            content: branch.content.clone(),
            parents: vec![],
            children: vec![],
            created_at: branch.created_at,
            updated_at: branch.updated_at,
        }
    }
}
