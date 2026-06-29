use crate::core::errors::Git2OkfError;
use git2::Repository;
use std::path::Path;

pub fn clone_repository(url: &str, target_path: &Path) -> Result<Repository, Git2OkfError> {
    let repo = Repository::clone(url, target_path)?;
    Ok(repo)
}
