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

    async fn delete_node(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let datastore = get_datastore(ctx)?;

        let result = data::delete_node(datastore, id)?;

        Ok(result)
    }
}

#[derive(Default)]
pub struct NodeQueries;

#[derive(InputObject)]
struct NodeWhere {
    id: Option<Uuid>,
    name: Option<String>,
}

#[Object]
impl NodeQueries {
    async fn node(&self, ctx: &Context<'_>, where_: NodeWhere) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        if let Some(id) = where_.id {
            let node = data::get_node_by_id(datastore, id)?.into();

            return Ok(node);
        }

        if let Some(name) = where_.name {
            let node = data::get_node_by_name(datastore, name)?.into();

            return Ok(node);
        }

        Err(Error::new("Invalid input"))
    }
}
