//! Authentication primitives. `password` and `jwt` are pure modules with
//! unit tests; `middleware` provides the `AccountId` axum extractor.

pub mod jwt;
pub mod middleware;
pub mod password;
