use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

use crate::GDResponse;

#[derive(Debug, FromRow, Serialize)]
pub struct MapPack {
    pub id: i32,
    pub pack_name: String,
    pub levels: String,
    pub stars: i32,
    pub coins: i16,
    pub difficulty: i16,
    pub text_color: String,
    pub bar_color: String,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for MapPack {
    fn to_gd(&self) -> String {
        let response = [
            format!("1:{}", self.id),
            format!("2:{}", self.pack_name),
            format!("3:{}", self.levels),
            format!("4:{}", self.stars),
            format!("5:{}", self.coins),
            format!("6:{}", self.difficulty),
            format!("7:{}", self.text_color),
            format!("8:{}", self.bar_color),
        ];

        response.join(":")
    }
}

impl MapPack {
    pub async fn get(pool: &PgPool) -> Result<Vec<Self>> {
        let map_packs = sqlx::query_as!(Self, "SELECT * FROM map_packs")
            .fetch_all(pool)
            .await?;

        Ok(map_packs)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct Gauntlet {
    pub id: i32,
    pub levels: String,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for Gauntlet {
    fn to_gd(&self) -> String {
        format!("1:{}:3:{}", self.id, self.levels)
    }
}

impl Gauntlet {
    pub async fn get(pool: &PgPool) -> Result<Vec<Self>> {
        let gauntlets = sqlx::query_as!(Self, "SELECT * FROM gauntlets")
            .fetch_all(pool)
            .await?;

        Ok(gauntlets)
    }
}
