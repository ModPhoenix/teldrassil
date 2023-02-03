use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{domain, graphql::get_db, service};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl Node {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> String {
        self.name.to_string()
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

    async fn parent(&self, _ctx: &Context<'_>) -> Result<Option<Node>> {
        Ok(None)
    }

    async fn context(&self, _ctx: &Context<'_>) -> Result<Vec<Node>> {
        Ok(vec![])
    }

    async fn meanings(&self, _ctx: &Context<'_>) -> Result<Vec<Node>> {
        Ok(vec![])
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let db = get_db(ctx)?;

        let children = service::get_node_children(
            db,
            service::GetNode {
                id: self.id.to_string().into(),
            },
        )
        .await?;

        Ok(children.into_iter().map(|node| node.into()).collect())
    }
}

impl From<domain::Node> for Node {
    fn from(node: domain::Node) -> Node {
        Node {
            id: node.id.into_inner().into_inner(),
            name: node.name.into_inner(),
            content: node.content.into_inner(),
            created_at: node.created_at.into_inner().into_inner(),
            updated_at: node.updated_at.into_inner().into_inner(),
        }
    }
}
