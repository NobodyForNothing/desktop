use crate::server::Server;

mod server;
mod fetcher;

#[tokio::main]
async fn main() {
    Server::start().await;
}
