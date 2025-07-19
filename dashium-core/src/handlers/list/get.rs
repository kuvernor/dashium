use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::List};

#[derive(Deserialize, Serialize, Debug)]
pub struct getGJLevelLists {
    accountID: i32,
    str: String,
    page: i32,
    #[serde(rename = "type")]
    search_type: u8,
    gjp2: String,
    secret: String,
}

pub async fn getGJLevelLists(
    State(pool): State<PgPool>,
    Form(form): Form<getGJLevelLists>,
) -> Result<String, AppError> {
    let search = &form.str;
    let page = form.page;

    let lists: Vec<List> = List::get_all(&pool, search).await?;

    if lists.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = lists.len();
    let page_info = format!("#{count}:{offset}:10");

    let mut list_string = String::new();

    for list in &lists {
        let temp = list.to_gd();
        list_string.push_str(&temp);
        list_string.push('|');
    }

    list_string.pop();

    let mut creator_string = String::new();

    for list in &lists {
        let temp = format!("{}:{}:{}", list.user_id, list.username, list.user_id);
        creator_string.push_str(&temp);
        creator_string.push('|');
    }

    creator_string.pop();

    Ok(format!("{list_string}#{creator_string}#{page_info}"))
}
