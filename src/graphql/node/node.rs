use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data, graphql::get_datastore};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub parents: Vec<Node>,
    pub children: Vec<Node>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    parents_ids: Vec<Uuid>,
    children_ids: Vec<Uuid>,
}

#[Object]
impl Node {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn content(&self) -> String {
        self.content.to_string()
    }

    async fn parents(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .parents_ids
            .iter()
            .flat_map(|id| data::node::get_node_by_id(datastore, *id))
            .map(|node_with_edges| node_with_edges.into())
            .collect::<Vec<_>>();

        Ok(parents)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .children_ids
            .iter()
            .flat_map(|id| data::node::get_node_by_id(datastore, *id))
            .map(|node_with_edges| node_with_edges.into())
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

impl From<data::node::NodeWithEdges> for Node {
    fn from(node_with_edges: data::node::NodeWithEdges) -> Node {
        let data::node::NodeWithEdges {
            node,
            parents,
            children,
        } = node_with_edges;

        let data::node::Node {
            id,
            name,
            content,
            created_at,
            updated_at,
            ..
        } = node;

        let parents_ids = parents;
        let children_ids = children;

        Node {
            id,
            name,
            content,
            parents: vec![],
            children: vec![],
            created_at,
            updated_at,
            parents_ids,
            children_ids,
        }
    }
}
