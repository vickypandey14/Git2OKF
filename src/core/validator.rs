use crate::core::errors::Git2OkfError;

pub fn validate_git_url(url: &str) -> Result<(), Git2OkfError> {
    if url.trim().is_empty() {
        return Err(Git2OkfError::ConfigError("URL cannot be empty".to_string()));
    }
    
    // Very basic check, in reality we'd parse with URL crate
    if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("git@") {
        return Err(Git2OkfError::ConfigError("Invalid or unsupported Git URL protocol".to_string()));
    }

    Ok(())
}

pub fn validate_repository_access(url: &str) -> Result<(), Git2OkfError> {
    // Lightweight pre-flight check can be added here.
    // For now, we rely on the URL validation.
    // Deep access checks (like git ls-remote) could be done here, but
    // to avoid blocking indefinitely, we'll just return Ok and let the git clone handle the failure.
    validate_git_url(url)
}
