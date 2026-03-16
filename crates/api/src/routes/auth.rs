use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize)]
pub struct AuthStatusResponse {
    /// true if a dashboard_password is configured
    pub protected: bool,
}

pub async fn get_auth_status(State(state): State<AppState>) -> impl IntoResponse {
    Json(AuthStatusResponse {
        protected: state.dashboard_password.is_some(),
    })
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn post_login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    match &state.dashboard_password {
        Some(password) if body.password == password.as_str() => (
            StatusCode::OK,
            Json(LoginResponse {
                token: body.password,
            }),
        )
            .into_response(),
        Some(_) => (StatusCode::UNAUTHORIZED, "Wrong password").into_response(),
        None => (StatusCode::BAD_REQUEST, "No password configured").into_response(),
    }
}
