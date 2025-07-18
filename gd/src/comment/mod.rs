pub mod upload;
pub use crate::comment::upload::uploadGJComment21;

pub mod get;
pub use crate::comment::get::getGJComments21;

pub mod delete;
pub use crate::comment::delete::deleteGJComment20;

pub mod history;
pub use crate::comment::history::getGJCommentHistory;
