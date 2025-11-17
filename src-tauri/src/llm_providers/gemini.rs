use super::traits::*;
use super::ProviderError;
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct GeminiProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl GeminiProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| {
                "https://generativelanguage.googleapis.com/v1".to_string()
            }),
            client: reqwest::Client::new(),
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    fn convert_messages(&self, messages: &[ChatMessage]) -> (Option<String>, Vec<serde_json::Value>) {
        let mut system_instruction = None;
        let mut contents = Vec::new();

        for msg in messages {
            match msg.role {
                ChatRole::System => {
                    // Gemini has a separate system_instruction field
                    system_instruction = Some(msg.content.clone());
                }
                ChatRole::User => {
                    contents.push(json!({
                        "role": "user",
                        "parts": [{"text": msg.content}]
                    }));
                }
                ChatRole::Assistant => {
                    contents.push(json!({
                        "role": "model",
                        "parts": [{"text": msg.content}]
                    }));
                }
            }
        }

        (system_instruction, contents)
    }
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsage>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiUsage {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: u32,
    #[serde(rename = "totalTokenCount")]
    total_token_count: u32,
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    fn id(&self) -> &'static str {
        "gemini"
    }

    fn name(&self) -> &'static str {
        "Google Gemini"
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, request.model, self.api_key
        );

        let (system_instruction, contents) = self.convert_messages(&request.messages);

        let mut body = json!({
            "contents": contents,
            "generationConfig": {}
        });

        if let Some(system) = system_instruction {
            body["systemInstruction"] = json!({
                "parts": [{"text": system}]
            });
        }

        if let Some(temp) = request.temperature {
            body["generationConfig"]["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            body["generationConfig"]["maxOutputTokens"] = json!(max_tokens);
        }
        if let Some(top_p) = request.top_p {
            body["generationConfig"]["topP"] = json!(top_p);
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
                "Gemini API error: {}",
                error_text
            )));
        }

        let gemini_response: GeminiResponse = response.json().await?;

        let candidate = gemini_response
            .candidates
            .first()
            .ok_or_else(|| ProviderError::ApiError("No candidates in response".to_string()))?;

        let text = candidate
            .content
            .parts
            .first()
            .map(|p| p.text.clone())
            .unwrap_or_default();

        Ok(ChatResponse {
            content: text,
            model: request.model,
            finish_reason: candidate.finish_reason.clone(),
            usage: gemini_response.usage_metadata.map(|u| Usage {
                prompt_tokens: u.prompt_token_count,
                completion_tokens: u.candidates_token_count,
                total_tokens: u.total_token_count,
            }),
        })
    }

    async fn stream_chat(
        &self,
        request: ChatRequest,
        tx: tokio::sync::mpsc::Sender<ChatChunk>,
    ) -> Result<(), ProviderError> {
        let url = format!(
            "{}/models/{}:streamGenerateContent?alt=sse&key={}",
            self.base_url, request.model, self.api_key
        );

        let (system_instruction, contents) = self.convert_messages(&request.messages);

        let mut body = json!({
            "contents": contents,
            "generationConfig": {}
        });

        if let Some(system) = system_instruction {
            body["systemInstruction"] = json!({
                "parts": [{"text": system}]
            });
        }

        if let Some(temp) = request.temperature {
            body["generationConfig"]["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            body["generationConfig"]["maxOutputTokens"] = json!(max_tokens);
        }
        if let Some(top_p) = request.top_p {
            body["generationConfig"]["topP"] = json!(top_p);
        }

        // Create EventSource for SSE streaming
        let event_source = EventSource::new(
            self.client
                .post(&url)
                .headers(self.create_headers())
                .json(&body)
        )?;

        let mut stream = event_source;

        while let Some(event) = stream.next().await {
            match event {
                Ok(Event::Open) => {
                    // Connection opened, continue
                }
                Ok(Event::Message(message)) => {
                    // Parse the SSE message data
                    if let Ok(gemini_response) = serde_json::from_str::<GeminiResponse>(&message.data) {
                        if let Some(candidate) = gemini_response.candidates.first() {
                            if let Some(part) = candidate.content.parts.first() {
                                let chunk = ChatChunk {
                                    delta: part.text.clone(),
                                    finish_reason: candidate.finish_reason.clone(),
                                };

                                if tx.send(chunk).await.is_err() {
                                    // Receiver dropped, stop streaming
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    // Stream error
                    tracing::error!("Gemini SSE stream error: {}", err);
                    return Err(ProviderError::ApiError(format!(
                        "Stream error: {}",
                        err
                    )));
                }
            }
        }

        Ok(())
    }

    async fn embed(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, ProviderError> {
        // Gemini supports embeddings via the embedding-001 model
        let url = format!(
            "{}/models/embedding-001:embedContent?key={}",
            self.base_url, self.api_key
        );

        let mut embeddings = Vec::new();

        for text in texts {
            let body = json!({
                "content": {
                    "parts": [{"text": text}]
                }
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
                    "Gemini embedding API error: {}",
                    error_text
                )));
            }

            #[derive(Deserialize)]
            struct EmbedResponse {
                embedding: EmbeddingData,
            }

            #[derive(Deserialize)]
            struct EmbeddingData {
                values: Vec<f32>,
            }

            let embed_response: EmbedResponse = response.json().await?;
            embeddings.push(embed_response.embedding.values);
        }

        Ok(embeddings)
    }
}
