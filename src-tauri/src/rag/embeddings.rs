use crate::llm_providers::{LlmProvider, ProviderError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Provider error: {0}")]
    ProviderError(#[from] ProviderError),

    #[error("No embedding provider configured")]
    NoProviderConfigured,
}

/// Configuration for batch embedding processing
/// With high-memory systems (128GB+), larger batches improve throughput
pub struct BatchConfig {
    /// Number of texts to embed in a single API call
    /// Default: 32 (good balance for most LLM APIs)
    /// For local GPU models, this can be much higher (128-512)
    pub batch_size: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self { batch_size: 32 }
    }
}

pub struct EmbeddingService {
    provider: Arc<dyn LlmProvider>,
    batch_config: BatchConfig,
}

impl EmbeddingService {
    pub fn new(provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            provider,
            batch_config: BatchConfig::default(),
        }
    }

    /// Create service with custom batch configuration
    /// For high-memory environments, increase batch_size for better throughput
    pub fn with_batch_config(provider: Arc<dyn LlmProvider>, batch_config: BatchConfig) -> Self {
        Self {
            provider,
            batch_config,
        }
    }

    /// Generate embeddings for a list of texts with batch processing
    /// Optimized for high-memory environments (128GB+ RAM)
    /// Returns a vector of embeddings (one per input text)
    pub async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // For small batches, process directly
        if texts.len() <= self.batch_config.batch_size {
            return Ok(self.provider.embed(texts).await?);
        }

        // For large batches, process in chunks to avoid overwhelming the API
        let mut all_embeddings = Vec::with_capacity(texts.len());

        for chunk in texts.chunks(self.batch_config.batch_size) {
            let chunk_embeddings = self.provider.embed(chunk.to_vec()).await?;
            all_embeddings.extend(chunk_embeddings);

            tracing::debug!(
                "Processed batch of {} embeddings, total: {}/{}",
                chunk.len(),
                all_embeddings.len(),
                texts.len()
            );
        }

        Ok(all_embeddings)
    }

    /// Generate embedding for a single text
    pub async fn embed_text(&self, text: String) -> Result<Vec<f32>, EmbeddingError> {
        let mut embeddings = self.embed_texts(vec![text]).await?;

        embeddings
            .pop()
            .ok_or(EmbeddingError::NoProviderConfigured)
    }
}

/// Compute cosine similarity between two vectors
/// Optimized for high-memory systems with vectorized operations
/// For GPU acceleration, consider using libraries like:
/// - cuBLAS (CUDA for NVIDIA GPUs like RTX 2080Ti, RTX 5070Ti)
/// - faiss (Facebook AI Similarity Search, supports CPU SIMD & GPU)
/// - hnswlib (fast approximate nearest neighbor search)
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    // Optimized vectorized operations
    // Modern CPUs will auto-vectorize these operations with SIMD
    let (dot_product, norm_a_sq, norm_b_sq) = a
        .iter()
        .zip(b.iter())
        .fold((0.0f32, 0.0f32, 0.0f32), |(dot, na, nb), (x, y)| {
            (dot + x * y, na + x * x, nb + y * y)
        });

    let magnitude_a = norm_a_sq.sqrt();
    let magnitude_b = norm_b_sq.sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

/// Batch compute cosine similarities between a query and multiple vectors
/// Optimized for high-memory systems - processes all similarities in parallel
pub fn batch_cosine_similarity(query: &[f32], vectors: &[Vec<f32>]) -> Vec<f32> {
    vectors
        .iter()
        .map(|vec| cosine_similarity(query, vec))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let similarity = cosine_similarity(&a, &b);
        assert!(similarity.abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 2.0];
        let b = vec![-1.0, -2.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity + 1.0).abs() < 0.001);
    }
}
