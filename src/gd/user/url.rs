use axum::Form;
use serde::{Deserialize, Serialize};

use crate::AppError;

#[derive(Deserialize, Serialize, Debug)]
pub struct UrlForm {
    accountID: i32,
    secret: String,

    #[serde(default)]
    #[serde(rename = "type")]
    endpoint_type: u8,
}

pub async fn url(Form(_form): Form<UrlForm>) -> Result<&'static str, AppError> {
    Ok("http://127.0.0.1")
}
