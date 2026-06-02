//! Persistence for the `project` table. Every function is scoped by
//! `account_id` — there is no way to fetch a project owned by another
//! account without first changing this file.

use sqlx::PgPool;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct ProjectRow {
    pub id:           Uuid,
    pub title:        String,
    pub client:       String,
    pub location:     String,
    pub project_date: Date,
    pub created_at:   OffsetDateTime,
}

pub struct NewProject<'a> {
    pub title:        &'a str,
    pub client:       &'a str,
    pub location:     &'a str,
    pub project_date: Date,
}

pub async fn create(
    pool: &PgPool,
    account_id: Uuid,
    p: NewProject<'_>,
) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO project (account_id, title, client, location, project_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        account_id,
        p.title,
        p.client,
        p.location,
        p.project_date,
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.id)
}

pub async fn list_by_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<ProjectRow>, sqlx::Error> {
    let rows = sqlx::query_as!(
        ProjectRow,
        r#"
        SELECT id, title, client, location, project_date, created_at
        FROM project
        WHERE account_id = $1
        ORDER BY created_at DESC
        "#,
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn find(
    pool: &PgPool,
    account_id: Uuid,
    project_id: Uuid,
) -> Result<Option<ProjectRow>, sqlx::Error> {
    let row = sqlx::query_as!(
        ProjectRow,
        r#"
        SELECT id, title, client, location, project_date, created_at
        FROM project
        WHERE id = $1 AND account_id = $2
        "#,
        project_id,
        account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
