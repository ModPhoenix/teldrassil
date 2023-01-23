use async_graphql::*;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    data::{
        self,
        branch::{create_branch, get_branch},
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

        let branch = data::branch::BranchData {
            id,
            name,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let branch = create_branch(datastore, branch, parent_id)?.into();

        Ok(branch)
    }
}

#[derive(Default)]
pub struct NodeQueries;

#[Object]
impl NodeQueries {
    async fn get_node(&self, ctx: &Context<'_>, id: Uuid) -> Result<Node> {
        let datastore = get_datastore(ctx)?;

        let branch = get_branch(datastore, id)?.into();

        Ok(branch)
    }
}
