use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use sha1::{Digest, Sha1};
use sqlx::PgPool;

pub fn is_ascii_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

pub fn salt_and_sha1(s: &str, salt: &str) -> String {
    let salted = s.to_owned() + salt;

    let mut hasher = Sha1::new();
    hasher.update(salted);
    let hashed = hasher.finalize();

    format!("{hashed:x}")
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
    let hash = match sqlx::query_scalar!("SELECT hash FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await
    {
        Ok(hash) => hash,
        Err(_) => return Ok(false),
    };

    let hash = PasswordHash::new(&hash).expect("Failed to parse hash");

    Ok(argon2.verify_password(gjp2.as_bytes(), &hash).is_ok())
}

pub fn base64_encode(input: &str) -> String {
    URL_SAFE.encode(input)
}

pub fn base64_decode(input: &str) -> Result<String> {
    Ok(String::from_utf8(URL_SAFE.decode(input)?)?)
}
