use std::collections::HashMap;
use std::fmt::format;
use std::io::Bytes;

pub(crate) trait BinSerializable {
    /// Read git object contents without header or compression.
    fn deserialize(data: Vec<u8>) -> Self;
    fn serialize(self) -> Vec<u8>;
}

pub enum GitObject {
    Commit,
    Tree,
    Tag,
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
    pub fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}

/// Raw userdata.
pub struct GitBlob {
    data: Vec<u8>,
}

impl BinSerializable for GitBlob {
    fn deserialize(data: Vec<u8>) -> Self {
        Self {
            data
        }
    }

    fn serialize(self) -> Vec<u8> {
        self.data
    }
}

// https://wyag.thb.lt/#orgfe2859f
pub struct GitCommit {
    /// Reference to a tree object.
    tree: String,
    /// References to commits this commit is based on.
    ///
    /// - merge commits may have multiple
    /// - the first commit may have none
    parent: Vec<String>,
    author: String,
    commiter: String,
    /// PGP signature of the object.
    gpgsig: String,
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
            print!("{}, {}", in_message_block, &line);
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
                    value = String::new();
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

fn kvlm_serialize(kvlm: Vec<(String, String)>) -> Vec<u8> { // TODO: test
    let mut out = String::new();
    for (k, v) in kvlm {
        let v = v.replace("\n", "\n ");
        if k == "__message__" {
            out.push_str(format!("\n{v}").as_str())
        } else {
            out.push_str(format!("{k} {v} \n").as_str());
        }
    }
    out.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use crate::git::objects::kvlm_parse;

    #[test]
    fn kvlm_parses() {
        let parsed = kvlm_parse("tree 29ff16c9c14e2652b22f8b78bb08a5a07930c147
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

Create first draft".as_bytes().bytes());
        // TODO: verify \n is wanted
        assert_eq!(parsed.get(0).unwrap().0, "tree");
        assert_eq!(parsed.get(0).unwrap().1, "29ff16c9c14e2652b22f8b78bb08a5a07930c147\n");
        assert_eq!(parsed.get(1).unwrap().0, "parent");
        assert_eq!(parsed.get(1).unwrap().1, "206941306e8a8af65b66eaaaea388a7ae24d49a0\n");
        assert_eq!(parsed.get(2).unwrap().0, "author");
        assert_eq!(parsed.get(2).unwrap().1, "Thibault Polge <thibault@thb.lt> 1527025023 +0200\n");
        assert_eq!(parsed.get(3).unwrap().0, "committer");
        assert_eq!(parsed.get(3).unwrap().1, "Thibault Polge <thibault@thb.lt> 1527025044 +0200\n");
        assert_eq!(parsed.get(4).unwrap().0, "gpgsig");
        let v4 = parsed.get(4).unwrap().1.clone();
        assert!(v4.clone().contains("-----BEGIN PGP SIGNATURE-----"));
        assert!(v4.clone().contains("iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL"));
        assert!(v4.contains("-----END PGP SIGNATURE-----"));
        assert_eq!(parsed.get(5).unwrap().0, "__message__");
        assert_eq!(parsed.get(5).unwrap().1, "Create first draft");
    }
}
