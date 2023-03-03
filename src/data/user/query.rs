use uuid::Uuid;

use super::model;
use crate::data::{DataError, Database, DbId, Result};

pub const USER_TABLE: &str = "user";

pub async fn new_user<M: Into<model::NewUser>>(model: M, db: &Database) -> Result<model::User> {
    let model: model::NewUser = model.into();

    let now = chrono::Utc::now();

    let content = model::User {
        id: DbId::new(USER_TABLE),
        email: model.email,
        username: model.username,
        password: model.password,
        created_at: now,
        updated_at: now,
    };

    let record: model::User = db.create(USER_TABLE).content(content).await?;

    get_user_by_id(record.id.uuid(), db).await
}

pub async fn get_user_by_id(id: Uuid, db: &Database) -> Result<model::User> {
    let record: Option<model::User> = db.select((USER_TABLE, id.to_string())).await?;

    Ok(record.ok_or(DataError::NotFound)?)
}

pub async fn get_user_by_email(email: String, db: &Database) -> Result<model::User> {
    let mut response = db
        .query(format!(
            "SELECT * FROM {} WHERE email = '{}'",
            USER_TABLE, email
        ))
        .await?;

    let user: Option<model::User> = response.take(0)?;

    Ok(user.ok_or(DataError::NotFound)?)
}
