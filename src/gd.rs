use axum::{Router, routing::post};
use sqlx::PgPool;

pub mod account;

pub fn routes() -> Router<PgPool> {
    Router::new().route("/accounts/registerGJAccount.php", post(account::register))
}
