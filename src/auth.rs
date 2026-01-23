use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::time::Duration;

const ADMIN_COOKIE: &str = "admin_session";

pub struct Admin;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private(ADMIN_COOKIE) {
            Some(cookie) if cookie.value() == "authenticated" => Outcome::Success(Admin),
            _ => Outcome::Forward(Status::Unauthorized),
        }
    }
}

pub fn login(cookies: &CookieJar<'_>) {
    let mut cookie = Cookie::new(ADMIN_COOKIE, "authenticated");
    cookie.set_max_age(Duration::hours(24));
    cookies.add_private(cookie);
}

pub fn logout(cookies: &CookieJar<'_>) {
    cookies.remove_private(ADMIN_COOKIE);
}

pub fn check_password(password: &str) -> bool {
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or_default();
    !admin_password.is_empty() && password == admin_password
}

pub fn is_admin(cookies: &CookieJar<'_>) -> bool {
    cookies
        .get_private(ADMIN_COOKIE)
        .map(|c| c.value() == "authenticated")
        .unwrap_or(false)
}
