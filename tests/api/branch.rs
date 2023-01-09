use serde_json::{json, Value};

use crate::{
    operations::GET_BRANCH,
    utils::{spawn_app, GraphQLRequest},
};

#[tokio::test]
async fn initialize_app_state_correct() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        GET_BRANCH,
        Some(json!({
            "id": "00000000-0000-0000-0000-000000000000"
        })),
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
    assert_eq!(
        response.json::<Value>().await.unwrap(),
        json!({
            "data": {
                "getBranch": {
                    "id": "00000000-0000-0000-0000-000000000000",
                    "name": "Root",
                    "parents": [],
                    "children": []
                }
            }
        })
    );
}
