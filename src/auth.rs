use rocket::http::{Cookies};

pub fn is_authenticated(cookies: &Cookies) -> bool {
    match cookies.find("session") {
        Some(_) => true,
        None => false
    }
}
