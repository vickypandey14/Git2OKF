use crate::core::errors::Git2OkfError;
use crate::core::types::AnalysisResult;
use crate::output::formatter::OutputFormatter;

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, Git2OkfError> {
        let json_str = serde_json::to_string_pretty(result)?;
        Ok(json_str)
    }
}
