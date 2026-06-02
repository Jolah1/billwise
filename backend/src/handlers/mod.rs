//! HTTP handler layer. Thin: extract → call domain/db → return JSON.
//! No business logic here beyond input validation; that belongs in
//! `domain/` or `db/`.

pub mod auth;
pub mod items;
pub mod projects;
pub mod sections;

use crate::error::AppError;

pub(crate) fn require_non_empty(value: &str, field: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!("{field} is required")));
    }
    Ok(())
}
