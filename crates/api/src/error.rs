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
