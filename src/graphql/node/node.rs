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
    fn from(node: data::node::Node) -> Node {
        match node.data {
            data::node::NodeData::Knowledge(knowledge) => Node {
                id: knowledge.id,
                name: knowledge.name.clone(),
                parents: vec![],
                children: vec![],
                created_at: knowledge.created_at,
                updated_at: knowledge.updated_at,
                parents_ids: node.parents,
                children_ids: node.children,
                data: NodeData::Knowledge(knowledge.into()),
            },
            data::node::NodeData::User(user) => Node {
                id: user.id,
                name: user.username.clone(),
                parents: vec![],
                children: vec![],
                created_at: user.created_at,
                updated_at: user.updated_at,
                parents_ids: node.parents,
                children_ids: node.children,
                data: NodeData::Knowledge(Knowledge {
                    id: user.id,
                    title: user.username,
                    content: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                }),
            },
        }
    }
}
