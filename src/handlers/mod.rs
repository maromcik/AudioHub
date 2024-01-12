pub mod homepage;
pub mod user;

pub use crate::handlers::user::register as create_user;
pub use crate::handlers::user::login as user_login_page;
pub use crate::handlers::user::login_user as login_user;
pub use crate::handlers::user::register_user as user_register_page;

pub use crate::handlers::homepage::homepage as index;
