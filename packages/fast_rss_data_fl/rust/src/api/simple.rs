// coppied from /rust/fast_rss_data
// TODO: use as dependency so that code may be update automatically
use bitcode::{Decode, Encode};


#[flutter_rust_bridge::frb(sync)]
pub fn encode(data: &RssSummary, compress: bool) -> Option<Vec<u8>> {
    let serialised = bitcode::encode(data);
    if compress {
        zstd::encode_all(serialised.as_slice(), 19).ok()
    } else {
        Some(serialised)
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn decode(data: &[u8], decompress: bool) -> Option<RssSummary> {
    if decompress {
        let uncompressed = zstd::decode_all(data).ok()?;
        bitcode::decode(uncompressed.as_slice()).ok()
    } else {
        bitcode::decode(data).ok()
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RssSummary {
    pub data: Vec<Channel>,
}

/// RSS channel.
#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Channel {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Item {
    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub guid: Option<String>,
}

impl Default for Channel {
    fn default() -> Self {
        Channel {
            title: None,
            link: None,
            description: None,
            language: None,
            items: Vec::new(),
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Item {
            title: None,
            link: None,
            description: None,
            guid: None,
        }
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
