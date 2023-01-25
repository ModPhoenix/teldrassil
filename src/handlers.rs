use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};
use hyper::{HeaderMap, StatusCode};

use crate::{graphql::WorldTreeSchema, service::jwt::get_claims_from_headers};

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn graphql_handler(
    Extension(schema): Extension<WorldTreeSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(claims) = get_claims_from_headers(&headers) {
        if let Ok(claims) = claims {
            req = req.data(claims);
        }
    }
    schema.execute(req).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));

    response::Html(source)
}
