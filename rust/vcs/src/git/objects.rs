
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
