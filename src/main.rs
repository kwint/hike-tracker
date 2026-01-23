#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod schema;

use db::DbConn;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/dashboard")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/admin/posts", routes::admin::posts::routes())
        .mount("/admin/groups", routes::admin::groups::routes())
        .mount("/scan", routes::scan::routes())
        .mount("/dashboard", routes::dashboard::routes())
}
