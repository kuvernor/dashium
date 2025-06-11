use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = setup_db().await?;
    let app = setup_app(pool);
    let listener = TcpListener::bind("localhost:2207").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn setup_db() -> anyhow::Result<PgPool> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}

fn setup_app(pool: PgPool) -> Router {
    Router::new().with_state(pool)
}
