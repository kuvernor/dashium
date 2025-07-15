use axum::Form;
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

use crate::{
    AppError,
    util::{base64_decode, base64_encode, cyclic_xor, salt_and_sha1},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    accountID: i32,
    chk: String,
    secret: String,
    udid: String,
}

pub async fn get_quests(Form(form): Form<GetForm>) -> Result<String, AppError> {
    let user_id = &form.accountID.to_string();
    let udid = &form.udid;
    let chk = &form.chk[5..];
    let key = b"19847";
    let chk = &cyclic_xor(&base64_decode(chk)?, key)?;

    let now = Local::now();
    let time_left = (now + Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
        .signed_duration_since(now)
        .num_seconds()
        .to_string();

    let quest1 = "1,1,500,10,Orb Quest".to_string();
    let quest2 = "2,2,6,20,Coin Quest".to_string();
    let quest3 = "3,3,10,30,Star Quest".to_string();

    let list =
        format!("quest:{user_id}:{chk}:{udid}:{user_id}:{time_left}:{quest1}:{quest2}:{quest3}");

    let list = base64_encode(&cyclic_xor(list.as_bytes(), key)?);
    let hash = salt_and_sha1(&list, "oC36fpYaPtdg");

    let response = format!("quest{}|{}", &list, &hash);
    Ok(response)
}
