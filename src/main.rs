use chrono::Utc;
use teldrassil::{
    data::{
        init_db,
        node::{
            model::NewNode,
            query::{get_node_children, new_node},
        },
    },
    data_old::{types::DatastoreType, utils::create_datastore},
    startup::run,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = std::net::TcpListener::bind("127.0.0.1:4000").unwrap();
    let datastore: DatastoreType = create_datastore();
    let db = init_db().await?;

    let now = Utc::now();

    let node1 = NewNode {
        name: "Root".to_string(),
        content: "Root content".to_string(),
        created_at: now,
        updated_at: now,
        parent_id: None,
    };

    let result1 = new_node(&db, node1).await?;

    let node2 = NewNode {
        name: "test".to_string(),
        content: "content".to_string(),
        created_at: now,
        updated_at: now,
        parent_id: Some(result1.id.clone()),
    };

    let result2 = new_node(&db, node2).await?;

    dbg!(&result2);

    get_node_children(&db, result1.id).await?;

    run(listener, datastore, db)?.await?;

    Ok(())
}
