mod accept;
pub use crate::relationship::accept::acceptGJFriendRequest20;

mod delete;
pub use crate::relationship::delete::deleteGJFriendRequests20;

mod get;
pub use crate::relationship::get::getGJFriendRequests20;

mod list;
pub use crate::relationship::list::getGJUserList20;

mod read;
pub use crate::relationship::read::readGJFriendRequests20;

mod remove;
pub use crate::relationship::remove::removeGJFriend20;

mod send;
pub use crate::relationship::send::uploadFriendRequest20;

mod block;
pub use crate::relationship::block::blockGJUser20;

mod unblock;
pub use crate::relationship::unblock::unblockGJUser20;
