use crate::core::errors::Git2OkfError;
use std::path::Path;
use std::collections::HashMap;

pub fn detect_language(path: &Path) -> Result<String, Git2OkfError> {
    let mut ext_counts: HashMap<String, usize> = HashMap::new();

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
            *ext_counts.entry(ext.to_string()).or_insert(0) += 1;
        }
    }

    let mut top_lang = "unknown".to_string();
    let mut max_count = 0;

    for (ext, count) in ext_counts {
        let lang = match ext.as_str() {
            "rs" => "rust",
            "js" | "jsx" => "javascript",
            "ts" | "tsx" => "typescript",
            "py" => "python",
            "php" => "php",
            "java" => "java",
            "go" => "go",
            _ => continue,
        };

        if count > max_count {
            max_count = count;
            top_lang = lang.to_string();
        }
    }

    Ok(top_lang)
}
