use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChunkingError {
    #[error("Failed to parse markdown: {0}")]
    MarkdownParseError(String),

    #[error("Failed to process chunking: {0}")]
    ChunkingProcessError(String),

    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub fn markdown_error<E: std::fmt::Display>(e: E) -> ChunkingError {
    ChunkingError::MarkdownParseError(e.to_string())
}

pub fn chunking_error<E: std::fmt::Display>(e: E) -> ChunkingError {
    ChunkingError::ChunkingProcessError(e.to_string())
}

pub fn tokenization_error<E: std::fmt::Display>(e: E) -> ChunkingError {
    ChunkingError::TokenizationError(e.to_string())
}

pub fn unknown_error<E: std::fmt::Display>(e: E) -> ChunkingError {
    ChunkingError::Unknown(e.to_string())
}
