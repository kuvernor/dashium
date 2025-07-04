use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::{Friendship, User},
    util::verify_gjp2,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListForm {
    #[serde(rename = "accountID")]
    user_id: i32,

    gjp2: String,

    #[serde(rename = "type")]
    list_type: u8,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn get_user_list(
    State(pool): State<PgPool>,
    Form(form): Form<ListForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let friends: Vec<Friendship> = Friendship::get_friends(&pool, user_id).await?;

    if friends.is_empty() {
        return Ok("-2".to_string());
    }

    let mut response = String::new();

    for friend in friends {
        let user2 = User::get_user(&pool, friend.user2).await?;
        let temp = User::to_gd(user2);

        response.push_str(&temp);
        response.push_str(":41:1");
        response.push('|');
    }

    response.pop();

    Ok(response)
}
