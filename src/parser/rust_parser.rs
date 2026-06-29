use crate::core::errors::Git2OkfError;
use crate::parser::ast::{CallNode, ClassNode, FunctionNode, ImportNode, ParsedFile};
use crate::parser::Parser;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser as TsParser, Query, QueryCursor};

pub struct RustParser;

impl RustParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for RustParser {
    type Output = ParsedFile;

    fn parse(&self, file_path: &Path) -> Result<Self::Output, Git2OkfError> {
        let source_code = fs::read_to_string(file_path).map_err(Git2OkfError::IoError)?;

        let mut parser = TsParser::new();
        let language = tree_sitter_rust::language();
        parser
            .set_language(language)
            .map_err(|e| Git2OkfError::DetectionError(e.to_string()))?;

        let tree = parser
            .parse(&source_code, None)
            .ok_or_else(|| Git2OkfError::DetectionError("Failed to parse AST".to_string()))?;

        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut imports = Vec::new();
        let mut calls = Vec::new();

        let mut cursor = QueryCursor::new();

        // 1. Functions
        if let Ok(func_query) = Query::new(language, "(function_item name: (identifier) @name)") {
            let func_matches = cursor.matches(&func_query, tree.root_node(), source_code.as_bytes());
            for m in func_matches {
                for cap in m.captures {
                    let node = cap.node;
                    functions.push(FunctionNode {
                        id: format!("fn_{}_{}", node.start_position().row, node.start_position().column),
                        name: node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string(),
                        line_start: node.start_position().row,
                        line_end: node.end_position().row,
                        visibility: None,
                        parameters: Vec::new(),
                        return_type: None,
                        is_async: false,
                        parent_class: None,
                        decorators: Vec::new(),
                    });
                }
            }
        }

        // 2. Classes (Structs/Traits)
        if let Ok(class_query) = Query::new(language, "(struct_item name: (type_identifier) @name)") {
            let class_matches = cursor.matches(&class_query, tree.root_node(), source_code.as_bytes());
            for m in class_matches {
                for cap in m.captures {
                    let node = cap.node;
                    classes.push(ClassNode {
                        id: format!("struct_{}_{}", node.start_position().row, node.start_position().column),
                        name: node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string(),
                        line_start: node.start_position().row,
                        line_end: node.end_position().row,
                        visibility: None,
                        decorators: Vec::new(),
                        parent_class: None,
                    });
                }
            }
        }

        if let Ok(trait_query) = Query::new(language, "(trait_item name: (type_identifier) @name)") {
            let trait_matches = cursor.matches(&trait_query, tree.root_node(), source_code.as_bytes());
            for m in trait_matches {
                for cap in m.captures {
                    let node = cap.node;
                    classes.push(ClassNode {
                        id: format!("trait_{}_{}", node.start_position().row, node.start_position().column),
                        name: node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string(),
                        line_start: node.start_position().row,
                        line_end: node.end_position().row,
                        visibility: None,
                        decorators: Vec::new(),
                        parent_class: None,
                    });
                }
            }
        }

        // 3. Imports
        if let Ok(import_query) = Query::new(language, "(use_declaration) @use") {
            let import_matches = cursor.matches(&import_query, tree.root_node(), source_code.as_bytes());
            for m in import_matches {
                for cap in m.captures {
                    let node = cap.node;
                    imports.push(ImportNode {
                        name: node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string(),
                        line_start: node.start_position().row,
                        line_end: node.end_position().row,
                    });
                }
            }
        }

        // 4. Calls
        if let Ok(call_query) = Query::new(language, "(call_expression function: (identifier) @func_name)") {
            let call_matches = cursor.matches(&call_query, tree.root_node(), source_code.as_bytes());
            for m in call_matches {
                for cap in m.captures {
                    let node = cap.node;
                    calls.push(CallNode {
                        name: node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string(),
                        line_start: node.start_position().row,
                        line_end: node.end_position().row,
                    });
                }
            }
        }

        Ok(ParsedFile {
            path: file_path.to_string_lossy().to_string(),
            language: "rust".to_string(),
            classes,
            functions,
            imports,
            calls,
            checksum: None,
        })
    }
}
