use tokenizers::Tokenizer;

use crate::chunk::Chunk;
use crate::error::ChunkingError;
use crate::paragraph_split::ParagraphSplitStrategy;

pub struct MarkdownChunker<'a> {
    sticky_headers: bool,
    paragraph_split_strategy: ParagraphSplitStrategy,
    current_chunk: Chunk<'a>,
    chunks: Vec<String>,
    tokenizer: &'a Tokenizer,
    #[cfg(feature = "nlp")]
    nlp_tokenizer: &'a NlpTokenizer,
}

impl<'a> MarkdownChunker<'a> {
    pub fn new(
        max_size: usize,
        sticky_headers: bool,
        paragraph_split_strategy: ParagraphSplitStrategy,
        tokenizer: &'a Tokenizer,
        #[cfg(feature = "nlp")] nlp_tokenizer: &'a NlpTokenizer,
    ) -> Self {
        Self {
            paragraph_split_strategy,
            chunks: Vec::<String>::new(),
            current_chunk: Chunk::<'a>::new(tokenizer, max_size),
            sticky_headers,
            tokenizer,
            #[cfg(feature = "nlp")]
            nlp_tokenizer,
        }
    }
    pub fn chunk(&mut self, text: &str) -> Result<Vec<String>, ChunkingError> {
        // process text into self.current_chunk and into self.chunk at the correct length
        Ok(self.finalize())
    }
    fn finalize(&mut self) -> Vec<String> {
        std::mem::take(&mut self.chunks)
    }
}
