use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain, graphql::get_db, service};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
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

    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Node>> {
        let db = get_db(ctx)?;

        let parent = service::get_node_parent(db, service::GetNode { id: self.id.into() })
            .await?
            .map(|node| node.clone().into());

        Ok(parent)
    }

    async fn context(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let db = get_db(ctx)?;

        let context =
            service::get_node_context(db, service::GetNode { id: self.id.into() }).await?;

        Ok(context.into_iter().map(|node| node.into()).collect())
    }

    async fn meanings(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let db = get_db(ctx)?;

        let meanings = service::get_node_meanings(
            db,
            service::GetNodeMeanings {
                id: self.id.into(),
                name: self.name.to_string().try_into()?,
            },
        )
        .await?;

        Ok(meanings.into_iter().map(|node| node.into()).collect())
    }

    async fn children(
        &self,
        ctx: &Context<'_>,
        input: Option<service::GetNodeChildrenInput>,
    ) -> Result<Vec<Node>> {
        let db = get_db(ctx)?;

        let input = input.unwrap();

        let input = service::GetNodeChildrenInput {
            id: self.id.to_string(),
            ..input
        };

        let children = service::get_node_children(input, db).await?;

        Ok(children.into_iter().map(|node| node.into()).collect())
    }
}

impl From<domain::Node> for Node {
    fn from(node: domain::Node) -> Node {
        Node {
            id: node.id.into_inner(),
            name: node.name.into_inner(),
            content: node.content.into_inner(),
            created_at: node.created_at.into_inner().into_inner(),
            updated_at: node.updated_at.into_inner().into_inner(),
        }
    }
}
