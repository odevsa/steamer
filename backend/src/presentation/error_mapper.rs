use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::domain::errors::DomainError;
use crate::presentation::dto::common::{ErrorBody, ErrorEnvelope};

pub struct ApiError(pub DomainError);

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self.0 {
            DomainError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, "validation_error", msg.clone())
            }
            DomainError::Security(msg) => (StatusCode::BAD_REQUEST, "security_error", msg.clone()),
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            DomainError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            DomainError::RateLimited(msg) => {
                (StatusCode::TOO_MANY_REQUESTS, "rate_limited", msg.clone())
            }
            DomainError::ExternalService(msg) => {
                tracing::warn!(error = %msg, "external service failure");
                (StatusCode::BAD_GATEWAY, "upstream_error", msg.clone())
            }
            DomainError::Storage(msg) | DomainError::Internal(msg) => {
                tracing::error!(error = %msg, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error".to_string(),
                )
            }
        };

        (
            status,
            Json(ErrorEnvelope {
                error: ErrorBody { code, message },
            }),
        )
            .into_response()
    }
}
