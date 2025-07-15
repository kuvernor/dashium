use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::FriendRequest, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    accountID: i32,
    gjp2: String,
    page: i32,
    #[serde(default)]
    getSent: i16,
    secret: String,
}

pub async fn get_friend_requests(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let page = form.page;
    let get_sent = form.getSent;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let friend_requests: Vec<FriendRequest> = match get_sent {
        1 => FriendRequest::get_all_sent(&pool, user_id).await?,
        _ => FriendRequest::get_all(&pool, user_id).await?,
    };

    if friend_requests.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = friend_requests.len();
    let end_string = format!("#{count}:{offset}:20");

    let mut response = String::new();

    for friend_request in friend_requests {
        response.push_str(&friend_request.to_gd());
        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
