//! `AccountId` extractor — applied to any protected handler. Rejects
//! missing/invalid/expired tokens with 401.

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::auth::jwt;
use crate::error::AppError;
use crate::routes::AppState;

/// The id of the authenticated account, extracted from a JWT in the
/// `Authorization: Bearer …` header. Pass straight into `db/` calls to
/// keep multi-tenant scoping explicit at the boundary.
#[derive(Debug, Clone, Copy)]
pub struct AccountId(pub Uuid);

#[async_trait]
impl FromRequestParts<AppState> for AccountId {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;
        let claims = jwt::decode_token(token, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized)?;
        Ok(AccountId(claims.sub))
    }
}
