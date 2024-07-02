
pub(crate) trait BinSerializable {
    fn deserialize(data: Vec<u8>) -> Self;

    fn serialize(self) -> Vec<u8>;
}

pub enum GitObject {
    Commit,
    Tree,
    Tag,
    Blob(GitBlob),
}

impl GitObject {
    /// Serialize a git object including the header.
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
