# Contributing to Git2OKF

Thank you for your interest in contributing to Git2OKF. To maintain code quality, consistency, and stability, please follow these guidelines when submitting contributions.

---

## Development Environment Setup

We officially support Windows Subsystem for Linux (WSL) running Ubuntu as our primary development environment. 

1. Install system toolchains:
   ```bash
   sudo apt update
   sudo apt install -y build-essential cmake pkg-config libssl-dev
   ```
2. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```
3. Install code tools components:
   ```bash
   rustup component add rustfmt clippy
   ```

Refer to the full setup instructions in `docs/wsl_setup.md` for troubleshooting compiling issues with `git2` and `tree-sitter`.

---

## Coding Standards & Code Verification

Before pushing any changes or opening a pull request, you must ensure that all code compiles, conforms to guidelines, and passes checks.

Run the verification checklist in order:

1. **Formatting**:
   Ensure code conforms to standard formatting:
   ```bash
   cargo fmt --check
   ```
   To format files automatically, run:
   ```bash
   cargo fmt
   ```
2. **Clippy Lints**:
   Check for warnings and standard code smells:
   ```bash
   cargo clippy --all-features -- -D warnings
   ```
3. **Testing**:
   Run all unit, framework, parser, and integration test suites:
   ```bash
   cargo test --all-features
   ```

---

## Commit Guidelines

Keep your commits small, logical, and reviewable. Use descriptive commit messages.

### Format
We recommend structured commit prefixes:
* `feat:` (new features)
* `fix:` (bug or compiler fixes)
* `docs:` (documentation updates)
* `refactor:` (improving structure without functional changes)
* `test:` (adding or modifying tests)

Avoid using emojis in commit messages or code comments.

---

## Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Keep your branch updated with upstream changes.
3. Ensure all automated tests and static analysis verification commands pass.
4. Update relevant documentation (such as `CHANGELOG.md` or `docs/work_done.md`) if introducing user-facing additions or architecture shifts.
5. Open your pull request with a descriptive summary of your changes.
