use std::sync::{Arc};
use colored::Colorize;
use log::{error, info};
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
        let updater = task::spawn(async move {
            let mut timer = time::interval(config::UPDATE_INTERVALL);
            timer.tick().await;
            loop {
                let data = fetcher::fetch_all(config::FEEDS).await;
                if let Some(data) = fast_rss_data::encode(&data, true) {
                    data_update.lock().await.data = data;
                    info!("Feed data updated successfully.")
                } else {
                    error!("Failed encoding feed data.")
                }

                timer.tick().await;
            }
        });
        let data_serve = data.clone();
        let server = task::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", config::PORT)).await.ok().unwrap();
            loop {
                if let Ok((mut soc, addr)) = listener.accept().await {
                    info!("Request from: '{}'.", addr);
                    let data = data_serve.lock().await;
                    let res = soc.write_all(&data.data.as_slice()).await;
                    // TODO: auth
                    if let Some(e) = res.err() {
                        eprintln!("{e}");
                    }
                }
            }
        });

        info!("Fast RSS server {}!", "started".green());
        server.await.unwrap();
        updater.abort_handle().abort();
        info!("Fast RSS server {}!", "stopped".red());

    }

}
