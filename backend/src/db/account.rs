//! Persistence for the `account` table.

use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct AccountCredentials {
    pub id: Uuid,
    pub password_hash: String,
}

/// Insert a new account. Returns the freshly minted id. The caller is
/// responsible for lowercasing the email and hashing the password.
///
/// A duplicate email surfaces as `sqlx::Error::Database` with SQLSTATE
/// 23505; `AppError::from(sqlx::Error)` maps that to `Conflict`.
pub async fn create(pool: &PgPool, email: &str, password_hash: &str) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO account (email, password_hash)
        VALUES ($1, $2)
        RETURNING id
        "#,
        email,
        password_hash,
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.id)
}

/// Look up credentials for login. Returns `None` if the email is unknown.
/// Callers MUST treat unknown-email and wrong-password identically (return
/// 401 in both cases) to avoid an account-enumeration oracle.
pub async fn find_credentials_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<AccountCredentials>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM account
        WHERE email = $1
        "#,
        email,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| AccountCredentials {
        id: r.id,
        password_hash: r.password_hash,
    }))
}
