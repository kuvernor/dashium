use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use sqlx::PgPool;

pub mod level;
pub mod list;
pub mod user;

pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self.0),
            )
                .into_response()
        }
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/user", get(user::search))
        .route("/user/{username}", get(user::get))
        .route("/user/count", get(user::count))
        .route("/level", get(level::search))
        .route("/level/user/{username}", get(level::user))
        .route("/level/{level}", get(level::get))
        .route("/level/count", get(level::count))
        .route("/level/daily", get(level::daily))
        .route("/level/weekly", get(level::weekly))
        .route("/level/event", get(level::event))
        .route("/list/count", get(list::count))
}
