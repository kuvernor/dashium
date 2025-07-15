use axum::{Form, extract::State};
use serde::Serialize;
use sqlx::PgPool;

use crate::AppError;
use crate::util::verify_gjp2;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;

#[derive(Serialize, Debug, DeserializeFirstDuplicate)]
pub struct UpdateForm {
    accountID: i32,
    userName: String,
    gjp2: String,
    seed: String,
    seed2: String,
    stars: i32,
    moons: i32,
    demons: i32,
    diamonds: i32,
    coins: i32,
    userCoins: i32,
    color1: i16,
    color2: i16,
    color3: i16,
    special: i16,
    iconType: i16,
    icon: i16,
    accIcon: i16,
    accShip: i16,
    accBall: i16,
    accBird: i16,
    accDart: i16,
    accRobot: i16,
    accSpider: i16,
    accExplosion: i16,
    accSwing: i16,
    accJetpack: i16,
    accGlow: i16,
    gameVersion: i16,
    binaryVersion: i16,
    sinfo: String,
    sinfod: i32,
    sinfog: i32,
    sinfoe: i32,

    #[serde(default)]
    dinfo: String,
    #[serde(default)]
    dinfow: i32,
    #[serde(default)]
    dinfog: i32,

    udid: String,
    uuid: String,
    secret: String,
}

pub async fn update_stats(
    State(pool): State<PgPool>,
    Form(form): Form<UpdateForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;

    let stars = form.stars;
    let moons = form.moons;
    let demons = form.demons;
    let diamonds = form.diamonds;
    let coins = form.coins;
    let user_coins = form.userCoins;

    let color1 = form.color1;
    let color2 = form.color2;
    let color3 = form.color3;

    let icon_type = form.iconType;
    let display_icon = form.icon;
    let icon = form.icon;
    let ship = form.accShip;
    let ball = form.accBall;
    let ufo = form.accBird;
    let wave = form.accDart;
    let robot = form.accRobot;
    let spider = form.accSpider;
    let explosion = form.accExplosion;
    let swing = form.accSwing;
    let jetpack = form.accJetpack;
    let glow = form.accGlow;

    let level_info = &form.sinfo;

    // FIXME: Construct these properly later
    let demon_info = "0,0,0,0,0,0,0,0,0,0,0,0";
    let platformer_info = "0,0,0,0,0,0";

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    sqlx::query!(
        r#"
        UPDATE users
        SET
        stars = $1,
        moons = $2,
        demons = $3,
        diamonds = $4,
        coins = $5,
        user_coins = $6,

        color1 = $7,
        color2 = $8,
        color3 = $9,

        icon_type = $10,
        display_icon = $11,
        icon = $12,
        ship = $13,
        ball = $14,
        ufo = $15,
        wave = $16,
        robot = $17,
        spider = $18,
        explosion = $19,
        swing = $20,
        jetpack = $21,
        glow = $22,

        level_info = $23,
        demon_info = $24,
        platformer_info = $25

        WHERE id = $26
        "#,
        stars,
        moons,
        demons,
        diamonds,
        coins,
        user_coins,
        color1,
        color2,
        color3,
        icon_type,
        display_icon,
        icon,
        ship,
        ball,
        ufo,
        wave,
        robot,
        spider,
        explosion,
        swing,
        jetpack,
        glow,
        level_info,
        demon_info,
        platformer_info,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(user_id.to_string())
}
