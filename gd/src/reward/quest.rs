use axum::Form;

use serde::{Deserialize, Serialize};

use crate::{
    AppError,
    util::{base64_decode, base64_encode, cyclic_xor, salt_and_sha1, time_until_midnight},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct getGJChallenges {
    accountID: i32,
    chk: String,
    secret: String,
    udid: String,
}

pub async fn getGJChallenges(Form(form): Form<getGJChallenges>) -> Result<String, AppError> {
    let user_id = &form.accountID.to_string();
    let udid = &form.udid;
    let chk = &cyclic_xor(&base64_decode(&form.chk[5..])?, b"19847")?;

    let time_left = time_until_midnight();

    let quest1 = "1,1,500,10,Orb Quest".to_string();
    let quest2 = "2,2,6,20,Coin Quest".to_string();
    let quest3 = "3,3,10,30,Star Quest".to_string();

    let list =
        format!("quest:{user_id}:{chk}:{udid}:{user_id}:{time_left}:{quest1}:{quest2}:{quest3}");

    let list = base64_encode(&cyclic_xor(list.as_bytes(), b"19847")?);
    let hash = salt_and_sha1(&list, "oC36fpYaPtdg");

    let response = format!("quest{}|{}", &list, &hash);
    Ok(response)
}
