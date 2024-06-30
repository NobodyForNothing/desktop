use std::{fs, os};
use std::fs::{create_dir, create_dir_all};
use std::path::{Path, PathBuf};
use log::{log, warn};

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
            let path = instance.repo_path(vec!("config"), None, Some(true));
            if path.is_some() || force {
                instance.config = RepoConfig::read(path.unwrap());
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
        let repo = Self::new(path, Some(true)).ok().expect("Force is passed");
        if repo.work_tree.is_file() || repo.work_tree.is_symlink() {
            Err(RepositoryInitError::NotADirectory)
        } else if repo.git_dir.read_dir().is_ok_and(|dir| dir.count() > 0) {
            Err(RepositoryInitError::AlreadyInitialized)
        } else {
            let success: Option<()> = {
                if !repo.git_dir.exists() {
                    create_dir_all(&repo.git_dir).ok()?;
                }

                repo.repo_path(vec!["branches"], Some(true), Some(false))?;
                repo.repo_path(vec!["objects"], Some(true), Some(false))?;
                repo.repo_path(vec!["refs", "tags"], Some(true), Some(false))?;
                repo.repo_path(vec!["refs", "heads"], Some(true), Some(false))?;

                let desc = repo.repo_path(vec!["description"], Some(false), Some(true))?;
                fs::write(desc, "Unnamed repository; edit this file 'description' to name the repository.\n")?;

                let head = repo.repo_path(vec!["HEAD"], Some(false), Some(true))?;
                fs::write(head, "ref: refs/heads/master\n")?;

                let config = repo.repo_path(vec!["config"], Some(false), Some(true))?;
                repo.config.write(config)?;

                Some(())
            };
            if success.is_none() {
                Err(RepositoryInitError::IOError)
            } else {
                Ok(repo)
            }
        }
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
                panic!("Tried to create dir where there already was a file: {}", &res_path);
            }
            if std::fs::create_dir_all(&res_path).is_err() {
                warn!("Failed to create {}", &res_path);
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
}

#[derive(Debug)]
enum RepositoryLoadError {
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
enum RepositoryInitError {
    NotADirectory,
    AlreadyInitialized,
    IOError,
}

struct RepoConfig {
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
    fn read(path: PathBuf) -> Self {
        Self::default() // TODO
    }

    fn write(&self, path: PathBuf) -> Option<()> {
        let txt = format!("[core]\n  repositoryformatversion = {}\n  filemode = {}\n  bare = {}\n",
            self.repository_format_version,
            self.file_mode,
            self.bare,
        );

        fs::write(path, txt)?;

        Some(())
    }
}
impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            repository_format_version: 0,
            file_mode: false,
            bare: false,
        }
    }
}