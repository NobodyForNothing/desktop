use crate::git::repo::Repository;

pub mod git;

// TODO: simple wrapper api
pub fn git_checkout(repository: Repository, commit_hash: String) -> bool {
    false
}
