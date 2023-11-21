use server::DatabaseServer;

mod buffer;
mod connection;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = DatabaseServer::new("127.0.0.1:3444".parse().unwrap());
    server.open().await?;

    Ok(())
}
