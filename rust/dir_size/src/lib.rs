use std::fs;
use std::path::Path;

/// Recursively get the size in bytes of a directory.
///
/// Doesn't follow symlinks and only adds the size
pub fn dir_size<P: AsRef<Path>>(dir: P) -> u128 {
    let mut size: u128 = 0;
    if let Ok(dir) = fs::read_dir(dir) {
        for file in dir {
            let file = file.unwrap();
            if file.file_type().unwrap().is_dir() {
                size += dir_size(file.path());
            } else if let Ok(meta) = file.metadata() {
                size += imp::size(meta);
            }
        }
    }
    size
}

#[cfg(linux)]
pub mod imp {
    use std::os::linux::fs::MetadataExt;
    pub fn size(f: std::fs::Metadata) -> u128 {
        f.st_size() as u128
    }
}

#[cfg(windows)]
mod imp {
    use std::os::windows::fs::MetadataExt;
    pub fn size(f: std::fs::Metadata) -> u128 {
        f.file_size() as u128
    }
}

#[cfg(unix)]
pub mod imp {
    use std::os::unix::fs::MetadataExt;
    pub fn size(f: std::fs::Metadata) -> u128 {
        f.size() as u128
    }
}
