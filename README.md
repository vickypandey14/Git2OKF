# Git2OKF

Convert any Git repository into structured AI-readable knowledge using Open Knowledge Format (OKF).

## Mission

Allow AI agents and LLMs to understand codebases structurally instead of relying on chunking and embeddings alone. Git2OKF analyzes source code architecture, relationships, dependencies, and internal project structure to export a standardized knowledge package.

## Project Status

The project is being developed in strict architectural phases. Currently, **Phase 1** is complete, and **Phase 2** is architecturally implemented but **NOT verified** (pending local verification of native C++ build tools required by `tree-sitter`).

> [!WARNING]
> **Architecture Freeze**: The project is currently frozen. No Phase 3 work, new modules, or code generation will occur until all verification commands in the root-level [verification.md](file:///c:/Users/vicky/OneDrive/Desktop/Git2OKF/verification.md) are run and pass successfully.

### Phase 1: Engine Foundation (Complete)
This phase built the core scaffolding, CLI, network layers, and repository-level metadata extractors.

| Feature Area | What it Does (Non-Technical Summary) | Status |
|---|---|---|
| Core Engine & Architecture | The foundation is fully built and highly modular. It is designed to easily plug in new features in the future. | Complete |
| Command Line Interface | Users can run the tool via the terminal by simply providing a repository URL. It supports both JSON and YAML output formats. | Complete |
| Smart Repository Cloning | Securely downloads code into temporary workspaces that automatically delete themselves after use. Includes timeout protection and shallow cloning to save bandwidth. | Complete |
| Multi-Language Detection | Scans the codebase to calculate the exact percentage of languages used (e.g., 80% Python, 20% JavaScript). | Complete |
| Framework Identification | Uses a point-based confidence system to reliably detect frameworks like Next.js, Laravel, or Django based on specific file structures. | Complete |
| Dependency Extraction | Automatically reads manifest files (like package.json or requirements.txt) to list what external packages the project relies on. | Complete |
| Security & Validation | Pre-checks repository URLs for errors before attempting downloads and handles network timeouts gracefully. | Complete |

### Phase 2: AST Parser Intelligence (Implemented / Unverified)
This phase transitioned the tool from guessing files to actively reading and understanding actual code syntax across 4 major languages using `tree-sitter`.

| Feature Area | What it Does (Non-Technical Summary) | Status |
|---|---|---|
| Advanced Syntax Parsing | Understands the actual structure of Rust, PHP, Python, and JavaScript code. It knows exactly where functions, classes, and imports live instead of just scanning text. | Implemented (Pending Verification) |
| Function Call Tracing | Extracts exactly which functions are calling other functions (e.g., identifying that a file calls `User::find()`), setting the stage for deep relationship mapping. | Implemented (Pending Verification) |
| Universal Syntax Standard | No matter what language it scans, it normalizes the extracted code into a single, unified data structure so AI can understand all codebases equally. | Implemented (Pending Verification) |
| Smart Language Dispatch | Automatically detects the file type (e.g., `.rs` or `.php`) and routes it to the correct specialized parser without hardcoding. | Implemented (Pending Verification) |
| Selective Compilation | The system is designed with feature flags so developers can choose to only compile the languages they care about to save time and file size. | Implemented (Pending Verification) |

### Quality Assurance (Ongoing)
| Feature Area | What it Does (Non-Technical Summary) | Status |
|---|---|---|
| Automated Testing | Fully covered by isolated integration tests and an automated CI pipeline to ensure zero broken code reaches production. | Complete |

## Setup and Usage

WSL (Ubuntu) is the officially supported development environment for Git2OKF. 

* For WSL/Ubuntu setup instructions (installing compilers, cmake, openssl-dev, and Rust), follow the [WSL Setup Guide](file:///c:/Users/vicky/OneDrive/Desktop/Git2OKF/docs/wsl_setup.md).
* Alternatively, if building natively on Windows, you must install the Microsoft Visual Studio C++ Build Tools and Rust.

Refer to [verification.md](file:///c:/Users/vicky/OneDrive/Desktop/Git2OKF/verification.md) for the complete lint/test check list.

### Basic Compilation
By default, compiling the engine includes the Rust parser:
```bash
cargo build --release
cargo run -- analyze <repository_url> --format json
```

### Selective Compilation (Feature Flags)
Because Git2OKF uses `tree-sitter`, you can selectively compile only the parsers you need to save massive amounts of build time and binary size.

```bash
# Compile with all parsers
cargo build --release --features "rust,php,javascript,python"

# Compile for a specific stack (e.g., full-stack JS + PHP)
cargo build --release --features "javascript,php"
```
