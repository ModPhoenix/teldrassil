use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use data::types::DatastoreType;
use graphql::WorldTreeSchema;

use crate::{
    data::utils::{create_datastore, init_datastore},
    graphql::{MutationRoot, QueryRoot},
    handlers::{graphql_handler, graphql_playground},
};

mod data;
mod graphql;
mod handlers;

#[tokio::main]
async fn main() {
    let datastore: DatastoreType = create_datastore();

    init_datastore(&datastore).unwrap();

    let schema: WorldTreeSchema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(datastore)
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
