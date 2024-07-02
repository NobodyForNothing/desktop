use std::ffi::OsStr;
use iniconf::{IniFile, IniFileOpenError};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use log::warn;
use sha1::{Sha1, Digest};
use crate::git;
use crate::git::objects::{GitBlob, GitObject, BinSerializable};

pub struct Repository {
    /// Where the files meant to be in version control live.
    work_tree: PathBuf,
    /// Storage of vcs data
    git_dir: PathBuf,

    config: RepoConfig,
}

impl Repository {
    /// Load an existing repository at [path].
    ///
    /// [force] (default false) ignores missing directories and always returns
    /// [Ok].
    pub fn new(path: PathBuf, force: Option<bool>) -> Result<Self, RepositoryLoadError> {
        let force = force.unwrap_or(false);

        let mut instance = Repository {
            work_tree: path.clone(),
            git_dir: path.join(".git"),
            config: RepoConfig::default(),
        };

        if instance.git_dir.is_dir() || force {
            let path = instance.git_dir.join("config");
            if path.is_file() || force {
                instance.config = RepoConfig::read(path).expect("IO is possible as per check above");
                if instance.config.repository_format_version <= 0 || force {
                    Ok(instance)
                } else {
                    Err(RepositoryLoadError::UnsupportedRepositoryFormatVersion {
                        actual: instance.config.repository_format_version,
                        supported: 0,
                    })
                }

            } else {
                Err(RepositoryLoadError::ConfigurationFileMissing)
            }
        } else {
            Err(RepositoryLoadError::NotAGitRepository)
        }
    }

    pub fn init(path: PathBuf) -> Result<Self, RepositoryInitError> {
        let mut repo = Self::new(path, Some(true)).ok().expect("Force is passed");
        if repo.work_tree.is_file() || repo.work_tree.is_symlink() {
            Err(RepositoryInitError::NotADirectory)
        } else if repo.git_dir.read_dir().is_ok_and(|dir| dir.count() > 0) {
            Err(RepositoryInitError::AlreadyInitialized)
        } else {
            if Self::init_fs(&mut repo).is_none() {
                Err(RepositoryInitError::IOError)
            } else {
                Ok(repo)
            }
        }
    }

    fn init_fs(repo: &mut Repository) -> Option<()> {
        if !repo.git_dir.exists() {
            fs::create_dir_all(&repo.git_dir).ok()?;
        }

        repo.repo_path(vec!["branches"], Some(true), Some(false))?;
        repo.repo_path(vec!["objects"], Some(true), Some(false))?;
        repo.repo_path(vec!["refs", "tags"], Some(true), Some(false))?;
        repo.repo_path(vec!["refs", "heads"], Some(true), Some(false))?;

        let desc = repo.repo_path(vec!["description"], Some(false), Some(true))?;
        fs::write(desc, "Unnamed repository; edit this file 'description' to name the repository.\n").ok()?;

        let head = repo.repo_path(vec!["HEAD"], Some(false), Some(true))?;
        fs::write(head, "ref: refs/heads/master\n").ok()?;

        let config = repo.repo_path(vec!["config"], Some(false), Some(true))?;
        repo.config = RepoConfig::read(config)?;
        repo.config.write()?;

        Some(())
    }

    /// Compute path under repo's gitdir.
    ///
    /// If [mkdir] is true directories specified in the path will be created.
    /// If [has_file] is true no directory will be created for the last item in
    /// [path_list]. [mkdir] and [has_file] default to false.
    fn repo_path<P: AsRef<Path>>(&self, path_list: Vec<P>, mkdir: Option<bool>, has_file: Option<bool>) -> Option<PathBuf> {
        if path_list.is_empty() {
            panic!("repo_path needs a path to join")
        }

        let mkdir = mkdir.unwrap_or(false);
        let has_file = has_file.unwrap_or(false);

        let dir_path_list = if has_file {
            &path_list[0..path_list.len() - 1]
        } else {
            &path_list
        };

        let mut res_path = self.git_dir.clone();
        for path in dir_path_list {
            res_path = res_path.join(path)
        }
        if mkdir {
            if res_path.exists() && !res_path.is_dir() {
                panic!("Tried to create dir where there already was a file: {:?}", &res_path);
            }
            if fs::create_dir_all(&res_path).is_err() {
                warn!("Failed to create {:?}", &res_path);
            }
        }

        if res_path.exists() {
            if has_file {
                res_path = res_path.join(path_list.last().expect("repo_path won't accept empty paths."))
            }
            Some(res_path)
        } else {
            None
        }
    }

    fn object_read(&self, sha: [u8; 20]) -> Option<GitObject> {
        let sha: String = sha.iter().map(|byte| format!("{:x}", byte)).collect();
        let path = self.repo_path(vec!["objects", &sha[0..2], &sha[2..sha.len()]], None, Some(true));
        if path.as_ref().is_some_and(|p| p.is_file()) {
            if let Ok(data) = fs::read(path.unwrap()) {
                let mut data = flate2::read::ZlibDecoder::new(&data[..]);
                let mut data = data.bytes();
                let mut obj_type = String::new();
                while let Some(Ok(byte)) = data.next() {
                    if byte == 20 {
                        break;
                    }
                    obj_type.push(char::from(byte));
                }

                let mut obj_len = String::new();
                while let Some(Ok(byte)) = data.next() {
                    if byte == 0x00 {
                        break;
                    }
                    obj_len.push(char::from(byte));
                }
                let obj_len = obj_len.parse::<u64>().expect("GitObject doesn't contain size");

                let remaining_bits: Vec<u8> = data.map(|e| e.unwrap()).collect();
                assert_eq!(obj_len as usize, remaining_bits.len());

                let obj = match obj_type.as_str() {
                    "commit" => GitObject::Commit,
                    "tree" => GitObject::Tree,
                    "tag" => GitObject::Tag,
                    "blob" => GitObject::Blob(GitBlob::deserialize(remaining_bits)),
                    _ => panic!("Unknown type {obj_type} for object {sha}"),
                };
                return Some(obj);
            }

        }
        None
    }

    pub fn object_write(&self, obj: GitObject) -> String {
        let obj = obj.serialize();
        let mut hasher = Sha1::new();
        hasher.update(&obj);
        let sha = hasher.finalize();
        let sha: String = sha.iter().map(|byte| format!("{:x}", byte)).collect();


        let path = self.repo_path(vec!["objects", &sha[..2], &sha[2..]], Some(true), Some(true));
        if let Some(path) = path {
            if !path.exists() {
                fs::write(path, obj).unwrap();
            }
        }

        sha
    }
}

#[derive(Debug)]
pub enum RepositoryLoadError {
    NotAGitRepository,
    ConfigurationFileMissing,
    UnsupportedRepositoryFormatVersion {
        /// Version from config file.
        actual: u8,
        /// Highest version supported by the program.
        supported: u8,
    },
}

#[derive(Debug)]
pub enum RepositoryInitError {
    NotADirectory,
    AlreadyInitialized,
    IOError,
}

struct RepoConfig {
    file: IniFile,
    ///  The version of the gitdir format.
    ///
    /// - 0 means the initial format
    /// - 1 the same with extensions
    repository_format_version: u8,
    /// Disable tracking of file modes (permissions) changes in the work tree.
    file_mode: bool,
    /// Indicates whether this repository has a worktree.
    bare: bool,
    // Always assume worktree is at `../`.
}
impl RepoConfig {
    /// Reads repo config from [path].
    ///
    /// Replaces the file if badly formatted.
    fn read(path: PathBuf) -> Option<Self> {
        if path.parent().is_some_and(|p| !p.is_dir()) {
            fs::create_dir_all(path.parent().unwrap()).ok()?;
        }
        if !(path.is_dir() || path.is_symlink()) {
            let x  =IniFile::open(path.clone());
            println!("{:#?}", &x);
            match x {
                Ok(file) => {
                    let version = file.get::<u8>("core", "repositoryformatversion");
                    let mode = file.get::<bool>("core", "filemode");
                    let bare = file.get::<bool>("core", "bare");
                    Some(Self {
                        file,
                        repository_format_version: version.unwrap_or(Self::default().repository_format_version),
                        file_mode: mode.unwrap_or(Self::default().file_mode),
                        bare: bare.unwrap_or(Self::default().bare),
                    })
                },
                Err(IniFileOpenError::FormatError) => {
                    warn!("Overriding repo config as it is badly formatted");
                    if fs::remove_file(&path).is_ok() {
                        if let Ok(file) = IniFile::open(path) {
                            Some(Self {
                                file,
                                ..Self::default()
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                Err(IniFileOpenError::IOError) => {
                    warn!("Couldn't open ini file: IOError");
                    None
                }
            }
        } else {
            warn!("Tried to read config from directory or symlink.");
            None
        }

    }

    fn write(&mut self) -> Option<()> {
        self.file.set_str("core", "repositoryformatversion", format!("{}", self.repository_format_version).as_str());
        self.file.set_str("core", "filemode", format!("{}", self.file_mode).as_str());
        self.file.set_str("core", "bare", format!("{}", self.bare).as_str());

        self.file.write().ok()
    }
}
impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            file: IniFile::default(),
            repository_format_version: 0,
            file_mode: false,
            bare: false,
        }
    }
}