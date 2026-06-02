//! Repository layer. Each module owns sqlx queries for one aggregate.
//! Every function that touches an account-scoped table takes the
//! `account_id` explicitly so multi-tenancy is impossible to forget.

pub mod account;
