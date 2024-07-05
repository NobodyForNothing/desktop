use std::io::{Bytes, Error, Read};

pub(crate) trait BinSerializable {
    /// Read git object contents without header or compression.
    fn deserialize(data: Vec<u8>) -> Self;
    fn serialize(self) -> Vec<u8>;
}

pub enum GitObject {
    Commit(GitCommit),
    Tree(GitTree),
    Tag(GitTag),
    Blob(GitBlob),
}

/// Like [GitObject], but without data.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum GitObjectType {
    Commit,
    Tree,
    Tag,
    Blob,
}

impl GitObject {
    /// Serialize a git object including the header and compression.
    pub fn serialize(self) -> Vec<u8> {
        match self {
            GitObject::Commit(commit) => commit.serialize(),
            GitObject::Tree(tree) => tree.serialize(),
            GitObject::Tag(tag) => tag.serialize(),
            GitObject::Blob(blob) => blob.serialize(),
        }
    }
}

/// Raw userdata.
pub struct GitBlob {
    data: Vec<u8>,
}

impl GitBlob {
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl BinSerializable for GitBlob {
    fn deserialize(data: Vec<u8>) -> Self {
        Self { data }
    }

    fn serialize(self) -> Vec<u8> {
        self.data
    }
}

// https://wyag.thb.lt/#orgfe2859f
pub struct GitCommit {
    kvlm: Vec<(String, String)>,
}

impl GitCommit {
    // TODO: constructor (no setters as immutable)

    /// Reference to a tree object.
    pub fn get_tree(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "tree")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }
    /// References to commits this commit is based on.
    ///
    /// - merge commits may have multiple
    /// - the first commit may have none
    pub fn get_parents(&self) -> Vec<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "parent")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .collect::<Vec<String>>()
    }
    /// Like: `Scott Chacon <schacon@gmail.com> 1243040974 -0700`
    pub fn get_author(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "author")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }
    pub fn get_commiter(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "committer")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }
    /// PGP signature of the object.
    pub fn get_gpgsig(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "gpgsig")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }
    pub fn get_message(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "__message__")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }
}

impl BinSerializable for GitCommit {
    fn deserialize(data: Vec<u8>) -> Self {
        GitCommit {
            kvlm: kvlm_parse(data.bytes()),
        }
    }

    fn serialize(self) -> Vec<u8> {
        kvlm_serialize(self.kvlm)
    }
}

#[derive(Clone)]
pub struct GitTag {
    kvlm: Vec<(String, String)>,
}

impl GitTag {
    pub fn new(obj_hash: String, tag_name: String, tagger: String, message: String) -> Self {
        let mut kvlm = Vec::new();
        kvlm.push(("object".to_string(), obj_hash));
        kvlm.push(("type".to_string(), "commit".to_string()));
        kvlm.push(("tag".to_string(), tag_name));
        kvlm.push(("tagger".to_string(), tagger));
        kvlm.push(("__message__".to_string(), message));
        GitTag { kvlm }
    }

    pub fn object_hash(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "object")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }

    /// Get the name of the tag.
    pub fn tag(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "tag")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }

    pub fn tagger(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "tagger")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }

    pub fn get_message(&self) -> Option<String> {
        self.kvlm
            .iter()
            .filter(|(k, _)| k == "__message__")
            .map(|(_k, v)| v.trim_matches(|e| e == '\n').to_string())
            .next()
    }
}

impl BinSerializable for GitTag {
    fn deserialize(data: Vec<u8>) -> Self {
        GitTag {
            kvlm: kvlm_parse(data.bytes()),
        }
    }

    fn serialize(self) -> Vec<u8> {
        kvlm_serialize(self.kvlm)
    }
}

pub struct GitTree {
    entries: Vec<GitTreeEntry>,
}

impl GitTree {
    pub fn entries(&self) -> &Vec<GitTreeEntry> {
        self.entries.as_ref()
    }
}

impl BinSerializable for GitTree {
    fn deserialize(data: Vec<u8>) -> Self {
        let mut data = data.bytes().peekable();
        let mut entries = Vec::new();
        while data.peek().is_some() {
            entries.push(GitTreeEntry::parse(&mut data));
        }
        GitTree { entries }
    }

    fn serialize(self) -> Vec<u8> {
        todo!()
    }
}

pub struct GitTreeEntry {
    /// Hash of a tree or a blob.
    obj_hash: String,
    /// File perm mode.
    mode: [u8; 6],
    /// File or dir name.
    path: String,
}

impl GitTreeEntry {
    /// Decodes format: `[mode] space [path] 0x00 [sha-1]`.
    fn parse(data: &mut impl Iterator<Item = Result<u8, Error>>) -> Self {
        let mut mode: [u8; 6] = [0; 6];
        let mut i = 0;
        while let Some(Ok(byte)) = data.next() {
            if byte == b' ' {
                break;
            }
            println!("{}", char::from(byte));
            if i >= mode.len() {
                println!("{} >= {}", i, mode.len());
                panic!("Incorrectly formatted git tree entry: mode too long")
            }
            mode[i] = byte;
            i += 1;
        }

        let mut path = String::new();
        while let Some(Ok(byte)) = data.next() {
            if byte == 0x00 {
                break;
            }
            path.push(char::from(byte));
        }

        let hash = data.take(40); // 2 chars per hex byte
        let hash = hash.map(|e| e.unwrap());
        let hash = String::from_utf8(hash.collect::<Vec<u8>>());

        GitTreeEntry {
            mode,
            path,
            obj_hash: hash.expect("invlaid hash"),
        }
    }

    /// Hash of a tree or a blob.
    pub fn obj_hash(&self) -> &String {
        &self.obj_hash
    }
    /// File perm mode.
    pub fn mode(&self) -> [u8; 6] {
        self.mode
    }
    /// File or dir name.
    pub fn path(&self) -> &String {
        &self.path
    }
}

/// Recursively parse a Key-Value List with Message.
///
/// Returns keys and values in the order they were parsed and the message with a
/// `__message__` key.
fn kvlm_parse(mut raw: Bytes<&[u8]>) -> Vec<(String, String)> {
    // https://git-scm.com/book/en/v2/Git-Internals-Git-Objects#_git_commit_objects
    let mut lines = Vec::new();
    {
        let mut line = String::new();
        while let Some(Ok(byte)) = raw.next() {
            line.push(char::from(byte));
            if byte == b'\n' {
                lines.push(line);
                line = String::new();
            }
        }
        lines.push(line);
    }

    let mut kv_entries = Vec::new();
    let mut in_message_block = false;
    {
        let mut key: Option<String> = None;
        let mut value = String::new();
        for line in lines {
            if in_message_block {
                value.push_str(line.as_str());
            } else if line.starts_with(" ") || line.is_empty() {
                // Value continues in this line
                value.push_str(line.as_str());
            } else if line == "\n" {
                // Start message block
                if let Some(last_key) = key {
                    let last_value = value.replace("\n ", "\n");
                    kv_entries.push((last_key, last_value));
                    key = None;
                }
                in_message_block = true;
                value = String::new();
            } else {
                // New value: save old kv and start new
                if let Some(last_key) = key {
                    let last_value = value.replace("\n ", "\n");
                    kv_entries.push((last_key, last_value));
                    value = String::new();
                    key = None;
                }
                if let Some((k, v)) = line.split_once(" ") {
                    key = Some(k.to_string());
                    assert!(value.is_empty());
                    value = v.to_string();
                } // non conformant lines are ignored as comments
            }
        }

        if in_message_block {
            kv_entries.push((String::from("__message__"), value));
        }
    }

    kv_entries
}

fn kvlm_serialize(kvlm: Vec<(String, String)>) -> Vec<u8> {
    // TODO: test
    let mut out = String::new();
    for (k, v) in kvlm {
        if k == "__message__" {
            out.push_str(format!("\n{v}").as_str())
        } else {
            let v = v.replace("\n", "\n ");
            out.push_str(format!("{k} {v} \n").as_str());
        }
    }
    out.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::git::objects::{kvlm_parse, BinSerializable, GitCommit, GitTree};

    const SAMPLE_COMMIT: &str = "tree 29ff16c9c14e2652b22f8b78bb08a5a07930c147
parent 206941306e8a8af65b66eaaaea388a7ae24d49a0
author Thibault Polge <thibault@thb.lt> 1527025023 +0200
committer Thibault Polge <thibault@thb.lt> 1527025044 +0200
gpgsig -----BEGIN PGP SIGNATURE-----\n \n iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL
 kGQdcBAAqPP+ln4nGDd2gETXjvOpOxLzIMEw4A9gU6CzWzm+oB8mEIKyaH0UFIPh
 rNUZ1j7/ZGFNeBDtT55LPdPIQw4KKlcf6kC8MPWP3qSu3xHqx12C5zyai2duFZUU
 wqOt9iCFCscFQYqKs3xsHI+ncQb+PGjVZA8+jPw7nrPIkeSXQV2aZb1E68wa2YIL
 3eYgTUKz34cB6tAq9YwHnZpyPx8UJCZGkshpJmgtZ3mCbtQaO17LoihnqPn4UOMr
 V75R/7FjSuPLS8NaZF4wfi52btXMSxO/u7GuoJkzJscP3p4qtwe6Rl9dc1XC8P7k
 NIbGZ5Yg5cEPcfmhgXFOhQZkD0yxcJqBUcoFpnp2vu5XJl2E5I/quIyVxUXi6O6c
 /obspcvace4wy8uO0bdVhc4nJ+Rla4InVSJaUaBeiHTW8kReSFYyMmDCzLjGIu1q
 doU61OM3Zv1ptsLu3gUE6GU27iWYj2RWN3e3HE4Sbd89IFwLXNdSuM0ifDLZk7AQ
 WBhRhipCCgZhkj9g2NEk7jRVslti1NdN5zoQLaJNqSwO1MtxTmJ15Ksk3QP6kfLB
 Q52UWybBzpaP9HEd4XnR+HuQ4k2K0ns2KgNImsNvIyFwbpMUyUWLMPimaV1DWUXo
 5SBjDB/V/W2JBFR+XKHFJeFwYhj7DD/ocsGr4ZMx/lgc8rjIBkI=
 =lgTX
 -----END PGP SIGNATURE-----

Create first draft";

    #[test]
    fn kvlm_parses() {
        let parsed = kvlm_parse(SAMPLE_COMMIT.as_bytes().bytes());
        // TODO: verify \n is wanted
        assert_eq!(parsed.get(0).unwrap().0, "tree");
        assert_eq!(
            parsed.get(0).unwrap().1,
            "29ff16c9c14e2652b22f8b78bb08a5a07930c147\n"
        );
        assert_eq!(parsed.get(1).unwrap().0, "parent");
        assert_eq!(
            parsed.get(1).unwrap().1,
            "206941306e8a8af65b66eaaaea388a7ae24d49a0\n"
        );
        assert_eq!(parsed.get(2).unwrap().0, "author");
        assert_eq!(
            parsed.get(2).unwrap().1,
            "Thibault Polge <thibault@thb.lt> 1527025023 +0200\n"
        );
        assert_eq!(parsed.get(3).unwrap().0, "committer");
        assert_eq!(
            parsed.get(3).unwrap().1,
            "Thibault Polge <thibault@thb.lt> 1527025044 +0200\n"
        );
        assert_eq!(parsed.get(4).unwrap().0, "gpgsig");
        let v4 = parsed.get(4).unwrap().1.clone();
        assert!(v4.clone().contains("-----BEGIN PGP SIGNATURE-----"));
        assert!(v4
            .clone()
            .contains("iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL"));
        assert!(v4.contains("-----END PGP SIGNATURE-----"));
        assert_eq!(parsed.get(5).unwrap().0, "__message__");
        assert_eq!(parsed.get(5).unwrap().1, "Create first draft");
    }

    #[test]
    fn git_commit_deserialize() {
        let commit = GitCommit::deserialize(SAMPLE_COMMIT.as_bytes().to_vec());
        assert_eq!(
            commit.get_tree(),
            Some(String::from("29ff16c9c14e2652b22f8b78bb08a5a07930c147"))
        );
        assert_eq!(
            commit.get_parents().iter().next().unwrap().clone(),
            String::from("206941306e8a8af65b66eaaaea388a7ae24d49a0")
        );
        assert_eq!(
            commit.get_author(),
            Some(String::from(
                "Thibault Polge <thibault@thb.lt> 1527025023 +0200"
            ))
        );
        assert_eq!(
            commit.get_commiter(),
            Some(String::from(
                "Thibault Polge <thibault@thb.lt> 1527025044 +0200"
            ))
        );
        assert!(commit
            .get_gpgsig()
            .unwrap()
            .contains("iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL"));
        assert_eq!(
            commit.get_message(),
            Some(String::from("Create first draft"))
        );
    }

    #[test]
    fn git_tree_deserialize() {
        let mut txt = "100644 testfile\x0029ff16c9c14e2652b22f8b78bb08a5a07930c147"
            .as_bytes()
            .to_vec();
        txt.append(
            &mut "100645 some other test files.txt\x00206941306e8a8af65b66eaaaea388a7ae24d49a0"
                .as_bytes()
                .to_vec(),
        );
        let tree = GitTree::deserialize(txt);

        assert_eq!(tree.entries.len(), 2);
        assert_eq!(tree.entries.get(0).unwrap().mode, "100644".as_bytes());
        assert_eq!(tree.entries.get(0).unwrap().path, "testfile");
        assert_eq!(
            tree.entries.get(0).unwrap().obj_hash,
            "29ff16c9c14e2652b22f8b78bb08a5a07930c147"
        );
        assert_eq!(tree.entries.get(1).unwrap().mode, "100645".as_bytes());
        assert_eq!(
            tree.entries.get(1).unwrap().path,
            "some other test files.txt"
        );
        assert_eq!(
            tree.entries.get(1).unwrap().obj_hash,
            "206941306e8a8af65b66eaaaea388a7ae24d49a0"
        );
    }
}
