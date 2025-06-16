pub mod register;
pub use crate::gd::user::register::register;

pub mod login;
pub use crate::gd::user::login::login;

pub mod url;
pub use crate::gd::user::url::url;

pub mod save;
pub use crate::gd::user::save::save_data;

pub mod load;
pub use crate::gd::user::load::load_data;
