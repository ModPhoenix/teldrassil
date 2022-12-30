use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use self::auth::AuthQuery;

mod auth;

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthQuery);

pub type WorldTreeSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
