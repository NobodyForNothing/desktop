use crate::server::Server;

mod server;
mod fetcher;

mod config {
    use std::time::Duration;

    pub const PORT: &str = "5678";
    pub const UPDATE_INTERVALL: Duration = Duration::from_secs(60 * 15);
    pub const FEEDS: &[&str] = &[
        "https://www.rssboard.org/files/sample-rss-2.xml",
        "https://lorem-rss.herokuapp.com/feed?unit=second&interval=30",
    ];
}

#[tokio::main]
async fn main() {
    Server::start().await;
}
