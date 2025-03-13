use crate::error::{tokenization_error, ChunkingError};
use tokenizers::Tokenizer;

pub struct Chunk<'a> {
    tokenizer: &'a Tokenizer,
    data: String,
    max_size: usize,
    current_token_size: usize,
}

impl<'a> Chunk<'a> {
    pub fn new(tokenizer: &'a Tokenizer, max_size: usize) -> Self {
        let special_token_count = Self::count_special_tokens(tokenizer);
        let adjusted_max_size = max_size.saturating_sub(special_token_count);
        Self {
            tokenizer,
            data: String::new(),
            max_size: adjusted_max_size,
            current_token_size: 0,
        }
    }
    pub fn push_str(&mut self, text: &str) -> Result<bool, ChunkingError> {
        let new_token_size = self
            .tokenizer
            .encode(text, true)
            .map_err(tokenization_error)?
            .get_ids()
            .len();
        if new_token_size + self.current_token_size > self.max_size {
            Ok(false)
        } else {
            self.data.push_str(text);
            self.current_token_size += new_token_size;
            Ok(true)
        }
    }
    pub fn finalize(&mut self) -> String {
        self.current_token_size = 0;
        std::mem::take(&mut self.data)
    }

    /// Determines the number of special tokens by encoding an empty string
    fn count_special_tokens(tokenizer: &Tokenizer) -> usize {
        match tokenizer.encode("", true) {
            Ok(encoding) => encoding.get_ids().len(),
            Err(_) => 0, // Fallback in case of error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizers::Tokenizer;

    fn get_test_tokenizer() -> Tokenizer {
        // Load a simple pre-trained tokenizer for testing
        Tokenizer::from_pretrained("facebook/bart-base", None).expect("Failed to load tokenizer")
    }

    #[test]
    fn test_special_token_count() {
        let tokenizer = get_test_tokenizer();
        let special_tokens = Chunk::count_special_tokens(&tokenizer);

        // BART typically adds <s> and </s>, so expect at least 2
        assert!(special_tokens >= 2, "Expected at least 2 special tokens");
    }

    #[test]
    fn test_chunk_creation() {
        let tokenizer = get_test_tokenizer();
        let chunk = Chunk::new(&tokenizer, 1024);

        assert_eq!(chunk.data, "", "Chunk should start empty");
        assert!(
            chunk.current_token_size == 0,
            "Chunk should have 0 tokens initially"
        );
    }

    #[test]
    fn test_push_str_within_limit() {
        let tokenizer = get_test_tokenizer();
        let mut chunk = Chunk::new(&tokenizer, 1024);

        let text = "This is a test sentence.";
        let added = chunk.push_str(text).expect("push_str failed");

        assert!(added, "Text should fit within the chunk");
        assert!(!chunk.data.is_empty(), "Data should be added");
    }

    #[test]
    fn test_push_str_exceeding_limit() {
        let tokenizer = get_test_tokenizer();
        let mut chunk = Chunk::new(&tokenizer, 10); // Small limit for testing

        let long_text = "This is a very long sentence that should exceed the token limit.";
        let added = chunk.push_str(long_text).expect("push_str failed");

        assert!(!added, "Text should be rejected if it exceeds the limit");
    }

    #[test]
    fn test_finalize() {
        let tokenizer = get_test_tokenizer();
        let mut chunk = Chunk::new(&tokenizer, 1024);

        let text = "Hello, world!";
        chunk.push_str(text).expect("push_str failed");

        let final_data = chunk.finalize();
        assert_eq!(final_data, text, "finalize() should return the stored data");
        assert!(
            chunk.data.is_empty(),
            "Chunk should be empty after finalize()"
        );
    }
}
