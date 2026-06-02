//! Persistence for the `section` table. Ownership is enforced via a JOIN
//! to `project` so a forged section_id from another account just
//! disappears (no row, no rows affected).

use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct SectionRow {
    pub id:         Uuid,
    pub project_id: Uuid,
    pub name:       String,
    pub sort_order: i32,
}

/// Insert a section, auto-assigning `sort_order` as the next slot. Returns
/// `None` if the project doesn't exist or isn't owned by `account_id`.
pub async fn create(
    pool: &PgPool,
    account_id: Uuid,
    project_id: Uuid,
    name: &str,
) -> Result<Option<SectionRow>, sqlx::Error> {
    let row = sqlx::query_as!(
        SectionRow,
        r#"
        INSERT INTO section (project_id, name, sort_order)
        SELECT p.id,
               $2,
               COALESCE((SELECT MAX(sort_order) + 1 FROM section WHERE project_id = p.id), 0)
        FROM project p
        WHERE p.id = $1 AND p.account_id = $3
        RETURNING id, project_id, name, sort_order
        "#,
        project_id,
        name,
        account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_by_project(
    pool: &PgPool,
    account_id: Uuid,
    project_id: Uuid,
) -> Result<Vec<SectionRow>, sqlx::Error> {
    let rows = sqlx::query_as!(
        SectionRow,
        r#"
        SELECT s.id, s.project_id, s.name, s.sort_order
        FROM section s
        JOIN project p ON p.id = s.project_id
        WHERE s.project_id = $1 AND p.account_id = $2
        ORDER BY s.sort_order, s.created_at
        "#,
        project_id,
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
