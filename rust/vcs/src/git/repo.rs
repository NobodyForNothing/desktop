use std::collections::BTreeMap;
use crate::git::objects::{
    BinSerializable, GitBlob, GitCommit, GitObject, GitObjectType, GitTag, GitTree,
};
use iniconf::{IniFile, IniFileOpenError};
use log::warn;
use sha1::{Digest, Sha1};
use std::fmt::format;
use std::fs;
use std::hash::Hash;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::git::index::GitIndex;

const MAX_REF_RESOLVE_DEPTH: u8 = 100;

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
                instance.config =
                    RepoConfig::read(path).expect("IO is possible as per check above");
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
        fs::write(
            desc,
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .ok()?;

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
    fn repo_path<P: AsRef<Path>>(
        &self,
        path_list: Vec<P>,
        mkdir: Option<bool>,
        has_file: Option<bool>,
    ) -> Option<PathBuf> {
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
                panic!(
                    "Tried to create dir where there already was a file: {:?}",
                    &res_path
                );
            }
            if fs::create_dir_all(&res_path).is_err() {
                warn!("Failed to create {:?}", &res_path);
            }
        }

        if res_path.exists() {
            if has_file {
                res_path = res_path.join(
                    path_list
                        .last()
                        .expect("repo_path won't accept empty paths."),
                )
            }
            Some(res_path)
        } else {
            None
        }
    }

    pub fn object_find(&self, name: String) -> ObjectRefResult {
        let name = name.trim();
        let obj_hash = if name == "HEAD" {
            if let Some(head) = self.head() {
                ObjectRefResult::Ok(head)
            } else {
                ObjectRefResult::NoResult
            }
        } else if name.len() == 6 {
            let path = self.repo_path(vec!["objects", &name[0..2]], None, None);
            if let Some(path) = path {
                let mut matching = Vec::new();
                if let Ok(dir) = fs::read_dir(path) {
                    for entry in dir {
                        if let Ok(entry) = entry {
                            let entry = entry.path();
                            if entry.starts_with(&name[2..6]) {
                                matching.push(entry);
                            }
                        }
                    }
                }

                if matching.len() == 1 {
                    ObjectRefResult::Ok(matching.first().unwrap().to_str().unwrap().to_string())
                } else if matching.len() < 1 {
                    ObjectRefResult::NoResult
                } else {
                    // if matching.count() > 1 {
                    ObjectRefResult::TooManyResults
                }
            } else {
                ObjectRefResult::NoResult
            }
        } else if name.len() == 20 {
            ObjectRefResult::Ok(name.to_string())
        } else {
            ObjectRefResult::NotARef
        };

        if let ObjectRefResult::Ok(obj_hash) = obj_hash {
            let path = self.repo_path(
                vec!["objects", &obj_hash[0..2], &obj_hash[2..obj_hash.len()]],
                None,
                Some(true),
            );
            if path.is_some_and(|p| p.is_file()) {
                ObjectRefResult::Ok(obj_hash)
            } else {
                ObjectRefResult::PointsToDeletedRef
            }
        } else {
            obj_hash
        }
    }

    fn head(&self) -> Option<String> {
        let obj_ref = self.repo_path(vec!["HEAD"], None, Some(true))?;
        let obj_ref = fs::read_to_string(obj_ref).ok()?;
        if let Some(obj_ref) = obj_ref.strip_prefix("ref: ") {
            let obj_ref = self.ref_resolve(&obj_ref.to_string())?;
            Some(obj_ref)
            // object_ref can only be full ref
        } else if let ObjectRefResult::Ok(hash) = self.object_find(obj_ref) {
            Some(hash)
        } else {
            None
        }
    }

    /// Load a git object by hash.
    pub fn object_read(&self, sha: String) -> Option<GitObject> {
        // let sha: String = sha.iter().map(|byte| format!("{:x}", byte)).collect();
        let path = self.repo_path(
            vec!["objects", &sha[0..2], &sha[2..sha.len()]],
            None,
            Some(true),
        );
        if path.as_ref().is_some_and(|p| p.is_file()) {
            if let Ok(data) = fs::read(path.unwrap()) {
                let data = flate2::read::ZlibDecoder::new(&data[..]);
                let mut data = data.bytes();
                let mut obj_type = String::new();
                while let Some(Ok(byte)) = data.next() {
                    if byte == b' ' {
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
                let obj_len = obj_len
                    .parse::<u64>()
                    .expect("GitObject doesn't contain size");

                let remaining_bits: Vec<u8> = data.map(|e| e.unwrap()).collect();
                assert_eq!(obj_len as usize, remaining_bits.len());

                let obj = match obj_type.as_str() {
                    "commit" => GitObject::Commit(GitCommit::deserialize(remaining_bits)),
                    "tree" => GitObject::Tree(GitTree::deserialize(remaining_bits)),
                    "tag" => GitObject::Tag(GitTag::deserialize(remaining_bits)),
                    "blob" => GitObject::Blob(GitBlob::deserialize(remaining_bits)),
                    _ => panic!("Unknown type {obj_type} for object {sha}"),
                };
                return Some(obj);
            }
        }
        None
    }

    /// Store a git object in the repo data and return its hash.
    pub fn object_write(&self, obj: GitObject) -> String {
        let obj = obj.serialize();
        let mut hasher = Sha1::new();
        hasher.update(&obj);
        let sha = hasher.finalize();
        let sha: String = sha.iter().map(|byte| format!("{:x}", byte)).collect();

        let path = self.repo_path(
            vec!["objects", &sha[..2], &sha[2..]],
            Some(true),
            Some(true),
        );
        if let Some(path) = path {
            if !path.exists() {
                fs::write(path, obj).unwrap();
            }
        }

        sha
    }

    /// Store a file at [path] in the repo.
    pub fn hash_object(&self, path: PathBuf, format: GitObjectType) -> Option<String> {
        let data = fs::read(path).ok()?;
        let data = match format {
            GitObjectType::Commit => todo!(),
            GitObjectType::Tree => todo!(),
            GitObjectType::Tag => todo!(),
            GitObjectType::Blob => GitObject::Blob(GitBlob::deserialize(data)),
        };
        Some(self.object_write(data))
    }

    /// Checks out a git tree to an empty (except git dir) work tree.
    pub fn tree_checkout(&self, tree: GitTree) -> bool {
        let mut dir = self
            .work_tree
            .read_dir()
            .expect("Work tree no longer exists");
        assert!(dir.count() <= 1, "Work tree no empty");

        self.tree_checkout_inner(tree, &self.work_tree).is_some()
    }
    fn tree_checkout_inner(&self, tree: GitTree, path: &PathBuf) -> Option<()> {
        for entry in tree.entries() {
            let path = path.join(entry.path());
            match self.object_read(entry.obj_hash().clone()) {
                Some(GitObject::Tree(tree)) => {
                    fs::create_dir(&path).ok()?;
                    self.tree_checkout_inner(tree, &path)?;
                }
                Some(GitObject::Blob(blob)) => {
                    fs::write(&path, blob.data()).ok()?;
                }
                _ => {}
            }
        }
        Some(())
    }

    /// Resolve a git ref (file path after ref) to a full object hash.
    fn ref_resolve(&self, git_ref: &String) -> Option<String> {
        self.ref_resolve_inner(git_ref, 0)
    }

    /// Resolve a git ref (file path after ref) to a full object hash.
    fn ref_resolve_inner(&self, git_ref: &String, depth: u8) -> Option<String> {
        if depth > MAX_REF_RESOLVE_DEPTH {
            panic!("ref_resolve_inner failed to resolve ref: MAX_REF_RESOLVE_DEPTH exceeded");
        }

        let mut git_ref = git_ref
            .split("/")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        let mut path = vec!["ref".to_string()];
        path.append(&mut git_ref);
        let path = self.repo_path(path, None, Some(true))?;

        let mut data = fs::read_to_string(path).ok()?;
        if let Some(stripped) = data.strip_suffix("\n") {
            data = stripped.to_string();
        }

        if data.starts_with("ref: ") {
            self.ref_resolve_inner(&data[5..].to_string(), depth + 1)
        } else {
            Some(data)
        }
    }

    /// Create a [name]d reference to an object [hash].
    fn ref_create(&self, name: String, hash: String) -> Option<()> {
        let path = self.repo_path(vec!["refs".to_string(), name], None, Some(true))?;
        fs::write(path, format!("{hash}\n")).ok()
    }

    /// Store a tag object and reference it in a tag ref.
    ///
    /// For tag refs without tag object ise [ref_create].
    pub fn create_tag(&self, tag: GitTag) -> Option<()> {
        let path = PathBuf::from("tags");
        let path = path.join(tag.tag()?);
        let tag_ref = self.object_write(GitObject::Tag(tag));

        self.ref_create(path.to_str()?.to_string(), tag_ref)
    }

    pub fn status(&self) -> Option<GitStatus> {
        let index = self.repo_path(vec!["index"], None, Some(true))?;
        let index = fs::read(index).ok()?;
        let index = GitIndex::decode(index.as_slice())?;
        let res = if let ObjectRefResult::Ok(head) = self.object_find("HEAD".to_string()) {
            if let Some(head) = self.tree_to_map("HEAD".to_string(), String::new()) {
                let mut added = Vec::new();
                let mut deleted = head.clone();
                let mut modified = Vec::new();
                let mut entries = index.entries();
                for index_entry in entries {
                    if head.get(&index_entry.0).is_some() {
                        if head.get(&index_entry.0).is_some_and(|e| e != &index_entry.1) {
                            modified.push(index_entry.0.clone());
                        }
                        deleted.remove(&index_entry.0);
                    } else {
                        added.push(index_entry.0);
                    }
                }
                Some((added, deleted, modified))
            } else { None }
        } else { None };

        if let Some((added, deleted, modified)) = res {
            Some(GitStatus {
                active_branch: self.get_active_branch().unwrap_or("HEAD".to_string()),
                added,
                modified,
                deleted: deleted.keys().map(|e| e.clone()).collect::<Vec<String>>(),
            })
        } else { None }
    }

    fn get_active_branch(&self) -> Option<String> {
        let head = self.repo_path(vec!["HEAD"], None, Some(true))?;
        let head = fs::read_to_string(head).ok()?;
        let head = head.strip_prefix("ref: refs/heads/")?;
        Some(head.to_string())
    }

    /// Creates a map from a tree with the full file path as key and the hash as value.
    fn tree_to_map(&self, tree_ref: String, prefix: String) -> Option<BTreeMap<String, String>> {
        let mut ret: BTreeMap<String, String> = BTreeMap::new();
        if let ObjectRefResult::Ok(tree) = self.object_find(tree_ref) {
            if let Some(GitObject::Tree(tree)) = self.object_read(tree) {
                for leaf in tree.entries() {
                    let path = PathBuf::from(&prefix);
                    let path = path.join(leaf.path());
                    let path = path.to_str()?.to_string();

                    match self.object_read(leaf.obj_hash().to_string()) {
                        Some(GitObject::Tree(..)) => {
                            let mut res = self.tree_to_map(leaf.obj_hash().clone(), path)?;
                            ret.append(&mut res);
                        }
                        Some(GitObject::Blob(..)) => {
                            ret.insert(path, leaf.obj_hash().clone());
                        },
                        _ => {}
                    }
                }
                Some(ret)
            } else {
                None
            }
        } else {
            None
        }
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
            let x = IniFile::open(path.clone());
            println!("{:#?}", &x);
            match x {
                Ok(file) => {
                    let version = file.get::<u8>("core", "repositoryformatversion");
                    let mode = file.get::<bool>("core", "filemode");
                    let bare = file.get::<bool>("core", "bare");
                    Some(Self {
                        file,
                        repository_format_version: version
                            .unwrap_or(Self::default().repository_format_version),
                        file_mode: mode.unwrap_or(Self::default().file_mode),
                        bare: bare.unwrap_or(Self::default().bare),
                    })
                }
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
                }
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
        self.file.set_str(
            "core",
            "repositoryformatversion",
            format!("{}", self.repository_format_version).as_str(),
        );
        self.file
            .set_str("core", "filemode", format!("{}", self.file_mode).as_str());
        self.file
            .set_str("core", "bare", format!("{}", self.bare).as_str());

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

pub enum ObjectRefResult {
    Ok(String),
    NoResult,
    TooManyResults,
    PointsToDeletedRef,
    NotARef,
}

pub struct GitStatus {
    active_branch: String,
    /// File names of new files.
    added: Vec<String>,
    /// File names of modified files.
    modified: Vec<String>,
    /// File names of removed files.
    deleted: Vec<String>,
    // TODO: changes not staged for commit
}
