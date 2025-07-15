use crate::AppError;
use crate::util::verify_gjp2;
use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, query_builder::QueryBuilder};

#[derive(Deserialize, Serialize, Debug)]
pub struct LikeForm {
    accountID: i32,
    secret: String,

    #[serde(rename = "type")]
    item_type: u8,
    chk: String,
    itemID: i32,
    gjp2: String,
    udid: String,
    uuid: String,
    like: u8,
}

pub async fn like(
    State(pool): State<PgPool>,
    Form(form): Form<LikeForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let item_id = form.itemID;
    let like = form.like;
    let item_type = form.item_type;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("1".to_string());
    }

    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE");

    match item_type {
        1 => query.push(" levels SET"),
        2 => query.push(" comments SET"),
        3 => query.push(" posts SET"),
        4 => query.push(" lists SET"),
        _ => return Ok("1".to_string()),
    };

    match like {
        0 => {
            query.push(" likes = likes - 1 WHERE id = ");
            query.push_bind(item_id);
        }
        1 => {
            query.push(" likes = likes + 1 WHERE id = ");
            query.push_bind(item_id);
        }
        _ => return Ok("1".to_string()),
    };

    query.build().execute(&pool).await.unwrap();

    Ok("1".to_string())
}
