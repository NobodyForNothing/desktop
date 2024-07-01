
pub trait GitObjectT {
    /* TODO
    fn init(data: Option<Vec<u8>>) -> Box<Self> {
        match data {
            None => Self::new(),
            Some(data) => {
                let mut obj = Self::new();
                obj.deserialize(data);
                obj
            }
        }
    }*/
    fn new() -> Self;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: Vec<u8>);
}

pub enum GitObject {
    Commit,
    Tree,
    Tag,
    Blob,
}
