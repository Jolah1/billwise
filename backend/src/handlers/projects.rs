//! Project endpoints. `create` and `list` are shallow; `detail` returns
//! the full tree (sections + items + server-computed totals) so the
//! mobile detail screen needs a single round trip.
//!
//! 404 (not 403) is returned when a project_id belongs to another
//! account — leaking existence would be an enumeration oracle.

use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::auth::middleware::AccountId;
use crate::db;
use crate::domain::item::{self as domain_item, ItemType};
use crate::domain::money::Kobo;
use crate::error::AppError;
use crate::handlers::require_non_empty;
use crate::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateReq {
    pub title:        String,
    pub client:       String,
    pub location:     String,
    pub project_date: Date,
}

#[derive(Debug, Serialize)]
pub struct ProjectResp {
    pub id:           Uuid,
    pub title:        String,
    pub client:       String,
    pub location:     String,
    pub project_date: Date,
    pub created_at:   OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct ProjectDetailResp {
    pub id:           Uuid,
    pub title:        String,
    pub client:       String,
    pub location:     String,
    pub project_date: Date,
    pub created_at:   OffsetDateTime,
    pub total_kobo:   i64,
    pub sections:     Vec<SectionDetailResp>,
}

#[derive(Debug, Serialize)]
pub struct SectionDetailResp {
    pub id:            Uuid,
    pub name:          String,
    pub sort_order:    i32,
    pub subtotal_kobo: i64,
    pub items:         Vec<ItemDetailResp>,
}

#[derive(Debug, Serialize)]
pub struct ItemDetailResp {
    pub id:          Uuid,
    pub description: String,
    pub quantity:    String,
    pub unit:        String,
    pub rate_kobo:   i64,
    pub item_type:   ItemType,
    pub sort_order:  i32,
    pub amount_kobo: i64,
}

pub async fn create(
    State(state): State<AppState>,
    AccountId(account_id): AccountId,
    Json(req): Json<CreateReq>,
) -> Result<impl IntoResponse, AppError> {
    require_non_empty(&req.title, "title")?;
    require_non_empty(&req.client, "client")?;
    require_non_empty(&req.location, "location")?;

    let id = db::project::create(
        &state.pool,
        account_id,
        db::project::NewProject {
            title:        req.title.trim(),
            client:       req.client.trim(),
            location:     req.location.trim(),
            project_date: req.project_date,
        },
    )
    .await?;

    let row = db::project::find(&state.pool, account_id, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok((StatusCode::CREATED, Json(ProjectResp::from(row))))
}

pub async fn list(
    State(state): State<AppState>,
    AccountId(account_id): AccountId,
) -> Result<Json<Vec<ProjectResp>>, AppError> {
    let rows = db::project::list_by_account(&state.pool, account_id).await?;
    Ok(Json(rows.into_iter().map(ProjectResp::from).collect()))
}

pub async fn detail(
    State(state): State<AppState>,
    AccountId(account_id): AccountId,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectDetailResp>, AppError> {
    let project = db::project::find(&state.pool, account_id, project_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let sections = db::section::list_by_project(&state.pool, account_id, project_id).await?;
    let items    = db::item::list_by_project(&state.pool, account_id, project_id).await?;

    let mut items_by_section: HashMap<Uuid, Vec<ItemDetailResp>> = HashMap::new();
    for row in items {
        let amount = domain_item::amount(&row.quantity, Kobo::new(row.rate_kobo))
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!(
                "amount overflow on item {}", row.id
            )))?;
        items_by_section.entry(row.section_id).or_default().push(ItemDetailResp {
            id:          row.id,
            description: row.description,
            quantity:    row.quantity.to_string(),
            unit:        row.unit,
            rate_kobo:   row.rate_kobo,
            item_type:   row.item_type,
            sort_order:  row.sort_order,
            amount_kobo: amount.as_i64(),
        });
    }

    let sections_resp: Vec<SectionDetailResp> = sections
        .into_iter()
        .map(|s| {
            let items = items_by_section.remove(&s.id).unwrap_or_default();
            let subtotal_kobo: i64 = items.iter().map(|i| i.amount_kobo).sum();
            SectionDetailResp {
                id:         s.id,
                name:       s.name,
                sort_order: s.sort_order,
                subtotal_kobo,
                items,
            }
        })
        .collect();

    let total_kobo: i64 = sections_resp.iter().map(|s| s.subtotal_kobo).sum();

    Ok(Json(ProjectDetailResp {
        id:           project.id,
        title:        project.title,
        client:       project.client,
        location:     project.location,
        project_date: project.project_date,
        created_at:   project.created_at,
        total_kobo,
        sections:     sections_resp,
    }))
}

impl From<db::project::ProjectRow> for ProjectResp {
    fn from(r: db::project::ProjectRow) -> ProjectResp {
        ProjectResp {
            id:           r.id,
            title:        r.title,
            client:       r.client,
            location:     r.location,
            project_date: r.project_date,
            created_at:   r.created_at,
        }
    }
}
