// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod llm_providers;
mod rag;
mod security;
mod validation;

use config::ConfigStore;
use rag::RagDatabase;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get app data directory
    let app_data_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
        .unwrap_or_else(|| {
            eprintln!("ERROR: Failed to get application config directory.");
            eprintln!("Please ensure the application has proper file system permissions.");
            std::process::exit(1);
        });

    // Initialize config store
    let config_store = Arc::new(Mutex::new(
        ConfigStore::new(app_data_dir.clone()).unwrap_or_else(|e| {
            eprintln!("ERROR: Failed to initialize configuration store: {}", e);
            eprintln!("Config directory: {:?}", app_data_dir);
            std::process::exit(1);
        }),
    ));

    // Initialize RAG database
    let db_path = app_data_dir.join("rag.db");
    let rag_db = Arc::new(Mutex::new(
        RagDatabase::new(db_path.clone())
            .await
            .unwrap_or_else(|e| {
                eprintln!("ERROR: Failed to initialize RAG database: {}", e);
                eprintln!("Database path: {:?}", db_path);
                std::process::exit(1);
            }),
    ));

    tracing::info!("Starting LLM Workbench...");

    tauri::Builder::default()
        .manage(config_store)
        .manage(rag_db)
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::get_providers,
            commands::update_provider,
            commands::delete_provider,
            commands::test_provider_connection,
            // Chat commands
            commands::send_chat_message,
            commands::send_chat_message_stream,
            // RAG commands
            commands::create_project,
            commands::list_projects,
            commands::delete_project,
            commands::list_documents,
            commands::delete_document,
            commands::add_document,
            commands::rag_search,
            commands::rag_chat,
            // Canvas commands
            commands::get_canvas_state,
            commands::save_canvas_state,
            // Conversation commands
            commands::create_conversation,
            commands::list_conversations,
            commands::get_conversation_with_messages,
            commands::update_conversation_title,
            commands::delete_conversation,
            commands::add_message,
            commands::get_conversation_messages,
            commands::delete_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
