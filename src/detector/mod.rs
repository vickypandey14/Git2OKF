use crate::core::errors::Git2OkfError;
use std::path::Path;

pub trait Detector {
    type Output;
    fn detect(&self, path: &Path) -> Result<Self::Output, Git2OkfError>;
}

pub mod language;
pub mod framework;
pub mod version;
pub mod dependency;
