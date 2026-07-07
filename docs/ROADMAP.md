# Git2OKF Project Roadmap

This document outlines the development lifecycle of the **Git2OKF** engine, tracking completed milestones, the current stabilization phase, and upcoming core features.

---

## 1. Completed Phases

### Phase 1: Engine Foundation
Established the project structure, CLI entry points, and high-level codebase diagnostics.
- Modular Rust architecture skeleton.
- CLI analyze commands powered by `clap`.
- Multi-language scanning and classification percentages.
- Heuristic and confidence-score based framework identification.
- Manifest dependency scanners (parsing package.json, composer.json, requirements.txt).
- Output abstraction layer (JSON & YAML formatting).
- Repository validation and shallow-clone pipeline with 60-second timeouts.
- Isolated integration testing suite and CI actions pipeline.

### Phase 2: AST Parser Intelligence (Implemented)
Introduced code understanding using AST node extractions.
- Dynamic `ParserRegistry` and `ParserEngine`.
- Tree-sitter query extraction engines for Rust, PHP, JavaScript, and Python.
- Unified Intermediate Representation (IR) AST schema (`ParsedFile`, `ClassNode`, `FunctionNode`, etc.).
- Selective compilation features gates (`[features]`).
- Parser-level unit tests for all target languages.

---

## 2. Current Phase: Phase 2 Verification & Stabilization

Due to native OS compiler requirements on tree-sitter wrappers, the architecture is currently under a frozen stabilization state pending local test suites verification.
- Install native Visual Studio C++ Build Tools on the host machine.
- Verify feature isolation via separate cargo builds.
- Ensure all 12 validation, formatting, linting, and testing commands compile and pass successfully.
- Address any OS-specific compilation or link warnings.

---

## 3. Future Phases

### Phase 3: Graph Builder
Build a multi-dimensional relationship model of the codebase.
- Define a strict graph representation (Nodes, Edges, Graph schema).
- Construct call-graph linkages, class hierarchies, and file-dependency chains.
- Track usage relationships (e.g., File A imports Class B; Function X calls Method Y).
- Implement a builder system to process AST nodes into the graph.

### Phase 4: Analyzer
Resolve symbols across codebases to generate structural context.
- Implement symbol table indexing for quick lookups.
- Resolve dynamic calls, references, and class methods.
- Calculate structural metrics (e.g., cyclomatic complexity, coupling, inheritance depth).
- Detect architectural code smells or dead code.

### Phase 5: OKF Generator
Package codebase intelligence into the Open Knowledge Format (OKF).
- Define the OKF schema for AI-friendly directory parsing.
- Generate semantic maps of files, classes, methods, and relationships.
- Bundle structural data, dependency trees, and repository metadata into a unified knowledge bundle.

### Phase 6: CLI Polish & Performance
Optimize processing speed and user interaction.
- Add support for progress bars, spinners, and interactive prompts.
- Implement incremental analysis (caching unchanged file ASTs).
- Parallelize parser traversal using work-stealing thread pools.
- Enhance custom output formatting and error reporting.

### Phase 7: v1.0 Release
Finalize stable codebase interface.
- Complete public API documentation and rustdocs.
- Finalize verification checks on standard operating systems (Windows, Linux, macOS).
- Publish official crates, binaries, and containerized variants.
