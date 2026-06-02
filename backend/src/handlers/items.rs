//! Item creation endpoint. Ownership traverses item → section → project
//! → account via SQL JOINs in `db::item::create`.
//!
//! `quantity` is wire-encoded as a string to avoid JS float precision
//! loss; it is parsed straight into a BigDecimal here. `amount_kobo` in
//! the response is the canonical server-derived value (Architecture
//! Rule 3) — a client-sent amount would be ignored.

use std::str::FromStr;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
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
    pub description: String,
    pub quantity:    String,
    pub unit:        String,
    pub rate_kobo:   i64,
    pub item_type:   ItemType,
}

#[derive(Debug, Serialize)]
pub struct ItemResp {
    pub id:          Uuid,
    pub section_id:  Uuid,
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
    Path(section_id): Path<Uuid>,
    Json(req): Json<CreateReq>,
) -> Result<impl IntoResponse, AppError> {
    require_non_empty(&req.description, "description")?;
    require_non_empty(&req.unit, "unit")?;

    if req.rate_kobo < 0 {
        return Err(AppError::BadRequest("rate_kobo must be >= 0".into()));
    }

    let quantity = BigDecimal::from_str(req.quantity.trim())
        .map_err(|_| AppError::BadRequest("quantity must be a decimal number".into()))?;
    if quantity.sign() == bigdecimal::num_bigint::Sign::Minus {
        return Err(AppError::BadRequest("quantity must be >= 0".into()));
    }

    let amount = domain_item::amount(&quantity, Kobo::new(req.rate_kobo))
        .ok_or_else(|| AppError::BadRequest("quantity x rate_kobo overflows".into()))?;

    let row = db::item::create(
        &state.pool,
        account_id,
        section_id,
        db::item::NewItem {
            description: req.description.trim(),
            quantity:    &quantity,
            unit:        req.unit.trim(),
            rate_kobo:   req.rate_kobo,
            item_type:   req.item_type,
        },
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok((StatusCode::CREATED, Json(ItemResp {
        id:          row.id,
        section_id:  row.section_id,
        description: row.description,
        quantity:    row.quantity.to_string(),
        unit:        row.unit,
        rate_kobo:   row.rate_kobo,
        item_type:   row.item_type,
        sort_order:  row.sort_order,
        amount_kobo: amount.as_i64(),
    })))
}
