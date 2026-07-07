use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Framework {
    Laravel,
    NextJs,
    React,
    Vue,
    Django,
    Flask,
    NodeExpress,
    SpringBoot,
    RustCargo,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameworkDetection {
    pub framework: Framework,
    pub confidence: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageStat {
    pub name: String,
    pub percentage: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileStats {
    pub total_files: usize,
    pub source_files: usize,
    pub config_files: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisResult {
    pub repository: String,
    pub branch: String,
    pub languages: Vec<LanguageStat>,
    pub framework: FrameworkDetection,
    pub dependencies: Vec<crate::detector::dependency::Dependency>,
    pub files: FileStats,
}
