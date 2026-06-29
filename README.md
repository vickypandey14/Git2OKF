# Git2OKF

Convert any Git repository into structured AI-readable knowledge using Open Knowledge Format (OKF).

## Mission
Allow AI agents and LLMs to understand codebases structurally instead of relying on chunking and embeddings alone. Git2OKF analyzes source code architecture, relationships, dependencies, and internal project structure to export a standardized knowledge package.

## Phase 1 (Foundation)
Current features included:
- CLI interface architecture
- Git repository cloning & metadata extraction
- Project language and framework signature detection

## Setup

Ensure you have Rust installed via [rustup](https://rustup.rs/). Then run:

```bash
cargo build
cargo run -- analyze <repository_url>
```
