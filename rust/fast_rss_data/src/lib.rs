use bitcode::{Decode, Encode};

pub fn encode(data: &RssSummary, compress: bool) -> Option<Vec<u8>> {
    let serialised = bitcode::encode(data);
    if compress {
        zstd::encode_all(serialised.as_slice(), 19).ok()
    } else {
        Some(serialised)
    }
}

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

#[test]
mod tests {
    use crate::{Channel, decode, encode, Item, RssSummary};

    #[test]
    fn deserializes_serialized_no_compression() {
        let summary = test_data();
        let serialised = encode(&summary, false).unwrap();
        let deserialised = decode(&serialised, false).unwrap();
        assert_eq!(summary, deserialised)
    }

    #[test]
    fn deserializes_serialized_compressed() {
        let summary = test_data();
        let serialised = encode(&summary, true).unwrap();
        let deserialised = decode(&serialised, true).unwrap();
        assert_eq!(summary, deserialised)
    }

    fn test_data() -> RssSummary {
        RssSummary {
            data: vec![
                Channel {
                    title: Some(String::from("test title")),
                    link: Some(String::from("test link")),
                    description: Some(String::from("test description")),
                    language: Some(String::from("test lang")),
                    items: vec![
                        Item {
                            title: Some(String::from("test item title")),
                            description: Some(String::from("test item description")),
                            link: Some(String::from("test item link")),
                            guid: Some(String::from("test item guid")),
                        }
                    ],
                }
            ],
        }
    }
}

