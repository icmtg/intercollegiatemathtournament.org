use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Session error: {0}")]
    Session(#[from] tower_sessions::session::Error),

    #[error("User already exists")]
    UserExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal server error")]
    Internal(#[from] Box<dyn std::error::Error>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::Session(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Session error"),
            ApiError::UserExists => (StatusCode::CONFLICT, "User already exists"),
            ApiError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
