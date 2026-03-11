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
