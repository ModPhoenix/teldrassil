use async_graphql::*;
use uuid::Uuid;

use crate::{data, graphql::utils::get_datastore};

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
        let node = data::Node::new_with_id(id, name, content);
        let node = data::create_node_with_parent(datastore, node, parent_id)?.into();

        Ok(node)
    }

    async fn update_node(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: String,
        content: String,
    ) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        let new_node = data::Node::new_with_id(id, name, content);
        let node = data::update_node(datastore, new_node)?.into();

        Ok(node)
    }
}

#[derive(Default)]
pub struct NodeQueries;

#[Object]
impl NodeQueries {
    async fn get_node(&self, ctx: &Context<'_>, id: Uuid) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        let node = data::get_node_by_id(datastore, id)?.into();

        Ok(node)
    }
}
