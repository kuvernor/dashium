use crate::GDResponse;
use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct FriendRequest {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub is_new: i16,
    pub username: String,
    pub icon: i16,
    pub color1: i16,
    pub color2: i16,
    pub icon_type: i16,
    pub glow: i16,
}

impl GDResponse for FriendRequest {
    fn to_gd(&self) -> String {
        let response = vec![
            format!("1:{}", self.username),
            format!("2:{}", self.sender_id),
            format!("9:{}", self.icon),
            format!("10:{}", self.color1),
            format!("11:{}", self.color2),
            format!("14:{}", self.icon_type),
            format!("15:{}", self.glow),
            format!("16:{}", self.sender_id),
            format!("32:{}", self.id),
            format!("35:{}", self.body),
            format!("37:{}", HumanTime::from(self.created_at)).replace(" ago", ""),
            format!("41:{}", self.is_new),
        ];

        response.join(":")
    }
}

impl FriendRequest {
    pub async fn get_all(pool: &PgPool, sender_id: i32) -> Result<Vec<Self>> {
        let friend_requests = sqlx::query_as!(
            Self,
            r#"
            SELECT
                fr.id,
                fr.sender_id,
                fr.recipient_id,
                fr.body,
                fr.created_at,
                fr.is_new,
                u.username,
                u.icon,
                u.color1,
                u.color2,
                u.icon_type,
                u.glow
            FROM friend_requests fr
            JOIN users u ON u.id = fr.sender_id
            WHERE fr.recipient_id = $1
            ORDER BY fr.created_at DESC
            "#,
            sender_id
        )
        .fetch_all(pool)
        .await?;

        Ok(friend_requests)
    }

    pub async fn get_all_sent(pool: &PgPool, sender_id: i32) -> Result<Vec<Self>> {
        let friend_requests = sqlx::query_as!(
            Self,
            r#"
            SELECT
                fr.id,
                fr.sender_id,
                fr.recipient_id,
                fr.body,
                fr.created_at,
                fr.is_new,
                u.username,
                u.icon,
                u.color1,
                u.color2,
                u.icon_type,
                u.glow
            FROM friend_requests fr
            JOIN users u ON u.id = fr.sender_id
            WHERE fr.sender_id = $1
            ORDER BY fr.created_at DESC
            "#,
            sender_id
        )
        .fetch_all(pool)
        .await?;

        Ok(friend_requests)
    }

    pub async fn send(pool: &PgPool, sender_id: i32, recipient_id: i32, body: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO friend_requests (sender_id, recipient_id, body) VALUES ($1, $2, $3)",
            sender_id,
            recipient_id,
            body
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(
        pool: &PgPool,
        sender_id: i32,
        recipient_id: i32,
        is_sender: bool,
    ) -> Result<()> {
        if is_sender {
            sqlx::query!(
                "DELETE FROM friend_requests WHERE sender_id = $1 AND recipient_id = $2",
                sender_id,
                recipient_id
            )
            .execute(pool)
            .await?;
        } else {
            sqlx::query!(
                "DELETE FROM friend_requests WHERE sender_id = $1 AND recipient_id = $2",
                recipient_id,
                sender_id
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn exists(pool: &PgPool, sender_id: i32, recipient_id: i32) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM friend_requests WHERE sender_id = $1 AND recipient_id = $2 OR recipient_id = $2 AND sender_id = $1) AS \"exists!\"",
            sender_id,
            recipient_id
        ).fetch_one(pool).await?;

        Ok(exists)
    }
}

#[derive(Debug, FromRow)]
#[allow(unused)]
pub struct Friendship {
    pub id: i32,
    pub user1: i32,
    pub user2: i32,
    pub is_new1: i16,
    pub is_new2: i16,
    pub created_at: DateTime<Utc>,
}

impl Friendship {
    pub async fn get_all(pool: &PgPool, user1: i32) -> Result<Vec<Self>> {
        let friends = sqlx::query_as!(
            Self,
            "SELECT * FROM friendships WHERE user1 = $1 ORDER BY created_at DESC",
            user1
        )
        .fetch_all(pool)
        .await?;

        Ok(friends)
    }

    pub async fn create(pool: &PgPool, user1: i32, user2: i32) -> Result<()> {
        sqlx::query!(
            "INSERT INTO friendships (user1, user2) VALUES ($1, $2)",
            user1,
            user2,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, sender_id: i32, recipient_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM friendships WHERE user1 = $1 AND user2 = $2",
            sender_id,
            recipient_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn exists(pool: &PgPool, sender_id: i32, recipient_id: i32) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM friendships WHERE user1 = $1 AND user2 = $2 OR user2 = $2 AND user1 = $1) AS \"exists!\"",
            sender_id,
            recipient_id
        ).fetch_one(pool).await?;

        Ok(exists)
    }
}

#[derive(Debug, FromRow)]
#[allow(unused)]
pub struct Block {
    pub id: i32,
    pub blocker_id: i32,
    pub blocked_id: i32,
    pub created_at: DateTime<Utc>,
}

impl Block {
    pub async fn get_all(pool: &PgPool, blocker_id: i32) -> Result<Vec<Self>> {
        let blocks = sqlx::query_as!(
            Block,
            "SELECT * FROM blocks WHERE blocker_id = $1 ORDER BY created_at DESC",
            blocker_id
        )
        .fetch_all(pool)
        .await?;

        Ok(blocks)
    }

    pub async fn block_user(pool: &PgPool, blocker_id: i32, blocked_id: i32) -> Result<()> {
        sqlx::query!(
            "INSERT INTO blocks (blocker_id, blocked_id) VALUES ($1, $2)",
            blocker_id,
            blocked_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn unblock_user(pool: &PgPool, blocker_id: i32, blocked_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM blocks WHERE blocker_id = $1 AND blocked_id = $2",
            blocker_id,
            blocked_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn exists(pool: &PgPool, blocker_id: i32, blocked_id: i32) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM blocks WHERE blocker_id = $1 AND blocked_id = $2) AS \"exists!\"",
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn is_blocked(pool: &PgPool, blocked_id: i32, blocker_id: i32) -> Result<bool> {
        let is_blocked = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM blocks WHERE blocked_id = $1 AND blocker_id = $2) AS \"is_blocked!\"",
            blocker_id,
            blocked_id
        )
        .fetch_one(pool)
        .await?;

        Ok(is_blocked)
    }
}
