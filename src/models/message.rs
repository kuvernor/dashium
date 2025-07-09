use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use sqlx::{PgPool, prelude::FromRow};

use crate::models::User;

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct Message {
    id: i32,
    sender_id: i32,
    subject: String,
    body: String,
    username: String,
    is_read: i16,
    is_sender: i16,
    recipient_id: i32,
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn to_gd(message: Self) -> String {
        let response: Vec<String> = vec![
            format!("1:{}", message.id),
            format!("2:{}", message.sender_id),
            format!("3:{}", message.sender_id),
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
        sender_id: i32,
        recipient_id: i32,
        subject: &str,
        body: &str,
    ) -> Result<()> {
        let username = User::username_from_id(pool, recipient_id).await?;

        sqlx::query_scalar!(
            "INSERT INTO messages (sender_id, subject, body, username, recipient_id) VALUES ($1, $2, $3, $4, $5)",
            sender_id,
            subject,
            body,
            username,
            recipient_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn download(pool: &PgPool, message_id: i32, sender_id: i32) -> Result<Self> {
        let mut message = sqlx::query_as!(
            Self,
            "SELECT * FROM messages WHERE id = $1 AND sender_id = $2",
            message_id,
            sender_id
        )
        .fetch_one(pool)
        .await?;

        message.is_read = 1;
        message.is_sender = 0;

        Ok(message)
    }

    pub async fn get_messages(pool: &PgPool, sender_id: i32) -> Result<Vec<Self>> {
        let mut messages = sqlx::query_as!(
            Self,
            "SELECT * from messages WHERE recipient_id = $1 ORDER BY created_at DESC",
            sender_id
        )
        .fetch_all(pool)
        .await?;

        for message in &mut messages {
            message.is_sender = 0;
        }

        Ok(messages)
    }

    pub async fn get_sent_messages(pool: &PgPool, sender_id: i32) -> Result<Vec<Self>> {
        let mut messages = sqlx::query_as!(
            Self,
            "SELECT * FROM messages WHERE sender_id = $1 ORDER BY created_at DESC",
            sender_id
        )
        .fetch_all(pool)
        .await?;

        for message in &mut messages {
            message.is_sender = 1;
        }

        Ok(messages)
    }

    pub async fn delete(
        pool: &PgPool,
        sender_id: i32,
        message_id: i32,
        is_sender: bool,
    ) -> Result<()> {
        if is_sender {
            sqlx::query!(
                "DELETE FROM messages WHERE id = $1 AND sender_id = $2",
                message_id,
                sender_id
            )
            .execute(pool)
            .await?;
        } else {
            sqlx::query!(
                "DELETE FROM messages WHERE id = $1 AND recipient_id = $2",
                message_id,
                sender_id
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
