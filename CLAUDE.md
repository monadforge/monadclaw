# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Monadclaw is a modular AI agent framework written in Rust. Features: Discord bot interface, multiple LLM provider support, short-term and long-term memory, React dashboard for status/chat/config. Personal project, not intended for production use.

## Detailed Specs

- Architecture & components: @docs/architecture.md
- Backend conventions (Rust): @docs/backend.md
- API design: @docs/api.md
- Frontend conventions (React dashboard): @docs/frontend.md
- Security rules: @docs/security.md
- Website (VitePress): `web/` — `cd web && npm run dev`

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