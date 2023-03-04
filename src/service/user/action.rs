use crate::{
    data::{user::query, Database},
    domain::User,
    service::ServiceError,
};

use super::ask;

pub async fn new_user<T>(params: T, db: &Database) -> Result<User, ServiceError>
where
    T: TryInto<ask::NewUser>,
    T::Error: Into<ServiceError>,
{
    Ok(query::new_user(params.try_into().map_err(Into::into)?, db)
        .await?
        .try_into()?)
}

pub async fn get_user_by_id<T>(params: T, db: &Database) -> Result<User, ServiceError>
where
    T: TryInto<ask::GetUser>,
    T::Error: Into<ServiceError>,
{
    let params: ask::GetUser = params.try_into().map_err(Into::into)?;

    Ok(query::get_user_by_id(params.id.into_inner(), db)
        .await?
        .try_into()?)
}

pub async fn get_user_by_email<T>(params: T, db: &Database) -> Result<User, ServiceError>
where
    T: TryInto<ask::UserCredentials>,
    T::Error: Into<ServiceError>,
{
    let params: ask::UserCredentials = params.try_into().map_err(Into::into)?;

    Ok(query::get_user_by_email(params.email.into_inner(), db)
        .await?
        .try_into()?)
}
