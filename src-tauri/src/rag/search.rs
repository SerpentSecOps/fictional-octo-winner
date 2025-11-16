use super::database::{Chunk, ChunkMatch, RagDatabase};
use super::embeddings::cosine_similarity;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] super::database::DatabaseError),
}

/// Search for chunks similar to the query embedding
/// Returns top-k most similar chunks with their similarity scores
pub async fn search_similar(
    db: &RagDatabase,
    project_id: i64,
    query_embedding: Vec<f32>,
    top_k: usize,
) -> Result<Vec<ChunkMatch>, SearchError> {
    // Get all chunks for the project
    let chunks = db.get_chunks_for_project(project_id).await?;

    if chunks.is_empty() {
        return Ok(Vec::new());
    }

    // Compute similarity for each chunk
    let mut scored_chunks: Vec<(f32, Chunk)> = chunks
        .into_iter()
        .map(|chunk| {
            let similarity = cosine_similarity(&query_embedding, &chunk.embedding);
            (similarity, chunk)
        })
        .collect();

    // Sort by similarity (descending)
    scored_chunks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Take top-k
    let top_chunks = scored_chunks.into_iter().take(top_k);

    // Build ChunkMatch results (need to fetch document names)
    let mut results = Vec::new();
    for (similarity, chunk) in top_chunks {
        // TODO: This is inefficient (N queries). Should be optimized with a JOIN
        let (_chunk, doc_name) = db.get_chunk_with_document(chunk.id).await?;

        results.push(ChunkMatch {
            chunk,
            similarity,
            document_name: doc_name,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_empty() {
        // TODO: Add proper test with in-memory SQLite
    }
}
