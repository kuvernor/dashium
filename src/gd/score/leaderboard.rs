use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{AppError, models::User};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    #[serde(rename = "type")]
    leaderboard_type: String,
}

pub async fn get_leaderboard(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let leaderboard_type = &form.leaderboard_type;

    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        SELECT  
            id,
            username,
            stars,
            demons,
            creator_points,
            color1,
            color2,
            coins,
            icon_type,
            display_icon,
            user_coins,
            message_setting,
            friend_setting,
            youtube,
            icon,
            ship,
            ball,
            ufo,
            wave,
            robot,
            glow,
            is_activated,
            rank,
            spider,
            twitter,
            twitch,
            diamonds,
            explosion,
            mod_level,
            comment_setting,
            color3,
            moons,
            swing,
            jetpack,
            demon_info,
            level_info,
            platformer_info
        FROM users
        "#,
    );

    match leaderboard_type.as_str() {
        "top" => {
            query.push(" ORDER BY stars DESC");
        }
        "creators" => {
            query.push(" ORDER BY creator_points DESC");
        }
        _ => return Ok("".to_string()),
    }

    let users = query.build_query_as().fetch_all(&pool).await?;

    let mut response = String::new();

    for user in users {
        let temp = User::to_gd(user);
        response.push_str(&temp);
        response.push('|');
    }

    response.pop();

    Ok(response)
}
