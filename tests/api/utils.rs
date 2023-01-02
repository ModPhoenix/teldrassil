use std::net::TcpListener;

use teldrassil::{
    data::{types::DatastoreType, utils::create_datastore},
    startup,
};

pub struct TestApp {
    pub address: String,
    pub datastore: DatastoreType,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let datastore: DatastoreType = create_datastore();

    let server = startup::run(listener, datastore.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, datastore }
}
