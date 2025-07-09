use crate::models::User;
use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
#[allow(unused)]
pub struct FriendRequest {
    id: i32,
    sender_id: i32,
    recipient_id: i32,
    body: String,
    created_at: DateTime<Utc>,
    is_new: i16,
}

impl FriendRequest {
    pub async fn to_gd(pool: &PgPool, friend_request: Self) -> Result<String> {
        let user = User::get_user(pool, friend_request.sender_id).await?;

        let response = vec![
            format!("1:{}", user.username),
            format!("2:{}", friend_request.sender_id),
            format!("9:{}", user.icon),
            format!("10:{}", user.color1),
            format!("11:{}", user.color2),
            format!("14:{}", user.icon_type),
            format!("15:{}", user.glow),
            format!("16:{}", friend_request.sender_id),
            format!("32:{}", friend_request.id),
            format!("35:{}", friend_request.body),
            format!("37:{}", HumanTime::from(friend_request.created_at)).replace(" ago", ""),
            format!("41:{}", friend_request.is_new),
        ];
        Ok(response.join(":"))
    }

    pub async fn get_all(pool: &PgPool, sender_id: i32) -> Result<Vec<Self>> {
        let friend_requests = sqlx::query_as!(
            Self,
            "SELECT * from friend_requests WHERE recipient_id = $1 ORDER BY created_at DESC",
            sender_id
        )
        .fetch_all(pool)
        .await?;

        Ok(friend_requests)
    }

    pub async fn get_all_sent(pool: &PgPool, sender_id: i32) -> Result<Vec<Self>> {
        let friend_requests = sqlx::query_as!(
            Self,
            "SELECT * FROM friend_requests WHERE sender_id = $1 ORDER BY created_at DESC",
            sender_id,
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
    id: i32,
    user1: i32,
    pub user2: i32,
    is_new1: i16,
    is_new2: i16,
    created_at: DateTime<Utc>,
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
    id: i32,
    blocker_id: i32,
    pub blocked_id: i32,
    created_at: DateTime<Utc>,
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
