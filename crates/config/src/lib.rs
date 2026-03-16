use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found at {0}")]
    NotFound(PathBuf),
    #[error("Failed to read config: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Provider '{0}' not found in config")]
    ProviderNotFound(String),
    #[error("API key env var '{0}' is not set")]
    MissingApiKey(String),
}

/// Per-provider settings in the TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Model identifier, e.g. "gpt-4o" or "claude-sonnet-4-6"
    pub model: String,
    /// Name of the environment variable holding the API key.
    pub api_key_env: String,
    /// Optional base URL for OpenAI-compatible custom endpoints (e.g. OpenRouter: https://openrouter.ai/api/v1/).
    /// When set, the provider is treated as an OpenAI-compatible API at this endpoint.
    pub base_url: Option<String>,
}

/// Top-level config shape, maps to `~/.config/monadclaw/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Name of the provider to use by default (must be a key in `providers`).
    pub active_provider: String,
    /// Map of provider name → provider settings.
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
    /// Optional dashboard password. When set, all API requests must supply it as a Bearer token.
    /// When absent, local (loopback) connections are allowed without auth; remote connections get 403.
    #[serde(default)]
    pub dashboard_password: Option<String>,
}

impl Config {
    /// Load config from the given path.
    pub fn load(path: &std::path::Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }
        let text = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&text)?;
        Ok(config)
    }

    /// Return the default config file path: `~/.config/monadclaw/config.toml`.
    /// Falls back to `./config.toml` if the home directory cannot be determined.
    pub fn default_path() -> PathBuf {
        directories::BaseDirs::new()
            .map(|b| b.config_dir().join("monadclaw").join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"))
    }

    /// Return the active `ProviderConfig`.
    pub fn active_provider_config(&self) -> Result<&ProviderConfig, ConfigError> {
        self.providers
            .get(&self.active_provider)
            .ok_or_else(|| ConfigError::ProviderNotFound(self.active_provider.clone()))
    }

    /// Resolve the API key for the active provider from the environment.
    pub fn resolve_api_key(&self) -> Result<String, ConfigError> {
        let provider = self.active_provider_config()?;
        std::env::var(&provider.api_key_env)
            .map_err(|_| ConfigError::MissingApiKey(provider.api_key_env.clone()))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::io::Write;

    use super::*;

    fn write_temp(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn parses_valid_toml() {
        let f = write_temp(
            r#"
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "OPENAI_API_KEY"
"#,
        );
        let config = Config::load(f.path()).unwrap();
        assert_eq!(config.active_provider, "openai");
        assert_eq!(config.providers["openai"].model, "gpt-4o");
    }

    #[test]
    fn missing_file_returns_not_found() {
        let result = Config::load(std::path::Path::new("/nonexistent/path/config.toml"));
        assert!(matches!(result, Err(ConfigError::NotFound(_))));
    }

    #[test]
    fn invalid_toml_returns_parse_error() {
        let f = write_temp("not valid toml ][");
        let result = Config::load(f.path());
        assert!(matches!(result, Err(ConfigError::Parse(_))));
    }

    #[test]
    fn active_provider_not_in_map_returns_error() {
        let f = write_temp(
            r#"
active_provider = "missing"

[providers.openai]
model = "gpt-4o"
api_key_env = "OPENAI_API_KEY"
"#,
        );
        let config = Config::load(f.path()).unwrap();
        assert!(matches!(
            config.active_provider_config(),
            Err(ConfigError::ProviderNotFound(_))
        ));
    }

    #[test]
    fn resolve_api_key_reads_env() {
        let f = write_temp(
            r#"
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "TEST_KEY_MONADCLAW"
"#,
        );
        // SAFETY: single-threaded test; no other thread reads this var concurrently.
        unsafe { std::env::set_var("TEST_KEY_MONADCLAW", "sk-test-123") };
        let config = Config::load(f.path()).unwrap();
        assert_eq!(config.resolve_api_key().unwrap(), "sk-test-123");
        // SAFETY: single-threaded test; no other thread reads this var concurrently.
        unsafe { std::env::remove_var("TEST_KEY_MONADCLAW") };
    }

    #[test]
    fn missing_env_var_returns_error() {
        let f = write_temp(
            r#"
active_provider = "openai"

[providers.openai]
model = "gpt-4o"
api_key_env = "DEFINITELY_NOT_SET_MONADCLAW"
"#,
        );
        // SAFETY: single-threaded test; no other thread reads this var concurrently.
        unsafe { std::env::remove_var("DEFINITELY_NOT_SET_MONADCLAW") };
        let config = Config::load(f.path()).unwrap();
        assert!(matches!(
            config.resolve_api_key(),
            Err(ConfigError::MissingApiKey(_))
        ));
    }
}
