use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

use crate::GDResponse;

#[derive(Debug, FromRow, Serialize)]
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
    pub likes: i32,
    pub reward: i32,
    pub requirement: i32,
    pub original: i32,
    pub unlisted: i16,
    pub rated: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for List {
    fn to_gd(&self) -> String {
        let rated = if self.rated { 1 } else { 0 };

        let response = [
            format!("1:{}", self.id),
            format!("2:{}", self.list_name),
            format!("3:{}", self.description),
            format!("5:{}", self.version),
            format!("7:{}", self.difficulty),
            format!("10:{}", self.downloads),
            format!("14:{}", self.likes),
            format!("19:{rated}"),
            format!("28:{}", DateTime::timestamp(&self.created_at)),
            format!("29:{}", DateTime::timestamp(&self.updated_at)),
            format!("49:{}", self.user_id),
            format!("50:{}", self.username),
            format!("51:{}", self.levels),
            format!("55:{}", self.reward),
            format!("56:{}", self.requirement),
        ];

        response.join(":")
    }
}

impl List {
    pub async fn get(pool: &PgPool, list_id: i32) -> Result<Self> {
        let list = sqlx::query_as!(Self, "SELECT * FROM lists WHERE id = $1", list_id)
            .fetch_one(pool)
            .await?;

        Ok(list)
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

    pub async fn get_by_name(pool: &PgPool, search: &str) -> Result<Vec<Self>> {
        let lists = sqlx::query_as!(
            Self,
            "SELECT * FROM lists WHERE list_name ILIKE '%' || $1 || '%'",
            search
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
