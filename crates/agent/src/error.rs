/// Errors that can occur during agent turn execution.
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("provider error: {0}")]
    Provider(String),
}
