use std::{pin::Pin, sync::Arc};

use futures::StreamExt;
use monadclaw_chat::ChatMessage;
use monadclaw_config::ProviderConfig;
use tokio_stream::Stream;
use tracing::debug;

/// A streaming event from an LLM provider.
#[derive(Debug, Clone)]
pub enum ProviderEvent {
    /// A text delta (token chunk).
    Delta(String),
    /// Stream finished normally.
    Done,
    /// An error occurred; no further events will be produced.
    Error(String),
}

/// Trait implemented by every LLM provider backend.
pub trait Provider: Send + Sync {
    /// Stream a chat completion from the given message history.
    fn stream(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Pin<Box<dyn Stream<Item = ProviderEvent> + Send>>;
}

/// LLM provider backed by the `genai` crate (OpenAI-compatible + Anthropic).
pub struct GenaiProvider {
    model: String,
    client: Arc<genai::Client>,
}

impl GenaiProvider {
    /// Construct a provider from config and a resolved API key.
    pub fn new(config: &ProviderConfig, api_key: &str) -> Self {
        Self {
            model: config.model.clone(),
            client: Arc::new(build_client(api_key, config.base_url.as_deref())),
        }
    }
}

impl Provider for GenaiProvider {
    fn stream(&self, messages: Vec<ChatMessage>) -> Pin<Box<dyn Stream<Item = ProviderEvent> + Send>> {
        let model = self.model.clone();
        let client = Arc::clone(&self.client);

        Box::pin(async_stream::stream! {
            use genai::chat::ChatStreamEvent;

            debug!(model = %model, "starting chat stream");

            let genai_messages = to_genai_messages(&messages);
            let chat_req = genai::chat::ChatRequest::new(genai_messages);

            let mut chat_stream = match client.exec_chat_stream(&model, chat_req, None).await {
                Ok(s) => s,
                Err(e) => {
                    yield ProviderEvent::Error(format!("{e}"));
                    return;
                }
            };

            while let Some(result) = chat_stream.stream.next().await {
                match result {
                    Ok(ChatStreamEvent::Chunk(chunk)) if !chunk.content.is_empty() => {
                        yield ProviderEvent::Delta(chunk.content);
                    }
                    Ok(ChatStreamEvent::End(_)) => {
                        yield ProviderEvent::Done;
                        return;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        yield ProviderEvent::Error(format!("{e}"));
                        return;
                    }
                }
            }

            yield ProviderEvent::Done;
        })
    }
}

fn to_genai_messages(messages: &[ChatMessage]) -> Vec<genai::chat::ChatMessage> {
    use monadclaw_chat::Role;
    messages
        .iter()
        .map(|m| match m.role {
            Role::User => genai::chat::ChatMessage::user(&m.content),
            Role::Assistant => genai::chat::ChatMessage::assistant(&m.content),
            Role::System => genai::chat::ChatMessage::system(&m.content),
        })
        .collect()
}

/// Build a `genai::Client` configured with the given API key and optional base URL.
///
/// When `base_url` is provided the client is configured as an OpenAI-compatible
/// endpoint (e.g. OpenRouter at `https://openrouter.ai/api/v1/`).
fn build_client(api_key: &str, base_url: Option<&str>) -> genai::Client {
    let key = api_key.to_string();

    if let Some(url) = base_url {
        let url = url.to_string();
        genai::Client::builder()
            .with_service_target_resolver(
                genai::resolver::ServiceTargetResolver::from_resolver_fn(
                    move |service_target: genai::ServiceTarget| {
                        Ok(genai::ServiceTarget {
                            endpoint: genai::resolver::Endpoint::from_owned(url.clone()),
                            auth: genai::resolver::AuthData::from_single(key.clone()),
                            model: genai::ModelIden::new(
                                genai::adapter::AdapterKind::OpenAI,
                                service_target.model.model_name,
                            ),
                        })
                    },
                ),
            )
            .build()
    } else {
        genai::Client::builder()
            .with_auth_resolver(genai::resolver::AuthResolver::from_resolver_fn(
                move |_model_iden| {
                    Ok(Some(genai::resolver::AuthData::from_single(key.clone())))
                },
            ))
            .build()
    }
}
