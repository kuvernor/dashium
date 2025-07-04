pub mod accept;
pub use crate::gd::relationship::accept::accept_friend_request;

pub mod delete;
pub use crate::gd::relationship::delete::delete_friend_request;

pub mod get;
pub use crate::gd::relationship::get::get_friend_requests;

pub mod list;
pub use crate::gd::relationship::list::get_user_list;

pub mod read;
pub use crate::gd::relationship::read::read_friend_request;

pub mod remove;
pub use crate::gd::relationship::remove::remove_friend;

pub mod send;
pub use crate::gd::relationship::send::send_friend_request;
