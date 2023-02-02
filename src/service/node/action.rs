use crate::{
    data::{node::query, Database},
    domain::Node,
    service::ServiceError,
};

use super::ask;

/// Creates a new [`Clip`].
pub async fn new_clip(db: &Database, params: ask::NewNode) -> Result<Node, ServiceError> {
    Ok(query::new_node(db, params).await?.try_into()?)
}
