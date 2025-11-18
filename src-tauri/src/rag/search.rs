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
/// 2. Diversity-aware re-ranking to avoid redundant results
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
    let mut candidates = search_similar(db, project_id, query_embedding, candidate_count).await?;

    if candidates.len() <= top_k {
        return Ok(candidates);
    }

    // Second stage: Diversity-aware re-ranking
    // Select results that are both relevant and diverse to avoid redundancy
    let mut selected = Vec::new();
    selected.push(candidates.remove(0)); // Always take the top result

    // For each remaining slot, select the candidate that maximizes:
    // relevance_score - (diversity_penalty * max_similarity_to_selected)
    let diversity_penalty = 0.3; // Tune this value (0.0 = no diversity, 1.0 = max diversity)

    while selected.len() < top_k && !candidates.is_empty() {
        let mut best_idx = 0;
        let mut best_score = f32::NEG_INFINITY;

        for (idx, candidate) in candidates.iter().enumerate() {
            // Calculate maximum similarity to already selected results
            let max_sim_to_selected = selected
                .iter()
                .map(|s| cosine_similarity(&candidate.chunk.embedding, &s.chunk.embedding))
                .fold(0.0f32, f32::max);

            // Penalize similar results
            let diversity_score =
                candidate.similarity - (diversity_penalty * max_sim_to_selected);

            if diversity_score > best_score {
                best_score = diversity_score;
                best_idx = idx;
            }
        }

        selected.push(candidates.remove(best_idx));
    }

    tracing::debug!(
        "Re-ranked {} candidates to {} diverse results",
        candidate_count,
        selected.len()
    );

    Ok(selected)
}

// TODO: Future enhancements for re-ranking:
// - Cross-encoder models (Hugging Face transformers for accurate relevance scoring)
// - Hybrid search (combine semantic embeddings with BM25 keyword matching)
// - MMR (Maximal Marginal Relevance) algorithm with configurable lambda
// - Query expansion for better recall

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical_vectors() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        let similarity = cosine_similarity(&v1, &v2);
        assert!((similarity - 1.0).abs() < 1e-6, "Identical vectors should have similarity of 1.0");
    }

    #[test]
    fn test_cosine_similarity_orthogonal_vectors() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        let similarity = cosine_similarity(&v1, &v2);
        assert!(similarity.abs() < 1e-6, "Orthogonal vectors should have similarity of 0.0");
    }

    #[test]
    fn test_cosine_similarity_opposite_vectors() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![-1.0, 0.0, 0.0];
        let similarity = cosine_similarity(&v1, &v2);
        assert!((similarity + 1.0).abs() < 1e-6, "Opposite vectors should have similarity of -1.0");
    }

    #[test]
    fn test_cosine_similarity_normalized() {
        let v1 = vec![2.0, 0.0, 0.0];
        let v2 = vec![3.0, 0.0, 0.0];
        let similarity = cosine_similarity(&v1, &v2);
        assert!((similarity - 1.0).abs() < 1e-6, "Parallel vectors should have similarity of 1.0");
    }

    #[test]
    fn test_cosine_similarity_general_case() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        let similarity = cosine_similarity(&v1, &v2);
        // Expected: (1*4 + 2*5 + 3*6) / (sqrt(14) * sqrt(77))
        // = 32 / sqrt(1078) ≈ 0.9746
        assert!(similarity > 0.97 && similarity < 0.98, "Expected similarity around 0.9746");
    }
}
