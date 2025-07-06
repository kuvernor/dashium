use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::{Block, Friendship, User},
    util::verify_gjp2,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListForm {
    accountID: i32,
    gjp2: String,
    #[serde(default)]
    #[serde(rename = "type")]
    list_type: u8,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn get_user_list(
    State(pool): State<PgPool>,
    Form(form): Form<ListForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let list_type = form.list_type;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    if list_type == 1 {
        let blocks: Vec<Block> = Block::get_all(&pool, user_id).await?;

        if blocks.is_empty() {
            return Ok("-2".to_string());
        }

        let mut response = String::new();

        for block in blocks {
            let target = User::get_user(&pool, block.blocked_id).await?;
            let temp = User::to_gd(target);

            response.push_str(&temp);
            response.push('|');
        }

        response.pop();

        return Ok(response);
    }

    let friends: Vec<Friendship> = Friendship::get_all(&pool, user_id).await?;

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
