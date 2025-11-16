use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow, Row};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Project not found: {0}")]
    ProjectNotFound(i64),

    #[error("Document not found: {0}")]
    DocumentNotFound(i64),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Conversation not found: {0}")]
    ConversationNotFound(i64),

    #[error("Message not found: {0}")]
    MessageNotFound(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub canvas_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub source_path: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: i64,
    pub document_id: i64,
    pub project_id: i64,
    pub content: String,
    pub embedding: Vec<f32>,
    pub chunk_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMatch {
    pub chunk: Chunk,
    pub similarity: f32,
    pub document_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub provider_id: String,
    pub model: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub conversation_id: i64,
    pub role: String,  // "system", "user", "assistant"
    pub content: String,
    pub created_at: String,
}

pub struct RagDatabase {
    pool: SqlitePool,
}

impl RagDatabase {
    pub async fn new(db_path: PathBuf) -> Result<Self, DatabaseError> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let db_url = format!("sqlite:{}", db_path.display());
        let pool = SqlitePool::connect(&db_url).await?;

        let db = Self { pool };
        db.init_schema().await?;

        Ok(db)
    }

    async fn init_schema(&self) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                canvas_state TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                source_path TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id INTEGER NOT NULL,
                project_id INTEGER NOT NULL,
                content TEXT NOT NULL,
                embedding BLOB NOT NULL,
                chunk_index INTEGER NOT NULL,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_chunks_project ON chunks(project_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_chunks_document ON chunks(document_id)")
            .execute(&self.pool)
            .await?;

        // Conversation tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                provider_id TEXT NOT NULL,
                model TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Project operations
    pub async fn create_project(&self, name: String) -> Result<Project, DatabaseError> {
        let id = sqlx::query("INSERT INTO projects (name) VALUES (?)")
            .bind(&name)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        self.get_project(id).await
    }

    pub async fn get_project(&self, id: i64) -> Result<Project, DatabaseError> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| DatabaseError::ProjectNotFound(id))
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, DatabaseError> {
        Ok(
            sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY updated_at DESC")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn delete_project(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_canvas_state(
        &self,
        project_id: i64,
        canvas_state: String,
    ) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE projects SET canvas_state = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(canvas_state)
            .bind(project_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Document operations
    pub async fn create_document(
        &self,
        project_id: i64,
        name: String,
        source_path: Option<String>,
    ) -> Result<Document, DatabaseError> {
        let id = sqlx::query("INSERT INTO documents (project_id, name, source_path) VALUES (?, ?, ?)")
            .bind(project_id)
            .bind(&name)
            .bind(&source_path)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        self.get_document(id).await
    }

    pub async fn get_document(&self, id: i64) -> Result<Document, DatabaseError> {
        sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| DatabaseError::DocumentNotFound(id))
    }

    pub async fn list_documents(&self, project_id: i64) -> Result<Vec<Document>, DatabaseError> {
        Ok(
            sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE project_id = ? ORDER BY created_at DESC")
                .bind(project_id)
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn delete_document(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Chunk operations
    pub async fn insert_chunk(
        &self,
        document_id: i64,
        project_id: i64,
        content: String,
        embedding: Vec<f32>,
        chunk_index: i32,
    ) -> Result<i64, DatabaseError> {
        let embedding_bytes = bincode::serialize(&embedding)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        let id = sqlx::query(
            "INSERT INTO chunks (document_id, project_id, content, embedding, chunk_index) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(document_id)
        .bind(project_id)
        .bind(content)
        .bind(embedding_bytes)
        .bind(chunk_index)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_chunks_for_project(&self, project_id: i64) -> Result<Vec<Chunk>, DatabaseError> {
        let rows = sqlx::query("SELECT id, document_id, project_id, content, embedding, chunk_index FROM chunks WHERE project_id = ?")
            .bind(project_id)
            .fetch_all(&self.pool)
            .await?;

        let mut chunks = Vec::new();
        for row in rows {
            let embedding_bytes: Vec<u8> = row.get("embedding");
            let embedding: Vec<f32> = bincode::deserialize(&embedding_bytes)
                .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

            chunks.push(Chunk {
                id: row.get("id"),
                document_id: row.get("document_id"),
                project_id: row.get("project_id"),
                content: row.get("content"),
                embedding,
                chunk_index: row.get("chunk_index"),
            });
        }

        Ok(chunks)
    }

    pub async fn get_chunk_with_document(
        &self,
        chunk_id: i64,
    ) -> Result<(Chunk, String), DatabaseError> {
        let row = sqlx::query(
            r#"
            SELECT c.id, c.document_id, c.project_id, c.content, c.embedding, c.chunk_index, d.name as doc_name
            FROM chunks c
            JOIN documents d ON c.document_id = d.id
            WHERE c.id = ?
            "#
        )
        .bind(chunk_id)
        .fetch_one(&self.pool)
        .await?;

        let embedding_bytes: Vec<u8> = row.get("embedding");
        let embedding: Vec<f32> = bincode::deserialize(&embedding_bytes)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        let chunk = Chunk {
            id: row.get("id"),
            document_id: row.get("document_id"),
            project_id: row.get("project_id"),
            content: row.get("content"),
            embedding,
            chunk_index: row.get("chunk_index"),
        };

        let doc_name: String = row.get("doc_name");

        Ok((chunk, doc_name))
    }

    // Conversation operations
    pub async fn create_conversation(
        &self,
        title: String,
        provider_id: String,
        model: String,
    ) -> Result<Conversation, DatabaseError> {
        let id = sqlx::query(
            "INSERT INTO conversations (title, provider_id, model) VALUES (?, ?, ?)"
        )
        .bind(&title)
        .bind(&provider_id)
        .bind(&model)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.get_conversation(id).await
    }

    pub async fn get_conversation(&self, id: i64) -> Result<Conversation, DatabaseError> {
        sqlx::query_as::<_, Conversation>("SELECT * FROM conversations WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| DatabaseError::ConversationNotFound(id))
    }

    pub async fn list_conversations(&self) -> Result<Vec<Conversation>, DatabaseError> {
        Ok(
            sqlx::query_as::<_, Conversation>(
                "SELECT * FROM conversations ORDER BY updated_at DESC"
            )
            .fetch_all(&self.pool)
            .await?,
        )
    }

    pub async fn update_conversation_title(
        &self,
        id: i64,
        title: String,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            "UPDATE conversations SET title = ?, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(title)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_conversation(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM conversations WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn touch_conversation(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE conversations SET updated_at = datetime('now') WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Message operations
    pub async fn add_message(
        &self,
        conversation_id: i64,
        role: String,
        content: String,
    ) -> Result<Message, DatabaseError> {
        let id = sqlx::query(
            "INSERT INTO messages (conversation_id, role, content) VALUES (?, ?, ?)"
        )
        .bind(conversation_id)
        .bind(&role)
        .bind(&content)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        // Touch the conversation to update its timestamp
        self.touch_conversation(conversation_id).await?;

        self.get_message(id).await
    }

    pub async fn get_message(&self, id: i64) -> Result<Message, DatabaseError> {
        sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| DatabaseError::MessageNotFound(id))
    }

    pub async fn get_conversation_messages(
        &self,
        conversation_id: i64,
    ) -> Result<Vec<Message>, DatabaseError> {
        Ok(
            sqlx::query_as::<_, Message>(
                "SELECT * FROM messages WHERE conversation_id = ? ORDER BY created_at ASC"
            )
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await?,
        )
    }

    pub async fn delete_message(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
