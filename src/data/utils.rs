use surrealdb::sql::statements::{BeginStatement, CommitStatement};

use super::{DataError, Database};

pub async fn begin_transaction(db: &Database) -> Result<(), DataError> {
    let response = db.query(BeginStatement).await?;

    dbg!(response);

    Ok(())
}

pub async fn commit_transaction(db: &Database) -> Result<(), DataError> {
    let response = db.query(CommitStatement).await?;

    dbg!(response);

    Ok(())
}
