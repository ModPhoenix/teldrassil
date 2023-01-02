use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};
use hyper::StatusCode;

use crate::graphql::WorldTreeSchema;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn graphql_handler(
    schema: Extension<WorldTreeSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));

    response::Html(source)
}
