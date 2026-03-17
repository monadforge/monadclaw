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
    use std::{collections::HashMap, pin::Pin, sync::Arc};

    use axum::http::StatusCode;
    use axum_test::{TestResponse, TestServer};
    use monadclaw_chat::ChatMessage;
    use monadclaw_config::{Config, ProviderConfig};
    use monadclaw_providers::{Provider, ProviderEvent};
    use tokio_stream::Stream;

    use crate::{router, state::AppState};

    struct StubProvider;

    impl Provider for StubProvider {
        fn stream(
            &self,
            _messages: Vec<ChatMessage>,
        ) -> Pin<Box<dyn Stream<Item = ProviderEvent> + Send>> {
            Box::pin(tokio_stream::empty())
        }
    }

    fn test_state() -> AppState {
        let mut providers = HashMap::new();
        providers.insert(
            "openai".to_string(),
            ProviderConfig {
                model: "gpt-4o".to_string(),
                api_key_env: "OPENAI_API_KEY".to_string(),
                base_url: None,
            },
        );
        AppState {
            config: Arc::new(Config {
                active_provider: "openai".to_string(),
                providers,
                dashboard_password: None,
                workspace_path: None,
            }),
            provider: Arc::new(StubProvider),
            workspace_context: Arc::new(String::new()),
            dashboard_password: None,
        }
    }

    #[tokio::test]
    async fn status_returns_ok() {
        let app = router(test_state());
        let server = TestServer::new(app);
        let response: TestResponse = server.get("/api/v1/status").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let json: serde_json::Value = response.json();
        assert_eq!(json["status"], "ok");
        assert_eq!(json["provider"], "openai");
        assert_eq!(json["model"], "gpt-4o");
    }

    #[tokio::test]
    async fn chat_rejects_empty_messages() {
        let app = router(test_state());
        let server = TestServer::new(app);
        let response: TestResponse = server
            .post("/api/v1/chat")
            .json(&serde_json::json!({ "messages": [] }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }
}
