use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub repository: String,
    pub language: String,
    pub framework: String,
    pub files: usize,
    pub branch: String,
}
