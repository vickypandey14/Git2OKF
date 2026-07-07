use crate::core::errors::Git2OkfError;
use std::path::Path;

pub trait Detector {
    type Output;
    fn detect(&self, path: &Path) -> Result<Self::Output, Git2OkfError>;
}

pub mod dependency;
pub mod framework;
pub mod language;
pub mod version;
