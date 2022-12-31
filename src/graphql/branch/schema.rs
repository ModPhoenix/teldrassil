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

use super::branch::Branch;

#[derive(Default)]
pub struct BranchMutations;

#[Object]
impl BranchMutations {
    async fn create_branch(&self, ctx: &Context<'_>) -> Result<Branch> {
        let datastore = get_datastore(ctx)?;

        let id = Uuid::new_v4();

        let branch = data::branch::Branch {
            id,
            name: "Test".to_string(),
            content: "Test content".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let branch = create_branch(datastore, branch)?.into();

        Ok(branch)
    }
}

#[derive(Default)]
pub struct BranchQueries;

#[Object]
impl BranchQueries {
    async fn get_branch(&self, ctx: &Context<'_>, id: Uuid) -> Result<Branch> {
        let datastore = get_datastore(ctx)?;

        let branch = get_branch(datastore, id)?.into();

        Ok(branch)
    }
}
