use teldrassil::{data::init_db, startup::run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = std::net::TcpListener::bind("127.0.0.1:4000").unwrap();
    let db = init_db().await?;

    run(listener, db).await?.await?;

    Ok(())
}
