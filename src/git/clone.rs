use crate::core::errors::Git2OkfError;
use git2::build::RepoBuilder;
use git2::{FetchOptions, Repository};
use std::path::Path;
use std::time::Duration;
use tokio::time::timeout;
use tracing::info;

pub async fn clone_repository(
    url: &str,
    target_path: &Path,
    depth: Option<u32>,
) -> Result<Repository, Git2OkfError> {
    info!("Cloning {} to {:?} (depth: {:?})", url, target_path, depth);

    let url = url.to_string();
    let target_path = target_path.to_path_buf();

    let clone_task = tokio::task::spawn_blocking(move || {
        let mut builder = RepoBuilder::new();
        let mut fetch_opts = FetchOptions::new();

        if let Some(d) = depth {
            // Note: libgit2 supports shallow clone depth via fetch options
            fetch_opts.depth(d as i32);
        }

        builder.fetch_options(fetch_opts);
        builder.clone(&url, &target_path)
    });

    match timeout(Duration::from_secs(60), clone_task).await {
        Ok(Ok(Ok(repo))) => Ok(repo),
        Ok(Ok(Err(e))) => Err(Git2OkfError::GitError(e)),
        Ok(Err(e)) => Err(Git2OkfError::Unknown(format!("Task panic: {}", e))),
        Err(_) => Err(Git2OkfError::TimeoutError(
            "Clone operation timed out after 60 seconds".to_string(),
        )),
    }
}
