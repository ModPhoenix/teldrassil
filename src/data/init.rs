use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use uuid::Uuid;

use crate::data::node::query::new_node;

use super::{
    node::{
        model,
        query::{get_node},
    },
    DataError, Database, DatabaseLocal,
};

async fn create_db_in_mem() -> Result<DatabaseLocal, DataError> {
    let db = Surreal::new::<Mem>(()).await?;

    Ok(db)
}

pub async fn init_db() -> Result<Database, DataError> {
    let db = create_db_in_mem().await?;

    db.use_ns("garden").use_db("tree").await?;

    Ok(db)
}

pub async fn populate_db(db: &Database) -> Result<(), DataError> {
    let root_node_id = Uuid::nil();

    let root_node = get_node(
        db,
        model::GetNode {
            id: root_node_id.clone(),
        },
    )
    .await;

    match root_node {
        Ok(root) => {
            println!("Root node is already exist: {:?}", root);
            Ok(())
        }
        Err(_) => {
            println!("Root node is not exist, initialize datastore...");

            let root_node = model::NewNode {
                id: Some(root_node_id.clone()),
                name: "Root".to_string(),
                content: "Root content".to_string(),
                parent_id: None,
            };
            new_node(db, root_node).await?;
            println!("Root node created with id: {}", root_node_id);

            let children_node = model::NewNode {
                id: None,
                name: "Node".to_string(),
                content: "Children content".to_string(),
                parent_id: Some(root_node_id.clone()),
            };

            new_node(db, children_node).await?;

            let children_node2 = model::NewNode {
                id: None,
                name: "Node".to_string(),
                content: "Children content 2".to_string(),
                parent_id: Some(root_node_id.clone()),
            };

            new_node(db, children_node2).await?;

            Ok(())
        }
    }
}
