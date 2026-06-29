use crate::core::errors::Git2OkfError;
use crate::git::clone::clone_repository;
use crate::git::metadata::extract_metadata;
use crate::detector::language::analyze_files_and_languages;
use crate::detector::framework::detect_framework;
use crate::core::types::AnalysisResult;
use tempfile::tempdir;
use tracing::{info, debug};

pub async fn handle_analyze(url: &str) -> Result<(), Git2OkfError> {
    info!("Starting analysis for repository: {}", url);
    
    // 1. Create temporary directory
    let temp_dir = tempdir().map_err(Git2OkfError::IoError)?;
    let clone_path = temp_dir.path();
    debug!("Created temporary directory at {:?}", clone_path);

    // 2. Clone repository
    info!("Cloning repository...");
    let repo = clone_repository(url, clone_path)?;

    // 3. Extract metadata
    info!("Extracting repository metadata...");
    let (repo_name, branch, commit_count) = extract_metadata(&repo, url)?;
    debug!("Repo: {}, Branch: {}, Commits: {}", repo_name, branch, commit_count);

    // 4. Detect language and files
    info!("Analyzing files and languages...");
    let (languages, file_stats) = analyze_files_and_languages(clone_path)?;
    
    // 5. Detect framework
    info!("Detecting framework...");
    let framework_detection = detect_framework(clone_path, &languages)?;

    // 6. Output JSON
    info!("Finalizing analysis result...");
    let result = AnalysisResult {
        repository: repo_name,
        branch,
        languages,
        framework: framework_detection,
        files: file_stats,
    };

    let json_output = serde_json::to_string_pretty(&result)?;
    println!("{}", json_output);

    Ok(())
}
