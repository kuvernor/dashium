use axum::{Router, routing::post};
use sqlx::PgPool;

pub mod message;
pub mod post;
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
        // Posts (aka account comments)
        .route("/getGJAccountComments20.php", post(post::get_posts))
        .route("/uploadGJAccComment20.php", post(post::upload_post))
        .route("/deleteGJAccComment20.php", post(post::delete_post))
        // Messages
        .route("/uploadGJMessage20.php", post(message::send_message))
        .route("/getGJMessages20.php", post(message::get_messages))
        .route("/downloadGJMessage20.php", post(message::download_message))
        .route("/deleteGJMessages20.php", post(message::delete_message))
}
