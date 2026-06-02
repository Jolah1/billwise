//! Authentication primitives. `password` and `jwt` are pure modules with
//! unit tests; `middleware` provides the `AccountId` axum extractor and
//! lives next to them in the next commit.

pub mod jwt;
pub mod password;
