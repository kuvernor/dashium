use anyhow::Result;
use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

#[cfg(debug_assertions)]
use tower_governor::key_extractor::GlobalKeyExtractor;

#[cfg(not(debug_assertions))]
use tower_governor::key_extractor::SmartIpKeyExtractor;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    let pool = setup_db().await?;
    let app = setup_app(pool);
    let listener = TcpListener::bind("127.0.0.1:2207").await?;
    tracing::info!("Server running at http://127.0.0.1:2207");

    #[cfg(debug_assertions)]
    tracing::warn!("Running in debug mode. Use only for local development.");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn setup_db() -> Result<PgPool> {
    tracing::info!("Connecting to the database...");
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}

fn setup_app(pool: PgPool) -> Router {
    #[cfg(debug_assertions)]
    let key_extractor = GlobalKeyExtractor;

    #[cfg(not(debug_assertions))]
    let key_extractor = SmartIpKeyExtractor;

    let governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .key_extractor(key_extractor)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_config.limiter().clone();
    let interval = Duration::from_secs(60);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    Router::new()
        .merge(dashium_core::routes())
        .nest("/api", dashium_api::routes())
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
        .layer(GovernorLayer {
            config: governor_config,
        })
}

fn setup_logging() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
