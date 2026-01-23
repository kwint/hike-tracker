use chrono::Utc;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::Route;
use rocket_dyn_templates::{context, Template};

use crate::db::DbConn;
use crate::models::{Group, NewScan, Post, Scan};

#[derive(FromForm)]
pub struct ScanForm {
    post_id: String,
}

#[get("/<group_id>")]
pub async fn scan_page(conn: DbConn, group_id: String) -> Result<Template, Status> {
    let gid = group_id.clone();
    let group = conn
        .run(move |c| Group::get_by_id(c, &gid))
        .await
        .ok()
        .flatten();

    let group = match group {
        Some(g) => g,
        None => return Err(Status::BadRequest),
    };

    let gid = group_id.clone();
    let posts = conn.run(|c| Post::get_all(c)).await.unwrap_or_default();
    let scans = conn
        .run(move |c| Scan::get_by_group(c, &gid))
        .await
        .unwrap_or_default();

    Ok(Template::render(
        "scan",
        context! {
            group: group,
            posts: posts,
            scans: scans,
        },
    ))
}

#[post("/<group_id>", data = "<form>")]
pub async fn record_scan(conn: DbConn, group_id: String, form: Form<ScanForm>) -> Redirect {
    let gid = group_id.clone();

    // Verify group exists
    let group_exists = conn
        .run(move |c| Group::get_by_id(c, &gid))
        .await
        .ok()
        .flatten()
        .is_some();

    if !group_exists {
        return Redirect::to("/");
    }

    let gid = group_id.clone();
    let post_id = form.post_id.clone();

    // Check if scan exists
    let existing_scan = conn
        .run(move |c| Scan::get_by_group_and_post(c, &gid, &post_id))
        .await
        .ok()
        .flatten();

    match existing_scan {
        Some(scan) => {
            // Already arrived, record departure
            if scan.departure_time.is_none() {
                let scan_id = scan.id.clone();
                let now = Utc::now().naive_utc();
                conn.run(move |c| Scan::set_departure_time(c, &scan_id, now))
                    .await
                    .ok();
            }
        }
        None => {
            // First scan, record arrival
            let gid = group_id.clone();
            let post_id = form.post_id.clone();
            conn.run(move |c| {
                let scan = NewScan::new(gid, post_id);
                Scan::insert(c, scan)
            })
            .await
            .ok();
        }
    }

    Redirect::to(format!("/scan/{}", group_id))
}

pub fn routes() -> Vec<Route> {
    routes![scan_page, record_scan]
}
