use async_graphql::*;

use super::{
    auth::AuthQuery,
    node::schema::{NodeMutations, NodeQueries},
};

#[derive(MergedObject, Default)]
pub struct MutationRoot(NodeMutations);

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthQuery, NodeQueries);

pub type WorldTreeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
