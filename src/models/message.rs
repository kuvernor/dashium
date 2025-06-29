use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_id: i32,
    user_id: i32,
    subject: String,
    body: String,
    username: String,
    is_read: i16,
    is_sender: i16,
    target_id: i32,
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn to_gd(message: Self) -> String {
        let response: Vec<String> = vec![
            format!("1:{}", message.message_id),
            format!("2:{}", message.user_id),
            format!("3:{}", message.user_id),
            format!("4:{}", message.subject),
            format!("5:{}", message.body),
            format!("6:{}", message.username),
            format!("7:{}", HumanTime::from(message.created_at)).replace(" ago", ""),
            format!("8:{}", message.is_read),
            format!("9:{}", message.is_sender),
        ];

        response.join(":")
    }

    pub async fn send(
        pool: &PgPool,
        user_id: i32,
        target_id: i32,
        subject: &str,
        body: &str,
    ) -> Result<()> {
        let is_sender = 1;
        let username = User::get_username(pool, target_id).await?;

        sqlx::query_scalar!(
            "INSERT INTO messages (user_id, subject, body, username, is_sender, target_id) VALUES ($1, $2, $3, $4, $5, $6)",
            user_id,
            subject,
            body,
            username,
            is_sender,
            target_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn download(pool: &PgPool, message_id: i32, user_id: i32) -> Result<Self> {
        let mut message = sqlx::query_as!(
            Self,
            "SELECT * FROM messages WHERE message_id = $1 AND user_id = $2",
            message_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        message.is_read = 1;

        Ok(message)
    }

    pub async fn get_messages(pool: &PgPool, user_id: i32) -> Result<Vec<Self>> {
        let messages =
            sqlx::query_as!(Self, "SELECT * from messages WHERE target_id = $1", user_id)
                .fetch_all(pool)
                .await?;

        Ok(messages)
    }

    pub async fn get_sent_messages(pool: &PgPool, user_id: i32) -> Result<Vec<Self>> {
        let messages = sqlx::query_as!(
            Self,
            "SELECT * FROM messages WHERE user_id = $1 AND is_sender = 1",
            user_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    pub async fn delete(pool: &PgPool, message_id: i32, user_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM messages WHERE message_id = $1 AND user_id = $2",
            message_id,
            user_id
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            "DELETE FROM messages WHERE message_id = $1 AND target_id = $2",
            message_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
