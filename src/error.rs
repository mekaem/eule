use thiserror::Error;

#[derive(Error, Debug)]
pub enum EuleError {
    #[error("Database error: {0}")]
    Database(#[from] sled::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Poise error: {0}")]
    Poise(#[from] poise::serenity_prelude::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Tracing setup failed: {0}")]
    TracingSetupFailed(String),

    #[error("Unknown error occurred")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, EuleError>;