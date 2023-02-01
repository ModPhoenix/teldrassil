use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use super::{Database, DatabaseLocal};

async fn new_db_in_mem() -> Result<DatabaseLocal, surrealdb::Error> {
    let db = Surreal::new::<Mem>(()).await?;

    Ok(db)
}

pub async fn init_db() -> Result<Database, surrealdb::Error> {
    let db = new_db_in_mem().await?;

    db.use_ns("garden").use_db("tree").await?;

    Ok(db)
}
