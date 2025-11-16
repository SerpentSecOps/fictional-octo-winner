pub mod database;
pub mod embeddings;
pub mod chunking;
pub mod search;

pub use database::{RagDatabase, Project, Document, Chunk, Conversation, Message, ChunkMatch};
pub use embeddings::EmbeddingService;
pub use chunking::chunk_text;
pub use search::search_similar;
