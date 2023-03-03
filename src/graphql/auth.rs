use async_graphql::*;

use crate::{
    data, domain,
    service::{
        self,
        jwt::{encode_jwt, Claims},
        NewUserInput,
    },
};

use super::{get_db, user::User};

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let Claims { sub, .. } = ctx
            .data::<Claims>()
            .map_err(|_| Error::new("Unauthorized"))?;
        let db = get_db(ctx)?;

        let params: service::GetUser = service::GetUserInput { id: sub.to_owned() }.try_into()?;

        let user = service::get_user_by_id(params, db).await?;

        Ok(user.into())
    }
}

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    async fn sign_up(&self, ctx: &Context<'_>, input: NewUserInput) -> Result<String> {
        let db = get_db(ctx)?;

        let input: service::NewUser = input.try_into()?;

        let user = service::new_user(input, db).await?.into();

        Ok(encode_jwt(&user)?)
    }

    async fn sign_in(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        #[graphql(secret)] password: String,
    ) -> Result<String> {
        let db = get_db(ctx)?;

        let user: domain::User = data::user::get_user_by_email(email, db).await?.try_into()?;

        if user.password.clone().into_inner() != password {
            return Err(Error::new("Invalid email or password"));
        }

        Ok(encode_jwt(&user)?)
    }
}
