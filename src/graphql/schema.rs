use async_graphql::*;

use super::{
    auth::{AuthMutations, AuthQuery},
    node::{NodeMutations, NodeQueries},
};

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthMutations, NodeMutations);

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthQuery, NodeQueries);

pub type WorldTreeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
