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

    pub async fn get_user_id(pool: &PgPool, username: &str) -> Result<i32> {
        let user_id: i32 =
            sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", username)
                .fetch_one(pool)
                .await?;

        Ok(user_id)
    }
    pub async fn verify_password(pool: &PgPool, user_id: i32, gjp2: &str) -> Result<bool> {
        let stored_gjp2: String =
            sqlx::query_scalar!("SELECT gjp2 FROM users WHERE id = $1", user_id)
                .fetch_one(pool)
                .await?;

        if gjp2 == stored_gjp2 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
