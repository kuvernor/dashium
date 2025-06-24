use axum::{Router, routing::post};
use sqlx::PgPool;

pub mod user;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/accounts/registerGJAccount.php", post(user::register))
        .route("/accounts/loginGJAccount.php", post(user::login))
        .route("/getAccountURL.php", post(user::url))
        .route(
            "/database/accounts/backupGJAccountNew.php",
            post(user::save_data),
        )
        .route(
            "/database/accounts/syncGJAccountNew.php",
            post(user::load_data),
        )
        .route("/updateGJUserScore22.php", post(user::update_stats))
        .route("/getGJUserInfo20.php", post(user::info))
        .route("/getGJUsers20.php", post(user::search))
}
