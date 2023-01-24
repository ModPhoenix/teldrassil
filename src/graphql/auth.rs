use async_graphql::*;
use uuid::Uuid;

use crate::{
    data::{self, USERS_NODE_ID},
    graphql::get_datastore,
};

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn me(&self) -> String {
        "me".to_string()
    }
}

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    async fn sign_up(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        username: String,
        #[graphql(validator(min_length = 8))] password: String,
    ) -> Result<String> {
        let datastore = get_datastore(ctx)?;

        let users_node_id = Uuid::parse_str(USERS_NODE_ID)?;

        let user = data::User::new(email, username, password);

        let _node = data::create_node(datastore, user.into(), users_node_id)?;

        // Ok(node)

        // Ok(encode_jwt(&user))
        Ok("".to_string())
    }
}
