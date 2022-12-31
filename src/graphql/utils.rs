use async_graphql::{Context, Result};

use crate::data::types::DatastoreType;

pub fn get_datastore<'a>(ctx: &'a Context<'_>) -> Result<&'a DatastoreType> {
    ctx.data::<DatastoreType>()
}
