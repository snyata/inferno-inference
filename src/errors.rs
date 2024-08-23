/// # Inferno Errors
/// ## Custom Error Handling for Inferno Library
///
/// 1. API Request Failed
/// 2. Invalid JSON Response
/// 3. Validation Failed
/// 4. Unexpected Error

// API Errors
#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("API request failed: {0}")]
    ApiRequestFailed(String),

    #[error("Logging Init Failed: {0}")]
    LoggingError(String),

    #[error("Invalid JSON response: {0}")]
    InvalidResponse(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type Result<T> = std::result::Result<T, CustomError>;
