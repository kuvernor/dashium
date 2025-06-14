use sha1::{Digest, Sha1};

pub fn hash_gjp2(password: &str) -> String {
    let salt = "mI29fmAnxgTs";
    let salted = password.to_owned() + salt;

    let mut hasher = Sha1::new();
    hasher.update(salted);
    let hashed = hasher.finalize();
    format!("{:x}", hashed)
}

pub fn is_ascii_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}
