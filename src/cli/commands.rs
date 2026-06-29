use crate::core::errors::Git2OkfError;
use crate::git::clone::clone_repository;
use crate::git::metadata::extract_metadata;
use crate::detector::language::detect_language;
use crate::detector::framework::detect_framework;
use crate::core::types::ProjectMetadata;
use tempfile::tempdir;

pub async fn handle_analyze(url: &str) -> Result<(), Git2OkfError> {
    // 1. Create temporary directory
    let temp_dir = tempdir().map_err(Git2OkfError::IoError)?;
    let clone_path = temp_dir.path();

    // 2. Clone repository
    let repo = clone_repository(url, clone_path)?;

    // 3. Extract metadata
    let (repo_name, branch, commit_count) = extract_metadata(&repo, url)?;

    // 4. Detect language and framework
    let language = detect_language(clone_path)?;
    let framework = detect_framework(clone_path, &language)?;
    
    // Count files roughly
    let files_count = walkdir::WalkDir::new(clone_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count();

    // 5. Output JSON
    let metadata = ProjectMetadata {
        repository: repo_name,
        language,
        framework,
        files: files_count,
        branch,
    };

    let json_output = serde_json::to_string_pretty(&metadata)?;
    println!("{}", json_output);

    // Temp dir is automatically cleaned up here
    Ok(())
}
