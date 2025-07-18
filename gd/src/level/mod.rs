pub mod upload;
pub use crate::level::upload::uploadGJLevel21;

pub mod search;
pub use crate::level::search::getGJLevels21;

pub mod download;
pub use crate::level::download::downloadGJLevel22;

pub mod delete;
pub use crate::level::delete::deleteGJLevelUser20;

pub mod report;
pub use crate::level::report::reportGJLevel;

pub mod daily;
pub use crate::level::daily::getGJDailyLevel;

pub mod update;
pub use crate::level::update::updateGJDesc20;
