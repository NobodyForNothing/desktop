use std::{fs, path::PathBuf};

use log::{debug, info, trace, warn, error};

use super::cli::Cli;

pub struct Config {
    pub dir: PathBuf,
}

impl Config {
    pub fn load(args: &Cli) -> Option<Self> {
        trace!("Loading configuration");
        let dir = args.dir.clone();
        let dir = if let Some(dir) = dir {
            dir
        } else {
            debug!("No dir supplied, attempting to fetch documents dir");
            if let Some(doc_dir) = dirs::document_dir() {
                doc_dir.join("snippets")
            } else {
                warn!("Could not fetch documents dir, falling back to local dir");
                PathBuf::from("./snippets")
            }
        };

        debug!("Using snippets directory: {:?}", dir);

        if dir.exists() {
            trace!("Snippets directory exists, doing nothing.");
        } else {
            info!("Recursively creating snippets directory at {}...", dir.display());
            if let Err(err) = fs::create_dir_all(&dir) {
                error!("Cant't create dir at {}: {:?}", dir.display(), err);
                return None;
            } else {
                trace!("Successfully created snippets directory.")
            }
        }

        if !dir.is_dir() {
            error!("Not a directory: {}", dir.display());
            return None;
        }

        if dir.metadata().is_ok_and(|meta| meta.permissions().readonly()) {
            warn!("Snippets directory is read only")
        }

        Some(Config {
            dir
        })
    }
}