pub mod get;
pub use crate::gd::message::get::get_messages;

pub mod send;
pub use crate::gd::message::send::send_message;

pub mod download;
pub use crate::gd::message::download::download_message;

pub mod delete;
pub use crate::gd::message::delete::delete_message;
