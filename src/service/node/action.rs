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

pub async fn update_node<T: Into<ask::UpdateNode>>(
    db: &Database,
    params: T,
) -> Result<Node, ServiceError> {
    Ok(query::update_node(db, params.into()).await?.try_into()?)
}

pub async fn delete_node<T: Into<ask::DeleteNode>>(
    db: &Database,
    params: T,
) -> Result<bool, ServiceError> {
    Ok(query::delete_node(db, params.into()).await?)
}

pub async fn get_node_children<T>(params: T, db: &Database) -> Result<Vec<Node>, ServiceError>
where
    T: TryInto<ask::GetNodeChildren>,
    T::Error: Into<ServiceError>,
{
    let params: ask::GetNodeChildren = params.try_into().map_err(Into::into)?;

    Ok(query::get_node_children(db, params)
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
