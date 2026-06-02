//! Router wiring + shared application state.
//!
//! Handlers receive `State<AppState>` and read what they need. Keeping
//! all state in one struct means future additions (auth secrets, caches)
//! don't ripple through every handler signature.

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // Read by handlers in upcoming slices.
    pub pool: PgPool,
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
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
