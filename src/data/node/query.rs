use crate::data::{node::model::NodeChildren, DataError, Database, DbId, Result};

use super::model::{self, GetNodeMeanings};

pub const NODE_TABLE: &str = "node";

pub async fn new_node<M: Into<model::NewNode>>(db: &Database, model: M) -> Result<model::Node> {
    let model: model::NewNode = model.into();

    let now = chrono::Utc::now();

    let content = model::Node {
        id: model
            .id
            .map(|id| DbId::from_uuid(NODE_TABLE, id))
            .unwrap_or(DbId::new(NODE_TABLE)),
        name: model.name,
        content: model.content,
        created_at: now,
        updated_at: now,
    };

    let record: model::Node = db.create(NODE_TABLE).content(content).await?;

    if let Some(parent_id) = model.parent_id {
        let res = db
            .query(format!(
                "RELATE {}->link->{}",
                DbId::from_uuid(NODE_TABLE, parent_id),
                record.id
            ))
            .await?;

        res.check()?;
    }

    get_node(db, record.id.uuid()).await
}

pub async fn get_node<M: Into<model::GetNode>>(db: &Database, model: M) -> Result<model::Node> {
    let model: model::GetNode = model.into();

    let id = model.id;

    let record: Option<model::Node> = db.select((NODE_TABLE, id.to_string())).await?;
    let record = record.ok_or(DataError::NotFound)?;

    Ok(model::Node {
        id: record.id,
        name: record.name,
        content: record.content,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub async fn update_node<M: Into<model::UpdateNode>>(
    db: &Database,
    model: M,
) -> Result<model::Node> {
    let model: model::UpdateNode = model.into();

    let id = model.id;

    let record: Option<model::Node> = db.select((NODE_TABLE, id.to_string())).await?;
    let mut record = record.ok_or(DataError::NotFound)?;

    record.name = model.name.unwrap_or(record.name);
    record.content = model.content.unwrap_or(record.content);
    record.updated_at = chrono::Utc::now();

    let record: model::Node = db
        .update((NODE_TABLE, id.to_string()))
        .content(record)
        .await?;

    Ok(record)
}

pub async fn delete_node<M: Into<model::DeleteNode>>(db: &Database, model: M) -> Result<bool> {
    let model: model::DeleteNode = model.into();

    let id = model.id;

    let record: Option<model::Node> = db.select((NODE_TABLE, id.to_string())).await?;
    record.ok_or(DataError::NotFound)?;

    db.delete((NODE_TABLE, id.to_string())).await?;

    Ok(true)
}

pub async fn get_node_children<M: Into<model::GetNode>>(
    db: &Database,
    model: M,
) -> Result<Vec<model::Node>> {
    let model: model::GetNode = model.into();

    let mut response = db
        .query(format!(
            "SELECT ->link->{}.* as children FROM {}",
            NODE_TABLE,
            DbId::from_uuid(NODE_TABLE, model.id)
        ))
        .await?;

    let data: Option<NodeChildren> = response.take(0)?;

    Ok(data.ok_or(DataError::NotFound)?.children)
}

pub async fn get_node_parent<M: Into<model::GetNode>>(
    db: &Database,
    model: M,
) -> Result<Option<model::Node>> {
    let model: model::GetNode = model.into();

    let mut response = db
        .query(format!(
            "SELECT <-link<-{}.* as parent FROM {}",
            NODE_TABLE,
            DbId::from_uuid(NODE_TABLE, model.id)
        ))
        .await?;

    let data: Option<model::NodeParent> = response.take(0)?;
    let data = data.ok_or(DataError::NotFound)?;
    let data = data.parent.first().cloned();

    Ok(data)
}

pub async fn get_node_meanings<M: Into<model::GetNodeMeanings>>(
    db: &Database,
    model: M,
) -> Result<Vec<model::Node>> {
    let model: model::GetNodeMeanings = model.into();

    let mut response = db
        .query(format!(
            "SELECT * FROM {} WHERE name = '{}' AND id != '{}'",
            NODE_TABLE,
            model.name,
            DbId::from_uuid(NODE_TABLE, model.id)
        ))
        .await?;

    let data: Vec<model::Node> = response.take(0)?;

    Ok(data)
}

pub async fn get_node_context<M: Into<model::GetNode>>(
    db: &Database,
    model: M,
) -> Result<Vec<model::Node>> {
    let model: model::GetNode = model.into();

    let mut context = vec![];

    let mut model = model;

    loop {
        let parent = get_node_parent(db, model).await?;

        if let Some(parent) = parent {
            let meanings = get_node_meanings(
                db,
                GetNodeMeanings {
                    id: parent.id.clone().uuid(),
                    name: parent.name.clone(),
                },
            )
            .await?;

            context.push(parent.clone());

            if meanings.len() > 0 {
                model = model::GetNode {
                    id: parent.id.uuid(),
                };
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(context)
}
