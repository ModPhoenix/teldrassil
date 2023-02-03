use async_graphql::{Context, Result};

use crate::{data::Database, data_old::types::DatastoreType};

pub fn get_datastore<'a>(ctx: &'a Context<'_>) -> Result<&'a DatastoreType> {
    ctx.data::<DatastoreType>()
}

pub fn get_db<'a>(ctx: &'a Context<'_>) -> Result<&'a Database> {
    ctx.data::<Database>()
}
