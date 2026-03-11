# Rust Backend Design — LLM Config + Chat

**Date:** 2026-03-11
**Status:** Approved

## Goal

A minimal Rust backend that:
1. Loads LLM provider config from a TOML file
2. Exposes `POST /api/v1/chat` with SSE streaming via the configured provider
3. Exposes `GET /api/v1/status` for the dashboard status page

## Workspace Layout

```
monadclaw/
├── Cargo.toml              # workspace root
├── apps/
│   └── server/             # binary: wires crates together, starts Axum
└── crates/
    ├── config/             # TOML loading + env var key resolution
    ├── providers/          # genai wrapper, streaming abstraction
    ├── chat/               # message types (ChatMessage, Role)
    └── api/                # Axum router, SSE handler, REST endpoints
```

## Crate Responsibilities

### `crates/config`

- Reads `~/.config/monadclaw/config.toml` (or path from `MONADCLAW_CONFIG` env var)
- Exposes a `Config` struct: `active_provider`, `providers` map (name → model + key_env)
- API keys are **never** stored in the file; resolved from env vars at startup
- Returns a typed error if the config file is missing or invalid

Example config:
```toml
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "OPENAI_API_KEY"

[providers.anthropic]
model = "claude-sonnet-4-6"
api_key_env = "ANTHROPIC_API_KEY"
```

### `crates/providers`

- Wraps `genai` crate; exposes one async function:
  ```rust
  pub async fn stream_chat(
      config: &ProviderConfig,
      messages: Vec<ChatMessage>,
  ) -> Result<impl Stream<Item = Result<String>>>
  ```
- Resolves the API key from the env var specified in config
- No business logic — pure transport layer

### `crates/chat`

- Defines shared types: `ChatMessage { role: Role, content: String }`, `Role { User, Assistant }`
- No I/O, no async — pure data types used by both `providers` and `api`

### `crates/api`

- Axum router with two routes:
  - `GET /api/v1/status` → JSON `{ "status": "ok", "provider": "<name>" }`
  - `POST /api/v1/chat` → SSE stream, one `data:` frame per token chunk
- Reads `Arc<Config>` from Axum state
- Maps provider errors to HTTP 502, config errors to HTTP 503

### `apps/server`

- `main.rs`: loads config, builds Axum app, listens on `0.0.0.0:3000`
- Wires `Arc<Config>` into router state
- Sets up `tracing-subscriber` for structured logging

## Data Flow

```
Dashboard → POST /api/v1/chat (JSON body: [{role, content}])
  → api crate deserializes messages
  → calls providers::stream_chat(config, messages)
  → genai streams tokens from provider
  → api wraps each chunk as SSE "data: <token>\n\n"
  → Dashboard receives stream, appends to chat bubble
```

## Error Handling

- `config` crate: `thiserror` for typed errors
- `providers` crate: `anyhow` for propagation
- `api` crate: maps errors to `(StatusCode, Json<ErrorBody>)`
- No `unwrap()` or `expect()` outside tests

## Key Dependencies

```toml
axum       = { features = ["ws"], version = "0.8" }
tokio      = { features = ["full"], version = "1" }
genai      = "0.5"
serde      = { features = ["derive"], version = "1" }
serde_json = "1"
toml       = "0.8"
thiserror  = "2"
anyhow     = "1"
tracing    = "0.1"
tracing-subscriber = { features = ["env-filter"], version = "0.3" }
futures    = "0.3"
```

## Out of Scope (this iteration)

- Authentication (Bearer token validation)
- Chat history persistence
- Discord bot integration
- PATCH /api/v1/config hot-reload
