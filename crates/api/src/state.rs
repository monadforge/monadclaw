use std::sync::Arc;

use monadclaw_config::Config;

/// Shared application state injected into every Axum handler.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    /// Resolved API key — read from the env var once at startup.
    pub api_key: Arc<String>,
}
