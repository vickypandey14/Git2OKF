use crate::core::errors::Git2OkfError;
use std::path::Path;

pub mod ast;
pub mod registry;
pub mod engine;

#[cfg(feature = "rust")]
pub mod rust_parser;

#[cfg(feature = "php")]
pub mod php_parser;

#[cfg(feature = "javascript")]
pub mod javascript_parser;

#[cfg(feature = "python")]
pub mod python_parser;

pub trait Parser {
    type Output;
    fn parse(&self, file_path: &Path) -> Result<Self::Output, Git2OkfError>;
}
