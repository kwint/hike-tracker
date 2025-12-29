#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

use rocket_dyn_templates::Template;
use std::sync::Mutex;

use db::Database;

pub struct AppState {
    pub db: Mutex<Database>,
}

#[get("/")]
fn index() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/dashboard")
}

#[launch]
fn rocket() -> _ {
    let db = Database::new("hike_tracker.db").expect("Failed to initialize database");

    rocket::build()
        .manage(AppState { db: Mutex::new(db) })
        .mount("/", routes![index])
        .mount("/admin/posts", routes::admin::posts::routes())
        .mount("/admin/groups", routes::admin::groups::routes())
        .mount("/scan", routes::scan::routes())
        .mount("/dashboard", routes::dashboard::routes())
        .attach(Template::fairing())
}
