use page_hunter::{Book, Page};

use crate::api::models::users::User;

pub type UserPage = Page<User>;
pub type UserBook = Book<User>;
