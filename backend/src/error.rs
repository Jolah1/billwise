//! Single error type for the HTTP layer. Handlers return `Result<_, AppError>`
//! and `?` over sqlx / anyhow / parsing errors. Internal errors log the full
//! cause chain but are not exposed to the client.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)] // Variants land as upcoming slices use them.
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Conflict(String),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::Unauthorized  => StatusCode::UNAUTHORIZED,
            AppError::Forbidden     => StatusCode::FORBIDDEN,
            AppError::NotFound      => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Conflict(_)   => StatusCode::CONFLICT,
            AppError::Internal(_)   => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            AppError::Unauthorized  => "unauthorized",
            AppError::Forbidden     => "forbidden",
            AppError::NotFound      => "not_found",
            AppError::BadRequest(_) => "bad_request",
            AppError::Conflict(_)   => "conflict",
            AppError::Internal(_)   => "internal",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let AppError::Internal(err) = &self {
            tracing::error!(error = ?err, "internal server error");
        }
        let body = Json(json!({
            "error":   self.code(),
            "message": self.to_string(),
        }));
        (self.status(), body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> AppError {
        if matches!(err, sqlx::Error::RowNotFound) {
            return AppError::NotFound;
        }
        if let sqlx::Error::Database(ref db_err) = err {
            // 23505 = unique_violation
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Conflict("resource already exists".into());
            }
        }
        AppError::Internal(anyhow::Error::new(err))
    }
}
