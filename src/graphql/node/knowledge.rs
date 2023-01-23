use crate::data;
use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl Knowledge {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn title(&self) -> String {
        self.title.to_string()
    }

    async fn content(&self) -> String {
        self.content.to_string()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl From<data::branch::Branch> for Knowledge {
    fn from(branch: data::branch::Branch) -> Knowledge {
        Knowledge {
            id: branch.data.id,
            title: branch.data.name.clone(),
            content: branch.data.content.clone(),
            created_at: branch.data.created_at,
            updated_at: branch.data.updated_at,
        }
    }
}
