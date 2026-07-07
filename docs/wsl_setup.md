# WSL (Ubuntu) Development Setup Guide

This guide describes how to configure a clean, Linux-based development environment using Windows Subsystem for Linux (WSL) running Ubuntu to compile, test, and verify **Git2OKF**. Using WSL bypasses the need to install the Microsoft Visual Studio C++ Build Tools on host Windows.

---

## Required Ubuntu Packages

Since Git2OKF relies on native libraries (`git2` uses libgit2, `tree-sitter` uses C/C++ parsers, and `reqwest` requires TLS support), the following Ubuntu packages must be installed:

* `build-essential` (installs `gcc`, `g++`, `make`, and essential system headers)
* `cmake` (required by `libgit2` compilation step in the `git2` crate)
* `pkg-config` (required to discover system dependencies like SSL)
* `libssl-dev` (required for SSL/TLS support inside `reqwest` and `git2`)

To install these, run:
```bash
sudo apt update
sudo apt install -y build-essential cmake pkg-config libssl-dev
```

---

## Rust Installation Steps

To install Rust inside your WSL instance, perform the following steps:

1. Run the official `rustup` installer:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Press `1` to proceed with the default installation.
3. Configure your current shell sessions:
   ```bash
   source "$HOME/.cargo/env"
   ```
4. Verify the installation:
   ```bash
   rustc --version
   cargo --version
   ```

---

## Build & Test Commands

Under WSL, compilation and testing utilize standard Cargo commands:

### Compilation
```bash
# Build with default features (Rust parser)
cargo build

# Build with all parsers enabled
cargo build --all-features

# Build for production
cargo build --release --all-features
```

### Testing
```bash
# Test all parser extractions and frameworks
cargo test --all-features
```

### Static Analysis & Formatting
```bash
# Check style formatting
cargo fmt --check

# Check lints and verify warnings
cargo clippy --all-features
```

---

## Troubleshooting Common Crate Compilation Issues

### 1. `openssl-sys` or `git2` failing to link
* **Error**: `Could not find directory of OpenSSL installation` or `pkg-config failed`.
* **Cause**: The compiler cannot find the development files for OpenSSL, which is required for HTTP/SSH transport in git operations and web requests.
* **Resolution**: Install `libssl-dev` and `pkg-config`:
  ```bash
  sudo apt install -y libssl-dev pkg-config
  ```

### 2. `git2` failing due to missing `cmake`
* **Error**: `Failed to find CMake. Please install CMake` during building libgit2.
* **Cause**: `git2` compiles libgit2 from source using CMake.
* **Resolution**: Install `cmake`:
  ```bash
  sudo apt install -y cmake
  ```

### 3. `tree-sitter` wrapper compiler error
* **Error**: `cc` or `c++` command not found while compiling `tree-sitter-rust`, `tree-sitter-php`, etc.
* **Cause**: Compilation of tree-sitter S-expression parsers requires a C/C++ compiler.
* **Resolution**: Install `build-essential`:
  ```bash
  sudo apt install -y build-essential
  ```
