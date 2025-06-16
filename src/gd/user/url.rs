use axum::Form;
use serde::{Deserialize, Serialize};

use crate::AppError;

#[derive(Deserialize, Serialize, Debug)]
pub struct UrlForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    secret: Option<String>,
    #[serde(rename = "type")]
    endpoint_type: Option<u8>,
}

pub async fn url(Form(_form): Form<UrlForm>) -> Result<String, AppError> {
    Ok("http://127.0.0.1".to_string())
}
