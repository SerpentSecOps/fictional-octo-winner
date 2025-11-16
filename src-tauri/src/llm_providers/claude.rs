use super::traits::*;
use super::ProviderError;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct ClaudeProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl ClaudeProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.anthropic.com".to_string()),
            client: reqwest::Client::new(),
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static("2023-06-01"),
        );
        headers
    }

    fn convert_messages(&self, messages: &[ChatMessage]) -> (Option<String>, Vec<serde_json::Value>) {
        let mut system_prompt = None;
        let mut claude_messages = Vec::new();

        for msg in messages {
            match msg.role {
                ChatRole::System => {
                    // Claude has a separate system field
                    system_prompt = Some(msg.content.clone());
                }
                ChatRole::User => {
                    claude_messages.push(json!({
                        "role": "user",
                        "content": msg.content
                    }));
                }
                ChatRole::Assistant => {
                    claude_messages.push(json!({
                        "role": "assistant",
                        "content": msg.content
                    }));
                }
            }
        }

        (system_prompt, claude_messages)
    }
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    model: String,
    stop_reason: Option<String>,
    usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ClaudeStreamEvent {
    #[serde(rename = "type")]
    event_type: String,

    #[serde(default)]
    delta: Option<ClaudeDelta>,

    #[serde(default)]
    message: Option<ClaudeMessageEvent>,
}

#[derive(Debug, Deserialize)]
struct ClaudeDelta {
    #[serde(rename = "type")]
    delta_type: String,

    #[serde(default)]
    text: Option<String>,

    #[serde(default)]
    stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClaudeMessageEvent {
    usage: ClaudeUsage,
}

#[async_trait]
impl LlmProvider for ClaudeProvider {
    fn id(&self) -> &'static str {
        "claude"
    }

    fn name(&self) -> &'static str {
        "Anthropic Claude"
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let url = format!("{}/v1/messages", self.base_url);

        let (system_prompt, messages) = self.convert_messages(&request.messages);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(4096),
        });

        if let Some(system) = system_prompt {
            body["system"] = json!(system);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }

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
                "Claude API error: {}",
                error_text
            )));
        }

        let claude_response: ClaudeResponse = response.json().await?;

        let text = claude_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        Ok(ChatResponse {
            content: text,
            model: claude_response.model,
            finish_reason: claude_response.stop_reason,
            usage: Some(Usage {
                prompt_tokens: claude_response.usage.input_tokens,
                completion_tokens: claude_response.usage.output_tokens,
                total_tokens: claude_response.usage.input_tokens
                    + claude_response.usage.output_tokens,
            }),
        })
    }

    async fn stream_chat(
        &self,
        request: ChatRequest,
        tx: tokio::sync::mpsc::Sender<ChatChunk>,
    ) -> Result<(), ProviderError> {
        use reqwest_eventsource::{Event, EventSource};
        use futures_util::stream::StreamExt;

        let url = format!("{}/v1/messages", self.base_url);

        let (system_prompt, messages) = self.convert_messages(&request.messages);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(4096),
            "stream": true,
        });

        if let Some(system) = system_prompt {
            body["system"] = json!(system);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }

        let req_builder = self
            .client
            .post(&url)
            .headers(self.create_headers())
            .json(&body);

        let mut event_source = EventSource::new(req_builder)?;

        while let Some(event) = event_source.next().await {
            match event {
                Ok(Event::Message(message)) => {
                    let event: ClaudeStreamEvent = match serde_json::from_str(&message.data) {
                        Ok(e) => e,
                        Err(e) => {
                            tracing::warn!("Failed to parse Claude event: {}", e);
                            continue;
                        }
                    };

                    match event.event_type.as_str() {
                        "content_block_delta" => {
                            if let Some(delta) = event.delta {
                                if let Some(text) = delta.text {
                                    let _ = tx
                                        .send(ChatChunk {
                                            delta: text,
                                            finish_reason: None,
                                        })
                                        .await;
                                }
                            }
                        }
                        "message_delta" => {
                            if let Some(delta) = event.delta {
                                if let Some(stop_reason) = delta.stop_reason {
                                    let _ = tx
                                        .send(ChatChunk {
                                            delta: String::new(),
                                            finish_reason: Some(stop_reason),
                                        })
                                        .await;
                                }
                            }
                        }
                        "message_stop" => {
                            break;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Open) => {
                    tracing::debug!("Claude stream opened");
                }
                Err(e) => {
                    tracing::error!("Claude stream error: {}", e);
                    return Err(ProviderError::ApiError(format!("Stream error: {}", e)));
                }
            }
        }

        event_source.close();
        Ok(())
    }
}
