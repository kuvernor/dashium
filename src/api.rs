use axum::Router;
use axum::routing::get;
use sqlx::PgPool;

pub mod level;
pub mod user;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/user/{username}", get(user::get))
        .route("/level/{level}", get(level::get))
}
