use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, FromRow)]
#[allow(dead_code)]
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

impl MapPack {
    pub fn to_gd(map_pack: &Self) -> String {
        let response = [
            format!("1:{}", map_pack.id),
            format!("2:{}", map_pack.pack_name),
            format!("3:{}", map_pack.levels),
            format!("4:{}", map_pack.stars),
            format!("5:{}", map_pack.coins),
            format!("6:{}", map_pack.difficulty),
            format!("7:{}", map_pack.text_color),
            format!("8:{}", map_pack.bar_color),
        ];

        response.join(":")
    }

    pub async fn get(pool: &PgPool) -> Result<Vec<Self>> {
        let map_packs = sqlx::query_as!(Self, "SELECT * FROM map_packs")
            .fetch_all(pool)
            .await?;

        Ok(map_packs)
    }
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct Gauntlet {
    pub id: i32,
    pub levels: String,
    pub created_at: DateTime<Utc>,
}

impl Gauntlet {
    pub fn to_gd(gauntlet: &Self) -> String {
        format!("1:{}:3:{}", gauntlet.id, gauntlet.levels)
    }

    pub async fn get(pool: &PgPool) -> Result<Vec<Self>> {
        let gauntlets = sqlx::query_as!(Self, "SELECT * FROM gauntlets")
            .fetch_all(pool)
            .await?;

        Ok(gauntlets)
    }
}
