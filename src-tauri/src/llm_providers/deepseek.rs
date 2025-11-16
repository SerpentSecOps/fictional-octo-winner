use super::traits::*;
use super::ProviderError;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct DeepSeekProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl DeepSeekProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.deepseek.com".to_string()),
            client: reqwest::Client::new(),
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        headers
    }

    fn convert_messages(&self, messages: &[ChatMessage]) -> Vec<serde_json::Value> {
        messages
            .iter()
            .map(|msg| {
                json!({
                    "role": match msg.role {
                        ChatRole::System => "system",
                        ChatRole::User => "user",
                        ChatRole::Assistant => "assistant",
                    },
                    "content": msg.content
                })
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
    usage: Option<DeepSeekUsage>,
    model: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamChunk {
    choices: Vec<DeepSeekStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamChoice {
    delta: DeepSeekDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekDelta {
    #[serde(default)]
    content: Option<String>,
}

#[async_trait]
impl LlmProvider for DeepSeekProvider {
    fn id(&self) -> &'static str {
        "deepseek"
    }

    fn name(&self) -> &'static str {
        "DeepSeek"
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let url = format!("{}/v1/chat/completions", self.base_url);

        let body = json!({
            "model": request.model,
            "messages": self.convert_messages(&request.messages),
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "top_p": request.top_p,
            "stream": false,
        });

        let response = self
            .client
            .post(&url)
            .headers(self.create_headers())
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(ProviderError::ApiError(format!(
                "DeepSeek API error: {}",
                error_text
            )));
        }

        let deepseek_response: DeepSeekResponse = response.json().await?;

        let choice = deepseek_response
            .choices
            .first()
            .ok_or_else(|| ProviderError::ApiError("No choices in response".to_string()))?;

        Ok(ChatResponse {
            content: choice.message.content.clone(),
            model: deepseek_response.model,
            finish_reason: choice.finish_reason.clone(),
            usage: deepseek_response.usage.map(|u| Usage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
        })
    }

    async fn stream_chat(
        &self,
        request: ChatRequest,
        tx: tokio::sync::mpsc::Sender<ChatChunk>,
    ) -> Result<(), ProviderError> {
        use reqwest_eventsource::{Event, EventSource};

        let url = format!("{}/v1/chat/completions", self.base_url);

        let body = json!({
            "model": request.model,
            "messages": self.convert_messages(&request.messages),
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "top_p": request.top_p,
            "stream": true,
        });

        let req = self
            .client
            .post(&url)
            .headers(self.create_headers())
            .json(&body)
            .build()?;

        let mut event_source = EventSource::new(req)?;

        while let Some(event) = event_source.next().await {
            match event {
                Ok(Event::Message(message)) => {
                    if message.data == "[DONE]" {
                        break;
                    }

                    let chunk: DeepSeekStreamChunk = match serde_json::from_str(&message.data) {
                        Ok(c) => c,
                        Err(e) => {
                            tracing::warn!("Failed to parse chunk: {}", e);
                            continue;
                        }
                    };

                    if let Some(choice) = chunk.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            let _ = tx
                                .send(ChatChunk {
                                    delta: content.clone(),
                                    finish_reason: choice.finish_reason.clone(),
                                })
                                .await;
                        }
                    }
                }
                Ok(Event::Open) => {
                    tracing::debug!("DeepSeek stream opened");
                }
                Err(e) => {
                    tracing::error!("DeepSeek stream error: {}", e);
                    return Err(ProviderError::ApiError(format!("Stream error: {}", e)));
                }
            }
        }

        event_source.close();
        Ok(())
    }
}
