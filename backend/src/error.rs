use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Post not found: {0}")]
    NotFound(String),

    #[error("Invalid post path: {0}")]
    InvalidPath(String),

    #[error("Filesystem I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal server error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, client_message) = match &self {
            AppError::NotFound(slug) => {
                (StatusCode::NOT_FOUND, format!("Post '{slug}' was not found"))
            }
            AppError::InvalidPath(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Io(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    (StatusCode::NOT_FOUND, "Resource not found".to_string())
                } else {
                    tracing::error!("Filesystem error: {:?}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal storage error".to_string(),
                    )
                }
            }
            AppError::Internal(err) => {
                tracing::error!("Internal error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": client_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

