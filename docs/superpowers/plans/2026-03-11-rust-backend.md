# Rust Backend Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Cargo workspace with four crates (chat, config, providers, api) and a server binary that streams LLM responses over SSE from a TOML-configured provider, then wire the dashboard chat page to consume the stream.

**Architecture:** Multi-crate workspace — `crates/chat` (types), `crates/config` (TOML loading), `crates/providers` (genai streaming wrapper), `crates/api` (Axum routes + SSE). Binary in `apps/server` wires them together. Dashboard `useChat` hook updated to consume SSE via `fetch` + `ReadableStream`. The API key is resolved from the environment variable at server startup and stored in `AppState` — this avoids repeated env lookups and fails fast if the key is missing.

**Tech Stack:** Rust 2024 edition (edition = "2024"), Axum 0.8, Tokio, genai 0.5, serde/toml, thiserror/anyhow, async-stream, futures; React dashboard uses `fetch` + `ReadableStream` for SSE.

---

## Chunk 1: Workspace + Chat Types

### Task 1: Workspace Cargo.toml

**Files:**
- Create: `Cargo.toml`

- [ ] **Step 1: Create workspace Cargo.toml**

```toml
[workspace]
members = [
  "apps/server",
  "crates/api",
  "crates/chat",
  "crates/config",
  "crates/providers",
]
resolver = "2"

[workspace.package]
edition      = "2024"
license      = "MIT"
version      = "0.1.0"

[workspace.lints.rust]
unused_qualifications = "deny"

[workspace.lints.clippy]
expect_used = "deny"
unwrap_used = "deny"

[workspace.dependencies]
# Async runtime
tokio = { features = ["full"], version = "1" }
# HTTP server
axum       = { features = ["macros"], version = "0.8" }
axum-extra = "0.10"
# Serialization
serde      = { features = ["derive"], version = "1" }
serde_json = "1"
# Error handling
anyhow    = "1"
thiserror = "2"
# Config formats
toml = "0.8"
# LLM
genai = "0.5"
# Logging
tracing            = "0.1"
tracing-subscriber = { features = ["env-filter"], version = "0.3" }
# Async utilities
async-stream = "0.3"
futures      = "0.3"
tokio-stream = "0.1"
# Filesystem
directories = "6"

# Workspace crates
monadclaw-api       = { path = "crates/api" }
monadclaw-chat      = { path = "crates/chat" }
monadclaw-config    = { path = "crates/config" }
monadclaw-providers = { path = "crates/providers" }
```

- [ ] **Step 2: Verify workspace parses**

```bash
cargo metadata --no-deps --format-version 1 2>&1 | head -5
```

Expected: JSON output starting with `{"version":1,...}` (no errors).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add Cargo workspace root"
```

---

### Task 2: `crates/chat` — shared message types

**Files:**
- Create: `crates/chat/Cargo.toml`
- Create: `crates/chat/src/lib.rs`

- [ ] **Step 1: Create Cargo.toml**

Create `crates/chat/Cargo.toml`:

```toml
[package]
edition.workspace = true
name              = "monadclaw-chat"
version.workspace = true

[dependencies]
serde      = { workspace = true }
serde_json = { workspace = true }

[lints]
workspace = true
```

- [ ] **Step 2: Write the implementation and tests**

Create `crates/chat/src/lib.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_json() {
        let msg = ChatMessage {
            role: Role::User,
            content: "hello".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let back: ChatMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn role_serializes_lowercase() {
        let json = serde_json::to_string(&Role::Assistant).unwrap();
        assert_eq!(json, "\"assistant\"");
    }
}
```

- [ ] **Step 3: Run tests to verify they pass**

```bash
cargo test -p monadclaw-chat
```

Expected:
```
test tests::role_serializes_lowercase ... ok
test tests::roundtrip_json ... ok
test result: ok. 2 passed
```

- [ ] **Step 4: Commit**

```bash
git add crates/chat/
git commit -m "feat(chat): add ChatMessage and Role types"
```

---

## Chunk 2: Config Crate

### Task 3: `crates/config` — TOML loading

**Files:**
- Create: `crates/config/Cargo.toml`
- Create: `crates/config/src/lib.rs`

- [ ] **Step 1: Create Cargo.toml**

Create `crates/config/Cargo.toml`:

```toml
[package]
edition.workspace = true
name              = "monadclaw-config"
version.workspace = true

[dependencies]
directories = { workspace = true }
serde       = { workspace = true }
thiserror   = { workspace = true }
toml        = { workspace = true }

[dev-dependencies]
tempfile = "3"

[lints]
workspace = true
```

- [ ] **Step 2: Write the implementation and tests**

Create `crates/config/src/lib.rs`:

```rust
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found at {0}")]
    NotFound(PathBuf),
    #[error("Failed to read config: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Provider '{0}' not found in config")]
    ProviderNotFound(String),
    #[error("API key env var '{0}' is not set")]
    MissingApiKey(String),
}

/// Per-provider settings in the TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Model identifier, e.g. "gpt-4o" or "claude-sonnet-4-6"
    pub model: String,
    /// Name of the environment variable holding the API key.
    pub api_key_env: String,
}

/// Top-level config shape, maps to `~/.config/monadclaw/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Name of the provider to use by default (must be a key in `providers`).
    pub active_provider: String,
    /// Map of provider name → provider settings.
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
}

impl Config {
    /// Load config from the given path.
    pub fn load(path: &std::path::Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }
        let text = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&text)?;
        Ok(config)
    }

    /// Return the default config file path: `~/.config/monadclaw/config.toml`.
    /// Falls back to `./config.toml` if the home directory cannot be determined.
    pub fn default_path() -> PathBuf {
        directories::BaseDirs::new()
            .map(|b| b.config_dir().join("monadclaw").join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"))
    }

    /// Return the active `ProviderConfig`.
    pub fn active_provider_config(&self) -> Result<&ProviderConfig, ConfigError> {
        self.providers
            .get(&self.active_provider)
            .ok_or_else(|| ConfigError::ProviderNotFound(self.active_provider.clone()))
    }

    /// Resolve the API key for the active provider from the environment.
    pub fn resolve_api_key(&self) -> Result<String, ConfigError> {
        let provider = self.active_provider_config()?;
        std::env::var(&provider.api_key_env)
            .map_err(|_| ConfigError::MissingApiKey(provider.api_key_env.clone()))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::io::Write;

    use super::*;

    fn write_temp(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn parses_valid_toml() {
        let f = write_temp(
            r#"
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "OPENAI_API_KEY"
"#,
        );
        let config = Config::load(f.path()).unwrap();
        assert_eq!(config.active_provider, "openai");
        assert_eq!(config.providers["openai"].model, "gpt-4o");
    }

    #[test]
    fn missing_file_returns_not_found() {
        let result = Config::load(std::path::Path::new("/nonexistent/path/config.toml"));
        assert!(matches!(result, Err(ConfigError::NotFound(_))));
    }

    #[test]
    fn invalid_toml_returns_parse_error() {
        let f = write_temp("not valid toml ][");
        let result = Config::load(f.path());
        assert!(matches!(result, Err(ConfigError::Parse(_))));
    }

    #[test]
    fn active_provider_not_in_map_returns_error() {
        let f = write_temp(
            r#"
active_provider = "missing"

[providers.openai]
model = "gpt-4o"
api_key_env = "OPENAI_API_KEY"
"#,
        );
        let config = Config::load(f.path()).unwrap();
        assert!(matches!(
            config.active_provider_config(),
            Err(ConfigError::ProviderNotFound(_))
        ));
    }

    #[test]
    fn resolve_api_key_reads_env() {
        let f = write_temp(
            r#"
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "TEST_KEY_MONADCLAW"
"#,
        );
        std::env::set_var("TEST_KEY_MONADCLAW", "sk-test-123");
        let config = Config::load(f.path()).unwrap();
        assert_eq!(config.resolve_api_key().unwrap(), "sk-test-123");
        std::env::remove_var("TEST_KEY_MONADCLAW");
    }

    #[test]
    fn missing_env_var_returns_error() {
        let f = write_temp(
            r#"
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "DEFINITELY_NOT_SET_MONADCLAW"
"#,
        );
        std::env::remove_var("DEFINITELY_NOT_SET_MONADCLAW");
        let config = Config::load(f.path()).unwrap();
        assert!(matches!(
            config.resolve_api_key(),
            Err(ConfigError::MissingApiKey(_))
        ));
    }
}
```

- [ ] **Step 3: Run tests**

```bash
cargo test -p monadclaw-config
```

Expected: 5 tests pass.

- [ ] **Step 4: Commit**

```bash
git add crates/config/
git commit -m "feat(config): TOML config loading with env key resolution"
```

---

## Chunk 3: Providers Crate

### Task 4: `crates/providers` — genai streaming wrapper

**Note on `api_key` parameter:** The spec says the provider resolves the key from the env var. In practice, the key is resolved once at server startup (in `apps/server/main.rs`) and stored in `AppState`. It is passed explicitly into `stream_chat` to avoid env lookups on every request and to fail fast at startup. This is an intentional deviation from the spec that improves reliability.

**Files:**
- Create: `crates/providers/Cargo.toml`
- Create: `crates/providers/src/lib.rs`

- [ ] **Step 1: Create Cargo.toml**

Create `crates/providers/Cargo.toml`:

```toml
[package]
edition.workspace = true
name              = "monadclaw-providers"
version.workspace = true

[dependencies]
anyhow           = { workspace = true }
async-stream     = { workspace = true }
futures          = { workspace = true }
genai            = { workspace = true }
monadclaw-chat   = { workspace = true }
monadclaw-config = { workspace = true }
tokio-stream     = { workspace = true }
tracing          = { workspace = true }

[lints]
workspace = true
```

- [ ] **Step 2: Write the provider module**

Create `crates/providers/src/lib.rs`:

```rust
use std::pin::Pin;

use futures::StreamExt;
use monadclaw_chat::ChatMessage;
use monadclaw_config::ProviderConfig;
use tokio_stream::Stream;
use tracing::{debug, instrument};

/// A single streaming event from the LLM.
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// A text delta (token chunk).
    Delta(String),
    /// Stream finished normally.
    Done,
    /// An error occurred; stream will not produce more events.
    Error(String),
}

/// Convert our `ChatMessage` types to genai's format.
fn to_genai_messages(messages: &[ChatMessage]) -> Vec<genai::chat::ChatMessage> {
    use monadclaw_chat::Role;
    messages
        .iter()
        .map(|m| match m.role {
            Role::User => genai::chat::ChatMessage::user(&m.content),
            Role::Assistant => genai::chat::ChatMessage::assistant(&m.content),
            Role::System => genai::chat::ChatMessage::system(&m.content),
        })
        .collect()
}

/// Build a `genai::Client` with an explicit API key.
fn build_client(api_key: &str) -> genai::Client {
    let key = api_key.to_string();
    genai::Client::builder()
        .with_auth_resolver(genai::resolver::AuthResolver::from_resolver_fn(
            move |_model_iden| {
                Ok(Some(genai::resolver::AuthData::from_single(key.clone())))
            },
        ))
        .build()
}

/// Stream chat completions from the configured provider.
///
/// `api_key` is the resolved value of the env var named in `config.api_key_env`.
/// It is passed explicitly so the server can resolve it once at startup.
#[instrument(skip(messages, api_key))]
pub fn stream_chat(
    config: &ProviderConfig,
    api_key: String,
    messages: Vec<ChatMessage>,
) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send>> {
    let model = config.model.clone();
    let client = build_client(&api_key);

    Box::pin(async_stream::stream! {
        use genai::chat::ChatStreamEvent;

        debug!(model = %model, "starting chat stream");

        let genai_messages = to_genai_messages(&messages);
        let chat_req = genai::chat::ChatRequest::new(genai_messages);

        let mut chat_stream = match client.exec_chat_stream(&model, chat_req, None).await {
            Ok(s) => s,
            Err(e) => {
                yield StreamEvent::Error(format!("{e}"));
                return;
            }
        };

        while let Some(result) = chat_stream.stream.next().await {
            match result {
                Ok(ChatStreamEvent::Chunk(chunk)) if !chunk.content.is_empty() => {
                    yield StreamEvent::Delta(chunk.content);
                }
                Ok(ChatStreamEvent::End(_)) => {
                    yield StreamEvent::Done;
                    return;
                }
                Ok(_) => {}
                Err(e) => {
                    yield StreamEvent::Error(format!("{e}"));
                    return;
                }
            }
        }

        yield StreamEvent::Done;
    })
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p monadclaw-providers
```

Expected: no errors. (No live LLM test — that would require a real API key.)

- [ ] **Step 4: Commit**

```bash
git add crates/providers/
git commit -m "feat(providers): genai streaming wrapper"
```

---

## Chunk 4: API Crate + Server Binary

### Task 5: `crates/api` — Axum routes and SSE handler

**SSE protocol used:** Simple `data:`-only frames. Delta chunks are sent as `data: <text>`. End of stream is `data: [DONE]`. Errors are `data: [ERROR] <message>`. This is intentionally simple to parse on the frontend.

**Files:**
- Create: `crates/api/Cargo.toml`
- Create: `crates/api/src/lib.rs`
- Create: `crates/api/src/state.rs`
- Create: `crates/api/src/error.rs`
- Create: `crates/api/src/routes/mod.rs`
- Create: `crates/api/src/routes/status.rs`
- Create: `crates/api/src/routes/chat.rs`

- [ ] **Step 1: Create Cargo.toml**

Create `crates/api/Cargo.toml`:

```toml
[package]
edition.workspace = true
name              = "monadclaw-api"
version.workspace = true

[dependencies]
anyhow              = { workspace = true }
axum                = { workspace = true }
futures             = { workspace = true }
monadclaw-chat      = { workspace = true }
monadclaw-config    = { workspace = true }
monadclaw-providers = { workspace = true }
serde               = { workspace = true }
serde_json          = { workspace = true }
thiserror           = { workspace = true }
tokio               = { workspace = true }
tokio-stream        = { workspace = true }
tracing             = { workspace = true }

[dev-dependencies]
axum-test = "0.7"
tokio     = { features = ["full"], workspace = true }

[lints]
workspace = true
```

- [ ] **Step 2: Create shared state**

Create `crates/api/src/state.rs`:

```rust
use std::sync::Arc;

use monadclaw_config::Config;

/// Shared application state injected into every Axum handler.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    /// Resolved API key — read from the env var once at startup.
    pub api_key: Arc<String>,
}
```

- [ ] **Step 3: Create error type**

Create `crates/api/src/error.rs`:

```rust
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn provider_unavailable(msg: impl ToString) -> Self {
        Self {
            status: StatusCode::BAD_GATEWAY,
            code: "PROVIDER_UNAVAILABLE",
            message: msg.to_string(),
        }
    }

    pub fn bad_request(msg: impl ToString) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "BAD_REQUEST",
            message: msg.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({ "error": { "code": self.code, "message": self.message } });
        (self.status, Json(body)).into_response()
    }
}
```

- [ ] **Step 4: Create status route with test**

Create `crates/api/src/routes/status.rs`:

```rust
use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::state::AppState;

pub async fn get_status(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "provider": state.config.active_provider,
        "model": state.config
            .providers
            .get(&state.config.active_provider)
            .map(|p| p.model.as_str())
            .unwrap_or("unknown"),
    }))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use monadclaw_config::{Config, ProviderConfig};

    use crate::{router, state::AppState};

    fn test_state() -> AppState {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            ProviderConfig {
                model: "gpt-4o".to_string(),
                api_key_env: "OPENAI_API_KEY".to_string(),
            },
        );
        AppState {
            config: Arc::new(Config {
                active_provider: "openai".to_string(),
                providers,
            }),
            api_key: Arc::new("sk-test".to_string()),
        }
    }

    #[tokio::test]
    async fn status_returns_ok() {
        let app = router(test_state());
        let server = TestServer::new(app).unwrap();
        let response = server.get("/api/v1/status").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let json: serde_json::Value = response.json();
        assert_eq!(json["status"], "ok");
        assert_eq!(json["provider"], "openai");
        assert_eq!(json["model"], "gpt-4o");
    }

    #[tokio::test]
    async fn chat_rejects_empty_messages() {
        let app = router(test_state());
        let server = TestServer::new(app).unwrap();
        let response = server
            .post("/api/v1/chat")
            .json(&serde_json::json!({ "messages": [] }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }
}
```

- [ ] **Step 5: Create chat SSE route**

Create `crates/api/src/routes/chat.rs`:

```rust
use axum::{
    Json,
    extract::State,
    response::sse::{Event, Sse},
};
use futures::StreamExt;
use monadclaw_chat::ChatMessage;
use monadclaw_providers::{StreamEvent, stream_chat};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::error;

use crate::{error::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
}

pub async fn post_chat(
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> Result<Sse<ReceiverStream<Result<Event, std::convert::Infallible>>>, ApiError> {
    if body.messages.is_empty() {
        return Err(ApiError::bad_request("messages must not be empty"));
    }

    let provider_config = state
        .config
        .active_provider_config()
        .map_err(ApiError::provider_unavailable)?;

    let api_key = (*state.api_key).clone();
    let mut llm_stream = stream_chat(provider_config, api_key, body.messages);

    let (tx, rx) = mpsc::channel::<Result<Event, std::convert::Infallible>>(32);

    tokio::spawn(async move {
        while let Some(event) = llm_stream.next().await {
            let data = match event {
                StreamEvent::Delta(text) => text,
                StreamEvent::Done => "[DONE]".to_string(),
                StreamEvent::Error(msg) => {
                    error!(error = %msg, "LLM stream error");
                    format!("[ERROR] {msg}")
                }
            };
            // Stop sending if the client has disconnected (send returns Err).
            if tx.send(Ok(Event::default().data(data))).await.is_err() {
                break;
            }
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)))
}
```

- [ ] **Step 6: Create routes mod and lib**

Create `crates/api/src/routes/mod.rs`:

```rust
pub mod chat;
pub mod status;
```

Create `crates/api/src/lib.rs`:

```rust
pub mod error;
pub mod routes;
pub mod state;

use axum::{Router, routing::{get, post}};
use state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/status", get(routes::status::get_status))
        .route("/api/v1/chat", post(routes::chat::post_chat))
        .with_state(state)
}
```

- [ ] **Step 7: Run the tests**

```bash
cargo test -p monadclaw-api
```

Expected:
```
test routes::status::tests::status_returns_ok ... ok
test routes::status::tests::chat_rejects_empty_messages ... ok
test result: ok. 2 passed
```

- [ ] **Step 8: Commit**

```bash
git add crates/api/
git commit -m "feat(api): Axum router with status and SSE chat routes"
```

---

### Task 6: `apps/server` — binary entry point

**Files:**
- Create: `apps/server/Cargo.toml`
- Create: `apps/server/src/main.rs`

- [ ] **Step 1: Create Cargo.toml**

Create `apps/server/Cargo.toml`:

```toml
[package]
edition.workspace = true
name              = "monadclaw-server"
version.workspace = true

[[bin]]
name = "monadclaw"
path = "src/main.rs"

[dependencies]
anyhow             = { workspace = true }
axum               = { workspace = true }
monadclaw-api      = { workspace = true }
monadclaw-config   = { workspace = true }
tokio              = { workspace = true }
tower-http         = { version = "0.6", features = ["cors"] }
tracing            = { workspace = true }
tracing-subscriber = { workspace = true }

[lints]
workspace = true
```

- [ ] **Step 2: Create main.rs**

Create `apps/server/src/main.rs`:

```rust
use std::sync::Arc;

use monadclaw_api::state::AppState;
use monadclaw_config::Config;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing — respects RUST_LOG env var.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "monadclaw=debug,tower_http=info".into()),
        )
        .init();

    // Load config (path overridden via MONADCLAW_CONFIG env var if set).
    let config_path = std::env::var("MONADCLAW_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    info!(path = %config_path.display(), "loading config");

    let config = Config::load(&config_path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to load config from {}: {e}\n\nCreate the file or set MONADCLAW_CONFIG.",
            config_path.display()
        )
    })?;

    // Resolve the API key at startup — fails fast if the env var is missing.
    let api_key = config.resolve_api_key().map_err(|e| {
        anyhow::anyhow!("{e}\n\nSet the environment variable and restart.")
    })?;

    let state = AppState {
        config: Arc::new(config),
        api_key: Arc::new(api_key),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = monadclaw_api::router(state).layer(cors);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    info!(%addr, "starting server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

- [ ] **Step 3: Build to verify it compiles**

```bash
cargo build -p monadclaw-server
```

Expected: compiles without errors (binary at `target/debug/monadclaw`).

- [ ] **Step 4: Commit**

```bash
git add apps/server/
git commit -m "feat(server): binary entry point with config loading and CORS"
```

---

## Chunk 5: Dashboard SSE Integration

### Task 7: Update `useChat` hook to consume SSE stream

Replace the React Query mutation in `useChat` with a `fetch`-based streaming reader. The backend emits plain `data:` SSE frames: token chunks as `data: <text>`, end of stream as `data: [DONE]`, errors as `data: [ERROR] <message>`.

**Files:**
- Modify: `dashboard/src/hooks/useChat.ts`
- Modify: `dashboard/src/api/chat.ts` (remove `sendMessage` — no longer used)
- Create: `dashboard/src/test/utils.tsx`
- Create: `dashboard/src/hooks/useChat.test.ts`

- [ ] **Step 1: Create test wrapper helper**

Create `dashboard/src/test/utils.tsx`:

```tsx
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import type { ReactNode } from 'react'

export function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })
  return function Wrapper({ children }: { children: ReactNode }) {
    return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  }
}
```

- [ ] **Step 2: Write the failing test**

Create `dashboard/src/hooks/useChat.test.ts`:

```ts
import { renderHook, act, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useChat } from './useChat'
import { createWrapper } from '../test/utils'

/** Build a ReadableStream that yields each string chunk in sequence. */
function makeStream(chunks: string[]): ReadableStream<Uint8Array> {
  let i = 0
  return new ReadableStream({
    pull(controller) {
      if (i < chunks.length) {
        controller.enqueue(new TextEncoder().encode(chunks[i++]))
      } else {
        controller.close()
      }
    },
  })
}

describe('useChat streaming', () => {
  beforeEach(() => {
    vi.resetAllMocks()
  })

  it('appends delta tokens to assistant message', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        body: makeStream([
          'data: Hello\n\n',
          'data:  world\n\n',
          'data: [DONE]\n\n',
        ]),
      }),
    )

    const { result } = renderHook(() => useChat(), { wrapper: createWrapper() })

    act(() => {
      result.current.send('hi')
    })

    await waitFor(() => {
      const assistant = result.current.messages.find(m => m.role === 'assistant')
      expect(assistant?.content).toBe('Hello world')
    })

    expect(result.current.isPending).toBe(false)
  })

  it('adds user message immediately', () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        body: makeStream(['data: [DONE]\n\n']),
      }),
    )

    const { result } = renderHook(() => useChat(), { wrapper: createWrapper() })

    act(() => {
      result.current.send('hello')
    })

    expect(result.current.messages[0]).toMatchObject({ role: 'user', content: 'hello' })
  })
})
```

- [ ] **Step 3: Run test to verify it fails**

```bash
cd dashboard && npm test -- useChat
```

Expected: test fails because `useChat` doesn't use `fetch` streaming yet.

- [ ] **Step 4: Rewrite `useChat` to use fetch + ReadableStream**

Replace the entire contents of `dashboard/src/hooks/useChat.ts`:

```ts
import { useState, useCallback } from 'react'
import type { ChatMessage } from '../types/api'

let _idCounter = 0
const nextId = () => String(++_idCounter)

export function useChat() {
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [isPending, setIsPending] = useState(false)

  const send = useCallback(
    (text: string) => {
      const userMsg: ChatMessage = { id: nextId(), role: 'user', content: text }
      const assistantId = nextId()
      const assistantMsg: ChatMessage = { id: assistantId, role: 'assistant', content: '' }

      setMessages(prev => [...prev, userMsg, assistantMsg])
      setIsPending(true)

      const payload = {
        messages: [...messages, userMsg].map(({ role, content }) => ({ role, content })),
      }

      fetch('/api/v1/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      })
        .then(response => {
          if (!response.ok || !response.body) {
            setIsPending(false)
            return
          }

          const reader = response.body.getReader()
          const decoder = new TextDecoder()
          let buffer = ''

          const processLines = (chunk: string) => {
            buffer += chunk
            const lines = buffer.split('\n')
            buffer = lines.pop() ?? ''

            for (const line of lines) {
              if (!line.startsWith('data:')) continue
              const data = line.slice(5).trim()
              if (data === '[DONE]' || data.startsWith('[ERROR]')) {
                setIsPending(false)
                return
              }
              setMessages(prev =>
                prev.map(m =>
                  m.id === assistantId ? { ...m, content: m.content + data } : m,
                ),
              )
            }
          }

          const read = (): Promise<void> =>
            reader.read().then(({ done, value }) => {
              if (done) {
                setIsPending(false)
                return
              }
              processLines(decoder.decode(value, { stream: true }))
              return read()
            })

          return read()
        })
        .catch(() => setIsPending(false))
    },
    [messages],
  )

  return { messages, send, isPending }
}
```

- [ ] **Step 5: Update `dashboard/src/api/chat.ts`**

Read the current file. Remove the `sendMessage` export (it was used by the old `useChat`). Keep `getChatHistory` if it exists and is used elsewhere. The file should contain only functions that are actually called from other components.

If the file only contained `sendMessage`, replace it entirely with:

```ts
// Chat API — history endpoint only.
// Message sending is handled by useChat via fetch streaming.
import { apiFetch } from './client'
import type { ChatMessage } from '../types/api'

export async function getChatHistory(): Promise<ChatMessage[]> {
  return apiFetch<ChatMessage[]>('/api/v1/chat/history')
}
```

If the file does not have `getChatHistory` either, it can be left empty or deleted (remove the import in any file that imports from it).

- [ ] **Step 6: Run all dashboard tests**

```bash
cd dashboard && npm test
```

Expected: all tests pass including the two new `useChat` streaming tests.

- [ ] **Step 7: Turn off mock mode**

Edit `dashboard/.env.development.local` (create the file if it does not exist):

```
VITE_USE_MOCK=false
```

- [ ] **Step 8: Commit**

```bash
cd ..
git add dashboard/src/hooks/useChat.ts dashboard/src/api/chat.ts dashboard/src/test/ dashboard/src/hooks/useChat.test.ts dashboard/.env.development.local
git commit -m "feat(dashboard): stream chat responses via fetch + ReadableStream"
```

---

## Running the Backend Locally (Reference)

> These are manual verification steps, not part of the automated implementation.

1. Create `~/.config/monadclaw/config.toml`:

```toml
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "OPENAI_API_KEY"
```

2. Set your API key and start the server:

```bash
export OPENAI_API_KEY=sk-...
cargo run -p monadclaw-server
# Server starts at http://localhost:3000
```

3. Start the dashboard:

```bash
cd dashboard && npm run dev
# Dashboard at http://localhost:5173
```

4. Open http://localhost:5173/chat and send a message. Tokens should appear as they stream in.

---

## Summary of Files Created

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace root |
| `crates/chat/src/lib.rs` | `ChatMessage`, `Role` types |
| `crates/config/src/lib.rs` | `Config`, `ProviderConfig`, TOML loading |
| `crates/providers/src/lib.rs` | `stream_chat()`, `StreamEvent` |
| `crates/api/src/lib.rs` | Axum router factory |
| `crates/api/src/state.rs` | `AppState` |
| `crates/api/src/error.rs` | `ApiError` → HTTP response |
| `crates/api/src/routes/status.rs` | `GET /api/v1/status` |
| `crates/api/src/routes/chat.rs` | `POST /api/v1/chat` → SSE |
| `apps/server/src/main.rs` | Binary: load config, start Axum |
| `dashboard/src/hooks/useChat.ts` | fetch SSE streaming hook |
| `dashboard/src/hooks/useChat.test.ts` | Streaming tests with mocked fetch |
| `dashboard/src/test/utils.tsx` | Test wrapper helper |
| `dashboard/src/api/chat.ts` | Thin API wrapper (history only) |
| `dashboard/.env.development.local` | Disable mock mode |
