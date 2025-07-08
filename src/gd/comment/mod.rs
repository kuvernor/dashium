pub mod upload;
pub use crate::gd::comment::upload::upload_comment;

pub mod get;
pub use crate::gd::comment::get::get_comments;

pub mod delete;
pub use crate::gd::comment::delete::delete_comment;

pub mod history;
pub use crate::gd::comment::history::get_history;
