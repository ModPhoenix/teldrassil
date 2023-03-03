use async_graphql::{Context, Result};

use crate::data::Database;

pub fn get_db<'a>(ctx: &'a Context<'_>) -> Result<&'a Database> {
    ctx.data::<Database>()
}
