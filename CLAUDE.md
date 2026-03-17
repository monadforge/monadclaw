# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Monadclaw is a modular AI agent framework written in Rust. Features: Discord bot interface, multiple LLM provider support, short-term and long-term memory, React dashboard for status/chat/config. Personal project, not intended for production use.

## Detailed Specs

- Architecture & components: @docs/conventions/architecture.md
- Backend conventions (Rust): @docs/conventions/backend.md
- API design: @docs/conventions/api.md
- Frontend conventions (React dashboard): @docs/conventions/frontend.md
- Security rules: @docs/conventions/security.md
- Auth policy: @docs/features/auth.md
- Agent workspace: @docs/features/workspace.md
- Website (VitePress): `web/` — `cd web && npm run dev`
- Local rules (not committed): @docs/local.md

## Implementation Roadmap

The primary goal is feature parity with openclaw / moltis: persistent sessions,
tool execution, Discord bot interface, and long-term memory. Work through phases in order.

| Phase | Spec | Status |
|-------|------|--------|
| 1 — Session persistence + system prompt | @docs/specs/phase-1-session-persistence.md | 🔨 Next |
| 2 — Tool execution (bash, file read/write, agent loop) | @docs/specs/phase-2-tool-execution.md | 🔄 Planned |
| 3 — Discord bot | @docs/specs/phase-3-discord-bot.md | 🔄 Planned |
| 4 — Long-term memory (SQLite FTS) | @docs/specs/phase-4-long-term-memory.md | 🔄 Planned |

When starting a new task, read the relevant phase spec first. Implement exactly
what the spec describes — no more, no less.

## Build Commands

```bash
# Dashboard (React + Vite) — cd dashboard first
npm run dev        # dev server at http://localhost:5173
npm run build      # production build → dashboard/dist/
npm run typecheck  # TypeScript check

# Rust backend
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

Core infrastructure is in place. Active development follows the roadmap above.
Current crates: `agent`, `api`, `chat`, `config`, `providers`.
Next: `crates/store` (Phase 1).