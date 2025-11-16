pub mod traits;
pub mod deepseek;
pub mod gemini;
pub mod claude;

pub use traits::{LlmProvider, ChatRequest, ChatResponse, ChatMessage, ChatRole, ChatChunk};
pub use deepseek::DeepSeekProvider;
pub use gemini::GeminiProvider;
pub use claude::ClaudeProvider;

use crate::config::ProviderConfig;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("EventSource error: {0}")]
    EventSourceError(#[from] reqwest_eventsource::Error),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

/// Create a provider instance from configuration
pub fn create_provider(config: &ProviderConfig) -> Result<Arc<dyn LlmProvider>, ProviderError> {
    let provider: Arc<dyn LlmProvider> = match config.provider_id.as_str() {
        "deepseek" => Arc::new(DeepSeekProvider::new(
            config.api_key.clone(),
            config.base_url.clone(),
        )),
        "gemini" => Arc::new(GeminiProvider::new(
            config.api_key.clone(),
            config.base_url.clone(),
        )),
        "claude" => Arc::new(ClaudeProvider::new(
            config.api_key.clone(),
            config.base_url.clone(),
        )),
        _ => {
            return Err(ProviderError::InvalidConfiguration(format!(
                "Unknown provider: {}",
                config.provider_id
            )))
        }
    };

    Ok(provider)
}
