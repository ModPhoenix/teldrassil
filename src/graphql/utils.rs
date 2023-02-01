use async_graphql::{Context, Result};

use crate::data_old::types::DatastoreType;

pub fn get_datastore<'a>(ctx: &'a Context<'_>) -> Result<&'a DatastoreType> {
    ctx.data::<DatastoreType>()
}
