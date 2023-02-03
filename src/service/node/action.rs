use crate::{
    data::{node::query, Database},
    domain::{Node, NodeError},
    service::ServiceError,
};

use super::ask;

pub async fn new_node<T: Into<ask::NewNode>>(
    db: &Database,
    params: T,
) -> Result<Node, ServiceError> {
    Ok(query::new_node(db, params.into()).await?.try_into()?)
}

pub async fn get_node<T: Into<ask::GetNode>>(
    db: &Database,
    params: T,
) -> Result<Node, ServiceError> {
    Ok(query::get_node(db, params.into()).await?.try_into()?)
}

pub async fn get_node_children<T: Into<ask::GetNode>>(
    db: &Database,
    params: T,
) -> Result<Vec<Node>, ServiceError> {
    Ok(query::get_node_children(db, params.into())
        .await?
        .into_iter()
        .map(|node| node.try_into())
        .collect::<Result<Vec<Node>, NodeError>>()?)
}

pub async fn get_node_parent<T: Into<ask::GetNode>>(
    db: &Database,
    params: T,
) -> Result<Option<Node>, ServiceError> {
    Ok(query::get_node_parent(db, params.into())
        .await?
        .map(|node| node.try_into())
        .transpose()?)
}

pub async fn get_node_meanings<T: Into<ask::GetNodeMeanings>>(
    db: &Database,
    params: T,
) -> Result<Vec<Node>, ServiceError> {
    Ok(query::get_node_meanings(db, params.into())
        .await?
        .into_iter()
        .map(|node| node.try_into())
        .collect::<Result<Vec<Node>, NodeError>>()?)
}

pub async fn get_node_context<T: Into<ask::GetNode>>(
    db: &Database,
    params: T,
) -> Result<Vec<Node>, ServiceError> {
    Ok(query::get_node_context(db, params.into())
        .await?
        .into_iter()
        .map(|node| node.try_into())
        .collect::<Result<Vec<Node>, NodeError>>()?)
}
