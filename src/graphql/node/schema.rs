use async_graphql::*;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    data::{
        self, knowledge,
        node::{create_node, get_node},
    },
    graphql::utils::get_datastore,
};

use super::Node;

#[derive(Default)]
pub struct NodeMutations;

#[Object]
impl NodeMutations {
    async fn create_node(
        &self,
        ctx: &Context<'_>,
        parent_id: Uuid,
        name: String,
        content: String,
    ) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        let id = Uuid::new_v4();

        let knowledge = data::node::NodeData::Knowledge(knowledge::Knowledge {
            id,
            name,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        let node = create_node(datastore, knowledge, parent_id)?.into();

        Ok(node)
    }
}

#[derive(Default)]
pub struct NodeQueries;

#[Object]
impl NodeQueries {
    async fn get_node(&self, ctx: &Context<'_>, id: Uuid) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        let node = get_node(datastore, id)?.into();

        Ok(node)
    }
}
