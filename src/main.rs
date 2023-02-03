use teldrassil::{
    data::init_db,
    data_old::{types::DatastoreType, utils::create_datastore},
    startup::run,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = std::net::TcpListener::bind("127.0.0.1:4000").unwrap();
    let datastore: DatastoreType = create_datastore();
    let db = init_db().await?;

    run(listener, datastore, db).await?.await?;

    Ok(())
}
