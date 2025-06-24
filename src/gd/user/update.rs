use axum::{Form, extract::State};
use serde::Serialize;
use sqlx::PgPool;
use tracing::info;

use crate::AppError;
use crate::util::verify_gjp2;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;

#[derive(Serialize, Debug, DeserializeFirstDuplicate)]
pub struct UpdateForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    #[serde(rename = "userName")]
    username: String,
    udid: String,
    uuid: String,
    #[serde(default)]
    gdw: u8,
    gjp2: String,
    secret: String,
    seed: String,
    seed2: String,

    stars: i32,
    moons: i32,
    demons: i32,
    diamonds: i32,
    coins: i32,
    #[serde(rename = "userCoins")]
    user_coins: i32,

    color1: i16,
    color2: i16,
    color3: i16,
    special: i16,

    #[serde(rename = "iconType")]
    icon_type: i16,
    #[serde(rename = "icon")]
    display_icon: i16,
    #[serde(rename = "accIcon")]
    icon: i16,
    #[serde(rename = "accShip")]
    ship: i16,
    #[serde(rename = "accBall")]
    ball: i16,
    #[serde(rename = "accBird")]
    ufo: i16,
    #[serde(rename = "accDart")]
    wave: i16,
    #[serde(rename = "accRobot")]
    robot: i16,
    #[serde(rename = "accSpider")]
    spider: i16,
    #[serde(rename = "accExplosion")]
    explosion: i16,
    #[serde(rename = "accSwing")]
    swing: i16,
    #[serde(rename = "accJetpack")]
    jetpack: i16,
    #[serde(rename = "accGlow")]
    glow: i16,

    #[serde(rename = "gameVersion")]
    game_version: i16,
    #[serde(rename = "binaryVersion")]
    binary_version: i16,

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
}

pub async fn update_stats(
    State(pool): State<PgPool>,
    Form(form): Form<UpdateForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;

    let stars = form.stars;
    let moons = form.moons;
    let demons = form.demons;
    let diamonds = form.diamonds;
    let coins = form.coins;
    let user_coins = form.user_coins;

    let color1 = form.color1;
    let color2 = form.color2;
    let color3 = form.color3;

    let icon_type = form.icon_type;
    let display_icon = form.display_icon;
    let icon = form.icon;
    let ship = form.ship;
    let ball = form.ball;
    let ufo = form.ufo;
    let wave = form.wave;
    let robot = form.robot;
    let spider = form.spider;
    let explosion = form.explosion;
    let swing = form.swing;
    let jetpack = form.jetpack;
    let glow = form.glow;

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

    info!("Updated data for {user_id}!");
    Ok(format!("{user_id}"))
}
