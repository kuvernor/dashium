use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sha1::{Digest, Sha1};
use sqlx::PgPool;

pub fn is_ascii_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

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
