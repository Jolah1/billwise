//! Runtime config loaded from environment variables. `.env` is read
//! best-effort; missing variables that have no default cause startup to
//! fail loudly rather than booting with a silent default.

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Context};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr:    SocketAddr,
    /// Used by the auth slice; loaded here so missing env trips startup.
    #[allow(dead_code)]
    pub jwt_secret: String,
    #[allow(dead_code)]
    pub jwt_ttl: Duration,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        // .env is optional — the process may be configured by the runtime.
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow!("DATABASE_URL is required"))?;
        let bind_addr_raw = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
        let bind_addr = SocketAddr::from_str(&bind_addr_raw)
            .with_context(|| format!("BIND_ADDR is not a valid socket address: {bind_addr_raw:?}"))?;
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| anyhow!("JWT_SECRET is required"))?;
        if jwt_secret.len() < 16 {
            return Err(anyhow!("JWT_SECRET must be at least 16 chars"));
        }
        let jwt_ttl_seconds: u64 = env::var("JWT_TTL_SECONDS")
            .unwrap_or_else(|_| "86400".into())
            .parse()
            .context("JWT_TTL_SECONDS must be a non-negative integer")?;

        Ok(Config {
            database_url,
            bind_addr,
            jwt_secret,
            jwt_ttl: Duration::from_secs(jwt_ttl_seconds),
        })
    }
}
