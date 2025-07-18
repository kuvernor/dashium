pub mod accept;
pub use crate::relationship::accept::acceptGJFriendRequest20;

pub mod delete;
pub use crate::relationship::delete::deleteGJFriendRequests20;

pub mod get;
pub use crate::relationship::get::getGJFriendRequests20;

pub mod list;
pub use crate::relationship::list::getGJUserList20;

pub mod read;
pub use crate::relationship::read::readGJFriendRequests20;

pub mod remove;
pub use crate::relationship::remove::removeGJFriend20;

pub mod send;
pub use crate::relationship::send::uploadFriendRequest20;

pub mod block;
pub use crate::relationship::block::blockGJUser20;

pub mod unblock;
pub use crate::relationship::unblock::unblockGJUser20;
