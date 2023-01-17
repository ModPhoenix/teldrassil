use assert_json_diff::assert_json_include;
use serde_json::{json, Value};

use crate::{
    operations::{CREATE_BRANCH_MUTATION, GET_BRANCH_QUERY},
    utils::{spawn_app, GraphQLRequest},
};

const ROOT_BRANCH_ID: &str = "00000000-0000-0000-0000-000000000000";

#[tokio::test]
async fn initialize_app_state_correct() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(GET_BRANCH_QUERY, Some(json!({ "id": ROOT_BRANCH_ID })));

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    assert_eq!(
        response.json::<Value>().await.unwrap(),
        json!({
            "data": {
                "getBranch": {
                    "id": ROOT_BRANCH_ID,
                    "name": "Root",
                    "content": "Root content",
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
        CREATE_BRANCH_MUTATION,
        Some(json!({ "parentId": ROOT_BRANCH_ID, "name": "Child", "content": "Child content" })),
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
                "createBranch": {
                    "name": "Child",
                    "content": "Child content",
                    "parents": [{
                        "id": ROOT_BRANCH_ID,
                        "name": "Root",
                    }],
                    "children": []
                }
            }
        }),
    );
}
