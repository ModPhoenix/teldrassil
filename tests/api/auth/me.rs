use serde_json::{json, Value};
use teldrassil::{data_old::get_user_by_id, service::jwt::decode_jwt};

use crate::{
    operations::{ME_QUERY, SIGN_UP_MUTATION},
    utils::{spawn_app, GraphQLRequest},
};

const TEST_EMAIL: &str = "test@test.org";
const TEST_USERNAME: &str = "modphoenix";

#[tokio::test]
async fn me_query_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = GraphQLRequest::new(
        SIGN_UP_MUTATION,
        Some(json!({ "email": TEST_EMAIL, "username": TEST_USERNAME, "password": "password" })),
    );

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

    // Act
    let me_request_body = GraphQLRequest::new(ME_QUERY, None);
    let me_response = client
        .post(&app.address)
        .json(&me_request_body)
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    let me_json: Value = me_response.json().await.unwrap();
    let me = me_json["data"]["me"].as_object().unwrap();

    // Assert
    assert_eq!(me["id"].as_str().unwrap(), user.id.to_string());
    assert_eq!(me["email"].as_str().unwrap(), user.email);
    assert_eq!(me["username"].as_str().unwrap(), user.username);
}

#[tokio::test]
async fn me_query_fails_with_unauthorized() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let me_request_body = GraphQLRequest::new(ME_QUERY, None);
    let me_response = client
        .post(&app.address)
        .json(&me_request_body)
        .send()
        .await
        .unwrap();

    let me_json: Value = me_response.json().await.unwrap();
    let errors = me_json["errors"].as_array().unwrap();

    // Assert
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["message"], "Unauthorized");
}
