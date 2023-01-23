use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data, graphql::get_datastore};

use super::Knowledge;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub parents: Vec<Node>,
    pub children: Vec<Node>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub data: NodeData,
    parents_ids: Vec<Uuid>,
    children_ids: Vec<Uuid>,
}

#[derive(Union, Debug, Clone, Serialize, Deserialize)]
pub enum NodeData {
    Knowledge(Knowledge),
}

#[Object]
impl Node {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn data(&self) -> NodeData {
        self.data.clone()
    }

    async fn parents(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .parents_ids
            .iter()
            .flat_map(|id| data::node::get_node(datastore, *id))
            .map(|branch| branch.into())
            .collect::<Vec<_>>();

        Ok(parents)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .children_ids
            .iter()
            .flat_map(|id| data::node::get_node(datastore, *id))
            .map(|branch| branch.into())
            .collect::<Vec<_>>();

        Ok(parents)
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl From<data::node::Node> for Node {
    fn from(branch: data::node::Node) -> Node {
        Node {
            id: branch.data.id,
            name: branch.data.name.clone(),
            data: NodeData::Knowledge(Knowledge {
                id: branch.data.id,
                title: branch.data.name.clone(),
                content: branch.data.content.clone(),
                created_at: branch.data.created_at,
                updated_at: branch.data.updated_at,
            }),
            parents: vec![],
            children: vec![],
            created_at: branch.data.created_at,
            updated_at: branch.data.updated_at,
            parents_ids: branch.parents,
            children_ids: branch.children,
        }
    }
}
