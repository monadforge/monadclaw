use monadclaw_chat::{ChatMessage, Role};
use monadclaw_providers::Provider;

use crate::error::AgentError;

/// A completed exchange: one user message paired with the assistant's reply.
#[derive(Debug, Clone)]
pub struct Exchange {
    pub user: String,
    pub assistant: String,
}

/// A prepared turn ready to be executed against a provider.
///
/// Created by [`AgentSession::begin_turn`]. Pass `messages` to
/// [`Provider::stream`], then call [`AgentSession::commit`] with the
/// collected response text to persist the exchange.
pub struct PendingTurn {
    /// The user input for this turn.
    pub input: String,
    /// Full message list (system + history + this user message) for the provider.
    pub messages: Vec<ChatMessage>,
}

/// Manages conversation state for a single agent session.
///
/// The session tracks completed exchanges (pairs of user input + assistant
/// reply) and an optional system prompt. It does **not** own a provider;
/// the caller supplies one at turn time.
///
/// # Stateless API usage
///
/// For the current HTTP endpoint (frontend sends full history each request):
/// 1. Reconstruct a session by replaying prior exchanges.
/// 2. Call [`begin_turn`] with the new user message.
/// 3. Pass `PendingTurn::messages` to the provider stream.
/// 4. Optionally call [`commit`] to persist the reply (useful for stateful sessions).
///
/// [`begin_turn`]: AgentSession::begin_turn
/// [`commit`]: AgentSession::commit
#[derive(Debug, Default)]
pub struct AgentSession {
    system_prompt: Option<String>,
    /// Pre-built context block from workspace files (SOUL.md, USER.md, etc.).
    /// Injected at the start of the system prompt when present.
    workspace_context: Option<String>,
    history: Vec<Exchange>,
}

impl AgentSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_system(prompt: impl Into<String>) -> Self {
        Self {
            system_prompt: Some(prompt.into()),
            workspace_context: None,
            history: Vec::new(),
        }
    }

    /// Create a session pre-loaded with workspace context.
    ///
    /// `workspace_context` is the output of [`WorkspaceContext::build_context`]
    /// and is prepended to the system prompt on every turn.
    pub fn with_workspace_context(context: impl Into<String>) -> Self {
        Self {
            system_prompt: None,
            workspace_context: Some(context.into()),
            history: Vec::new(),
        }
    }

    /// Set an additional system prompt alongside workspace context.
    pub fn set_system_prompt(&mut self, prompt: impl Into<String>) {
        self.system_prompt = Some(prompt.into());
    }

    /// Replay a completed exchange into history without calling the provider.
    ///
    /// Used to reconstruct a session from stored conversation data.
    pub fn replay(&mut self, user: impl Into<String>, assistant: impl Into<String>) {
        self.history.push(Exchange {
            user: user.into(),
            assistant: assistant.into(),
        });
    }

    /// Prepare the next turn.
    ///
    /// Builds the full message list (system prompt + history + new user message)
    /// without modifying session state. Call [`commit`] after streaming to
    /// persist the assistant reply.
    ///
    /// [`commit`]: AgentSession::commit
    pub fn begin_turn(&self, input: impl Into<String>) -> PendingTurn {
        let input = input.into();
        let messages = self.build_messages(&input);
        PendingTurn { input, messages }
    }

    /// Commit a completed turn to history.
    pub fn commit(&mut self, turn: PendingTurn, response: impl Into<String>) {
        self.history.push(Exchange {
            user: turn.input,
            assistant: response.into(),
        });
    }

    /// All completed exchanges in order.
    pub fn history(&self) -> &[Exchange] {
        &self.history
    }

    /// Build the full message list for the provider from current state.
    fn build_messages(&self, user_input: &str) -> Vec<ChatMessage> {
        // Build the system content: workspace context + optional system prompt.
        let system_content: Option<String> = match (&self.workspace_context, &self.system_prompt) {
            (Some(ctx), Some(sys)) => Some(format!("{ctx}\n\n---\n\n{sys}")),
            (Some(ctx), None) => Some(ctx.clone()),
            (None, Some(sys)) => Some(sys.clone()),
            (None, None) => None,
        };

        let has_system = system_content.is_some() as usize;
        let mut msgs = Vec::with_capacity(has_system + self.history.len() * 2 + 1);

        if let Some(content) = system_content {
            msgs.push(ChatMessage { role: Role::System, content });
        }

        for exchange in &self.history {
            msgs.push(ChatMessage { role: Role::User, content: exchange.user.clone() });
            msgs.push(ChatMessage { role: Role::Assistant, content: exchange.assistant.clone() });
        }

        msgs.push(ChatMessage { role: Role::User, content: user_input.to_string() });
        msgs
    }
}

/// Run a single turn to completion, collecting the full response text.
///
/// This is a convenience function for non-streaming callers (e.g. Discord bot).
/// For streaming use cases (e.g. HTTP SSE), use [`AgentSession::begin_turn`]
/// and drive the stream directly.
pub async fn run_turn(
    session: &mut AgentSession,
    input: impl Into<String>,
    provider: &dyn Provider,
) -> Result<String, AgentError> {
    use monadclaw_providers::ProviderEvent;

    let turn = session.begin_turn(input);
    let mut stream = provider.stream(turn.messages.clone());
    let mut response = String::new();

    use futures::StreamExt;
    while let Some(event) = stream.next().await {
        match event {
            ProviderEvent::Delta(text) => response.push_str(&text),
            ProviderEvent::Done => break,
            ProviderEvent::Error(msg) => return Err(AgentError::Provider(msg)),
        }
    }

    session.commit(PendingTurn { input: turn.input, messages: turn.messages }, &response);
    Ok(response)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::pin::Pin;

    use monadclaw_chat::ChatMessage;
    use monadclaw_providers::{Provider, ProviderEvent};
    use futures::stream::Stream;

    use super::*;

    // ── Stub provider for tests ─────────────────────────────────────────────

    struct EchoProvider;

    impl Provider for EchoProvider {
        fn stream(
            &self,
            messages: Vec<ChatMessage>,
        ) -> Pin<Box<dyn Stream<Item = ProviderEvent> + Send>> {
            let last = messages.last().map(|m| m.content.clone()).unwrap_or_default();
            let reply = format!("echo: {last}");
            Box::pin(async_stream::stream! {
                yield ProviderEvent::Delta(reply);
                yield ProviderEvent::Done;
            })
        }
    }

    // ── AgentSession unit tests ─────────────────────────────────────────────

    #[test]
    fn new_session_has_no_history() {
        let session = AgentSession::new();
        assert!(session.history().is_empty());
    }

    #[test]
    fn begin_turn_includes_user_message() {
        let session = AgentSession::new();
        let turn = session.begin_turn("hello");
        assert_eq!(turn.input, "hello");
        assert_eq!(turn.messages.len(), 1);
        assert_eq!(turn.messages[0].content, "hello");
        assert_eq!(turn.messages[0].role, Role::User);
    }

    #[test]
    fn with_system_prepends_system_message() {
        let session = AgentSession::with_system("You are helpful.");
        let turn = session.begin_turn("hi");
        assert_eq!(turn.messages.len(), 2);
        assert_eq!(turn.messages[0].role, Role::System);
        assert_eq!(turn.messages[0].content, "You are helpful.");
        assert_eq!(turn.messages[1].role, Role::User);
    }

    #[test]
    fn commit_appends_exchange_to_history() {
        let mut session = AgentSession::new();
        let turn = session.begin_turn("ping");
        session.commit(turn, "pong");
        assert_eq!(session.history().len(), 1);
        assert_eq!(session.history()[0].user, "ping");
        assert_eq!(session.history()[0].assistant, "pong");
    }

    #[test]
    fn second_turn_includes_prior_exchange() {
        let mut session = AgentSession::new();
        let t1 = session.begin_turn("first");
        session.commit(t1, "reply one");

        let t2 = session.begin_turn("second");
        // system(0) + user(1) + assistant(2) + user(3) — no system here, so 3 messages
        assert_eq!(t2.messages.len(), 3);
        assert_eq!(t2.messages[0].role, Role::User);
        assert_eq!(t2.messages[0].content, "first");
        assert_eq!(t2.messages[1].role, Role::Assistant);
        assert_eq!(t2.messages[1].content, "reply one");
        assert_eq!(t2.messages[2].role, Role::User);
        assert_eq!(t2.messages[2].content, "second");
    }

    #[test]
    fn replay_builds_history_without_provider() {
        let mut session = AgentSession::new();
        session.replay("q1", "a1");
        session.replay("q2", "a2");
        assert_eq!(session.history().len(), 2);
        let turn = session.begin_turn("q3");
        // user + assistant + user + assistant + user = 5 messages
        assert_eq!(turn.messages.len(), 5);
    }

    #[tokio::test]
    async fn run_turn_collects_response_and_commits() {
        let mut session = AgentSession::new();
        let reply = run_turn(&mut session, "hello", &EchoProvider).await.unwrap();
        assert_eq!(reply, "echo: hello");
        assert_eq!(session.history().len(), 1);
        assert_eq!(session.history()[0].assistant, "echo: hello");
    }

    #[tokio::test]
    async fn run_turn_multi_turn_context() {
        let mut session = AgentSession::new();
        run_turn(&mut session, "first", &EchoProvider).await.unwrap();
        let reply = run_turn(&mut session, "second", &EchoProvider).await.unwrap();
        // EchoProvider echoes the last message; with history the last message is "second"
        assert_eq!(reply, "echo: second");
        assert_eq!(session.history().len(), 2);
    }
}
