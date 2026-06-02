//! Router wiring + shared application state.
//!
//! Handlers receive `State<AppState>` and read what they need. Keeping
//! all state in one struct means future additions (auth secrets, caches)
//! don't ripple through every handler signature.

use std::sync::Arc;
use std::time::Duration;

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Arc<str>,
    pub jwt_ttl: Duration,
}

pub fn router(state: AppState) -> Router {
    // Permissive CORS for local Expo development. Tighten for production
    // (allow only the deployed mobile origin + custom URL scheme).
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login",    post(handlers::auth::login))
        .route("/projects",
               get(handlers::projects::list).post(handlers::projects::create))
        .route("/projects/:project_id", get(handlers::projects::detail))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
