//! HTTP handler layer. Thin: extract → call domain/db → return JSON.
//! No business logic here beyond input validation; that belongs in
//! `domain/` or `db/`.

pub mod auth;
