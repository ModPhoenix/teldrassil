use crate::data::{node::model::NodeChildren, DataError, Database, DbId, Result};

use super::model;

const NODE: &str = "node";

pub async fn new_node<M: Into<model::NewNode>>(db: &Database, model: M) -> Result<model::Node> {
    let model: model::NewNode = model.into();

    let now = chrono::Utc::now();

    let content = model::Node {
        id: DbId::new(NODE),
        name: model.name,
        content: model.content,
        created_at: now,
        updated_at: now,
    };

    let record: model::Node = db.create(NODE).content(content).await?;

    if let Some(parent_id) = model.parent_id {
        let res = db
            .query(format!("RELATE {}->link->{}", parent_id, record.id))
            .await?;

        res.check()?;
    }

    get_node(db, record.id).await
}

pub async fn get_node<M: Into<model::GetNode>>(db: &Database, model: M) -> Result<model::Node> {
    let model: model::GetNode = model.into();

    let id = model.id.id();

    let record: Option<model::Node> = db.select((NODE, id)).await?;
    let record = record.ok_or(DataError::NotFound)?;

    Ok(model::Node {
        id: record.id,
        name: record.name,
        content: record.content,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub async fn get_node_children<M: Into<model::GetNode>>(
    db: &Database,
    model: M,
) -> Result<Vec<model::Node>> {
    let model: model::GetNode = model.into();

    dbg!(&model);

    let mut response = db
        .query(format!(
            "SELECT ->link->node.* as children FROM {}",
            model.id
        ))
        .await?;

    let data: Option<NodeChildren> = response.take(0)?;

    Ok(data.ok_or(DataError::NotFound)?.children)
}
