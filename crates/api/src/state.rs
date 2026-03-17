use std::sync::Arc;

use monadclaw_config::Config;
use monadclaw_providers::Provider;

/// Shared application state injected into every Axum handler.
#[derive(Clone)]
pub struct AppState {
    /// Full config — used by the status endpoint to report active provider / model.
    pub config: Arc<Config>,
    /// Active LLM provider, ready to stream completions.
    pub provider: Arc<dyn Provider>,
    /// Pre-built workspace context string (SOUL.md, USER.md, etc.).
    /// Injected at the front of the system prompt on every turn.
    pub workspace_context: Arc<String>,
    /// Optional dashboard password. None = no password configured.
    pub dashboard_password: Option<Arc<String>>,
}
