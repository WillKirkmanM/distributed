use crate::network::Server;

pub mod command;
pub mod network;
pub mod store;
pub mod transaction;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:8080").await;
    server.run().await;
}

