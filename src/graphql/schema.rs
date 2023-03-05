use async_graphql::*;

use super::{
    node::{NodeMutations, NodeQueries},
    user::{AuthMutations, AuthQuery},
};

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthMutations, NodeMutations);

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthQuery, NodeQueries);

pub type WorldTreeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
