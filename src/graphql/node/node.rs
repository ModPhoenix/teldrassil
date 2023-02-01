use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data_old, graphql::get_datastore};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub parent: Option<Box<Node>>,
    pub context: Vec<Node>,
    pub meanings: Vec<Node>,
    pub children: Vec<Node>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    parent_id: Option<Uuid>,
    meaning_ids: Vec<Uuid>,
    context_ids: Vec<Uuid>,
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

    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Node>> {
        let datastore = get_datastore(ctx)?;

        match self.parent_id {
            Some(id) => {
                let parent = data_old::node::get_node_by_id(datastore, id)?;

                return Ok(Some(parent.into()));
            }
            None => return Ok(None),
        }
    }

    async fn context(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let context = self
            .context_ids
            .iter()
            .flat_map(|id| data_old::node::get_node_by_id(datastore, *id))
            .map(|node_with_edges| node_with_edges.into())
            .collect::<Vec<_>>();

        Ok(context)
    }

    async fn meanings(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let meanings = self
            .meaning_ids
            .iter()
            .flat_map(|id| data_old::node::get_node_by_id(datastore, *id))
            .map(|node_with_edges| node_with_edges.into())
            .collect::<Vec<_>>();

        Ok(meanings)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Node>> {
        let datastore = get_datastore(ctx)?;

        let parents = self
            .children_ids
            .iter()
            .flat_map(|id| data_old::node::get_node_by_id(datastore, *id))
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

impl From<data_old::node::NodeWithEdges> for Node {
    fn from(node_with_edges: data_old::node::NodeWithEdges) -> Node {
        let data_old::node::NodeWithEdges {
            node,
            parent,
            children,
            context,
            meanings,
        } = node_with_edges;

        let data_old::node::Node {
            id,
            name,
            content,
            created_at,
            updated_at,
            ..
        } = node;

        let parent_id = parent;
        let children_ids = children;

        Node {
            id,
            name,
            content,
            parent: None,
            context: vec![],
            meanings: vec![],
            children: vec![],
            created_at,
            updated_at,
            parent_id,
            children_ids,
            meaning_ids: meanings,
            context_ids: context,
        }
    }
}
