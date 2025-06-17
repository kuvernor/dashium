use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use gd_response_derive::GDResponse;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, Debug, FromRow, GDResponse)]
pub struct User {}

impl User {
    pub fn generate_gjp2(password: &str) -> String {
        let salt = "mI29fmAnxgTs";
        let salted_password = password.to_owned() + salt;

        let mut hasher = Sha1::new();
        hasher.update(salted_password);
        let hashed = hasher.finalize();

        format!("{:x}", hashed)
    }

    /// Hashes a GJP2 with Argon2
    pub fn hash_gjp2(gjp2: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(gjp2.as_bytes(), &salt)
            .expect("Failed to hash")
            .to_string();
        Ok(password_hash)
    }

    pub async fn verify_gjp2(pool: &PgPool, user_id: i32, gjp2: &str) -> Result<bool> {
        let argon2 = Argon2::default();
        let hash: String = sqlx::query_scalar!("SELECT hash FROM users WHERE id = $1", user_id)
            .fetch_one(pool)
            .await?;

        let hash = PasswordHash::new(&hash).expect("Failed to parse hash");

        Ok(argon2.verify_password(gjp2.as_bytes(), &hash).is_ok())
    }

    pub async fn register(
        pool: &PgPool,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<()> {
        let gjp2 = User::generate_gjp2(password);
        let hash = User::hash_gjp2(&gjp2)?;

        sqlx::query!(
            "INSERT INTO users (username, hash, email) VALUES ($1, $2, $3)",
            username,
            hash,
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

    pub async fn save_data(pool: &PgPool, user_id: i32, save_data: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET save_data = $1
            WHERE id = $2
            "#,
            save_data,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn load_data(pool: &PgPool, user_id: i32) -> Result<String> {
        let save_data = sqlx::query_scalar!("SELECT save_data FROM users WHERE id = $1", user_id)
            .fetch_one(pool)
            .await?;

        Ok(save_data)
    }

    pub fn to_gd(&self) -> String {
        self.to_gd_response(":")
    }
}
