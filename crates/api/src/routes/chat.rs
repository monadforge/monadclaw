use axum::{
    Json,
    extract::State,
    response::sse::{Event, Sse},
};
use futures::StreamExt;
use monadclaw_chat::ChatMessage;
use monadclaw_providers::{StreamEvent, stream_chat};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::error;

use crate::{error::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
}

pub async fn post_chat(
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> Result<Sse<ReceiverStream<Result<Event, std::convert::Infallible>>>, ApiError> {
    if body.messages.is_empty() {
        return Err(ApiError::bad_request("messages must not be empty"));
    }

    let provider_config = state
        .config
        .active_provider_config()
        .map_err(ApiError::provider_unavailable)?;

    let api_key = (*state.api_key).clone();
    let mut llm_stream = stream_chat(provider_config, api_key, body.messages);

    let (tx, rx) = mpsc::channel::<Result<Event, std::convert::Infallible>>(32);

    tokio::spawn(async move {
        while let Some(event) = llm_stream.next().await {
            let data = match event {
                StreamEvent::Delta(text) => text,
                StreamEvent::Done => "[DONE]".to_string(),
                StreamEvent::Error(msg) => {
                    error!(error = %msg, "LLM stream error");
                    format!("[ERROR] {msg}")
                }
            };
            // Stop sending if the client has disconnected (send returns Err).
            if tx.send(Ok(Event::default().data(data))).await.is_err() {
                break;
            }
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)))
}
