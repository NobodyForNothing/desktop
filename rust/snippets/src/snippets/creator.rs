use std::{fs, path::PathBuf};

pub struct SnipCreator {
    path: PathBuf,
}

impl SnipCreator {
    pub fn new(path: PathBuf) -> Self {
        SnipCreator { path }
    }

    pub fn add(&self, name: String) -> bool {
        fs::write(self.path.join(name), "todo");
        true
    }
}