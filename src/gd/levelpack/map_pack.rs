use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::MapPack, util::salt_and_sha1};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    page: i32,
    secret: String,
}

pub async fn get_map_packs(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let page = form.page;

    let map_packs: Vec<MapPack> = MapPack::get(&pool).await?;

    if map_packs.is_empty() {
        return Ok("#0:0:10".to_string());
    }

    let hash = generate_hash(&map_packs);

    let offset = page * 10;
    let count = map_packs.len();
    let end_string = format!("#{count}:{offset}:10#");

    let mut response = String::new();

    for map_pack in &map_packs {
        response.push_str(&map_pack.to_gd());
        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);
    response.push_str(&hash);

    Ok(response)
}

fn generate_hash(map_packs: &Vec<MapPack>) -> String {
    let mut hash = String::new();

    for map_pack in map_packs {
        let pack_id = map_pack.id.to_string();
        let first = pack_id.chars().next().unwrap_or('0');
        let last = pack_id.chars().next_back().unwrap_or('0');

        let stars = map_pack.stars.to_string();
        let coins = map_pack.coins.to_string();

        hash.push(first);
        hash.push(last);
        hash.push_str(&stars);
        hash.push_str(&coins);
    }

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
