use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    AppError, GDResponse,
    models::Level,
    util::{is_numeric, salt_and_sha1},
};

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct SearchForm {
    accountID: i32,
    binaryVersion: i16,
    coins: i16,
    #[serde(default)]
    completedLevels: String,
    #[serde(default)]
    customSong: i16,
    #[serde(default)]
    demonFilter: i16,
    diff: String,
    #[serde(default)]
    epic: i16,
    featured: i16,
    #[serde(default)]
    followed: String,
    gameVersion: i16,
    #[serde(default)]
    gauntlet: i32,
    gjp2: String,
    #[serde(default)]
    legendary: i16,
    len: String,
    #[serde(default)]
    local: i16,
    #[serde(default)]
    mythic: i16,
    #[serde(default)]
    noStar: i16,
    onlyCompleted: i16,
    original: i16,
    page: i32,
    secret: String,
    #[serde(default)]
    song: i32,
    #[serde(default)]
    star: i16,
    str: String,
    total: i32,
    twoPlayer: i16,
    uncompleted: i16,
    #[serde(rename = "type")]
    search_type: i16,
}

pub async fn search_levels(
    State(pool): State<PgPool>,
    Form(form): Form<SearchForm>,
) -> Result<String, AppError> {
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

    let levels = match search_type {
        7 => {
            let levels = sqlx::query_as!(
                Level,
                "SELECT * FROM levels WHERE objects > 9999 ORDER BY created_at DESC"
            )
            .fetch_all(&pool)
            .await?;

            levels
        }
        11 => {
            let levels = sqlx::query_as!(
                Level,
                "SELECT * FROM levels WHERE rated = 1 ORDER BY rated_at DESC, created_at DESC"
            )
            .fetch_all(&pool)
            .await?;

            levels
        }
        12 => {
            let user_ids: Vec<i32> = followed
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            let levels = sqlx::query_as!(
                Level,
                r#"
                SELECT * FROM levels
                WHERE user_id = ANY($1)
                ORDER BY created_at DESC
                "#,
                &user_ids
            )
            .fetch_all(&pool)
            .await?;

            levels
        }
        13 => {
            let levels = sqlx::query_as!(
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
            .await?;

            levels
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

            let levels = sqlx::query_as!(
                Level,
                r#"
                SELECT *
                FROM levels
                WHERE id = ANY($1)
                "#,
                &level_ids
            )
            .fetch_all(&pool)
            .await?;

            levels
        }
        27 => {
            let levels = sqlx::query_as!(
                Level,
                r#"
                SELECT DISTINCT levels.*
                FROM levels
                JOIN suggestions ON suggestions.level_id = levels.id
                "#
            )
            .fetch_all(&pool)
            .await?;

            levels
        }
        _ => {
            let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM levels");

            query.push(" WHERE COINS = ");
            query.push_bind(coins);

            query.push(" AND epic = ");
            query.push_bind(epic);

            query.push(" AND legendary = ");
            query.push_bind(legendary);

            query.push(" AND featured = ");
            query.push_bind(featured);

            query.push(" AND two_player = ");
            query.push_bind(two_player);

            query.push(" AND mythic = ");
            query.push_bind(mythic);

            query.push(" AND original = ");
            query.push_bind(original);

            match difficulty.as_str() {
                "-" | "" => (),
                "-2" => {
                    query.push(" AND demon_difficulty in (");
                    let mut separated = query.separated(", ");
                    separated.push_bind(demon_difficulty);
                    separated.push_unseparated(") ");
                }
                _ => {
                    let difficulties: Vec<i16> = difficulty
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
                if local == 1 {
                    query.push(" AND user_id = ");
                    query.push_bind(user_id);
                } else {
                    let user_id = search.parse::<i32>().unwrap_or(0);
                    query.push(" AND user_id = ");
                    query.push_bind(user_id);
                }
            }

            if no_star == 1 && star == 0 {
                query.push(" AND rated = 0");
            } else if star == 1 {
                query.push(" AND rated = 1");
            }

            let level_ids = completed_levels.trim_matches(|c| c == '(' || c == ')');
            let level_ids: Vec<i32> = level_ids
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            if only_completed == 1 {
                query.push(" AND id = ANY(");
                query.push_bind(level_ids);
                query.push(")");
            } else if uncompleted == 1 {
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
                        query.push(" ORDER BY downloads DESC")
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

            let levels = query.build_query_as().fetch_all(&pool).await?;

            levels
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
        let verified_coins = level.verified_coins.to_string();
        hash.push(first);
        hash.push(last);
        hash.push_str(&stars);
        hash.push_str(&verified_coins);
    }

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
