use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::Gauntlet, util::salt_and_sha1};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    secret: String,
}

pub async fn get_gauntlets(
    State(pool): State<PgPool>,
    Form(_form): Form<GetForm>,
) -> Result<String, AppError> {
    let gauntlets: Vec<Gauntlet> = Gauntlet::get(&pool).await?;

    if gauntlets.is_empty() {
        return Ok("-2".to_string());
    }

    let hash = generate_hash(&gauntlets);

    let mut response = String::new();

    for gauntlet in &gauntlets {
        response.push_str(&gauntlet.to_gd());
        response.push('|');
    }

    response.pop();
    response.push('#');
    response.push_str(&hash);

    Ok(response)
}

fn generate_hash(gauntlets: &Vec<Gauntlet>) -> String {
    let mut hash = String::new();

    for gauntlet in gauntlets {
        let gauntlet_id = &gauntlet.id.to_string();
        let levels = &gauntlet.levels;

        hash.push_str(gauntlet_id);
        hash.push_str(levels);
    }

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
