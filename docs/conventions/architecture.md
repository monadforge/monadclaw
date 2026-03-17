# Architecture

## Overview

Monadclaw is a modular AI agent framework with the following components:

```
┌─────────────────────────────────────────────┐
│                  Interfaces                  │
│   Discord Bot    │    REST API (Dashboard)   │
└─────────┬────────┴────────────┬─────────────┘
          │                     │
┌─────────▼─────────────────────▼─────────────┐
│               Core (Rust)                    │
│   Agent Loop  │  Memory  │  Provider Router  │
└─────────────────────────────────────────────┘
          │                     │
┌─────────▼──────┐   ┌──────────▼─────────────┐
│  LLM Providers │   │   Storage              │
│  OpenAI / etc. │   │   Short-term / Long-term│
└────────────────┘   └────────────────────────┘
```

## Repository Structure

```
monadclaw/
├── src/          # Rust core
├── dashboard/    # React dashboard (status, chat, config)
├── web/          # VitePress public website & documentation
├── docs/         # Internal specs and conventions (not published)
└── CLAUDE.md     # AI assistant rules
```

## Core Modules

- **Agent Loop** — orchestrates conversations, tool calls, memory retrieval
- **Memory** — short-term (conversation window) + long-term (persistent store)
- **Provider Router** — abstracts LLM providers behind a unified trait
- **Interfaces** — Discord bot and REST API are separate crates/modules
- **Dashboard** — React app for monitoring and configuration
- **Website** (`web/`) — VitePress site, built and deployed independently

## Design Principles

- Core logic has zero dependency on interface layer
- Providers are interchangeable via trait objects
- Memory is pluggable (in-memory, SQLite, external)
- Configuration is explicit, no hidden global state
