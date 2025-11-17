use crate::rag::{Conversation, Message, RagDatabase};
use crate::validation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::config_commands::CommandResult;

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    pub title: String,
    pub provider_id: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct AddMessageRequest {
    pub conversation_id: i64,
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ConversationWithMessages {
    pub conversation: Conversation,
    pub messages: Vec<Message>,
}

/// Create a new conversation
#[tauri::command]
pub async fn create_conversation(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    request: CreateConversationRequest,
) -> Result<CommandResult<Conversation>, String> {
    // Validate inputs
    if let Err(e) = validation::validate_name("conversation title", &request.title) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("provider_id", &request.provider_id) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("model", &request.model) {
        return Ok(CommandResult::err(e.to_string()));
    }

    let db = rag_db.lock().await;

    match db
        .create_conversation(request.title, request.provider_id, request.model)
        .await
    {
        Ok(conversation) => Ok(CommandResult::ok(conversation)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// List all conversations
#[tauri::command]
pub async fn list_conversations(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
) -> Result<CommandResult<Vec<Conversation>>, String> {
    let db = rag_db.lock().await;

    match db.list_conversations().await {
        Ok(conversations) => Ok(CommandResult::ok(conversations)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Get a conversation with its messages
#[tauri::command]
pub async fn get_conversation_with_messages(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    conversation_id: i64,
) -> Result<CommandResult<ConversationWithMessages>, String> {
    let db = rag_db.lock().await;

    let conversation = match db.get_conversation(conversation_id).await {
        Ok(c) => c,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    let messages = match db.get_conversation_messages(conversation_id).await {
        Ok(m) => m,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    Ok(CommandResult::ok(ConversationWithMessages {
        conversation,
        messages,
    }))
}

/// Update conversation title
#[tauri::command]
pub async fn update_conversation_title(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    conversation_id: i64,
    title: String,
) -> Result<CommandResult<()>, String> {
    // Validate title
    if let Err(e) = validation::validate_name("conversation title", &title) {
        return Ok(CommandResult::err(e.to_string()));
    }

    let db = rag_db.lock().await;

    match db.update_conversation_title(conversation_id, title).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Delete a conversation
#[tauri::command]
pub async fn delete_conversation(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    conversation_id: i64,
) -> Result<CommandResult<()>, String> {
    let db = rag_db.lock().await;

    match db.delete_conversation(conversation_id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Add a message to a conversation
#[tauri::command]
pub async fn add_message(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    request: AddMessageRequest,
) -> Result<CommandResult<Message>, String> {
    // Validate inputs
    if let Err(e) = validation::validate_not_empty("role", &request.role) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("content", &request.content) {
        return Ok(CommandResult::err(e.to_string()));
    }
    // Limit message content to reasonable size (1MB)
    if let Err(e) = validation::validate_length("content", &request.content, None, Some(1_048_576)) {
        return Ok(CommandResult::err(e.to_string()));
    }

    let db = rag_db.lock().await;

    match db
        .add_message(request.conversation_id, request.role, request.content)
        .await
    {
        Ok(message) => Ok(CommandResult::ok(message)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Get messages for a conversation
#[tauri::command]
pub async fn get_conversation_messages(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    conversation_id: i64,
) -> Result<CommandResult<Vec<Message>>, String> {
    let db = rag_db.lock().await;

    match db.get_conversation_messages(conversation_id).await {
        Ok(messages) => Ok(CommandResult::ok(messages)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Delete a message
#[tauri::command]
pub async fn delete_message(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    message_id: i64,
) -> Result<CommandResult<()>, String> {
    let db = rag_db.lock().await;

    match db.delete_message(message_id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}
