use chrono::Local;
use serde_json::{json, Value};
use teldrassil::{data_old::get_user_by_id, service::jwt::decode_jwt};

use crate::{
    operations::SIGN_UP_MUTATION,
    utils::{spawn_app, GraphQLRequest},
};

const TEST_EMAIL: &str = "test@test.org";
const TEST_USERNAME: &str = "modphoenix";

#[tokio::test]
async fn sign_up_mutation_works() {
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

    let json: Value = response.json().await.unwrap();
    let token = json["data"]["signUp"].as_str().unwrap();
    let claims = decode_jwt(token.to_string()).unwrap();
    let user = get_user_by_id(&app.datastore, claims.sub.parse().unwrap()).unwrap();

    // Assert
    // assert that the user node is created
    assert_eq!(user.id, claims.sub.parse().unwrap());
    assert_eq!(user.email, TEST_EMAIL);
    assert_eq!(user.username, TEST_USERNAME);

    // assert that the claims are valid
    assert_eq!(claims.email, TEST_EMAIL);
    assert!(claims.exp > Local::now().timestamp());
}

#[tokio::test]
async fn sign_up_mutation_fails_with_invalid_email() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_UP_MUTATION,
        Some(json!({ "email": "invalid", "username": TEST_USERNAME, "password": "password" })),
    );

    // Act
    let response = client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    let json: Value = response.json().await.unwrap();
    let errors = json["errors"].as_array().unwrap();

    // Assert
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0]["message"],
        "Failed to parse \"String\": invalid email"
    );
}
