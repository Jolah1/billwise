//! Persistence for the `item` table. The Postgres `item_type` ENUM is
//! converted at the SQL boundary (`$x::item_type` on write, `::text` on
//! read) so `domain::item::ItemType` stays free of sqlx derives.

use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::item::ItemType;

#[derive(Debug)]
pub struct ItemRow {
    pub id:          Uuid,
    pub section_id:  Uuid,
    pub description: String,
    pub quantity:    BigDecimal,
    pub unit:        String,
    pub rate_kobo:   i64,
    pub item_type:   ItemType,
    pub sort_order:  i32,
}

pub struct NewItem<'a> {
    pub description: &'a str,
    pub quantity:    &'a BigDecimal,
    pub unit:        &'a str,
    pub rate_kobo:   i64,
    pub item_type:   ItemType,
}

/// Insert an item, auto-assigning `sort_order`. Returns `None` if the
/// section doesn't exist or isn't owned (via its project) by `account_id`.
pub async fn create(
    pool: &PgPool,
    account_id: Uuid,
    section_id: Uuid,
    it: NewItem<'_>,
) -> Result<Option<ItemRow>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO item (section_id, description, quantity, unit, rate_kobo, item_type, sort_order)
        SELECT s.id,
               $2, $3, $4, $5, $6::TEXT::item_type,
               COALESCE((SELECT MAX(sort_order) + 1 FROM item WHERE section_id = s.id), 0)
        FROM section s
        JOIN project p ON p.id = s.project_id
        WHERE s.id = $1 AND p.account_id = $7
        RETURNING id,
                  section_id,
                  description,
                  quantity,
                  unit,
                  rate_kobo,
                  item_type::text AS "item_type!: String",
                  sort_order
        "#,
        section_id,
        it.description,
        it.quantity,
        it.unit,
        it.rate_kobo,
        it.item_type.as_str(),
        account_id,
    )
    .fetch_optional(pool)
    .await?;

    row.map(|r| {
        let item_type: ItemType = r.item_type.parse().map_err(|e| {
            sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unknown item_type returned from DB: {e}"),
            )))
        })?;
        Ok(ItemRow {
            id:          r.id,
            section_id:  r.section_id,
            description: r.description,
            quantity:    r.quantity,
            unit:        r.unit,
            rate_kobo:   r.rate_kobo,
            item_type,
            sort_order:  r.sort_order,
        })
    })
    .transpose()
}

/// All items in all sections of a project, ordered by section then item
/// `sort_order`. Account-scoped via the JOIN to `project`.
pub async fn list_by_project(
    pool: &PgPool,
    account_id: Uuid,
    project_id: Uuid,
) -> Result<Vec<ItemRow>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT i.id,
               i.section_id,
               i.description,
               i.quantity,
               i.unit,
               i.rate_kobo,
               i.item_type::text AS "item_type!: String",
               i.sort_order
        FROM item i
        JOIN section s ON s.id = i.section_id
        JOIN project p ON p.id = s.project_id
        WHERE p.id = $1 AND p.account_id = $2
        ORDER BY s.sort_order, i.sort_order
        "#,
        project_id,
        account_id,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|r| {
            let item_type: ItemType = r.item_type.parse().map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("unknown item_type returned from DB: {e}"),
                )))
            })?;
            Ok(ItemRow {
                id:          r.id,
                section_id:  r.section_id,
                description: r.description,
                quantity:    r.quantity,
                unit:        r.unit,
                rate_kobo:   r.rate_kobo,
                item_type,
                sort_order:  r.sort_order,
            })
        })
        .collect()
}
