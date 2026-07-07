# Phase 2 AST Parser Verification Checklist

This document tracks the verification process for the **Phase 2 AST Parser Intelligence** implementation. Due to local dependencies on compiler tools (required by `tree-sitter` and `git2` crates), Phase 2 is currently **Implemented** but **NOT VERIFIED**.

WSL (Windows Subsystem for Linux running Ubuntu) is the **officially supported** primary development and verification environment.

> [!IMPORTANT]
> The project architecture is currently **FROZEN**. No Phase 3 work, new modules, or code generation may begin until all verification steps in this checklist compile and pass successfully on either verification path.

---

## Verification Checklist

Follow the steps below to verify the architecture:

- [ ] **Step 1: Environment Preparation**
  * Select a verification path below and set up compiler tooling.
  
- [ ] **Step 2: Clean and Format Verification**
  * Verify syntax formatting.
  * Run clippy for code linting and sanity checks.

- [ ] **Step 3: Feature-Gate Build Verification**
  * Build each individual parser compiler path to verify feature isolation.
  * Build with all features enabled.

- [ ] **Step 4: Parser Unit Test Verification**
  * Run tests for each language parser feature flag.
  * Run all integration and framework tests.

---

## Environment Verification Paths

Choose either Option A (Recommended) or Option B to set up your environment toolchain:

### Option A: WSL / Linux Path (Officially Supported)
This path is the primary development environment and avoids installing heavy Windows IDE dependencies.

1. Ensure WSL (Ubuntu) is installed.
2. Install the necessary system dependencies:
   ```bash
   sudo apt update
   sudo apt install -y build-essential cmake pkg-config libssl-dev
   ```
3. Install Rust using the rustup script:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```
4. Refer to the full [wsl_setup.md](file:///c:/Users/vicky/OneDrive/Desktop/Git2OKF/docs/wsl_setup.md) for detailed configuration and troubleshooting.

### Option B: Windows MSVC Path
This path compiles code natively on Windows using Visual Studio C++ build tools.

1. Install the Microsoft Visual Studio C++ Build Tools (specifically the "Desktop development with C++" workload).
2. Install Rust for Windows via `rustup-init.exe`.

---

## Required Commands

Once your chosen environment path is configured, execute the following commands inside your terminal (Ubuntu bash for WSL, or PowerShell/CMD for Windows MSVC) to verify the build:

### 1. Code Style and Linters
Verify style guidelines and run static analysis across all feature sets:
```bash
# Check code formatting (must return exit code 0)
cargo fmt --check

# Run Clippy static analyzer (must compile with zero warnings/errors)
cargo clippy --all-features
```

### 2. Compilation (Feature Isolation)
Verify that each feature flag compiles in isolation without missing dependencies:
```bash
# Build Rust parser target only
cargo build --features rust

# Build PHP parser target only
cargo build --features php

# Build JavaScript parser target only
cargo build --features javascript

# Build Python parser target only
cargo build --features python

# Build all parser targets simultaneously
cargo build --all-features
```

### 3. Unit and Integration Tests
Verify AST nodes extraction correctness for each language parser:
```bash
# Test Rust AST parser
cargo test --features rust

# Test PHP AST parser
cargo test --features php

# Test JavaScript AST parser
cargo test --features javascript

# Test Python AST parser
cargo test --features python

# Test everything (all parsers + framework + cleanup tests)
cargo test --all-features
```

---

## Status Summary
* **Phase 2 Status**: `IMPLEMENTED`
* **Verification Status**: `PENDING`
* **Current Action**: Awaiting environment setup and local verification. Phase 3 (Graph & Analyzer) is strictly blocked until this checklist is fully checked off.
