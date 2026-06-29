use crate::core::errors::Git2OkfError;
use git2::Repository;
use super::repository::{get_repository_name, get_current_branch, get_commit_count};
use tracing::debug;

pub fn extract_metadata(repo: &Repository, url: &str) -> Result<(String, String, usize), Git2OkfError> {
    debug!("Extracting metadata from Git repository instance");
    let repo_name = get_repository_name(url);
    let branch = get_current_branch(repo).unwrap_or_else(|_| "main".to_string());
    let commit_count = get_commit_count(repo).unwrap_or(0);
    
    Ok((repo_name, branch, commit_count))
}
