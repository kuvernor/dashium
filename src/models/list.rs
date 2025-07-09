use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct List {
    pub id: i32,
    pub list_name: String,
    pub user_id: i32,
    pub username: String,
    pub levels: String,
    pub description: String,
    pub version: i32,
    pub difficulty: i16,
    pub downloads: i32,
    pub reward: i32,
    pub requirement: i32,
    pub original: i32,
    pub unlisted: i16,
    pub likes: i32,
    pub is_rated: i16,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl List {
    pub fn to_gd(list: &Self) -> String {
        let response = [
            format!("1:{}", list.id),
            format!("2:{}", list.list_name),
            format!("3:{}", list.description),
            format!("5:{}", list.version),
            format!("7:{}", list.difficulty),
            format!("10:{}", list.downloads),
            format!("14:{}", list.likes),
            format!("19:{}", list.is_rated),
            format!("28:{}", DateTime::timestamp(&list.created_at)),
            format!("29:{}", DateTime::timestamp(&list.updated_at)),
            format!("49:{}", list.user_id),
            format!("50:{}", list.username),
            format!("51:{}", list.levels),
            format!("55:{}", list.reward),
            format!("56:{}", list.requirement),
        ];

        response.join(":")
    }

    pub async fn get_all(pool: &PgPool, search: &str) -> Result<Vec<Self>> {
        let list_id = search.parse::<i32>().unwrap_or_default();

        let lists = sqlx::query_as!(
            Self,
            "SELECT * FROM lists WHERE list_name ILIKE '%' || $1 || '%' OR id = $2",
            search,
            list_id
        )
        .fetch_all(pool)
        .await?;

        Ok(lists)
    }

    pub async fn delete(pool: &PgPool, user_id: i32, list_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM lists WHERE user_id = $1 AND id = $2",
            user_id,
            list_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
