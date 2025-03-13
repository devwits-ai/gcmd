mod chunk;
mod error;
mod markdown_chunker;
mod paragraph_split;
#[cfg(feature = "nlp")]
use nlprule::Tokenizer as NlpTokenizer;
use tokenizers::Tokenizer;

use crate::error::ChunkingError;
use crate::markdown_chunker::MarkdownChunker;
use crate::paragraph_split::ParagraphSplitStrategy;

pub struct Chunker<'a> {
    tokenizer: &'a Tokenizer,
    #[cfg(feature = "nlp")]
    nlp_tokenizer: Option<&NlpTokenizer>,
    paragraph_split_strategy: ParagraphSplitStrategy,
    sticky_headers: bool,
    max_tokens: usize,
}

impl<'a> Chunker<'a> {
    pub fn new(max_tokens: usize, tokenizer: &'a Tokenizer) -> Self {
        Self {
            tokenizer,
            #[cfg(feature = "nlp")]
            nlp_tokenizer: None,
            #[cfg(feature = "nlp")]
            paragraph_split_strategy: ParagraphSplitStrategy::Nlp,
            #[cfg(not(feature = "nlp"))]
            paragraph_split_strategy: ParagraphSplitStrategy::SimpleSplit,
            max_tokens,
            sticky_headers: false,
        }
    }

    #[cfg(feature = "nlp")]
    pub fn with_nlp_language(&mut self, lang: &str) -> Self {
        let mut tokenizer_bytes: [u8] = match lang {
            #[cfg(feature = "lang-en")]
            "en" => include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("en"))),

            #[cfg(feature = "lang-fr")]
            "fr" => include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("fr"))),

            #[cfg(feature = "lang-de")]
            "de" => include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("de"))),

            #[cfg(feature = "lang-es")]
            "es" => include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("es"))),

            _ => panic!("Unsupported or missing language feature for '{}'", lang),
        };
        let nlp_tokenizer =
            NlpTokenizer::from_reader(&mut tokenizer_bytes).expect("Valid tokenizer binary");

        self.nlp_tokenizer = Some(nlp_tokenizer);
        self
    }

    pub fn with_paragraph_split_strategy(mut self, strategy: ParagraphSplitStrategy) -> Self {
        self.paragraph_split_strategy = strategy;
        self
    }
    pub fn paragraph_split_strategy(&self) -> ParagraphSplitStrategy {
        self.paragraph_split_strategy
    }

    pub fn with_sticky_headers(mut self, enable: bool) -> Self {
        self.sticky_headers = enable;
        self
    }
    pub fn sticky_headers(&self) -> bool {
        self.sticky_headers
    }

    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = max;
        self
    }
    pub fn max_tokens(&self) -> usize {
        self.max_tokens
    }

    pub fn chunk(&self, text: &str) -> Result<Vec<String>, ChunkingError> {
        MarkdownChunker::new(
            self.max_tokens,
            self.sticky_headers,
            self.paragraph_split_strategy,
            self.tokenizer,
            #[cfg(feature = "nlp")]
            self.nlp_tokenizer,
        )
        .chunk(text)
    }
}
