use axum::{Router, routing::post};
use sqlx::PgPool;

pub mod account;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/accounts/registerGJAccount.php", post(account::register))
        .route("/accounts/loginGJAccount.php", post(account::login))
        .route("/getAccountURL.php", post(account::url))
        .route(
            "/database/accounts/backupGJAccountNew.php",
            post(account::save_data),
        )
        .route(
            "/database/accounts/syncGJAccountNew.php",
            post(account::load_data),
        )
}
