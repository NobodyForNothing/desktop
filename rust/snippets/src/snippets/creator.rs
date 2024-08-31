use std::{fs, path::PathBuf};

use log::trace;

pub struct SnipCreator {
    path: PathBuf,
}

impl SnipCreator {
    pub fn new(path: PathBuf) -> Self {
        SnipCreator { path }
    }

    pub fn add(&self, name: String, content: String,) -> bool {
        trace!("Adding note '{}' with content:\n", content);
        fs::write(self.path.join(name), content).unwrap();
        true
    }
}