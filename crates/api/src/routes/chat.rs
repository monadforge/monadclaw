use axum::{
    Json,
    extract::State,
    response::sse::{Event, Sse},
};
use futures::StreamExt;
use monadclaw_agent::AgentSession;
use monadclaw_chat::ChatMessage;
use monadclaw_providers::ProviderEvent;
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

    // Build a session from the workspace context + request history.
    let mut session = if state.workspace_context.is_empty() {
        AgentSession::new()
    } else {
        AgentSession::with_workspace_context(state.workspace_context.as_str())
    };

    // Replay all messages except the last as history, then begin the new turn.
    let (prior, last) = body.messages.split_at(body.messages.len().saturating_sub(1));
    for pair in prior.chunks(2) {
        if let [user, assistant] = pair {
            session.replay(&user.content, &assistant.content);
        }
    }

    let user_input = last
        .first()
        .map(|m| m.content.as_str())
        .unwrap_or_default();
    let turn = session.begin_turn(user_input);
    let mut llm_stream = state.provider.stream(turn.messages);

    let (tx, rx) = mpsc::channel::<Result<Event, std::convert::Infallible>>(32);

    tokio::spawn(async move {
        while let Some(event) = llm_stream.next().await {
            let data = match event {
                ProviderEvent::Delta(text) => text,
                ProviderEvent::Done => "[DONE]".to_string(),
                ProviderEvent::Error(msg) => {
                    error!(error = %msg, "LLM stream error");
                    format!("[ERROR] {msg}")
                }
            };
            if tx.send(Ok(Event::default().data(data))).await.is_err() {
                break;
            }
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)))
}
