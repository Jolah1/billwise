//! POST /auth/register, POST /auth/login.
//!
//! Both endpoints return the same shape on success: a freshly issued JWT
//! and the account id. Registration is auto-login — convenient for the
//! mobile MVP, easy to split later if we add email verification.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{jwt, password};
use crate::db;
use crate::error::AppError;
use crate::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResp {
    pub account_id: Uuid,
    pub token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterReq>,
) -> Result<impl IntoResponse, AppError> {
    validate_email(&req.email)?;
    validate_password(&req.password)?;

    let email = normalise_email(&req.email);
    let hash = password::hash(&req.password).map_err(AppError::Internal)?;
    let account_id = db::account::create(&state.pool, &email, &hash).await?;
    let token = issue_token(account_id, &state)?;

    Ok((StatusCode::CREATED, Json(AuthResp { account_id, token })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<AuthResp>, AppError> {
    let email = normalise_email(&req.email);
    let creds = db::account::find_credentials_by_email(&state.pool, &email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let ok = password::verify(&req.password, &creds.password_hash).map_err(AppError::Internal)?;
    if !ok {
        return Err(AppError::Unauthorized);
    }
    let token = issue_token(creds.id, &state)?;
    Ok(Json(AuthResp { account_id: creds.id, token }))
}

fn issue_token(account_id: Uuid, state: &AppState) -> Result<String, AppError> {
    jwt::issue(account_id, &state.jwt_secret, state.jwt_ttl).map_err(AppError::Internal)
}

fn normalise_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn validate_email(email: &str) -> Result<(), AppError> {
    let e = email.trim();
    if e.len() < 3 || !e.contains('@') || e.starts_with('@') || e.ends_with('@') {
        return Err(AppError::BadRequest("invalid email address".into()));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.chars().count() < 8 {
        return Err(AppError::BadRequest(
            "password must be at least 8 characters".into(),
        ));
    }
    Ok(())
}
