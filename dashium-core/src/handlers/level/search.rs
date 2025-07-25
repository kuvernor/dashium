use axum::extract::{RawForm, State};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    AppError, GDResponse,
    models::Level,
    util::{int_to_bool, is_numeric, salt_and_sha1},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct getGJLevels21 {
    accountID: i32,
    binaryVersion: i16,
    #[serde(deserialize_with = "int_to_bool")]
    coins: bool,
    #[serde(default)]
    completedLevels: String,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    customSong: bool,
    #[serde(default)]
    demonFilter: i16,
    diff: Vec<String>,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    epic: bool,
    #[serde(deserialize_with = "int_to_bool")]
    featured: bool,
    #[serde(default)]
    followed: String,
    gameVersion: i16,
    #[serde(default)]
    gauntlet: i32,
    gjp2: String,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    legendary: bool,
    len: String,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    local: bool,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    mythic: bool,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    noStar: bool,
    #[serde(deserialize_with = "int_to_bool")]
    onlyCompleted: bool,
    #[serde(deserialize_with = "int_to_bool")]
    original: bool,
    page: i32,
    secret: String,
    #[serde(default)]
    song: i32,
    #[serde(deserialize_with = "int_to_bool")]
    #[serde(default)]
    star: bool,
    str: String,
    total: i32,
    #[serde(deserialize_with = "int_to_bool")]
    twoPlayer: bool,
    #[serde(deserialize_with = "int_to_bool")]
    uncompleted: bool,
    #[serde(rename = "type")]
    search_type: i16,
}

pub async fn getGJLevels21(
    State(pool): State<PgPool>,
    RawForm(form): RawForm,
) -> Result<String, AppError> {
    let form: getGJLevels21 = serde_html_form::from_bytes(&form)?;

    let user_id = form.accountID;
    let page = form.page;
    let search = &form.str;
    let search_type = form.search_type;
    let coins = form.coins;
    let epic = form.epic;
    let legendary = form.legendary;
    let featured = form.featured;
    let two_player = form.twoPlayer;
    let mythic = form.mythic;
    let length = &form.len;
    let difficulty = form.diff;
    let demon_difficulty = form.demonFilter;
    let local = form.local;
    let star = form.star;
    let no_star = form.noStar;
    let followed = &form.followed;
    let completed_levels = &form.completedLevels;
    let only_completed = form.onlyCompleted;
    let uncompleted = form.uncompleted;
    let original = form.original;

    let levels =
        match search_type {
            7 => {
                sqlx::query_as!(
                    Level,
                    "SELECT * FROM levels WHERE objects > 9999 ORDER BY created_at DESC"
                )
                .fetch_all(&pool)
                .await?
            }
            11 => sqlx::query_as!(
                Level,
                "SELECT * FROM levels WHERE rated = TRUE ORDER BY rated_at DESC, created_at DESC"
            )
            .fetch_all(&pool)
            .await?,
            12 => {
                let user_ids: Vec<i32> = followed
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();

                sqlx::query_as!(
                    Level,
                    r#"
                SELECT * FROM levels
                WHERE user_id = ANY($1)
                ORDER BY created_at DESC
                "#,
                    &user_ids
                )
                .fetch_all(&pool)
                .await?
            }
            13 => {
                sqlx::query_as!(
                    Level,
                    r#"
                    SELECT levels.*
                    FROM levels
                    WHERE levels.user_id IN (
                        SELECT 
                        CASE 
                            WHEN friendships.user1 = $1 THEN friendships.user2
                            ELSE friendships.user1
                        END AS friend_id
                        FROM friendships
                        WHERE friendships.user1 = $1 OR friendships.user2 = $1
                        ORDER BY created_at DESC
                    );
                    "#,
                    user_id
                )
                .fetch_all(&pool)
                .await?
            }
            21 => {
                sqlx::query_as!(
                    Level,
                    r#"
                    SELECT levels.*
                    FROM levels
                    JOIN daily_levels ON levels.id = daily_levels.level_id
                    ORDER BY daily_levels.id DESC
                    "#
                )
                .fetch_all(&pool)
                .await?
            }
            22 => {
                sqlx::query_as!(
                    Level,
                    r#"
                    SELECT levels.*
                    FROM levels
                    JOIN weekly_demons ON levels.id = weekly_demons.level_id
                    ORDER BY weekly_demons.id DESC
                    "#
                )
                .fetch_all(&pool)
                .await?
            }
            23 => {
                sqlx::query_as!(
                    Level,
                    r#"
                    SELECT levels.*
                    FROM levels
                    JOIN event_levels ON levels.id = event_levels.level_id
                    ORDER BY event_levels.id DESC
                    "#
                )
                .fetch_all(&pool)
                .await?
            }

            25 => {
                let list_levels = sqlx::query!(
                    r#"
                SELECT levels FROM lists WHERE id = $1
                "#,
                    search.parse::<i32>().unwrap_or(0)
                )
                .fetch_one(&pool)
                .await?;

                let level_ids: Vec<i32> = list_levels
                    .levels
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();

                sqlx::query_as!(
                    Level,
                    r#"
                SELECT *
                FROM levels
                WHERE id = ANY($1)
                "#,
                    &level_ids
                )
                .fetch_all(&pool)
                .await?
            }
            27 => {
                sqlx::query_as!(
                    Level,
                    r#"
                SELECT DISTINCT levels.*
                FROM levels
                JOIN suggestions ON suggestions.level_id = levels.id
                "#
                )
                .fetch_all(&pool)
                .await?
            }
            _ => {
                let mut query: QueryBuilder<Postgres> =
                    QueryBuilder::new("SELECT * FROM levels WHERE 1 = 1");

                if coins {
                    query.push(" AND coins = TRUE");
                }

                if epic {
                    query.push(" AND epic_rating = 1");
                }

                if legendary {
                    query.push(" AND epic_rating = 2");
                }

                if featured {
                    query.push(" AND featured = TRUE");
                }

                if two_player {
                    query.push(" AND two_player = TRUE");
                }

                if mythic {
                    query.push(" AND epic_rating = 3");
                }

                if original {
                    query.push(" AND original = TRUE");
                }

                match difficulty[0].as_str() {
                    "-" | "" => (),
                    "-2" => {
                        query.push(" AND demon_difficulty in (");
                        let mut separated = query.separated(", ");
                        separated.push_bind(demon_difficulty);
                        separated.push_unseparated(") ");
                    }
                    _ => {
                        let difficulties: Vec<i16> = difficulty[0]
                            .split(',')
                            .filter_map(|s| s.trim().parse::<i16>().ok())
                            .collect();

                        if !difficulties.is_empty() {
                            query.push(" AND difficulty IN (");
                            let mut separated = query.separated(", ");
                            for difficulty in difficulties {
                                separated.push_bind(difficulty);
                            }
                            separated.push_unseparated(")");
                        }
                    }
                }

                match length.as_str() {
                    "-" | "" => (),
                    _ => {
                        let lengths: Vec<i16> = length
                            .split(',')
                            .filter_map(|s| s.trim().parse::<i16>().ok())
                            .collect();

                        if !lengths.is_empty() {
                            query.push(" AND length IN (");
                            let mut separated = query.separated(", ");
                            for length in lengths {
                                separated.push_bind(length);
                            }
                            separated.push_unseparated(")");
                        }
                    }
                }

                if search_type == 5 {
                    if local {
                        query.push(" AND user_id = ");
                        query.push_bind(user_id);
                    } else {
                        let user_id = search.parse::<i32>().unwrap_or(0);
                        query.push(" AND user_id = ");
                        query.push_bind(user_id);
                    }
                }

                if no_star && star {
                    query.push(" AND rated = FALSE");
                } else if star {
                    query.push(" AND rated = TRUE");
                }

                let level_ids = completed_levels.trim_matches(|c| c == '(' || c == ')');
                let level_ids: Vec<i32> = level_ids
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();

                if only_completed {
                    query.push(" AND id != ANY(");
                    query.push_bind(level_ids);
                    query.push(")");
                } else if uncompleted {
                    query.push(" AND id <> ALL(");
                    query.push_bind(level_ids);
                    query.push(")");
                }

                match search_type {
                    0 => {
                        if !is_numeric(search) {
                            query.push(" AND level_name ILIKE '%' || ");
                            query.push_bind(search);
                            query.push(" || '%'");
                            query.push(" ORDER BY likes DESC")
                        } else {
                            let level_id = search.parse::<i32>().unwrap_or(0);
                            query.push(" AND id = ");
                            query.push_bind(level_id)
                        }
                    }
                    1 => query.push(" ORDER BY downloads DESC"),
                    2 => query.push(" ORDER BY likes DESC"),
                    3 => {
                        query.push(" AND created_at >= NOW() - INTERVAL '14 days'");
                        query.push(" ORDER BY downloads DESC")
                    }
                    4 => query.push(" ORDER BY created_at DESC"),
                    _ => &mut query,
                };

                query.build_query_as().fetch_all(&pool).await?
            }
        };

    if levels.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = levels.len();
    let page_info = format!("{count}:{offset}:10");

    let song_string = String::new();

    let response = [
        level_string(&levels),
        creator_string(&levels),
        song_string,
        page_info,
        generate_hash(&levels),
    ];

    Ok(response.join("#"))
}

fn level_string(levels: &Vec<Level>) -> String {
    let mut level_string = String::new();

    for level in levels {
        level_string.push_str(&level.to_gd());
        level_string.push('|');
    }

    level_string.pop();
    level_string
}

fn creator_string(levels: &Vec<Level>) -> String {
    let mut creator_string = String::new();

    for level in levels {
        let creator_id = level.user_id;
        let creator_name = &level.username;

        let temp = format!("{creator_id}:{creator_name}:{creator_id}");
        creator_string.push_str(&temp);
        creator_string.push('|');
    }

    creator_string.pop();
    creator_string
}

pub fn generate_hash(levels: &Vec<Level>) -> String {
    let mut hash = String::new();

    for level in levels {
        let level_id = level.id.to_string();
        let first = level_id.chars().next().unwrap_or('0');
        let last = level_id.chars().next_back().unwrap_or('0');
        let stars = level.stars.to_string();
        let verified_coins = if level.verified_coins { "1" } else { "0" };

        hash.push(first);
        hash.push(last);
        hash.push_str(&stars);
        hash.push_str(verified_coins);
    }

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
