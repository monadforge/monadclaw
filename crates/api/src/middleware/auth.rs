use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::state::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response<Body> {
    // ConnectInfo is set when using into_make_service_with_connect_info.
    // Falls back to true (local) in test environments where it is absent.
    let is_local = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip().is_loopback())
        .unwrap_or(true);

    match &state.dashboard_password {
        // No password, local connection → allow
        None if is_local => next.run(req).await,
        // No password, remote connection → deny
        None => (
            StatusCode::FORBIDDEN,
            "Remote access requires a dashboard_password in config",
        )
            .into_response(),
        // Password configured → validate Bearer token
        Some(password) => {
            if bearer_matches(req.headers(), password) {
                next.run(req).await
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid or missing token").into_response()
            }
        }
    }
}

fn bearer_matches(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|token| token == expected)
        .unwrap_or(false)
}
