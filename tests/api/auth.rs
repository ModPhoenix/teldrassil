use chrono::Local;
use serde_json::{json, Value};
use teldrassil::{data::get_node, service::jwt::decode_jwt};

use crate::{
    operations::{SIGN_IN_MUTATION, SIGN_UP_MUTATION},
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
    let user_node = get_node(&app.datastore, claims.sub.parse().unwrap()).unwrap();

    // Assert
    // assert that the user node is created
    assert_eq!(claims.sub, user_node.data.id().to_string());
    match user_node.data {
        teldrassil::data::NodeData::User(user) => {
            assert_eq!(user.id, claims.sub.parse().unwrap());
            assert_eq!(user.email, TEST_EMAIL);
            assert_eq!(user.username, TEST_USERNAME);
        }
        _ => panic!("User node is not a user"),
    }

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

#[tokio::test]
async fn sign_in_mutation_works() {
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
        Some(json!({ "email": TEST_EMAIL, "password": "password" })),
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
async fn sign_in_mutation_user_not_found() {
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
