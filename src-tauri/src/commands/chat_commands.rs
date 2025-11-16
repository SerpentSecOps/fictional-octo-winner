use crate::config::ConfigStore;
use crate::llm_providers::{create_provider, ChatChunk, ChatMessage, ChatRequest, ChatResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use super::config_commands::CommandResult;

#[derive(Debug, Deserialize)]
pub struct SendChatRequest {
    pub provider_id: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub stream: bool,
}

/// Send a chat message (non-streaming)
#[tauri::command]
pub async fn send_chat_message(
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    request: SendChatRequest,
) -> Result<CommandResult<ChatResponse>, String> {
    let store = config_store.lock().await;

    // Get provider config
    let provider_config = match store.get_provider(&request.provider_id) {
        Ok(config) => config,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    drop(store);

    // Create provider instance
    let provider = match create_provider(&provider_config) {
        Ok(p) => p,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Send chat request
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        top_p: request.top_p,
        stream: false,
    };

    match provider.chat(chat_request).await {
        Ok(response) => Ok(CommandResult::ok(response)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Send a streaming chat message
/// Chunks are emitted via the 'chat-chunk' event
#[tauri::command]
pub async fn send_chat_message_stream(
    app_handle: AppHandle,
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    request: SendChatRequest,
    request_id: String, // Unique ID for this request
) -> Result<CommandResult<()>, String> {
    let store = config_store.lock().await;

    // Get provider config
    let provider_config = match store.get_provider(&request.provider_id) {
        Ok(config) => config,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    drop(store);

    // Create provider instance
    let provider = match create_provider(&provider_config) {
        Ok(p) => p,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Create channel for streaming
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ChatChunk>(100);

    // Spawn task to receive chunks and emit events
    let app_handle_clone = app_handle.clone();
    let request_id_clone = request_id.clone();
    tokio::spawn(async move {
        while let Some(chunk) = rx.recv().await {
            #[derive(Clone, Serialize)]
            struct ChunkEvent {
                request_id: String,
                delta: String,
                finish_reason: Option<String>,
            }

            let _ = app_handle_clone.emit_all(
                "chat-chunk",
                ChunkEvent {
                    request_id: request_id_clone.clone(),
                    delta: chunk.delta,
                    finish_reason: chunk.finish_reason,
                },
            );
        }

        // Emit completion event
        let _ = app_handle_clone.emit_all("chat-complete", request_id_clone);
    });

    // Send streaming request
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        top_p: request.top_p,
        stream: true,
    };

    tokio::spawn(async move {
        if let Err(e) = provider.stream_chat(chat_request, tx).await {
            tracing::error!("Streaming error: {}", e);
        }
    });

    Ok(CommandResult::ok(()))
}
