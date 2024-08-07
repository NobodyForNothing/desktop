use crate::server::Server;

mod channel;
mod server;
mod fetcher;
mod formatter;

#[tokio::main]
async fn main() {
    Server::start().await;
}
