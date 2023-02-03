use async_graphql::*;
use uuid::Uuid;

use crate::{
    graphql::get_db,
    service::{self, GetNodeInput, NewNodeInput, UpdateNodeInput},
};

use super::Node;

#[derive(Default)]
pub struct NodeMutations;

#[Object]
impl NodeMutations {
    async fn create_node(&self, ctx: &Context<'_>, input: NewNodeInput) -> Result<Node> {
        let db = get_db(ctx)?;

        let input: service::NewNode = input.try_into()?;

        let new_node = service::new_node(db, input).await?.into();

        Ok(new_node)
    }

    async fn update_node(&self, ctx: &Context<'_>, input: UpdateNodeInput) -> Result<Node> {
        let db = get_db(ctx)?;

        let input: service::UpdateNode = input.try_into()?;

        let updated_node = service::update_node(db, input).await?.into();

        Ok(updated_node)
    }

    // async fn delete_node(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
    //     let datastore = get_datastore(ctx)?;

    //     let result = data_old::delete_node(datastore, id)?;

    //     Ok(result)
    // }
}

#[derive(Default)]
pub struct NodeQueries;

#[derive(InputObject)]
struct NodeWhere {
    id: Uuid,
}

#[Object]
impl NodeQueries {
    async fn node(&self, ctx: &Context<'_>, where_: GetNodeInput) -> Result<Node> {
        let db = get_db(ctx)?;

        let where_: service::GetNode = where_.try_into()?;

        let node = service::get_node(db, where_).await?.into();

        return Ok(node);
    }
}
