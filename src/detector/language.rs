use crate::core::errors::Git2OkfError;
use crate::core::types::{FileStats, LanguageStat};
use crate::detector::Detector;
use std::collections::HashMap;
use std::path::Path;
use tracing::debug;

pub struct LanguageDetector;

impl Detector for LanguageDetector {
    type Output = (Vec<LanguageStat>, FileStats);

    fn detect(&self, path: &Path) -> Result<Self::Output, Git2OkfError> {
        debug!("Starting file and language analysis at {:?}", path);
        let mut ext_counts: HashMap<String, usize> = HashMap::new();
        let mut total_files = 0;
        let mut source_files = 0;
        let mut config_files = 0;

        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            total_files += 1;
            let file_name = entry.file_name().to_string_lossy().to_lowercase();

            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                let ext = ext.to_lowercase();

                match ext.as_str() {
                    "json" | "toml" | "yaml" | "yml" | "xml" | "ini" | "env" | "config" => {
                        config_files += 1;
                    }
                    "rs" | "js" | "jsx" | "ts" | "tsx" | "py" | "php" | "java" | "go" | "c"
                    | "cpp" | "h" | "cs" | "rb" | "swift" | "kt" => {
                        source_files += 1;
                        *ext_counts.entry(ext).or_insert(0) += 1;
                    }
                    _ => {
                        if file_name == "dockerfile"
                            || file_name == "makefile"
                            || file_name.ends_with("rc")
                        {
                            config_files += 1;
                        }
                    }
                }
            } else {
                if file_name == "dockerfile" || file_name == "makefile" {
                    config_files += 1;
                }
            }
        }

        let mut language_map: HashMap<String, usize> = HashMap::new();
        for (ext, count) in ext_counts {
            let lang = match ext.as_str() {
                "rs" => "rust",
                "js" | "jsx" => "javascript",
                "ts" | "tsx" => "typescript",
                "py" => "python",
                "php" => "php",
                "java" => "java",
                "go" => "go",
                "c" | "h" => "c",
                "cpp" | "hpp" => "cpp",
                "cs" => "csharp",
                "rb" => "ruby",
                "swift" => "swift",
                "kt" => "kotlin",
                _ => continue,
            };
            *language_map.entry(lang.to_string()).or_insert(0) += count;
        }

        let mut languages: Vec<LanguageStat> = Vec::new();
        if source_files > 0 {
            for (name, count) in language_map {
                let percentage = ((count as f64 / source_files as f64) * 100.0).round() as u8;
                languages.push(LanguageStat { name, percentage });
            }
        }

        languages.sort_by(|a, b| b.percentage.cmp(&a.percentage));

        let stats = FileStats {
            total_files,
            source_files,
            config_files,
        };

        debug!(
            "Finished analysis: {} total files, {} source files, {} config files",
            total_files, source_files, config_files
        );
        Ok((languages, stats))
    }
}
