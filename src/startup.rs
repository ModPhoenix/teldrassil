use std::net::TcpListener;

use anyhow::Result;

use async_graphql::{extensions::ApolloTracing, EmptySubscription, Schema};
use axum::{
    http::{self, HeaderValue},
    routing::{get, IntoMakeService},
    Extension, Router, Server,
};
use hyper::{server::conn::AddrIncoming, Method};
use tower_http::cors::CorsLayer;

use crate::{
    data::{populate_db, Database},
    graphql::{MutationRoot, QueryRoot, WorldTreeSchema},
    handlers::{graphql_handler, graphql_playground, health_check},
};

pub async fn run(
    listener: TcpListener,
    database: Database,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    populate_db(&database).await.unwrap();

    let schema: WorldTreeSchema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(database)
    .extension(ApolloTracing)
    .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health_check", get(health_check))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
        );

    println!("GraphiQL IDE: {}", listener.local_addr()?);

    let server = Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
