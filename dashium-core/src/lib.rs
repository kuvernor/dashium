#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use axum::{Router, routing::post};
use sqlx::PgPool;

mod error;
pub use crate::error::AppError;

pub mod handlers;
use handlers::*;

pub mod models;
pub mod util;

pub trait GDResponse {
    fn to_gd(&self) -> String;
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route(
            "/accounts/registerGJAccount.php",
            post(user::registerGJAccount),
        )
        .route("/accounts/loginGJAccount.php", post(user::loginGJAccount))
        .route(
            "/database/accounts/backupGJAccountNew.php",
            post(user::backupGJAccountNew),
        )
        .route(
            "/database/accounts/syncGJAccountNew.php",
            post(user::syncGJAccountNew),
        )
        .route("/updateGJUserScore22.php", post(user::updateGJUserScore22))
        .route("/getGJUserInfo20.php", post(user::getGJUserInfo20))
        .route("/getGJUsers20.php", post(user::getGJUsers20))
        .route(
            "/updateGJAccSettings20.php",
            post(user::updateGJAccSettings20),
        )
        // Misc
        .route("/getAccountURL.php", post(misc::getAccountURL))
        .route("/likeGJItem211.php", post(misc::likeGJItem211))
        // Posts (aka account comments)
        .route(
            "/getGJAccountComments20.php",
            post(post::getGJAccountComments20),
        )
        .route(
            "/uploadGJAccComment20.php",
            post(post::uploadGJAccComment20),
        )
        .route(
            "/deleteGJAccComment20.php",
            post(post::deleteGJAccComment20),
        )
        // Messages
        .route("/uploadGJMessage20.php", post(message::uploadGJMessage20))
        .route("/getGJMessages20.php", post(message::getGJMessages20))
        .route(
            "/downloadGJMessage20.php",
            post(message::downloadGJMessage20),
        )
        .route("/deleteGJMessages20.php", post(message::deleteGJMessages20))
        // Relationships
        .route(
            "/uploadFriendRequest20.php",
            post(relationship::uploadFriendRequest20),
        )
        .route(
            "/getGJFriendRequests20.php",
            post(relationship::getGJFriendRequests20),
        )
        .route(
            "/readGJFriendRequests20.php",
            post(relationship::readGJFriendRequests20),
        )
        .route(
            "/acceptGJFriendRequest20.php",
            post(relationship::acceptGJFriendRequest20),
        )
        .route(
            "/deleteGJFriendRequests20.php",
            post(relationship::deleteGJFriendRequests20),
        )
        .route("/getGJUserList20.php", post(relationship::getGJUserList20))
        .route(
            "/removeGJFriend20.php",
            post(relationship::removeGJFriend20),
        )
        .route("/blockGJUser20.php", post(relationship::blockGJUser20))
        .route("/unblockGJUser20.php", post(relationship::unblockGJUser20))
        // Levels
        .route("/uploadGJLevel21.php", post(level::uploadGJLevel21))
        .route("/getGJLevels21.php", post(level::getGJLevels21))
        .route("/downloadGJLevel22.php", post(level::downloadGJLevel22))
        .route("/deleteGJLevelUser20.php", post(level::deleteGJLevelUser20))
        .route("/reportGJLevel.php", post(level::reportGJLevel))
        .route("/getGJDailyLevel.php", post(level::getGJDailyLevel))
        .route("/updateGJDesc20.php", post(level::updateGJDesc20))
        // Moderator
        .route("/requestUserAccess.php", post(moderator::requestUserAccess))
        .route("/suggestGJStars20.php", post(moderator::suggestGJStars20))
        // Comments
        .route("/uploadGJComment21.php", post(comment::uploadGJComment21))
        .route("/getGJComments21.php", post(comment::getGJComments21))
        .route("/deleteGJComment20.php", post(comment::deleteGJComment20))
        .route(
            "/getGJCommentHistory.php",
            post(comment::getGJCommentHistory),
        )
        // Levelpacks
        .route("/getGJMapPacks21.php", post(levelpack::getGJMapPacks21))
        .route("/getGJGauntlets21.php", post(levelpack::getGJGauntlets21))
        // Level lists
        .route("/uploadGJLevelList.php", post(list::uploadGJLevelList))
        .route("/getGJLevelLists.php", post(list::getGJLevelLists))
        .route("/deleteGJLevelList.php", post(list::deleteGJLevelList))
        // Scores
        .route("/getGJScores20.php", post(score::getGJScores20))
        .route("/getGJLevelScores211.php", post(score::getGJLevelScores211))
        // Rewards
        .route("/getGJChallenges.php", post(reward::getGJChallenges))
}
