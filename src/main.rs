use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use graphql::WorldTreeSchema;

use crate::{
    graphql::QueryRoot,
    handlers::{graphql_handler, graphql_playground},
};

mod graphql;
mod handlers;

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(WorldTreeSchema::default())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
