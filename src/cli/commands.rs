use crate::core::errors::Git2OkfError;
use crate::core::types::AnalysisResult;
use crate::core::validator::validate_repository_access;
use crate::detector::framework::FrameworkDetector;
use crate::detector::language::LanguageDetector;
use crate::detector::Detector;
use crate::git::clone::clone_repository;
use crate::git::metadata::extract_metadata;
use crate::output::formatter::OutputFormatter;
use crate::output::json::JsonFormatter;
use crate::output::yaml::YamlFormatter;
use std::fs;
use tempfile::tempdir;
use tracing::{debug, info};

pub async fn handle_analyze(
    url: &str,
    format: &str,
    output_path: Option<&str>,
    depth: Option<u32>,
) -> Result<(), Git2OkfError> {
    info!("Starting analysis for repository: {}", url);

    // 0. Validate Repository
    validate_repository_access(url)?;

    // 1. Create temporary directory
    let temp_dir = tempdir().map_err(Git2OkfError::IoError)?;
    let clone_path = temp_dir.path();
    debug!("Created temporary directory at {:?}", clone_path);

    // 2. Clone repository
    info!("Cloning repository...");
    let repo = clone_repository(url, clone_path, depth).await?;

    // 3. Extract metadata
    info!("Extracting repository metadata...");
    let (repo_name, branch, commit_count) = extract_metadata(&repo, url)?;
    debug!(
        "Repo: {}, Branch: {}, Commits: {}",
        repo_name, branch, commit_count
    );

    // 4. Detect language and files
    info!("Analyzing files and languages...");
    let lang_detector = LanguageDetector;
    let (languages, file_stats) = lang_detector.detect(clone_path)?;

    // 5. Detect framework
    info!("Detecting framework...");
    let fw_detector = FrameworkDetector::new(languages.clone());
    let framework_detection = fw_detector.detect(clone_path)?;

    // 5.5 Detect dependencies
    info!("Detecting dependencies...");
    let dep_detector = crate::detector::dependency::DependencyDetector;
    let dependencies = dep_detector.detect(clone_path)?;

    // 6. Output Generation
    info!("Finalizing analysis result...");
    let result = AnalysisResult {
        repository: repo_name,
        branch,
        languages,
        framework: framework_detection,
        dependencies,
        files: file_stats,
    };

    let formatted_output = match format.to_lowercase().as_str() {
        "yaml" | "yml" => YamlFormatter.format(&result)?,
        _ => JsonFormatter.format(&result)?, // Default to json
    };

    if let Some(path) = output_path {
        fs::write(path, &formatted_output).map_err(Git2OkfError::IoError)?;
        info!("Output successfully written to {}", path);
    } else {
        println!("{}", formatted_output);
    }

    Ok(())
}
