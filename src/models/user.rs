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

    pub async fn is_username_taken(pool: &PgPool, username: &str) -> Result<bool> {
        let exists: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) AS \"exists!\"",
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn is_email_taken(pool: &PgPool, email: &str) -> Result<bool> {
        let exists: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) AS \"exists!\"",
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }
}
