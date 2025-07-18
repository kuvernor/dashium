mod upload;
pub use crate::comment::upload::uploadGJComment21;

mod get;
pub use crate::comment::get::getGJComments21;

mod delete;
pub use crate::comment::delete::deleteGJComment20;

mod history;
pub use crate::comment::history::getGJCommentHistory;
