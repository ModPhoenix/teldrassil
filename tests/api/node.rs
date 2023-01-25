use assert_json_diff::assert_json_include;
use serde_json::{json, Value};

use crate::{
    operations::{CREATE_NODE_MUTATION, GET_NODE_QUERY},
    utils::{spawn_app, GraphQLRequest},
};

const ROOT_NODE_ID: &str = "00000000-0000-0000-0000-000000000000";

#[tokio::test]
async fn initialize_app_state_correct() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(GET_NODE_QUERY, Some(json!({ "id": ROOT_NODE_ID })));

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    assert_json_include!(
        actual: response.json::<Value>().await.unwrap(),
        expected: json!({
            "data": {
                "getNode": {
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
async fn create_branch_mutation_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        CREATE_NODE_MUTATION,
        Some(json!({ "parentId": ROOT_NODE_ID, "name": "Child", "content": "Child content" })),
    );

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    assert_json_include!(
        actual: response.json::<Value>().await.unwrap(),
        expected: json!({
            "data": {
                "createNode": {
                    "name": "Child",
                    "parents": [{
                        "id": ROOT_NODE_ID,
                        "name": "Root",
                    }],
                    "children": []
                }
            }
        }),
    );
}
