use anyhow::Result;
use chrono::Utc;
use indradb::MemoryDatastore;
use uuid::Uuid;

use crate::data::{create_node, Knowledge, User};

use super::{create_not_connected_knowledge_node, get_node, types::DatastoreType};

pub fn create_datastore() -> DatastoreType {
    // let datastore = RocksdbDatastore::new(Path::new("./datastore"), None)?;
    let datastore = MemoryDatastore::default();

    datastore
}

pub fn init_datastore(datastore: &DatastoreType) -> Result<()> {
    let root_node_id = Uuid::default();

    match get_node(datastore, root_node_id) {
        Ok(root) => {
            println!("Root node is already exist: {:?}", root);
            Ok(())
        }
        Err(_) => {
            println!("Root node is not exist, initialize datastore...");

            let root_node = Knowledge {
                id: root_node_id,
                name: "Root".to_string(),
                content: "Root content".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            create_not_connected_knowledge_node(datastore, root_node)?;
            println!("Root node created with id: {}", root_node_id);

            let users_node_data = Knowledge::new("Users".to_string(), "Users content".to_string());
            let users_node = create_node(datastore, users_node_data.into(), root_node_id)?;
            println!("Users node created with id: {}", users_node.data.id());

            let admin = User {
                id: Uuid::new_v4(),
                email: "admin@localhost".to_string(),
                username: "admin".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            let admin_node = create_node(datastore, admin.into(), users_node.data.id())?;
            println!("Admin user node created with id: {}", admin_node.data.id());

            Ok(())
        }
    }
}
