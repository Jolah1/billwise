//! Section endpoints. A section lives under a project; ownership is
//! enforced in the SQL JOIN, not in the URL — a forged project_id from
//! another account simply yields no rows.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::middleware::AccountId;
use crate::db;
use crate::error::AppError;
use crate::handlers::require_non_empty;
use crate::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateReq {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SectionResp {
    pub id:         Uuid,
    pub project_id: Uuid,
    pub name:       String,
    pub sort_order: i32,
}

impl From<db::section::SectionRow> for SectionResp {
    fn from(r: db::section::SectionRow) -> SectionResp {
        SectionResp {
            id:         r.id,
            project_id: r.project_id,
            name:       r.name,
            sort_order: r.sort_order,
        }
    }
}

pub async fn create(
    State(state): State<AppState>,
    AccountId(account_id): AccountId,
    Path(project_id): Path<Uuid>,
    Json(req): Json<CreateReq>,
) -> Result<impl IntoResponse, AppError> {
    require_non_empty(&req.name, "name")?;
    let row = db::section::create(&state.pool, account_id, project_id, req.name.trim())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok((StatusCode::CREATED, Json(SectionResp::from(row))))
}

pub async fn list(
    State(state): State<AppState>,
    AccountId(account_id): AccountId,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<SectionResp>>, AppError> {
    let rows = db::section::list_by_project(&state.pool, account_id, project_id).await?;
    Ok(Json(rows.into_iter().map(SectionResp::from).collect()))
}
