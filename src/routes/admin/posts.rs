use rocket::form::Form;
use rocket::response::Redirect;
use rocket::Route;
use rocket_dyn_templates::{context, Template};

use crate::db::DbConn;
use crate::models::{NewPost, Post};

#[derive(FromForm)]
pub struct NewPostForm {
    name: String,
    order: i32,
}

#[get("/")]
pub async fn posts(conn: DbConn) -> Template {
    let posts = conn.run(|c| Post::get_all(c)).await.unwrap_or_default();
    Template::render("admin/posts", context! { posts: posts })
}

#[post("/", data = "<form>")]
pub async fn create_post(conn: DbConn, form: Form<NewPostForm>) -> Redirect {
    let name = form.name.clone();
    let order = form.order;
    conn.run(move |c| {
        let post = NewPost::new(name, order);
        Post::insert(c, post)
    })
    .await
    .ok();

    Redirect::to("/admin/posts")
}

#[get("/<id>/delete")]
pub async fn delete_post(conn: DbConn, id: String) -> Redirect {
    conn.run(move |c| Post::delete(c, &id)).await.ok();
    Redirect::to("/admin/posts")
}

pub fn routes() -> Vec<Route> {
    routes![posts, create_post, delete_post]
}
