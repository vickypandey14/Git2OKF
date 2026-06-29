use crate::parser::Parser;
use std::collections::HashMap;

pub struct ParserRegistry {
    parsers: HashMap<String, Box<dyn Parser<Output = crate::parser::ast::ParsedFile> + Send + Sync>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
        }
    }

    pub fn register(&mut self, extension: &str, parser: Box<dyn Parser<Output = crate::parser::ast::ParsedFile> + Send + Sync>) {
        self.parsers.insert(extension.to_string(), parser);
    }

    pub fn get(&self, extension: &str) -> Option<&Box<dyn Parser<Output = crate::parser::ast::ParsedFile> + Send + Sync>> {
        self.parsers.get(extension)
    }
}
