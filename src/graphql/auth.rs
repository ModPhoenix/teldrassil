use async_graphql::*;

use crate::{
    data_old,
    graphql::get_datastore,
    service::jwt::{encode_jwt, Claims},
};

use super::user::User;

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let Claims { sub, .. } = ctx
            .data::<Claims>()
            .map_err(|_| Error::new("Unauthorized"))?;

        let datastore = get_datastore(ctx)?;

        let user = data_old::user::get_user_by_id(datastore, sub.parse()?)?;

        Ok(user.into())
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
        #[graphql(validator(min_length = 8), secret)] password: String,
    ) -> Result<String> {
        let datastore = get_datastore(ctx)?;

        let user = data_old::user::User::new(email, username, password);
        data_old::user::create_user(datastore, user.clone().into())?;

        Ok(encode_jwt(&user)?)
    }

    async fn sign_in(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        #[graphql(secret)] password: String,
    ) -> Result<String> {
        let datastore = get_datastore(ctx)?;

        let user = data_old::user::get_user_by_email(datastore, email.clone())?;
        if user.password != password {
            return Err(Error::new("Invalid email or password"));
        }

        Ok(encode_jwt(&user)?)
    }
}
