# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-06-29

> [!WARNING]
> **Phase 2 Implementation Freeze**: Phase 2 is architecturally implemented but **NOT verified** on the local Windows environment due to native C++ compiler dependency requirements (required by tree-sitter wrappers). An architecture freeze is in place. No new features, code changes, or Phase 3 additions will occur until verification commands listed in [verification.md](file:///c:/Users/vicky/OneDrive/Desktop/Git2OKF/verification.md) are run and pass.

### Added
- Phase 2 AST Parser Intelligence Architecture.
- `tree-sitter` integration with `tree-sitter-rust`, `tree-sitter-php`, `tree-sitter-javascript`, and `tree-sitter-python`.
- Tree-sitter Query API implementation for robust syntax extraction.
- Strict Intermediate Representation (IR) AST Schema (`ParsedFile`, `RepositoryParseResult`, `ClassNode`, `FunctionNode`, `CallNode`, `ImportNode`).
- Dynamic Parser Registry (`ParserRegistry`) and Parser Engine (`ParserEngine`).
- Isolated Tree-sitter AST tests for Rust, PHP, JavaScript, and Python in `tests/parser_tests.rs`.
- Scaffolded future architectures for `src/graph` and `src/analyzer`.
- Added selective compilation via Cargo `[features]` flags (e.g., `cargo build --features "php,javascript"`).
- Phase 1.75 Pre-Phase 2 mandatory architectural improvements.
- Trait-Based Detector Architecture (`Detector` trait using `Git2OkfError`).
- Output Abstraction Layer (`OutputFormatter` trait, JSON & YAML support).
- Repository Validation Layer (`validate_git_url`, `validate_repository_access`).
- Timeout Protection on clone operations (`tokio::time::timeout` 60s).
- Shallow clone support via `--depth` parameter.
- Dependency Detector reading `package.json`, `composer.json`, `requirements.txt`.
- Isolated test suite in `tests/` (`framework_tests`, `language_tests`, `cleanup_tests`, etc.).
- GitHub Actions CI Pipeline (`.github/workflows/test.yml`).
- Phase 1.5 architectural improvements.
- Strict internal Rust types (`Framework`, `AnalysisResult`, `LanguageStat`, `FileStats`) to replace loose JSON serialization.
- Multi-language support that recursively calculates precise percentage breakdowns across codebases.
- Advanced file counting separating `source_files` and `config_files`.
- Framework detection additive confidence scoring model (0-100 scale).
- Structured logging using `tracing` and `tracing-subscriber` across all core modules.
- Integration test suite (`tests/integration_tests.rs`) using dynamically generated mock directories.
- `src/lib.rs` created to cleanly expose module library functions to external test suites.
- Initial project scaffolding for Phase 1.
- `src/cli`: CLI interface implementation using `clap` for the `analyze` command.
- `src/git`: Modules for repository cloning, branch detection, and commit counting.
- `src/detector`: Modular file signature-based detection for languages and frameworks (Laravel, Next.js, Django, Rust, etc.).
- `src/core`: Foundation for errors (`thiserror`), config, and JSON output formatting.
- `Cargo.toml` dependencies prepared for asynchronous execution (`tokio`), networking (`reqwest`), parsing, and serialization.
- Stubs for future `parser`, `analyzer`, `graph`, `okf`, and `output` modules.
