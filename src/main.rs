use teldrassil::{
    data::{types::DatastoreType, utils::create_datastore},
    startup::run,
};

#[tokio::main]
async fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000").unwrap();
    let datastore: DatastoreType = create_datastore();

    run(listener, datastore).unwrap().await.unwrap();
}
