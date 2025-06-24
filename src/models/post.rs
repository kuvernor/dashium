use anyhow::Result;
use chrono::NaiveDateTime;
use gd_response_derive::GDResponse;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use std::cmp::Ordering;

use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, FromRow, GDResponse)]
pub struct Post {
    #[response(2)]
    post: String,
    #[response(3)]
    user_id: i32,
    #[response(4)]
    likes: i32,
    #[response(6)]
    id: i32,
    #[response(7)]
    is_spam: i16,
    #[response(9)]
    created_at: NaiveDateTime,
}

impl Post {
    pub async fn to_gd(pool: &PgPool, user_id: i32, page: i32) -> Result<String> {
        let mut posts = sqlx::query_as!(
            Self,
            r#"
            SELECT
            id,
            user_id,
            post,
            likes, 
            created_at,
            is_spam
            FROM posts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        let offset = page * 10;
        let count = posts.len();
        let end_string = format!("#{}:{}", count, offset);

        // sort newest first
        posts.reverse();

        let mut response = String::new();
        for post in posts {
            let x = post.to_gd_response("~");

            response.push_str(&x);
            response.push('|');
        }

        response.pop();
        response.push_str(&end_string);

        Ok(response)
    }

    pub async fn upload(pool: &PgPool, post: &str, user_id: i32, gjp2: &str) -> Result<String> {
        if !verify_gjp2(pool, user_id, gjp2).await? {
            return Ok("".to_string());
        }
        let post = sqlx::query_scalar!(
            "INSERT INTO posts (post, user_id, account_id) VALUES ($1, $2, $3) RETURNING id",
            post,
            user_id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(post.to_string())
    }

    pub async fn delete(pool: &PgPool, comment_id: i32, user_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM posts WHERE id = $1 and user_id = $2",
            comment_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
