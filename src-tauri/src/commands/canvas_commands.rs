use crate::rag::RagDatabase;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::config_commands::CommandResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct CanvasState {
    pub nodes: Vec<CanvasNode>,
    pub edges: Vec<CanvasEdge>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CanvasNode {
    pub id: String,
    pub node_type: String,
    pub position: Position,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CanvasEdge {
    pub id: String,
    pub source: String,
    pub target: String,
}

/// Get canvas state for a project
#[tauri::command]
pub async fn get_canvas_state(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    project_id: i64,
) -> Result<CommandResult<Option<CanvasState>>, String> {
    let db = rag_db.lock().await;

    match db.get_project(project_id).await {
        Ok(project) => {
            if let Some(state_json) = project.canvas_state {
                match serde_json::from_str::<CanvasState>(&state_json) {
                    Ok(state) => Ok(CommandResult::ok(Some(state))),
                    Err(e) => Ok(CommandResult::err(format!(
                        "Failed to parse canvas state: {}",
                        e
                    ))),
                }
            } else {
                Ok(CommandResult::ok(None))
            }
        }
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}

/// Save canvas state for a project
#[tauri::command]
pub async fn save_canvas_state(
    rag_db: tauri::State<'_, Arc<Mutex<RagDatabase>>>,
    project_id: i64,
    state: CanvasState,
) -> Result<CommandResult<()>, String> {
    let state_json = match serde_json::to_string(&state) {
        Ok(json) => json,
        Err(e) => return Ok(CommandResult::err(format!("Serialization error: {}", e))),
    };

    let db = rag_db.lock().await;

    match db.update_canvas_state(project_id, state_json).await {
        Ok(_) => Ok(CommandResult::ok(())),
        Err(e) => Ok(CommandResult::err(e.to_string())),
    }
}
