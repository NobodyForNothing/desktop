use std::sync::{Arc};
use tokio::sync::{Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::{task, time};
use crate::{config, fetcher};

pub struct Server {
    data: Vec<u8>,
}

impl Server {
    pub async fn start() {
        let data = Server {
            data: Vec::new(),
        };
        let data = Mutex::new(data);
        let data = Arc::new(data);

        let data_update = data.clone();
        task::spawn(async move {
            let mut timer = time::interval(config::UPDATE_INTERVALL);
            loop {
                let data = fetcher::fetch_all(config::FEEDS).await;
                let data = fast_rss_data::encode(&data, true).unwrap();
                data_update.lock().await.data = data;
                timer.tick().await;
            }
        });
        let data_serve = data.clone();
        task::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", config::PORT)).await.ok().unwrap();
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
