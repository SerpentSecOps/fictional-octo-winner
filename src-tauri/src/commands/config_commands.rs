use crate::config::{ConfigStore, MaskedProviderConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct CommandResult<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProviderRequest {
    pub provider_id: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub default_model: Option<String>,
    pub enabled: Option<bool>,
}

/// Get all providers (masked, without API keys)
#[tauri::command]
pub async fn get_providers(
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
) -> Result<CommandResult<Vec<MaskedProviderConfig>>, String> {
    let store = config_store.lock().await;

    match store.get_all_providers_masked() {
        Ok(providers) => Ok(CommandResult::ok(providers)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Update or create a provider configuration
#[tauri::command]
pub async fn update_provider(
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    request: UpdateProviderRequest,
) -> Result<CommandResult<()>, String> {
    let store = config_store.lock().await;

    match store.update_provider(
        request.provider_id,
        request.api_key,
        request.base_url,
        request.default_model,
        request.enabled,
    ) {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Delete a provider configuration
#[tauri::command]
pub async fn delete_provider(
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    provider_id: String,
) -> Result<CommandResult<()>, String> {
    let store = config_store.lock().await;

    match store.delete_provider(&provider_id) {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Test provider connection
#[tauri::command]
pub async fn test_provider_connection(
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    provider_id: String,
) -> Result<CommandResult<String>, String> {
    use crate::llm_providers::{create_provider, ChatMessage, ChatRequest, ChatRole};

    let store = config_store.lock().await;

    // Get provider config
    let provider_config = match store.get_provider(&provider_id) {
        Ok(config) => config,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    drop(store); // Release lock

    // Create provider instance
    let provider = match create_provider(&provider_config) {
        Ok(p) => p,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Send a simple test request
    let test_request = ChatRequest {
        model: provider_config
            .default_model
            .clone()
            .unwrap_or_else(|| "default".to_string()),
        messages: vec![ChatMessage {
            role: ChatRole::User,
            content: "Hello, this is a test. Please respond with 'OK'.".to_string(),
        }],
        temperature: Some(0.7),
        max_tokens: Some(50),
        top_p: None,
        stream: false,
    };

    match provider.chat(test_request).await {
        Ok(response) => Ok(CommandResult::ok(format!(
            "Connection successful. Response: {}",
            response.content
        ))),
        Err(e) => Ok(CommandResult::err(format!("Connection failed: {}", e))),
    }
}
