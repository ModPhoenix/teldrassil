use crate::{
    data::{user::query, Database},
    domain::User,
    service::ServiceError,
};

use super::ask;

pub async fn new_user<T: Into<ask::NewUser>>(
    params: T,
    db: &Database,
) -> Result<User, ServiceError> {
    Ok(query::new_user(params.into(), db).await?.try_into()?)
}

pub async fn get_user_by_id<T: Into<ask::GetUser>>(
    params: T,
    db: &Database,
) -> Result<User, ServiceError> {
    let params: ask::GetUser = params.into();

    Ok(query::get_user_by_id(params.id.into_inner(), db)
        .await?
        .try_into()?)
}
