use assert_json_diff::assert_json_include;
use serde_json::{json, Value};
use teldrassil::data_old::utils::ROOT_NODE_ID;

use crate::{
    operations::UPDATE_NODE_MUTATION,
    utils::{spawn_app, GraphQLRequest},
};

#[tokio::test]
async fn update_node_mutation_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        UPDATE_NODE_MUTATION,
        Some(
            json!({ "id": ROOT_NODE_ID, "name": "Root updated", "content": "Root content updated" }),
        ),
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
                "updateNode": {
                    "id": ROOT_NODE_ID,
                    "name": "Root updated",
                    "content": "Root content updated",
                    "parents": [],
                    "children": []
                }
            }
        }),
    );
}
