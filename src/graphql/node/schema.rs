use async_graphql::*;
use uuid::Uuid;

use crate::{data_old, graphql::utils::get_datastore};

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
        let node = data_old::Node::new_with_id(id, name, content);
        let node = data_old::create_node_with_parent(datastore, node, parent_id)?.into();

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

        let new_node = data_old::Node::new_with_id(id, name, content);
        let node = data_old::update_node(datastore, new_node)?.into();

        Ok(node)
    }

    async fn delete_node(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let datastore = get_datastore(ctx)?;

        let result = data_old::delete_node(datastore, id)?;

        Ok(result)
    }
}

#[derive(Default)]
pub struct NodeQueries;

#[derive(InputObject)]
struct NodeWhere {
    id: Uuid,
}

#[Object]
impl NodeQueries {
    async fn node(&self, ctx: &Context<'_>, where_: NodeWhere) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        let node = data_old::get_node_by_id(datastore, where_.id)?.into();

        return Ok(node);
    }
}
