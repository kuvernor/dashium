pub mod get;
pub use crate::message::get::getGJMessages20;

pub mod send;
pub use crate::message::send::uploadGJMessage20;

pub mod download;
pub use crate::message::download::downloadGJMessage20;

pub mod delete;
pub use crate::message::delete::deleteGJMessages20;
