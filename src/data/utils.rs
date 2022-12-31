use anyhow::Result;
use chrono::Utc;
use indradb::MemoryDatastore;
use uuid::Uuid;

use crate::data::branch::BranchData;

use super::{
    branch::{create_not_connected_branch, get_branch},
    types::DatastoreType,
};

pub fn create_datastore() -> DatastoreType {
    // let datastore = RocksdbDatastore::new(Path::new("./datastore"), None)?;
    let datastore = MemoryDatastore::default();

    datastore
}

pub fn init_datastore(datastore: &DatastoreType) -> Result<()> {
    let id = Uuid::default();

    match get_branch(datastore, id) {
        Ok(root) => {
            println!("Root branch is already exist: {:?}", root);
            Ok(())
        }
        Err(_) => {
            println!("Root branch is not exist, creating root branch...");

            let branch = BranchData {
                id,
                name: "Root".to_string(),
                content: "Root content".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            create_not_connected_branch(datastore, branch)?;

            println!("Root branch created with id: {}", id);
            Ok(())
        }
    }
}
