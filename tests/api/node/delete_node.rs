use assert_json_diff::assert_json_include;
use serde_json::{json, Value};
use teldrassil::data::utils::ROOT_NODE_ID;

use crate::{
    operations::{CREATE_NODE_MUTATION, DELETE_NODE_MUTATION, NODE_QUERY},
    utils::{spawn_app, GraphQLRequest},
};

#[tokio::test]
async fn delete_node_mutation_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Create a node to delete
    let create_node_request_body = GraphQLRequest::new(
        CREATE_NODE_MUTATION,
        Some(
            json!({ "parentId": ROOT_NODE_ID, "name": "Node to delete", "content": "Node to delete content" }),
        ),
    );
    let create_node_response = client
        .post(&app.address)
        .json(&create_node_request_body)
        .send()
        .await
        .unwrap();
    let create_node_response_body = create_node_response.json::<Value>().await.unwrap();

    let node_id = create_node_response_body["data"]["createNode"]["id"]
        .as_str()
        .unwrap();
    let request_body = GraphQLRequest::new(DELETE_NODE_MUTATION, Some(json!({ "id": node_id })));

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // check that the node is deleted
    let get_node_request_body = GraphQLRequest::new(NODE_QUERY, Some(json!({ "id": node_id })));
    let get_node_response = client
        .post(&app.address)
        .json(&get_node_request_body)
        .send()
        .await
        .unwrap();
    let get_node_response_body = get_node_response.json::<Value>().await.unwrap();

    // Assert
    assert_eq!(get_node_response_body["data"]["getNode"], Value::Null);
    assert!(response.status().is_success());
    assert_json_include!(
        actual: response.json::<Value>().await.unwrap(),
        expected: json!({
            "data": {
                "deleteNode": true
            }
        }),
    );
}

#[tokio::test]
async fn delete_node_mutation_fails_if_node_does_not_exist() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        DELETE_NODE_MUTATION,
        Some(json!({ "id": "10000000-0000-0000-0000-000000000000" })),
    );

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), 200);
    assert_json_include!(
        actual: response.json::<Value>().await.unwrap(),
        expected: json!({
            "data": {
                "deleteNode": false
            }
        }),
    );
}
