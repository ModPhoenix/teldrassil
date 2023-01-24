use assert_json_diff::assert_json_include;
use serde_json::{json, Value};

use crate::{
    operations::SIGN_UP_MUTATION,
    utils::{spawn_app, GraphQLRequest},
};

const TEST_EMAIL: &str = "test@test.org";
const TEST_USERNAME: &str = "modphoenix";

#[tokio::test]
async fn create_branch_mutation_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_UP_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "username": TEST_USERNAME, "password": "password" })),
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
                "signUp": {
                    "email": TEST_EMAIL,
                    "username": TEST_USERNAME,
                }
            }
        }),
    );
}
