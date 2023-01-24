use anyhow::Result;
use chrono::Utc;
use indradb::MemoryDatastore;
use uuid::Uuid;

use crate::data::{create_node, Knowledge, User};

use super::{create_not_connected_knowledge_node, get_node, types::DatastoreType};

pub const ROOT_NODE_ID: Uuid = Uuid::nil();
pub const USERS_NODE_ID: &str = "00000000-0000-0000-0000-000000000001";
// pub const USERS_NODE_ID: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

pub fn create_datastore() -> DatastoreType {
    // let datastore = RocksdbDatastore::new(Path::new("./datastore"), None)?;
    let datastore = MemoryDatastore::default();

    datastore
}

pub fn init_datastore(datastore: &DatastoreType) -> Result<()> {
    match get_node(datastore, ROOT_NODE_ID) {
        Ok(root) => {
            println!("Root node is already exist: {:?}", root);
            Ok(())
        }
        Err(_) => {
            println!("Root node is not exist, initialize datastore...");

            let root_node = Knowledge {
                id: ROOT_NODE_ID,
                name: "Root".to_string(),
                content: "Root content".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            create_not_connected_knowledge_node(datastore, root_node)?;
            println!("Root node created with id: {}", ROOT_NODE_ID);

            let users_node_data = Knowledge::new_with_id(
                USERS_NODE_ID.parse().unwrap(),
                "Users".to_string(),
                "Users content".to_string(),
            );
            let users_node = create_node(datastore, users_node_data.into(), ROOT_NODE_ID)?;
            println!("Users node created with id: {}", users_node.data.id());

            let admin = User::new(
                "admin@localhost".to_string(),
                "admin".to_string(),
                "admin".to_string(),
            );

            let admin_node = create_node(datastore, admin.clone().into(), users_node.data.id())?;
            println!("Admin user node created with id: {}", admin_node.data.id());
            println!("Admin username: {}", admin.username);
            println!("Admin user email: {}", admin.email);
            println!("Admin user password: {}", admin.password);
            println!("Datastore initialized");

            Ok(())
        }
    }
}
