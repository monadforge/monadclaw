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
