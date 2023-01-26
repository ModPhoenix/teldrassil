use assert_json_diff::assert_json_include;
use serde_json::{json, Value};
use teldrassil::data::utils::ROOT_NODE_ID;

use crate::{
    operations::NODE_QUERY,
    utils::{spawn_app, GraphQLRequest},
};

#[tokio::test]
async fn node_query_by_id_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body =
        GraphQLRequest::new(NODE_QUERY, Some(json!({ "where": { "id": ROOT_NODE_ID } })));

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    let json = response.json::<Value>().await.unwrap();
    assert_json_include!(
        actual: json,
        expected: json!({
            "data": {
                "node": {
                    "id": ROOT_NODE_ID,
                    "name": "Root",
                    "parents": [],
                    "children": []
                }
            }
        })
    );
}

#[tokio::test]
async fn node_query_by_name_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body =
        GraphQLRequest::new(NODE_QUERY, Some(json!({ "where": { "name": "Root" } })));

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    let json = response.json::<Value>().await.unwrap();
    assert_json_include!(
        actual: json,
        expected: json!({
            "data": {
                "node": {
                    "id": ROOT_NODE_ID,
                    "name": "Root",
                    "parents": [],
                    "children": []
                }
            }
        })
    );
}

#[tokio::test]
async fn node_query_by_name_fail() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body =
        GraphQLRequest::new(NODE_QUERY, Some(json!({ "where": { "name": "Root2" } })));

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    let json = response.json::<Value>().await.unwrap();
    println!("{:#?}", json);
    assert_json_include!(
        actual: json,
        expected: json!({
            "data": null,
            "errors": [
                {
                    "message": "Node not found",
                }
            ]
        })
    );
}
