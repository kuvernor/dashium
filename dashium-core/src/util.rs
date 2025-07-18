use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use chrono::{Datelike, Duration, Local, Weekday};
use serde::{Deserialize, Deserializer};
use sha1::{Digest, Sha1};
use sqlx::PgPool;

/// Returns `true` if a string is ASCII alphanumeric.
pub fn is_ascii_alphanumeric(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Returns `true` if a string is numeric.
pub fn is_numeric(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_digit())
}

/// Appends a given salt to the string and returns the SHA1 hash.
pub fn salt_and_sha1(input: &str, salt: &str) -> String {
    let salted = input.to_owned() + salt;

    let mut hasher = Sha1::new();
    hasher.update(salted);
    let hashed = hasher.finalize();

    format!("{hashed:x}")
}

/// Hashes a GJP2 with Argon2.
pub fn hash_gjp2(gjp2: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(gjp2.as_bytes(), &salt)
        .expect("Failed to hash")
        .to_string();
    Ok(password_hash)
}

/// Returns `true` if the given GJP2 matches the one stored in the database.
pub async fn verify_gjp2(pool: &PgPool, user_id: i32, gjp2: &str) -> Result<bool> {
    let argon2 = Argon2::default();
    let hash = sqlx::query_scalar!("SELECT hash FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await?;

    let hash = PasswordHash::new(&hash).expect("Failed to parse hash");

    Ok(argon2.verify_password(gjp2.as_bytes(), &hash).is_ok())
}

/// Encodes a string as Base64.
pub fn base64_encode(input: &str) -> String {
    URL_SAFE.encode(input)
}

/// Decodes a string from Base64.
pub fn base64_decode(input: &str) -> Result<Vec<u8>> {
    Ok(URL_SAFE.decode(input)?)
}

/// Computes the cyclic XOR for the input and returns it as a String.
pub fn cyclic_xor(data: &[u8], key: &[u8]) -> Result<String> {
    let xor: Vec<u8> = data
        .iter()
        .zip(key.iter().cycle())
        .map(|(b, k)| b ^ k)
        .collect();

    Ok(String::from_utf8(xor)?)
}

/// Returns the time in seconds to the next midnight (00:00).
pub fn time_until_midnight() -> String {
    let now = Local::now();
    (now + Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .signed_duration_since(now)
        .num_seconds()
        .to_string()
}

/// Returns the time in seconds to the next Monday midnight (00:00).
pub fn time_until_monday() -> String {
    let now = Local::now();

    let days = match now.weekday() {
        Weekday::Mon => 7,
        Weekday::Tue => 6,
        Weekday::Wed => 5,
        Weekday::Thu => 4,
        Weekday::Fri => 3,
        Weekday::Sat => 2,
        Weekday::Sun => 1,
    };

    (now.date_naive() + Duration::days(days as i64))
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .signed_duration_since(now)
        .num_seconds()
        .to_string()
}

/// Deserializes a `0` or `1` as a `bool` with the `#[serde(deserialize_with = "int_to_bool")]` attribute.
pub fn int_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let num: u8 = Deserialize::deserialize(deserializer)?;
    match num {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::custom("number has to be 0 or 1")),
    }
}
