use std::net::TcpListener;

use anyhow::Result;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    http::{self, HeaderValue},
    routing::{get, IntoMakeService},
    Extension, Router, Server,
};
use hyper::{server::conn::AddrIncoming, Method};
use tower_http::cors::CorsLayer;

use crate::{
    data::{types::DatastoreType, utils::init_datastore},
    graphql::{MutationRoot, QueryRoot, WorldTreeSchema},
    handlers::{graphql_handler, graphql_playground, health_check},
};

pub fn run(
    listener: TcpListener,
    datastore: DatastoreType,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
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
        .route("/health_check", get(health_check))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        );

    println!("GraphiQL IDE: {}", listener.local_addr()?);

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
