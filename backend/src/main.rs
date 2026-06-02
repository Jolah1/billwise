//! BillWise backend entry point.
//!
//! Boot order:
//!   1. tracing subscriber
//!   2. load config from env / .env
//!   3. connect Postgres pool
//!   4. run sqlx migrations
//!   5. bind axum on $BIND_ADDR and serve

mod auth;
mod config;
mod domain;
mod error;
mod routes;

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,billwise=debug,tower_http=info")),
        )
        .init();

    let config = config::Config::from_env()?;
    tracing::info!(bind_addr = %config.bind_addr, "starting billwise");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("connecting to Postgres")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("running migrations")?;
    tracing::info!("migrations applied");

    let app = routes::router(routes::AppState { pool });

    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .with_context(|| format!("binding {}", config.bind_addr))?;
    tracing::info!(addr = %config.bind_addr, "listening");
    axum::serve(listener, app)
        .await
        .context("axum::serve")?;
    Ok(())
}
