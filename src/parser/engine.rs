use crate::core::errors::Git2OkfError;
use crate::parser::ast::{ParsedFile, RepositoryParseResult};
use crate::parser::registry::ParserRegistry;
use std::path::Path;
use tracing::{debug, info};
use walkdir::WalkDir;

pub struct ParserEngine {
    registry: ParserRegistry,
}

impl ParserEngine {
    pub fn new(registry: ParserRegistry) -> Self {
        Self { registry }
    }

    pub fn parse_repository(&self, repo_name: &str, repo_path: &Path) -> Result<RepositoryParseResult, Git2OkfError> {
        info!("Starting parser engine on repository: {}", repo_name);
        let mut parsed_files = Vec::new();
        let mut total_functions = 0;
        let mut total_classes = 0;

        for entry in WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Some(parser) = self.registry.get(ext) {
                    debug!("Parsing file: {:?}", path);
                    match parser.parse(path) {
                        Ok(parsed_file) => {
                            total_functions += parsed_file.functions.len();
                            total_classes += parsed_file.classes.len();
                            parsed_files.push(parsed_file);
                        }
                        Err(e) => {
                            debug!("Failed to parse {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        info!("Parsed {} files", parsed_files.len());

        Ok(RepositoryParseResult {
            repository_name: repo_name.to_string(),
            files: parsed_files,
            total_functions,
            total_classes,
            total_dependencies: 0, // This will be populated from the detector later
        })
    }
}
