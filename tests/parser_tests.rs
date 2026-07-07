#[cfg(feature = "rust")]
#[test]
fn test_rust_parser_extraction() {
    use git2okf::parser::rust_parser::RustParser;
    use git2okf::parser::Parser;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.rs");
    let mut file = File::create(&file_path).unwrap();

    let code = r#"
        use std::collections::HashMap;

        struct MyStruct {
            field: i32,
        }

        fn calculate() {
            println!("Hello");
        }
    "#;

    file.write_all(code.as_bytes()).unwrap();

    let parser = RustParser::new();
    let result = parser.parse(&file_path).unwrap();

    assert_eq!(result.functions.len(), 1);
    assert_eq!(result.functions[0].name, "calculate");

    assert_eq!(result.classes.len(), 1);
    assert_eq!(result.classes[0].name, "MyStruct");

    assert_eq!(result.imports.len(), 1);
}

#[cfg(feature = "php")]
#[test]
fn test_php_parser_extraction() {
    use git2okf::parser::php_parser::PhpParser;
    use git2okf::parser::Parser;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.php");
    let mut file = File::create(&file_path).unwrap();

    let code = r#"
        <?php
        use Illuminate\Support\Facades\Route;

        class UserController {
            public function show($id) {
                User::find($id);
            }
        }
    "#;

    file.write_all(code.as_bytes()).unwrap();

    let parser = PhpParser::new();
    let result = parser.parse(&file_path).unwrap();

    assert_eq!(result.functions.len(), 1);
    assert_eq!(result.classes.len(), 1);
    assert_eq!(result.classes[0].name, "UserController");
    assert_eq!(result.calls.len(), 1);
}

#[cfg(feature = "javascript")]
#[test]
fn test_javascript_parser_extraction() {
    use git2okf::parser::javascript_parser::JavascriptParser;
    use git2okf::parser::Parser;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.js");
    let mut file = File::create(&file_path).unwrap();

    let code = r#"
        import React from 'react';

        class MyComponent {
            render() { return null; }
        }

        const runTask = () => {
            console.log("Running");
        };
    "#;

    file.write_all(code.as_bytes()).unwrap();

    let parser = JavascriptParser::new();
    let result = parser.parse(&file_path).unwrap();

    assert_eq!(result.functions.len(), 1); // Arrow function
    assert_eq!(result.classes.len(), 1);
    assert_eq!(result.classes[0].name, "MyComponent");
    assert_eq!(result.imports.len(), 1);
}

#[cfg(feature = "python")]
#[test]
fn test_python_parser_extraction() {
    use git2okf::parser::python_parser::PythonParser;
    use git2okf::parser::Parser;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.py");
    let mut file = File::create(&file_path).unwrap();

    let code = r#"
        import os
        from sys import path

        class Processor:
            def process(self):
                print("Processing")

        def start():
            Processor().process()
    "#;

    file.write_all(code.as_bytes()).unwrap();

    let parser = PythonParser::new();
    let result = parser.parse(&file_path).unwrap();

    assert_eq!(result.functions.len(), 2); // process and start
    assert_eq!(result.classes.len(), 1);
    assert_eq!(result.classes[0].name, "Processor");
    assert_eq!(result.imports.len(), 2);
    assert_eq!(result.calls.len(), 3); // print, Processor(), and Processor().process()
}
