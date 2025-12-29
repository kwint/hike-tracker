use image::{ImageEncoder, Luma};
use qrcode::QrCode;
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::Route;
use rocket::State;
use rocket_dyn_templates::{context, Template};

use crate::models::{Group, Post};
use crate::AppState;

#[derive(FromForm)]
pub struct NewGroup {
    name: String,
}

#[derive(FromForm)]
pub struct NewPost {
    name: String,
    order: i32,
    #[field(default = false)]
    is_finish: bool,
}

#[get("/groups")]
pub fn groups(state: &State<AppState>) -> Template {
    let db = state.db.lock().unwrap();
    let groups = Group::get_all(db.conn()).unwrap_or_default();
    Template::render("admin/groups", context! { groups: groups })
}

#[post("/groups", data = "<form>")]
pub fn create_group(state: &State<AppState>, form: Form<NewGroup>) -> Redirect {
    let db = state.db.lock().unwrap();

    let group = Group::new(form.name.clone());
    group.insert(db.conn()).unwrap();

    Redirect::to("/admin/groups")
}

#[get("/groups/<id>/delete")]
pub fn delete_group(state: &State<AppState>, id: &str) -> Redirect {
    let db = state.db.lock().unwrap();
    let _ = Group::delete(db.conn(), id);
    Redirect::to("/admin/groups")
}

#[get("/groups/<id>/qr")]
pub fn group_qr(_state: &State<AppState>, id: &str) -> (ContentType, Vec<u8>) {
    let url = format!("/scan/{}", id);

    let code = QrCode::new(url.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().min_dimensions(200, 200).build();

    let mut png_data: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ExtendedColorType::L8,
        )
        .unwrap();

    (ContentType::PNG, png_data)
}

#[get("/posts")]
pub fn posts(state: &State<AppState>) -> Template {
    let db = state.db.lock().unwrap();
    let posts = Post::get_all(db.conn()).unwrap_or_default();
    Template::render("admin/posts", context! { posts: posts })
}

#[post("/posts", data = "<form>")]
pub fn create_post(state: &State<AppState>, form: Form<NewPost>) -> Redirect {
    let db = state.db.lock().unwrap();

    let post = Post::new(form.name.clone(), form.order, form.is_finish);
    let _ = post.insert(db.conn());

    Redirect::to("/admin/posts")
}

#[get("/posts/<id>/delete")]
pub fn delete_post(state: &State<AppState>, id: &str) -> Redirect {
    let db = state.db.lock().unwrap();
    let _ = Post::delete(db.conn(), id);
    Redirect::to("/admin/posts")
}

pub fn routes() -> Vec<Route> {
    routes![
        groups,
        create_group,
        delete_group,
        group_qr,
        posts,
        create_post,
        delete_post
    ]
}
