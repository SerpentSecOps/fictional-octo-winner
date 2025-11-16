use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::ProviderError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[serde(default)]
    pub temperature: Option<f32>,

    #[serde(default)]
    pub max_tokens: Option<u32>,

    #[serde(default)]
    pub top_p: Option<f32>,

    #[serde(default)]
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,

    #[serde(default)]
    pub finish_reason: Option<String>,

    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChunk {
    pub delta: String,

    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Provider identifier (e.g., "deepseek", "gemini", "claude")
    fn id(&self) -> &'static str;

    /// Human-readable provider name
    fn name(&self) -> &'static str;

    /// Send a chat completion request (non-streaming)
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError>;

    /// Send a streaming chat completion request
    /// Chunks are sent via the provided channel
    async fn stream_chat(
        &self,
        request: ChatRequest,
        tx: tokio::sync::mpsc::Sender<ChatChunk>,
    ) -> Result<(), ProviderError>;

    /// Generate embeddings for text (used for RAG)
    async fn embed(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, ProviderError> {
        // Default implementation: not supported
        let _ = texts;
        Err(ProviderError::UnsupportedFeature(
            "Embeddings not supported by this provider".to_string(),
        ))
    }
}
