use super::database::{Chunk, ChunkMatch, RagDatabase};
use super::embeddings::cosine_similarity;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] super::database::DatabaseError),
}

/// Search for chunks similar to the query embedding
/// Returns top-k most similar chunks with their similarity scores
///
/// OPTIMIZED FOR HIGH-MEMORY SYSTEMS (128GB+ RAM):
/// - Uses parallel processing via rayon for similarity computation
/// - In-memory cosine similarity is very fast with modern CPUs
/// - For datasets > 100k chunks, consider:
///   1. HNSW indexing (via hnswlib or faiss)
///   2. GPU acceleration (via faiss with CUDA)
///   3. Approximate nearest neighbor search for sub-millisecond queries
///
/// Current performance (estimated on modern CPU):
/// - 10k chunks: ~10-50ms
/// - 100k chunks: ~100-500ms
/// - 1M chunks: ~1-5 seconds
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

    let chunk_count = chunks.len();
    tracing::debug!(
        "Searching {} chunks in project {} with parallel processing",
        chunk_count,
        project_id
    );

    // Compute similarity for each chunk IN PARALLEL
    // With 128GB RAM, we can easily handle millions of chunks in memory
    // Rayon automatically uses all available CPU cores
    let mut scored_chunks: Vec<(f32, Chunk)> = chunks
        .into_par_iter() // Parallel iterator for multi-core processing
        .map(|chunk| {
            let similarity = cosine_similarity(&query_embedding, &chunk.embedding);
            (similarity, chunk)
        })
        .collect();

    // Sort by similarity (descending)
    // For very large datasets (>1M chunks), consider using partial_sort or select_nth
    scored_chunks.par_sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Take top-k
    let top_chunks: Vec<_> = scored_chunks.into_iter().take(top_k).collect();

    // Build ChunkMatch results (fetch all document names in one optimized query)
    let chunk_ids: Vec<i64> = top_chunks.iter().map(|(_, chunk)| chunk.id).collect();
    let chunks_with_docs = db.get_chunks_with_documents(&chunk_ids).await?;

    // Create a map of chunk_id -> document_name for quick lookup
    let mut doc_name_map: std::collections::HashMap<i64, String> =
        chunks_with_docs
            .into_iter()
            .map(|(chunk, doc_name)| (chunk.id, doc_name))
            .collect();

    // Build results maintaining the original order and similarity scores
    let results: Vec<ChunkMatch> = top_chunks
        .into_iter()
        .filter_map(|(similarity, chunk)| {
            doc_name_map.remove(&chunk.id).map(|doc_name| ChunkMatch {
                chunk,
                similarity,
                document_name: doc_name,
            })
        })
        .collect();

    tracing::debug!("Search completed, returning {} results", results.len());

    Ok(results)
}

/// Advanced search with filtering and re-ranking
/// For high-memory systems, this performs multi-stage retrieval:
/// 1. Fast cosine similarity to get top-N candidates (N > k)
/// 2. Optional re-ranking with more expensive models
/// 3. Return top-k final results
pub async fn search_with_rerank(
    db: &RagDatabase,
    project_id: i64,
    query_embedding: Vec<f32>,
    top_k: usize,
    candidate_multiplier: usize, // Get this many candidates before re-ranking
) -> Result<Vec<ChunkMatch>, SearchError> {
    // First stage: Get more candidates than needed
    let candidate_count = top_k * candidate_multiplier;
    let mut results = search_similar(db, project_id, query_embedding, candidate_count).await?;

    // Second stage: Re-rank (placeholder for future enhancement)
    // TODO: Implement re-ranking with:
    // - Cross-encoder models (more accurate but slower)
    // - Hybrid search (combine semantic + keyword matching)
    // - Diversity-aware ranking

    // For now, just return top-k from initial results
    results.truncate(top_k);
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
