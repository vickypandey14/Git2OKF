# Git2OKF

Convert any Git repository into structured AI-readable knowledge using Open Knowledge Format (OKF).

## Mission

Allow AI agents and LLMs to understand codebases structurally instead of relying on chunking and embeddings alone. Git2OKF analyzes source code architecture, relationships, dependencies, and internal project structure to export a standardized knowledge package.

## Final Pre-Phase 2 Audit and Current Project Status

The Phase 1 foundation is completely built. The engine can successfully clone, scan, and categorize repositories. The current status of the capabilities is detailed below in plain terms.

| Feature Area | What it Does (Non-Technical Summary) | Status |
|---|---|---|
| Core Engine & Architecture | The foundation is fully built and highly modular. It is designed to easily plug in new parser features in the future. | Complete |
| Command Line Interface | Users can run the tool via the terminal by simply providing a repository URL. It supports both JSON and YAML output formats. | Complete |
| Smart Repository Cloning | Securely downloads code into temporary workspaces that automatically delete themselves after use. Includes timeout protection and shallow cloning to save bandwidth. | Complete |
| Multi-Language Detection | Scans the codebase to calculate the exact percentage of languages used (e.g., 80% Python, 20% JavaScript). | Complete |
| Framework Identification | Uses a point-based confidence system to reliably detect frameworks like Next.js, Laravel, or Django based on specific file structures. | Complete |
| Dependency Extraction | Automatically reads manifest files (like package.json or requirements.txt) to list what external packages the project relies on. | Complete |
| Security & Validation | Pre-checks repository URLs for errors before attempting downloads and handles network timeouts gracefully. | Complete |
| Quality Assurance | Fully covered by isolated integration tests and an automated CI pipeline to ensure zero broken code reaches production. | Complete |

## Setup and Usage

Ensure you have the Microsoft Visual Studio C++ Build Tools and Rust installed via rustup. Then run:

```bash
cargo build --release
cargo run -- analyze <repository_url> --format json
```
