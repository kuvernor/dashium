pub fn is_ascii_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}
