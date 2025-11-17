use crate::config::ConfigStore;
use crate::llm_providers::{create_provider, ChatMessage, ChatRequest, ChatRole};
use crate::rag::{chunk_text, search_similar, ChunkMatch, Document, EmbeddingService, Project, RagDatabase};
use crate::validation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::config_commands::CommandResult;

/// Create a new RAG project
#[tauri::command]
pub async fn create_project(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    name: String,
) -> Result<CommandResult<Project>, String> {
    // Validate project name
    if let Err(e) = validation::validate_name("project name", &name) {
        return Ok(CommandResult::err(e.to_string()));
    }

    let db = rag_db.lock().await;

    match db.create_project(name).await {
        Ok(project) => Ok(CommandResult::ok(project)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// List all RAG projects
#[tauri::command]
pub async fn list_projects(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
) -> Result<CommandResult<Vec<Project>>, String> {
    let db = rag_db.lock().await;

    match db.list_projects().await {
        Ok(projects) => Ok(CommandResult::ok(projects)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Delete a project
#[tauri::command]
pub async fn delete_project(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    project_id: i64,
) -> Result<CommandResult<()>, String> {
    let db = rag_db.lock().await;

    match db.delete_project(project_id).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// List documents in a project
#[tauri::command]
pub async fn list_documents(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    project_id: i64,
) -> Result<CommandResult<Vec<Document>>, String> {
    let db = rag_db.lock().await;

    match db.list_documents(project_id).await {
        Ok(documents) => Ok(CommandResult::ok(documents)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct AddDocumentRequest {
    pub project_id: i64,
    pub name: String,
    pub content: String,
    pub provider_id: String, // Provider to use for embeddings
}

#[derive(Debug, Serialize)]
pub struct AddDocumentResponse {
    pub document_id: i64,
    pub chunks_created: usize,
}

/// Add a document to a project and generate embeddings
#[tauri::command]
pub async fn add_document(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    request: AddDocumentRequest,
) -> Result<CommandResult<AddDocumentResponse>, String> {
    // Validate inputs
    if let Err(e) = validation::validate_name("document name", &request.name) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_document_content(&request.content) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("provider_id", &request.provider_id) {
        return Ok(CommandResult::err(e.to_string()));
    }

    // Get provider for embeddings
    let store = config_store.lock().await;
    let provider_config = match store.get_provider(&request.provider_id) {
        Ok(config) => config,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };
    drop(store);

    let provider = match create_provider(&provider_config) {
        Ok(p) => p,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    let embedding_service = EmbeddingService::new(provider);

    // Create document
    let db = rag_db.lock().await;
    let document = match db
        .create_document(request.project_id, request.name, None)
        .await
    {
        Ok(doc) => doc,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Chunk the text
    let chunks = chunk_text(&request.content, None);

    // Generate embeddings for all chunks
    let embeddings = match embedding_service.embed_texts(chunks.clone()).await {
        Ok(emb) => emb,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Insert chunks with embeddings
    let mut chunks_created = 0;
    for (idx, (chunk_text, embedding)) in chunks.iter().zip(embeddings.iter()).enumerate() {
        match db
            .insert_chunk(
                document.id,
                request.project_id,
                chunk_text.clone(),
                embedding.clone(),
                idx as i32,
            )
            .await
        {
            Ok(_) => chunks_created += 1,
            Err(e) => {
                tracing::error!("Failed to insert chunk {}: {}", idx, e);
            }
        }
    }

    drop(db);

    Ok(CommandResult::ok(AddDocumentResponse {
        document_id: document.id,
        chunks_created,
    }))
}

#[derive(Debug, Deserialize)]
pub struct RagSearchRequest {
    pub project_id: i64,
    pub query: String,
    pub provider_id: String,
    pub top_k: usize,
}

/// Search for relevant chunks
#[tauri::command]
pub async fn rag_search(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    request: RagSearchRequest,
) -> Result<CommandResult<Vec<ChunkMatch>>, String> {
    // Validate inputs
    if let Err(e) = validation::validate_query(&request.query) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_top_k(request.top_k) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("provider_id", &request.provider_id) {
        return Ok(CommandResult::err(e.to_string()));
    }

    // Get provider for query embedding
    let store = config_store.lock().await;
    let provider_config = match store.get_provider(&request.provider_id) {
        Ok(config) => config,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };
    drop(store);

    let provider = match create_provider(&provider_config) {
        Ok(p) => p,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    let embedding_service = EmbeddingService::new(provider);

    // Generate query embedding
    let query_embedding = match embedding_service.embed_text(request.query).await {
        Ok(emb) => emb,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Search
    let db = rag_db.lock().await;
    match search_similar(&db, request.project_id, query_embedding, request.top_k).await {
        Ok(results) => Ok(CommandResult::ok(results)),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct RagChatRequest {
    pub project_id: i64,
    pub query: String,
    pub provider_id: String,
    pub model: String,
    pub top_k: usize,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct RagChatResponse {
    pub response: String,
    pub sources: Vec<ChunkMatch>,
    pub model: String,
}

/// Chat with RAG context
#[tauri::command]
pub async fn rag_chat(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    config_store: tauri::State<'_, Arc<Mutex<ConfigStore>>>,
    request: RagChatRequest,
) -> Result<CommandResult<RagChatResponse>, String> {
    // Validate inputs
    if let Err(e) = validation::validate_query(&request.query) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_top_k(request.top_k) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("provider_id", &request.provider_id) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Err(e) = validation::validate_not_empty("model", &request.model) {
        return Ok(CommandResult::err(e.to_string()));
    }
    if let Some(temp) = request.temperature {
        if let Err(e) = validation::validate_temperature(temp) {
            return Ok(CommandResult::err(e.to_string()));
        }
    }
    if let Some(max_tokens) = request.max_tokens {
        if let Err(e) = validation::validate_max_tokens(max_tokens) {
            return Ok(CommandResult::err(e.to_string()));
        }
    }

    // First, perform RAG search
    let search_request = RagSearchRequest {
        project_id: request.project_id,
        query: request.query.clone(),
        provider_id: request.provider_id.clone(),
        top_k: request.top_k,
    };

    let search_result = rag_search(rag_db, config_store.clone(), search_request).await?;

    let sources = match search_result.data {
        Some(s) => s,
        None => {
            return Ok(CommandResult::err(
                search_result.error.unwrap_or_else(|| "Search failed".to_string()),
            ))
        }
    };

    // Build context from sources
    let context = sources
        .iter()
        .enumerate()
        .map(|(i, chunk_match)| {
            format!(
                "[Source {}: {}]\n{}",
                i + 1,
                chunk_match.document_name,
                chunk_match.chunk.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    // Build prompt with context
    let system_message = format!(
        "You are a helpful assistant. Use the following context to answer the user's question.\n\nContext:\n{}",
        context
    );

    // Get provider
    let store = config_store.lock().await;
    let provider_config = match store.get_provider(&request.provider_id) {
        Ok(config) => config,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };
    drop(store);

    let provider = match create_provider(&provider_config) {
        Ok(p) => p,
        Err(e) => return Ok(CommandResult::err(e.to_string())),
    };

    // Send chat request with context
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![
            ChatMessage {
                role: ChatRole::System,
                content: system_message,
            },
            ChatMessage {
                role: ChatRole::User,
                content: request.query,
            },
        ],
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        top_p: None,
        stream: false,
    };

    match provider.chat(chat_request).await {
        Ok(response) => Ok(CommandResult::ok(RagChatResponse {
            response: response.content,
            sources,
            model: response.model,
        })),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}
