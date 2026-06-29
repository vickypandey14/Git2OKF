# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-06-29

### Added
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
