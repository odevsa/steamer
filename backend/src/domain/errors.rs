use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("resource already exists: {0}")]
    Conflict(String),
    #[error("rate limit exceeded: {0}")]
    RateLimited(String),
    #[error("security violation: {0}")]
    Security(String),
    #[error("upstream service error: {0}")]
    ExternalService(String),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("internal error: {0}")]
    Internal(String),
}
