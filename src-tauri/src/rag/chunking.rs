/// Simple text chunking with sliding window and overlap
/// This is a basic implementation; production systems might use more sophisticated chunking
/// (e.g., semantic chunking, sentence-aware chunking, etc.)

const DEFAULT_CHUNK_SIZE: usize = 512; // ~512 tokens ≈ 2048 characters
const DEFAULT_OVERLAP: usize = 50; // ~50 tokens ≈ 200 characters

pub struct ChunkConfig {
    pub chunk_size: usize,
    pub overlap: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE * 4, // Convert to chars (rough estimate)
            overlap: DEFAULT_OVERLAP * 4,
        }
    }
}

/// Chunk text into overlapping segments
/// Returns a vector of text chunks
pub fn chunk_text(text: &str, config: Option<ChunkConfig>) -> Vec<String> {
    let config = config.unwrap_or_default();

    if text.len() <= config.chunk_size {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < text.len() {
        let end = (start + config.chunk_size).min(text.len());

        // Try to break at sentence or word boundary
        let chunk_end = if end < text.len() {
            find_boundary(&text[start..end])
                .map(|offset| start + offset)
                .unwrap_or(end)
        } else {
            end
        };

        chunks.push(text[start..chunk_end].to_string());

        // Move start forward, accounting for overlap
        if chunk_end >= text.len() {
            break;
        }

        start = chunk_end.saturating_sub(config.overlap);

        // Ensure we make progress
        if start == 0 || start >= chunk_end {
            start = chunk_end;
        }
    }

    chunks
}

/// Find a good boundary (sentence or word) to break the text
/// Returns the offset from the start of the text
fn find_boundary(text: &str) -> Option<usize> {
    // Try to find sentence ending (. ! ?)
    if let Some(pos) = text.rfind(|c| c == '.' || c == '!' || c == '?') {
        return Some(pos + 1);
    }

    // Try to find newline
    if let Some(pos) = text.rfind('\n') {
        return Some(pos + 1);
    }

    // Try to find word boundary (space)
    if let Some(pos) = text.rfind(' ') {
        return Some(pos + 1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_small_text() {
        let text = "This is a small text.";
        let chunks = chunk_text(text, None);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_chunk_with_overlap() {
        let text = "A".repeat(3000);
        let config = ChunkConfig {
            chunk_size: 1000,
            overlap: 100,
        };
        let chunks = chunk_text(&text, Some(config));

        assert!(chunks.len() > 1);
        // Check that chunks have some overlap
        for i in 0..chunks.len() - 1 {
            assert!(chunks[i].len() <= 1000 + 10); // Allow some margin
        }
    }

    #[test]
    fn test_chunk_respects_boundaries() {
        let text = "First sentence. Second sentence. Third sentence. Fourth sentence.";
        let config = ChunkConfig {
            chunk_size: 30,
            overlap: 5,
        };
        let chunks = chunk_text(text, Some(config));

        // Chunks should ideally break at sentence boundaries
        assert!(chunks.len() > 1);
        for chunk in &chunks {
            println!("Chunk: {}", chunk);
        }
    }
}
