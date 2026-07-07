use crate::core::errors::Git2OkfError;
use crate::detector::Detector;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

pub struct DependencyDetector;

impl Detector for DependencyDetector {
    type Output = Vec<Dependency>;

    fn detect(&self, path: &Path) -> Result<Self::Output, Git2OkfError> {
        debug!("Starting dependency detection");
        let mut deps = Vec::new();

        // Check package.json
        let pkg_json_path = path.join("package.json");
        if pkg_json_path.exists() {
            if let Ok(content) = fs::read_to_string(&pkg_json_path) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(dependencies) = json.get("dependencies").and_then(|v| v.as_object())
                    {
                        for (name, version) in dependencies {
                            deps.push(Dependency {
                                name: name.clone(),
                                version: version.as_str().unwrap_or("unknown").to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Check composer.json
        let comp_json_path = path.join("composer.json");
        if comp_json_path.exists() {
            if let Ok(content) = fs::read_to_string(&comp_json_path) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(require) = json.get("require").and_then(|v| v.as_object()) {
                        for (name, version) in require {
                            deps.push(Dependency {
                                name: name.clone(),
                                version: version.as_str().unwrap_or("unknown").to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Check requirements.txt
        let req_txt_path = path.join("requirements.txt");
        if req_txt_path.exists() {
            if let Ok(content) = fs::read_to_string(&req_txt_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    if let Some((name, version)) = line.split_once("==") {
                        deps.push(Dependency {
                            name: name.trim().to_string(),
                            version: version.trim().to_string(),
                        });
                    } else if let Some((name, version)) = line.split_once(">=") {
                        deps.push(Dependency {
                            name: name.trim().to_string(),
                            version: format!(">={}", version.trim()),
                        });
                    } else {
                        deps.push(Dependency {
                            name: line.to_string(),
                            version: "latest".to_string(),
                        });
                    }
                }
            }
        }

        debug!("Detected {} dependencies", deps.len());
        Ok(deps)
    }
}
