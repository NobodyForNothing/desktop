use std::fs;

use colored::{Colorize, CustomColor};
use creator::SnipCreator;
use log::{debug, error, info, trace};
use snip::{Snip, SnipBuilder};

use crate::input::config::Config;

mod creator;
mod snip;

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
                let mut builders = Vec::new();
                let mut initial_length = 0;
                for f in dir {
                    initial_length += 1;
                    if let Ok(f) = f {
                        if let Some(f) = SnipBuilder::try_new(f.path()) {
                            builders.push(f)
                        }
                    }
                }

                info!("Indexed {} / {} files in snippets dir", builders.len(), initial_length);
                Some(IndexedSnipptsFolder {
                    config: &self.config,
                    snippets: builders,
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
    snippets: Vec<SnipBuilder>,
}

impl<'a> IndexedSnipptsFolder<'a> {
    pub fn open(self) -> SnipptsFolder {
        let snips = self.snippets.into_iter()
            .filter_map(|b| b.build());
        SnipptsFolder {
            snippets: snips.collect(),
            creator: SnipCreator::new(self.config.dir.clone()),
        }
    }
}

pub struct SnipptsFolder {
    snippets: Vec<Snip>,
    creator: SnipCreator,
}

impl SnipptsFolder {
    pub fn add(&self, name: String, tags: Vec<String>, content: String) {
        trace!("Constructing note from name: '{}', tags: ['{}'], content: '\n{}'", &name, tags.join("','"), &content);
        let content = format!("{}\n{}", tags.join(","), content.as_str());
        self.creator.add(name, content);
    }
    pub fn list(&self) {
        println!("{}", "Available snippets:".bold().underline());
        for snip in &self.snippets {
            println!("{}{}\t\t{}", "> ".cyan(), snip.name, snip.tags.join(",").custom_color(CustomColor::new(200, 200, 200)).italic());
        }
    }
}
