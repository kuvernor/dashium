use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {}

impl User {
    pub async fn register(pool: &PgPool, username: &str, gjp2: &str, email: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users (username, gjp2, email) VALUES ($1, $2, $3)",
            username,
            gjp2,
            email
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
