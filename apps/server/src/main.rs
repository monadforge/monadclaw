use std::{net::SocketAddr, sync::Arc};

use monadclaw_agent::{WorkspaceContext, seed_workspace};
use monadclaw_api::state::AppState;
use monadclaw_config::Config;
use monadclaw_providers::GenaiProvider;
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

    // Resolve config path (MONADCLAW_CONFIG env var overrides default).
    let config_path = std::env::var("MONADCLAW_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    // First-run: create ~/.monadclaw/ and seed a default config.toml if missing.
    match Config::seed(&config_path) {
        Ok(true) => {
            info!(path = %config_path.display(), "first run — config template created");
            eprintln!(
                "\nFirst run detected.\n\
                 A default config was created at: {}\n\
                 Edit it to set your provider and API key, then restart.\n",
                config_path.display()
            );
            return Ok(());
        }
        Ok(false) => {}
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to create config directory: {e}"
            ));
        }
    }

    info!(path = %config_path.display(), "loading config");

    let config = Config::load(&config_path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to load config from {}: {e}\n\nEdit {} or set MONADCLAW_CONFIG.",
            config_path.display(),
            config_path.display()
        )
    })?;

    // Seed workspace files on first run; load context for injection into prompts.
    let workspace_dir = config.workspace_dir();
    info!(path = %workspace_dir.display(), "workspace directory");
    seed_workspace(&workspace_dir);

    let workspace_context = WorkspaceContext::load(&workspace_dir);
    if workspace_context.is_bootstrapping() {
        info!("workspace is new — BOOTSTRAP.md active");
    }

    // Resolve the API key at startup — fails fast if the env var is missing.
    let api_key = config.resolve_api_key().map_err(|e| {
        anyhow::anyhow!("{e}\n\nSet the environment variable and restart.")
    })?;

    // Build the LLM provider once; it is shared across all requests.
    let provider = Arc::new(GenaiProvider::new(config.active_provider_config()?, &api_key));

    let state = AppState {
        dashboard_password: config.dashboard_password.clone().map(Arc::new),
        workspace_context: Arc::new(workspace_context.build_context()),
        provider,
        config: Arc::new(config),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = monadclaw_api::router(state).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!(%addr, "starting server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
