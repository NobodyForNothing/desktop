use std::fs;

use colored::Colorize;
use creator::SnipCreator;
use log::{debug, error, info};

use crate::input::config::Config;

mod creator;

pub struct SnipetsFolderBuilder<'a> {
    config: &'a Config,
}

impl<'a> SnipetsFolderBuilder<'a> {
    pub fn new(config: &'a Config) -> Self {
        SnipetsFolderBuilder { config }
    }

    pub fn validate(self) -> Option<ValidatedSnipptsFolder<'a>> {
        // TODO
        Some(ValidatedSnipptsFolder{ config: &self.config })
    }
}

pub struct ValidatedSnipptsFolder<'a>{
    config: &'a Config,
}

impl<'a> ValidatedSnipptsFolder<'a> {
    pub fn index(self) -> Option<IndexedSnipptsFolder<'a>> {
        match fs::read_dir(&self.config.dir) {
            Ok(dir) => {
                let mut files = Vec::new();
                let mut initial_length = 0;
                for f in dir {
                    initial_length += 1;
                    if let Ok(f) = f {
                        let f = f.path();
                        if f.is_file() {
                            if let Some(f) = f.file_name() {
                                if let Some(f) = f.to_str() {
                                    files.push(f.to_string());
                                }
                            }
                        }
                    }
                }

                info!("Indexed {} / {} files in snippets dir", files.len(), initial_length);
                Some(IndexedSnipptsFolder {
                    config: &self.config,
                    index: files,
                })
            },
            Err(err) => {
                error!("Couldn't index snippets dir");
                debug!("{:?}", err);
                None
            },
        }
    }
}

pub struct IndexedSnipptsFolder<'a>{
    config: &'a Config,
    index: Vec<String>,
}

impl<'a> IndexedSnipptsFolder<'a> {
    pub fn open(self) -> SnipptsFolder {
        SnipptsFolder {
            index: self.index,
            creator: SnipCreator::new(self.config.dir.clone()),
        }
    }
}

pub struct SnipptsFolder {
    index: Vec<String>,
    creator: SnipCreator,
}

impl SnipptsFolder {
    pub fn add(&self, name: String) {
        self.creator.add(name);
    }
    pub fn list(&self) {
        println!("{}", "Available snippets:".bold().underline());
        for file in &self.index {
            println!("{}{}", "> ".cyan(), file);
        }
    }
}
