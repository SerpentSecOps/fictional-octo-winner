# RAG System Optimization Guide for High-Memory Environments

This document describes optimizations for the RAG (Retrieval-Augmented Generation) system on high-memory systems (128GB+ RAM) with GPU resources.

## Current Optimizations (Implemented)

### 1. Parallel Processing
- **Rayon-based parallel similarity computation**: Automatically uses all CPU cores for cosine similarity
- **Parallel sorting**: Multi-threaded sorting of search results
- **Batch embedding processing**: Process multiple texts in batches (configurable batch size)

### 2. Memory-Efficient Operations
- **In-memory search**: With 128GB RAM, can handle millions of chunks (est. 10-50 million)
- **Vectorized cosine similarity**: Single-pass computation of dot product and norms
- **Pre-allocated vectors**: Reduces memory allocations during search

### 3. Batch Configuration
```rust
// Default batch size: 32 (good for cloud LLM APIs)
let service = EmbeddingService::new(provider);

// For local GPU models, use larger batches
let batch_config = BatchConfig { batch_size: 128 };
let service = EmbeddingService::with_batch_config(provider, batch_config);
```

## Performance Characteristics

### Current In-Memory Search Performance (Estimated)
| Chunk Count | Search Time (approx) | Memory Usage |
|-------------|---------------------|--------------|
| 10,000      | 10-50ms            | ~100MB       |
| 100,000     | 100-500ms          | ~1GB         |
| 1,000,000   | 1-5 seconds        | ~10GB        |
| 10,000,000  | 10-50 seconds      | ~100GB       |

*Based on 1536-dimensional embeddings (OpenAI/Gemini/DeepSeek size)*

### Memory Usage per Chunk
- Embedding vector (1536 dimensions): ~6KB
- Chunk text (avg 2KB): ~2KB
- Metadata: ~1KB
- **Total per chunk**: ~9KB
- **For 10M chunks**: ~90GB

With 128GB RAM, you can comfortably handle **10+ million chunks** in memory.

## Advanced Optimizations (TODO)

### 1. GPU-Accelerated Search (CUDA)
For systems with NVIDIA GPUs (RTX 2080Ti, RTX 5070Ti), consider:

#### Option A: FAISS (Facebook AI Similarity Search)
```toml
# Add to Cargo.toml
faiss = { version = "0.11", features = ["gpu"] }
```

**Benefits**:
- GPU-accelerated similarity search
- 10-100x faster for large datasets (>100k chunks)
- Supports approximate nearest neighbor (ANN) algorithms
- HNSW, IVF, PQ indexing methods

**Implementation**:
```rust
use faiss::{Index, IndexImpl, IndexFlatIP};

// Create GPU index
let mut index = IndexFlatIP::new(1536)?; // 1536 = embedding dimension
index.add(&embeddings)?; // Add all embeddings

// Search
let (distances, labels) = index.search(&query_embedding, top_k)?;
```

#### Option B: CuBLAS (NVIDIA CUDA BLAS)
```toml
cublas = "0.5"
```

**Benefits**:
- Native NVIDIA GPU acceleration
- Optimal for matrix operations (batch similarity)
- Can process 100k+ similarities in <10ms on modern GPUs

### 2. Approximate Nearest Neighbor (ANN) Indexing

For extremely large datasets (>1M chunks), exact search becomes slow. Use ANN:

#### HNSW (Hierarchical Navigable Small World)
```toml
hnswlib = "0.1"
```

**Benefits**:
- Sub-millisecond search even with millions of vectors
- High recall (>95% accuracy)
- Memory-efficient graph structure

**Trade-offs**:
- Index build time: ~1-5 minutes for 1M vectors
- Slightly less accurate than exact search (but configurable)

### 3. Hybrid Search (Semantic + Keyword)

Combine vector similarity with traditional keyword search:

```rust
// 1. Vector similarity (semantic)
let semantic_results = search_similar(db, query_embedding, 100).await?;

// 2. BM25 keyword search (requires full-text index)
let keyword_results = bm25_search(db, query_text, 100).await?;

// 3. Combine with reciprocal rank fusion
let final_results = rerank_fusion(semantic_results, keyword_results, top_k);
```

**Benefits**:
- Better recall for specific terms (names, codes, IDs)
- Semantic search handles synonyms and concepts
- Best of both worlds

### 4. Local Embedding Models (GPU-Accelerated)

Instead of cloud APIs, run embedding models locally on your GPUs:

#### Option A: ONNX Runtime with CUDA
```toml
ort = { version = "1.16", features = ["cuda"] }
```

**Models to consider**:
- `all-MiniLM-L6-v2` (22M params, 384 dims, fast)
- `all-mpnet-base-v2` (110M params, 768 dims, balanced)
- `gte-large` (335M params, 1024 dims, high quality)

**Benefits**:
- No API costs
- Much faster (local GPU inference)
- Batch sizes of 512-2048 on RTX GPUs
- Process 10k chunks in <1 second

#### Option B: Candle (Rust ML framework)
```toml
candle-core = "0.3"
candle-transformers = "0.3"
```

**Benefits**:
- Pure Rust, no Python dependencies
- CUDA support via cuDNN
- Memory-efficient tensor operations

### 5. Multi-GPU Support

With multiple GPUs (RTX 2080Ti + RTX 5070Ti), distribute work:

```rust
// Shard embeddings across GPUs
let embeddings_gpu0 = embeddings[0..n/2].to_device(Device::Cuda(0))?;
let embeddings_gpu1 = embeddings[n/2..].to_device(Device::Cuda(1))?;

// Search on both GPUs in parallel
let (results0, results1) = tokio::join!(
    search_on_gpu(query, embeddings_gpu0, 0),
    search_on_gpu(query, embeddings_gpu1, 1),
);

// Merge results
let final_results = merge_top_k(results0, results1, top_k);
```

### 6. Persistent Vector Index Cache

For very large datasets, persist the search index:

```rust
// Build index once
let index = build_hnsw_index(embeddings)?;
index.save("rag_index.bin")?;

// Load on startup
let index = HnswIndex::load("rag_index.bin")?;
```

**Benefits**:
- Skip index rebuild on restart
- Share index across multiple instances
- Faster startup time

## Recommended Configuration by Dataset Size

### Small (< 10k chunks)
- Current implementation is optimal
- In-memory search is fast enough
- No additional optimizations needed

### Medium (10k - 100k chunks)
- Current parallel search works well
- Consider HNSW for <10ms queries
- Optional: Local embedding models for cost savings

### Large (100k - 1M chunks)
- **Recommended**: HNSW indexing
- **Optional**: GPU acceleration with FAISS
- **Consider**: Hybrid search for better recall

### Very Large (> 1M chunks)
- **Required**: HNSW or FAISS GPU
- **Recommended**: Local embedding models
- **Consider**: Multi-GPU sharding
- **Optional**: Distributed search (multiple machines)

## Implementation Roadmap

### Phase 1: Current (Completed ✅)
- ✅ Parallel CPU search with rayon
- ✅ Batch embedding processing
- ✅ Optimized cosine similarity
- ✅ Input validation

### Phase 2: GPU Acceleration (Recommended Next)
1. Add local embedding model support (ONNX Runtime + CUDA)
2. GPU-accelerated similarity search (FAISS or cuBLAS)
3. Benchmark improvements

### Phase 3: Advanced Indexing (For >100k chunks)
1. HNSW indexing with hnswlib
2. Persistent index storage
3. Index versioning and updates

### Phase 4: Hybrid & Re-ranking (Quality improvements)
1. BM25 keyword search integration
2. Cross-encoder re-ranking
3. Reciprocal rank fusion

## Hardware Utilization Notes

### Your System (128GB RAM, RTX 2080Ti + RTX 5070Ti)

**Current RAG system can handle**:
- 10+ million chunks comfortably in RAM
- Parallel search across all CPU cores
- Sub-second search for <100k chunks

**With GPU acceleration, you could achieve**:
- <10ms search for millions of chunks (FAISS GPU)
- 10k embeddings generated per second (local models)
- Near-instant retrieval even with massive document sets

**GPU Memory Considerations**:
- RTX 2080Ti: 11GB VRAM → ~1.8M vectors in GPU memory (1536 dims)
- RTX 5070Ti: 16GB VRAM → ~2.6M vectors in GPU memory
- Combined: Can hold 4M+ vectors entirely in GPU memory

## Example: Enabling GPU Acceleration

```rust
// Future enhancement (not yet implemented)
use faiss::gpu::{StandardGpuResources, GpuIndexFlatIP};

pub struct GpuRagDatabase {
    cpu_db: RagDatabase,
    gpu_index: Option<GpuIndexFlatIP>,
    gpu_resources: StandardGpuResources,
}

impl GpuRagDatabase {
    pub async fn search_gpu(
        &self,
        query_embedding: Vec<f32>,
        top_k: usize,
    ) -> Result<Vec<ChunkMatch>> {
        // Search on GPU
        let (distances, labels) = self.gpu_index
            .as_ref()
            .unwrap()
            .search(&query_embedding, top_k)?;

        // Fetch chunk details from SQLite
        let chunks = self.cpu_db.get_chunks_by_ids(labels).await?;

        Ok(chunks)
    }
}
```

## Questions?

For assistance with GPU acceleration or advanced RAG features, consult:
- [FAISS documentation](https://github.com/facebookresearch/faiss)
- [hnswlib documentation](https://github.com/nmslib/hnswlib)
- [ONNX Runtime Rust](https://github.com/pykeio/ort)
- [Candle ML framework](https://github.com/huggingface/candle)
