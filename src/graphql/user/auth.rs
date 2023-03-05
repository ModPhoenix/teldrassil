use async_graphql::*;

use crate::{
    domain,
    graphql::get_db,
    service::{
        self,
        jwt::{encode_jwt, Claims},
        SignInInput, SignUpInput,
    },
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
        let db = get_db(ctx)?;

        let input = service::GetUserInput { id: sub.to_owned() };

        let user = service::get_user_by_id(input, db).await?;

        Ok(user.into())
    }
}

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    async fn sign_up(&self, ctx: &Context<'_>, input: SignUpInput) -> Result<String> {
        let db = get_db(ctx)?;

        let user = service::new_user(input, db).await?.into();

        Ok(encode_jwt(&user)?)
    }

    async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<String> {
        let db = get_db(ctx)?;

        let user: domain::User = service::get_user_by_email(input.clone(), db)
            .await?
            .try_into()?;

        if user.password.clone().into_inner() != input.password {
            return Err(Error::new("Invalid email or password"));
        }

        Ok(encode_jwt(&user)?)
    }
}
