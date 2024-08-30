use std::{fs::{self, Metadata}, path::PathBuf, time::SystemTime};

use log::{debug, trace, warn};

use super::lang::Lang;

pub struct Snip {
    pub content: String,
    pub name: String,
    pub last_mod: SystemTime,
    pub tags: Vec<String>,
    pub lang: Lang,
}

#[derive(Debug)]
pub struct SnipBuilder {
    file: PathBuf,
    meta: Metadata,
    name: String,
}

impl SnipBuilder {
    /// Gather outer file information (fast)
    pub fn try_new(f: PathBuf) -> Option<Self> {
        debug!("Creating SnipBuilder from {:?}", f);
        if !f.is_file() {
            warn!("{} is not a file, skipping.", f.display());
            return None;
        }
        trace!("Starting filename extraction for {:?}.", &f);
        let name = f.file_name()?;
        let name = name.to_str()?;
        let name = name.to_string();
        debug!("Extracted file name from snippet {:?}: {}.", &f, &name);

        trace!("Starting metadata extraction for {:?}.", &f);
        let meta = f.metadata().ok()?;
        debug!("Extracted metadata from snippet {:?}: {:#?}.", &f, &meta);

        Some(SnipBuilder {
            file: f,
            name,
            meta
        })
    }

    /// Parse file content and metadata (slower).
    pub fn build(self) -> Option<Snip> {
        trace!("Building snippet {:?}", &self);

        let last_mod = self.meta.modified()
            .inspect_err(|err| {
                warn!("Cant fetch modified date from snippet {}", self.name);
                debug!("{:#?}", err);
            })
            .ok()?;

        let file_content = fs::read_to_string(self.file)
            .inspect_err(|err| {
                warn!("Cant read snippet to string: {}", self.name);
                debug!("{:#?}", err);
            })
            .ok()?;

        let tags = if let Some(tags) = file_content.lines().next() {
            debug!("Parsing tag line {}", &tags);
            tags.split(',').map(|e|e.to_string()).collect::<Vec<String>>()
        } else {
            warn!("No tag line in snippet: {}", self.name);
            return None;
        };

        let content = file_content.lines()
            .skip(2)
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        

        Some(Snip {
            content,
            name: self.name,
            last_mod,
            tags,
            lang: Lang::Md,
        })
    }
}

