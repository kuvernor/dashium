use anyhow::Result;
use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

pub mod error;
pub use crate::error::AppError;

mod gd;
mod models;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = setup_db().await?;
    let app = setup_app(pool);
    let listener = TcpListener::bind("localhost:2207").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn setup_db() -> Result<PgPool> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}

fn setup_app(pool: PgPool) -> Router {
    Router::new().nest("/gd", gd::routes()).with_state(pool)
}
