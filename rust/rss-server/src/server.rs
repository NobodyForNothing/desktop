use std::sync::{Arc};
use tokio::sync::{Mutex};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::{task, time};
use crate::fetcher;
use crate::formatter::format;

pub struct Server {
    data: Vec<u8>,
}

impl Server {
    pub async fn start() {
        let data = Server {
            data: zstd::bulk::compress(b"loading...", 19).unwrap(),
        };
        let data = Mutex::new(data);
        let data = Arc::new(data);

        let data_update = data.clone();
        task::spawn(async move {
            let mut timer = time::interval(Duration::from_secs(60 * 15));
            loop {
                if let Some(data) = fetcher::fetch("https://www.rssboard.org/files/sample-rss-2.xml").await {
                    data_update.lock().await.data = format(data);
                }
                timer.tick().await;
            }
        });
        let data_serve = data.clone();
        task::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:5678").await.ok().unwrap();
            loop {
                if let Ok((mut soc, addr)) = listener.accept().await {
                    println!("New request from: {}", addr);
                    let data = data_serve.lock().await;
                    let res = soc.write_all(&data.data.as_slice()).await;
                    // TODO: auth
                    if let Some(e) = res.err() {
                        eprintln!("{e}");
                    }
                }
            }
        }).await.unwrap();

    }

}
