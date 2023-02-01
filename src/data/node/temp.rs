use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::statements::{BeginStatement, CommitStatement};
use uuid::Uuid;

use crate::surreal::Database;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Node {
    pub fn new(name: String, content: String) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        Self {
            id: id.clone(),
            name,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_with_id(id: String, name: String, content: String) -> Self {
        let now = Utc::now();

        Self {
            id: id.clone(),
            name,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeWithEdges {
    pub node: Node,
    pub parent: Option<Uuid>,
    pub context: Vec<Uuid>,
    pub meanings: Vec<Uuid>,
    pub children: Vec<Uuid>,
}

impl NodeWithEdges {
    pub fn new(
        node: Node,
        parent: Option<Uuid>,
        context: Vec<Uuid>,
        meanings: Vec<Uuid>,
        children: Vec<Uuid>,
    ) -> Self {
        Self {
            node,
            parent,
            context,
            meanings,
            children,
        }
    }
}

const NODE_TABLE_NAME: &str = "node";

pub fn create_node_sql() -> String {
    format!(
        "CREATE {NODE_TABLE_NAME}
        SET name = $name,
            content = $content,
            created_at = $created_at,
            updated_at = $updated_at,
            parent = $parent",
    )
}

pub async fn new_node(db: &Database, data: Node) -> Result<Node> {
    let record: Node = db.create(NODE_TABLE_NAME).content(data).await?;

    Ok(record)
}

pub async fn create_node_with_parent(db: &Database, data: Node, parent_id: String) -> Result<()> {
    // let vertex = create_node(datastore, data)?;

    // get_node_by_id(datastore, parent_id.clone())
    //     .map_err(|_| anyhow::anyhow!("Parent node not found"))?;

    // let k = EdgeKey::new(parent_id, node_edge_identifier(), vertex.id);
    // datastore.create_edge(&k)?;

    // let node_with_edges = get_node_by_id(datastore, vertex.id)?;

    // Ok(node_with_edges)

    // ________________

    // let sql = sql! {
    //     CREATE user
    //     SET name = $name,
    //         company = $company
    // };

    // let mut results = db
    //     .query(sql)
    //     .bind(User {
    //         id: "john".to_owned(),
    //         name: "John Doe".to_owned(),
    //         company: "ACME Corporation".to_owned(),
    //     })
    //     .await?;

    // // print the created user:
    // let user: Option<User> = results.take(0)?;
    // println!("{user:?}");

    // let mut response = db
    //     .query(sql!(SELECT * FROM user WHERE name.first = "John"))
    //     .await?;

    // // print all users:
    // let users: Vec<User> = response.take(0)?;
    // println!("{users:?}");

    begin_transaction(db).await?;

    let new_node = create_node(db, data.clone()).await?;

    let mut response = db
        // Start transaction
        .query(BeginStatement)
        .query(create_node_sql())
        .bind(data.clone())
        .query(format!(
            "RELATE node:`{}`->link->node:`{}`",
            data.id, parent_id
        ))
        // Commit transaction
        .query(CommitStatement)
        .await?;

    let node: Option<Node> = response.take(0)?;
    println!("{node:?}");

    // let record: Node = db.create(NODE_TABLE_NAME).content(data).await?;

    Ok(())
}
