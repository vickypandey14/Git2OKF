use crate::core::errors::Git2OkfError;
use std::path::Path;

pub fn detect_framework(path: &Path, language: &str) -> Result<String, Git2OkfError> {
    let mut framework = "unknown".to_string();

    let has_file = |name: &str| path.join(name).exists();

    match language {
        "php" => {
            if has_file("composer.json") && has_file("artisan") {
                framework = "laravel".to_string();
            }
        }
        "javascript" | "typescript" => {
            if has_file("package.json") {
                if has_file("next.config.js") || has_file("next.config.mjs") {
                    framework = "next.js".to_string();
                } else if has_file("nuxt.config.js") || has_file("nuxt.config.ts") {
                    framework = "nuxt".to_string();
                } else if has_file("vue.config.js") {
                    framework = "vue".to_string();
                } else {
                    // Would need package.json parsing for React/Express in real life
                    framework = "node".to_string();
                }
            }
        }
        "python" => {
            if has_file("requirements.txt") || has_file("pyproject.toml") {
                if has_file("manage.py") {
                    framework = "django".to_string();
                } else {
                    framework = "flask".to_string(); // Simplified for Phase 1
                }
            }
        }
        "rust" => {
            if has_file("Cargo.toml") {
                framework = "rust".to_string(); // Cargo projects
            }
        }
        "java" => {
            if has_file("pom.xml") || has_file("build.gradle") {
                framework = "spring boot".to_string(); // Simplified
            }
        }
        _ => {}
    }

    Ok(framework)
}
