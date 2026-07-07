use crate::core::errors::Git2OkfError;
use git2::Repository;

pub fn get_repository_name(url: &str) -> String {
    let normalized = url.replace(':', "/");
    let parts: Vec<&str> = normalized.split('/').collect();
    if parts.len() >= 2 {
        let name = parts[parts.len() - 1].trim_end_matches(".git");
        let owner = parts[parts.len() - 2];
        format!("{}/{}", owner, name)
    } else {
        "unknown/unknown".to_string()
    }
}

pub fn get_current_branch(repo: &Repository) -> Result<String, Git2OkfError> {
    let head = repo.head()?;
    let name = head.shorthand().unwrap_or("unknown");
    Ok(name.to_string())
}

pub fn get_commit_count(repo: &Repository) -> Result<usize, Git2OkfError> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    Ok(revwalk.count())
}
