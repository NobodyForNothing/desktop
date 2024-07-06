use std::ops::Add;
use std::path::PathBuf;
use std::time;

const DIR_CACHE_SIGNATURE: &[u8] = "DIRC".as_bytes();
const SUPPORTED_INDEX_VERSION: u8 = 2;

pub struct GitIndex {
    version: u32,
    entries: Vec<GitIndexEntry>,
}

/// A file entry in a git index.
struct GitIndexEntry {
    /// The last time a file's metadata changed.
    meta_changed_time: time::Duration,
    /// The last time a file's data changed.
    data_change_time: time::Duration,
    /// The ID of device containing this file
    dev: u32,
    /// The file's inode number
    ino: u32,
    /// The object type.
    mode_type: ObjectType,
    /// The object permissions, an integer.
    mode_perms: u16,
    /// User ID of owner.
    uid: u32,
    /// Group ID of owner.
    gid: u32,
    /// Size of this object, in bytes.
    fsize: u32,
    /// The object's SHA hash.
    hash: String,
    flags: GitIndexEntryFlags,
    /// Full fs path of the object.
    name: PathBuf,
}

enum ObjectType {
    Regular, // b1000
    Symlink, // b1010
    GitLink, // b1110
}

struct GitIndexEntryFlags {
    flag_assume_valid: bool,
    flag_extended: bool,
    flag_stage: u16,
    /// Stored in 12 bits so max is 0xFFF(4095). 0xFFF is interpreted as "at
    /// least".
    name_length: u16,
}

impl GitIndex {
    pub fn decode(data: &[u8]) -> Option<Self> {
        let header = &data[..12];
        let header = {
            let signature = &header[..4]; // magic bytes
            let version = read_int_u32(&header, 4);
            let obj_count = read_int_u32(&header, 8);
            (signature, version, obj_count)
        };
        if (header.0 == DIR_CACHE_SIGNATURE) && (header.1 == SUPPORTED_INDEX_VERSION as u32) {
            let data = &data[12..];
            let mut entries = Vec::new();
            let mut idx = 0;
            for _ in 0..header.2 {
                let ctime = read_time(data, 0);
                let mtime = read_time(data, 8);
                let dev = read_int_u32(data, 16);
                let ino = read_int_u32(data, 20);
                let unused = read_int_u32(data, 24);
                assert_eq!(unused, 0);

                let mode = read_int_u16(data, 26);
                let mode_type = (mode as u32) >> 12;
                let mode_type = ObjectType::decode(mode_type);
                let mode_perms = mode & 0b0000000111111111;

                let uid = read_int_u32(data, idx + 28);
                let gid = read_int_u32(data, idx + 32);
                let fsize = read_int_u32(data, idx + 36);
                let hash = &data[idx + 40..idx + 60];
                let hash = hash.iter().map(|byte| format!("{:x}", byte)).collect();
                let flags = GitIndexEntryFlags::decode(data, idx + 60);
                idx += 62;

                let name: &[u8] = if flags.name_length < 0xFFF {
                    let len = flags.name_length as usize;
                    idx += len + 1;
                    &data[idx..len]
                } else {
                    let end = data[idx..].iter().position(|&x| x == 0x00).map(|pos| pos + idx)?;
                    let name = &data[idx..end];
                    idx = end + 1;
                    name
                };
                let name = String::from_utf8(name.to_vec()).ok()?;

                // Data is padded on multiples of eight bytes for pointer
                // alignment, so we skip as many bytes as we need for the next
                // read to start at the right position.
                idx = 8 * idx.div_ceil(8);

                entries.push(GitIndexEntry {
                    meta_changed_time: ctime,
                    data_change_time: mtime,
                    dev,
                    ino,
                    mode_type,
                    mode_perms,
                    uid,
                    gid,
                    fsize,
                    hash,
                    flags,
                    name: PathBuf::from(name),
                });
            }
            Some(GitIndex {
                version: header.1,
                entries,
            })
        } else {
            // Unsupported format
            None
        }
    }
}

impl ObjectType {
    fn decode(data: u32) -> Self {
        match data {
            0b1000u32 => ObjectType::Regular,
            0b1010u32 => ObjectType::Symlink,
            0b1110u32 => ObjectType::GitLink,
            _ => panic!("Unrecognized object type: {:b}", data),
        }
    }
}

impl GitIndexEntryFlags {
    fn decode(data: &[u8], offset: usize) -> Self {
        let flags = read_int_u16(data, offset);
        let flag_assume_valid = (flags & 0b1000000000000000) != 0;
        let flag_extended = (flags & 0b0100000000000000) != 0;
        assert!(!flag_extended);
        let flag_stage = flags & 0b0011000000000000;
        let name_length = flags & 0b0000111111111111;
        GitIndexEntryFlags {
            flag_assume_valid,
            flag_extended,
            flag_stage,
            name_length,
        }
    }
}

/// Reads 8 bytes as duration since unix epoch.
///
/// 4 bytes seconds since epoch, 4 bytes nanoseconds after seconds since epoch
fn read_time(data: &[u8], offset: usize) -> time::Duration {
    let timestamp = read_int_u32(data, offset);
    let nanos = read_int_u32(data, offset + 4);
    time::Duration::new(timestamp as u64, nanos)
}

fn read_int_u32(data: &[u8], offset: usize) -> u32 {
    let mut res = [0u8; 4];
    res.clone_from_slice(&data[offset..(offset + 4)]);
    u32::from_be_bytes(res)
}

fn read_int_u16(data: &[u8], offset: usize) -> u16 {
    let mut res = [0u8; 2];
    res.clone_from_slice(&data[offset..(offset + 2)]);
    u16::from_be_bytes(res)
}
