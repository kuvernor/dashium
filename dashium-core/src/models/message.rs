use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

use crate::GDResponse;

#[derive(Debug, FromRow, Serialize)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub subject: String,
    pub body: String,
    pub username: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for Message {
    fn to_gd(&self) -> String {
        let is_read = if self.is_read { "1" } else { "0" };

        let response: Vec<String> = vec![
            format!("1:{}", self.id),
            format!("2:{}", self.sender_id),
            format!("3:{}", self.sender_id),
            format!("4:{}", self.subject),
            format!("6:{}", self.username),
            format!("7:{}", HumanTime::from(self.created_at)).replace(" ago", ""),
            format!("8:{is_read}"),
        ];

        response.join(":")
    }
}

impl Message {
    pub async fn send(
        pool: &PgPool,
        sender_id: i32,
        recipient_id: i32,
        subject: &str,
        body: &str,
    ) -> Result<()> {
        sqlx::query_scalar!(
            "INSERT INTO messages (sender_id, recipient_id, subject, body) VALUES ($1, $2, $3, $4)",
            sender_id,
            recipient_id,
            subject,
            body
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn download(pool: &PgPool, message_id: i32, recipient_id: i32) -> Result<Self> {
        let message = sqlx::query_as!(
            Self,
            r#"
            WITH updated AS (
                UPDATE messages
                SET is_read = TRUE
                WHERE id = $1 AND recipient_id = $2
                RETURNING *
            )
            SELECT updated.*, u.username
            FROM updated
            JOIN users u ON updated.sender_id = u.id
            "#,
            message_id,
            recipient_id
        )
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    pub async fn get_messages(pool: &PgPool, sender_id: i32, get_sent: bool) -> Result<Vec<Self>> {
        let messages = if get_sent {
            sqlx::query_as!(
                Self,
                r#"
                SELECT m.* ,u.username
                FROM messages m
                JOIN users u ON m.recipient_id = u.id
                WHERE m.sender_id = $1
                ORDER BY m.created_at DESC
                "#,
                sender_id
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                Self,
                r#"
            SELECT
                m.id,
                m.sender_id,
                m.recipient_id,
                m.subject,
                m.body,
                m.created_at,
                m.is_read,
                u.username
            FROM messages m
            JOIN users u ON m.sender_id = u.id
            WHERE m.recipient_id = $1
            ORDER BY m.created_at DESC
            "#,
                sender_id
            )
            .fetch_all(pool)
            .await?
        };

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
