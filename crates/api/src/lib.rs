pub mod error;
pub mod middleware;
pub mod routes;
pub mod state;

use axum::{Router, middleware as axum_middleware, routing::{get, post}};
use state::AppState;

pub fn router(state: AppState) -> Router {
    // Public routes — no auth check
    let public = Router::new()
        .route("/api/v1/auth/status", get(routes::auth::get_auth_status))
        .route("/api/v1/auth/login", post(routes::auth::post_login));

    // Protected routes — three-tier auth middleware applied
    let protected = Router::new()
        .route("/api/v1/status", get(routes::status::get_status))
        .route("/api/v1/chat", post(routes::chat::post_chat))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::auth_middleware,
        ));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}
