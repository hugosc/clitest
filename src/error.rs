use thiserror::Error;

/// Custom error type for the fruit CLI application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("fruitdata error: {0}")]
    FruitData(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("An unexpected error occurred: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
