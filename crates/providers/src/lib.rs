use std::pin::Pin;

use futures::StreamExt;
use monadclaw_chat::ChatMessage;
use monadclaw_config::ProviderConfig;
use tokio_stream::Stream;
use tracing::{debug, instrument};

/// A single streaming event from the LLM.
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// A text delta (token chunk).
    Delta(String),
    /// Stream finished normally.
    Done,
    /// An error occurred; stream will not produce more events.
    Error(String),
}

/// Convert our `ChatMessage` types to genai's format.
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

/// Build a `genai::Client` with an explicit API key.
///
/// When `base_url` is provided the client is configured as an OpenAI-compatible
/// endpoint at that URL (e.g. Kimi at `https://api.moonshot.cn/v1`).
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

/// Stream chat completions from the configured provider.
///
/// `api_key` is the resolved value of the env var named in `config.api_key_env`.
/// It is passed explicitly so the server can resolve it once at startup.
#[instrument(skip(messages, api_key))]
pub fn stream_chat(
    config: &ProviderConfig,
    api_key: String,
    messages: Vec<ChatMessage>,
) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send>> {
    let model = config.model.clone();
    let client = build_client(&api_key, config.base_url.as_deref());

    Box::pin(async_stream::stream! {
        use genai::chat::ChatStreamEvent;

        debug!(model = %model, "starting chat stream");

        let genai_messages = to_genai_messages(&messages);
        let chat_req = genai::chat::ChatRequest::new(genai_messages);

        let mut chat_stream = match client.exec_chat_stream(&model, chat_req, None).await {
            Ok(s) => s,
            Err(e) => {
                yield StreamEvent::Error(format!("{e}"));
                return;
            }
        };

        while let Some(result) = chat_stream.stream.next().await {
            match result {
                Ok(ChatStreamEvent::Chunk(chunk)) if !chunk.content.is_empty() => {
                    yield StreamEvent::Delta(chunk.content);
                }
                Ok(ChatStreamEvent::End(_)) => {
                    yield StreamEvent::Done;
                    return;
                }
                Ok(_) => {}
                Err(e) => {
                    yield StreamEvent::Error(format!("{e}"));
                    return;
                }
            }
        }

        yield StreamEvent::Done;
    })
}
