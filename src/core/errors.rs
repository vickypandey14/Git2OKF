use thiserror::Error;

#[derive(Error, Debug)]
pub enum Git2OkfError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Detection error: {0}")]
    DetectionError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
