use crate::core::errors::Git2OkfError;
use crate::core::types::AnalysisResult;
use crate::output::formatter::OutputFormatter;

pub struct YamlFormatter;

impl OutputFormatter for YamlFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, Git2OkfError> {
        let yaml_str = serde_yaml::to_string(result).map_err(|e| Git2OkfError::Unknown(e.to_string()))?;
        Ok(yaml_str)
    }
}
