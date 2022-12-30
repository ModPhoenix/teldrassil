use async_graphql::Object;

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn me(&self) -> String {
        "me".to_string()
    }
}
