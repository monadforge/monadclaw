# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Monadclaw is a minimalist personal claw with modular and extensible features, written in Rust. This is a personal project, not intended for production use.

## Build Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run a single test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Project Status

This is a new project with the initial repository structure. The Rust crate structure (Cargo.toml, src/) will be added as development progresses.