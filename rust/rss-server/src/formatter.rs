use tokio::io::AsyncReadExt;
use crate::channel::Channel;

/// Format rss streams and compress with zstd.
pub fn format(data: Vec<Channel>) -> Vec<u8> {
    let mut res: Vec<u8> = vec![
        0, // channel count
    ];
    for channel in data {
        let mut channel_data: Vec<u8> = Vec::new();
        channel_data.append(&mut serialize(channel.title));
        channel_data.append(&mut serialize(channel.description));
        channel_data.append(&mut serialize(channel.link));
        channel_data.append(&mut serialize(channel.language));

        let mut all_items_data: Vec<u8> = Vec::new();
        for item in channel.items {
            let mut item_data: Vec<u8> = Vec::new();
            item_data.append(&mut serialize(item.guid));
            item_data.append(&mut serialize(item.title));
            item_data.append(&mut serialize(item.description));
            item_data.append(&mut serialize(item.link));

            if item_data.len() > 2_i64.pow(32) as usize {
                panic!("Tried to serialize too large item")
            }
            let len = item_data.len() as u32;
            let len = len.to_le_bytes();
            all_items_data.push(len[0]);
            all_items_data.push(len[1]);
            all_items_data.push(len[2]);
            all_items_data.push(len[3]);
            all_items_data.append(&mut item_data);
        }

        if all_items_data.len() > 2_i64.pow(32) as usize {
            panic!("Tried to serialize too large item")
        }
        let len = all_items_data.len() as u32;
        let len = len.to_le_bytes();
        channel_data.push(len[0]);
        channel_data.push(len[1]);
        channel_data.push(len[2]);
        channel_data.push(len[3]);
        channel_data.append(&mut all_items_data);

        res[0] += 1;
        res.append(&mut channel_data);
    }

    zstd::bulk::compress(res.as_slice(), 19).unwrap()
}

/// Serialize strings to binary.
///
/// When [data] is None one null byte is returned. When data is Some data is
/// encoded:
///
/// 1. size header(4 bytes) (little endian) states the amount of bytes following
/// 2. bytes of string
///
/// Panics when string is larger than 2^32 bytes
fn serialize(data: Option<String>) -> Vec<u8> {
    if let Some(data) = data {
        let data = data.as_bytes();
        if data.len() > 2_i64.pow(32) as usize {
            panic!("Tried to serialize too large string: {}", data.len());
            let mut str = String::new();
            /*tokio::spawn(async {
                &data.clone().read_to_string(&mut str).await.unwrap();
                panic!("Tried to serialize too large string: {}", str);
            });
            return vec![];*/
        }
        let len = data.len() as u32;
        let len = len.to_le_bytes();
        let mut res = vec![
            len[0],
            len[1],
            len[2],
            len[3],
        ];
        res.append(&mut data.to_vec());
        res
    } else {
        vec![0]
    }
}
