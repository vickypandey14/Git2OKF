use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryParseResult {
    pub repository_name: String,
    pub files: Vec<ParsedFile>,
    pub total_functions: usize,
    pub total_classes: usize,
    pub total_dependencies: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedFile {
    pub path: String,
    pub language: String,
    pub classes: Vec<ClassNode>,
    pub functions: Vec<FunctionNode>,
    pub imports: Vec<ImportNode>,
    pub calls: Vec<CallNode>,
    pub checksum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassNode {
    pub id: String,
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub visibility: Option<String>,
    pub decorators: Vec<String>,
    pub parent_class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionNode {
    pub id: String,
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub visibility: Option<String>,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub parent_class: Option<String>,
    pub decorators: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportNode {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallNode {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
}
