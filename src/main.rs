use anyhow::Result;
use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub mod error;
pub use crate::error::AppError;

mod gd;
mod models;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    let pool = setup_db().await?;
    let app = setup_app(pool);
    let listener = TcpListener::bind("localhost:2207").await?;
    info!("Server running at http://localhost:2207");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn setup_db() -> Result<PgPool> {
    info!("Connecting to the database...");
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}

fn setup_app(pool: PgPool) -> Router {
    Router::new().nest("/gd", gd::routes()).with_state(pool)
}

fn setup_logging() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
