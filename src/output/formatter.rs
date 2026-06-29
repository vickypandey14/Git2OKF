use crate::core::errors::Git2OkfError;
use crate::core::types::AnalysisResult;

pub trait OutputFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, Git2OkfError>;
}
