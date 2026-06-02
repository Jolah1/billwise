//! Pure domain layer. No sqlx, no axum — every public item here is testable
//! without a database or an HTTP runtime.
//!
//! See `docs/ARCHITECTURE.md` for the load-bearing rules these types enforce.

pub mod item;
pub mod money;
pub mod price;
