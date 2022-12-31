use async_graphql::*;

use super::{
    auth::AuthQuery,
    branch::schema::{BranchMutations, BranchQueries},
};

#[derive(MergedObject, Default)]
pub struct MutationRoot(BranchMutations);

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthQuery, BranchQueries);

pub type WorldTreeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
