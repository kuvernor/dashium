pub mod register;
pub use crate::gd::account::register::register;

pub mod login;
pub use crate::gd::account::login::login;

pub mod url;
pub use crate::gd::account::url::url;

pub mod save;
pub use crate::gd::account::save::save_data;

pub mod load;
pub use crate::gd::account::load::load_data;
