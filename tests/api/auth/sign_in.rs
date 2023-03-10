use chrono::Local;
use serde_json::{json, Value};
use teldrassil::service::jwt::decode_jwt;

use crate::{
    operations::{SIGN_IN_MUTATION, SIGN_UP_MUTATION},
    utils::{spawn_app, GraphQLRequest},
};

const TEST_EMAIL: &str = "test@test.org";
const TEST_USERNAME: &str = "modphoenix";
const TEST_PASSWORD: &str = "ujv6uqa1uqr*rgq*MFA";

#[tokio::test]
async fn sign_in_mutation_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_UP_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "username": TEST_USERNAME, "password": TEST_PASSWORD })),
    );

    client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    let sign_in_request_body = GraphQLRequest::new(
        SIGN_IN_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "password": TEST_PASSWORD })),
    );

    // Act
    let response = client
        .post(&app.address)
        .json(&sign_in_request_body)
        .send()
        .await
        .unwrap();

    let json: Value = response.json().await.unwrap();
    println!("{:?}", json);
    let token = json["data"]["signIn"].as_str().unwrap();
    let claims = decode_jwt(token.to_string()).unwrap();

    // Assert
    // assert that the claims are valid
    assert_eq!(claims.email, TEST_EMAIL);
    assert!(claims.exp > Local::now().timestamp());
}

#[tokio::test]
async fn sign_in_mutation_fails_with_invalid_email() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_IN_MUTATION,
        Some(json!({ "email": "invalid", "password": "password" })),
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

#[tokio::test]
async fn sign_in_mutation_fails_with_invalid_password() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_UP_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "username": TEST_USERNAME, "password": "password" })),
    );

    client
        .post(&app.address)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    let sign_in_request_body = GraphQLRequest::new(
        SIGN_IN_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "password": "invalid" })),
    );

    // Act
    let response = client
        .post(&app.address)
        .json(&sign_in_request_body)
        .send()
        .await
        .unwrap();

    let json: Value = response.json().await.unwrap();
    let errors = json["errors"].as_array().unwrap();

    // Assert
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["message"], "Invalid email or password");
}

#[tokio::test]
async fn sign_in_mutation_fails_with_user_not_found() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_IN_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "password": "password" })),
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
    assert_eq!(errors[0]["message"], "Invalid email or password");
}
