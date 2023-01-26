use anyhow::Result;
use chrono::Utc;
use indradb::{Datastore, Identifier, MemoryDatastore};
use uuid::Uuid;

use crate::data::{
    create_user, node::create_node, node_name_identifier, user_email_identifier, User,
};

use super::{
    node::{get_node_by_id, Node},
    types::DatastoreType,
};

pub fn identifier<S: Into<String>>(s: S) -> Identifier {
    Identifier::new(s).unwrap()
}

pub const ROOT_NODE_ID: Uuid = Uuid::nil();

pub fn create_datastore() -> DatastoreType {
    // let datastore = RocksdbDatastore::new(Path::new("./datastore"), None)?;
    let datastore = MemoryDatastore::default();

    datastore
}

pub fn init_datastore(datastore: &DatastoreType) -> Result<()> {
    match get_node_by_id(datastore, ROOT_NODE_ID) {
        Ok(root) => {
            println!("Root node is already exist: {:?}", root);
            Ok(())
        }
        Err(_) => {
            println!("Root node is not exist, initialize datastore...");

            let root_node = Node {
                id: ROOT_NODE_ID,
                name: "Root".to_string(),
                content: "Root content".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            create_node(datastore, root_node)?;
            println!("Root node created with id: {}", ROOT_NODE_ID);

            let admin = User::new(
                "admin@localhost".to_string(),
                "admin".to_string(),
                "admin".to_string(),
            );

            create_user(datastore, admin.clone().into())?;
            println!("Admin user node created with id: {}", admin.id);
            println!("Admin username: {}", admin.username);
            println!("Admin user email: {}", admin.email);
            println!("Admin user password: {}", admin.password);

            datastore.index_property(user_email_identifier())?;
            datastore.index_property(node_name_identifier())?;

            println!("Datastore initialized");

            Ok(())
        }
    }
}
