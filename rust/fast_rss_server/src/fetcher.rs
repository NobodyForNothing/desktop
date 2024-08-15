use log::{debug, warn};
use reqwest::IntoUrl;
use roxmltree::{Document, Node};
use fast_rss_data::{Channel, Item, RssSummary};

pub async fn fetch_all(urls: &[&str]) -> RssSummary {
    let mut channels = Vec::new();
    for url in urls {
        if let Some(mut data) = fetch(url.to_string()).await {
            channels.append(&mut data)
        } else {
            warn!("Couldn't fetch channels from feed: '{}'.", &url);
        }
    }
    RssSummary { data: channels }
}

pub async fn fetch<T: IntoUrl>(url: T) -> Option<Vec<Channel>> {
    let url = url.into_url().ok()?;
    let res = reqwest::get(url.clone());
    let res = res.await.ok()?;
    if res.status().as_u16() == 200 {
        let res = res.text().await.ok()?;
        let res = Document::parse(res.as_str()).ok()?;
        let node = res.root().first_child()?;
        if node.tag_name().name() == "rss" {
            match node.attribute("version")? {
                "2.0" => {
                    let data = parse_rss_v2(node);
                    Some(data)
                }
                _ => {
                    debug!("Non v2.0 RSS feed: '{url}'");
                    None
                }
            }
        } else {
            debug!("Non RSS feed: '{url}'");
            None
        }
    } else {
        debug!("Cant connect to: '{url}'");
        None
    }
}

/// Parse rss of version=2.0.
fn parse_rss_v2(rss: Node) -> Vec<Channel> {
    let mut channels: Vec<Channel> = Vec::new();
    for channel_node in rss.children() {
        if channel_node.tag_name().name() == "channel" {
            let mut channel = Channel::default();
            for node in channel_node.children() {
                match node.tag_name().name() {
                    "title" => {
                        if let Some(text) = node.text() {
                            channel.title = Some(text.to_string());
                        }
                    }
                    "link" => {
                        if let Some(text) = node.text() {
                            channel.link = Some(text.to_string());
                        }
                    }
                    "description" => {
                        if let Some(text) = node.text() {
                            channel.description = Some(text.to_string());
                        }
                    }
                    "language" => {
                        if let Some(text) = node.text() {
                            channel.language = Some(text.to_string());
                        }
                    }
                    "item" => {
                        let mut item = Item::default();
                        for node in node.children() {
                            match node.tag_name().name() {
                                "title" => {
                                    if let Some(text) = node.text() {
                                        item.title = Some(text.to_string());
                                    }
                                }
                                "link" => {
                                    if let Some(text) = node.text() {
                                        item.link = Some(text.to_string());
                                    }
                                }
                                "description" => {
                                    if let Some(text) = node.text() {
                                        item.description = Some(text.to_string());
                                    }
                                }
                                "guid" => {
                                    if let Some(text) = node.text() {
                                        item.guid = Some(text.to_string());
                                    }
                                },
                                _ => {}
                            }
                        }
                        channel.items.push(item);
                    }
                    _ => {}
                }
            }
            channels.push(channel);
        }
    }
    channels
}