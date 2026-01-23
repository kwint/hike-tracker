use chrono::{NaiveDateTime, TimeDelta, Utc};
use rocket::http::Status;
use rocket::Route;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use crate::db::DbConn;
use crate::models::{Group, Post, Scan};

#[derive(Serialize)]
pub struct PostScanInfo {
    pub post: Post,
    pub scan: Option<Scan>,
    pub idle_time: Option<TimeDelta>,
}

#[derive(Serialize)]
pub struct GroupDetail {
    pub group: Group,
    pub post_scans: Vec<PostScanInfo>,
    pub total_time: Option<TimeDelta>,
    pub idle_time: TimeDelta,
    pub walking_time: Option<TimeDelta>,
}

#[get("/")]
pub async fn dashboard(conn: DbConn) -> Template {
    let groups = conn.run(|c| Group::get_all(c)).await.unwrap_or_default();
    let posts = conn.run(|c| Post::get_all(c)).await.unwrap_or_default();

    let mut group_stats: Vec<GroupDetail> = Vec::new();

    for group in groups {
        let gid = group.id.clone();
        let scans = conn
            .run(move |c| Scan::get_by_group(c, &gid))
            .await
            .unwrap_or_default();
        group_stats.push(group_detail(group, &scans, posts.clone()));
    }

    Template::render(
        "dashboard",
        context! { group_stats: group_stats, posts: posts},
    )
}

#[get("/group/<id>")]
pub async fn group_detail_page(conn: DbConn, id: String) -> Result<Template, Status> {
    let gid = id.clone();
    let group = conn
        .run(move |c| Group::get_by_id(c, &gid))
        .await
        .ok()
        .flatten();

    let group = match group {
        Some(g) => g,
        None => return Err(Status::BadRequest),
    };

    let posts = conn.run(|c| Post::get_all(c)).await.unwrap_or_default();

    let gid = group.id.clone();
    let scans = conn
        .run(move |c| Scan::get_by_group(c, &gid))
        .await
        .unwrap_or_default();

    let detail = group_detail(group, &scans, posts);

    Ok(Template::render(
        "dashboard_detail",
        context! { detail: detail },
    ))
}

fn now_naive() -> NaiveDateTime {
    Utc::now().naive_utc()
}

fn group_detail(group: Group, scans: &[Scan], posts: Vec<Post>) -> GroupDetail {
    let post_scans: Vec<PostScanInfo> = posts
        .into_iter()
        .map(|post| {
            let scan = scans.iter().find(|s| s.post_id == post.id).cloned();
            let idle_time = scan
                .as_ref()
                .map(|scan| scan.departure_time.unwrap_or_else(now_naive) - scan.arrival_time);
            PostScanInfo {
                post,
                scan,
                idle_time,
            }
        })
        .collect();

    let idle_time = post_scans.iter().filter_map(|ps| ps.idle_time).sum();

    let total_time = group
        .start_time
        .map(|start| group.finish_time.unwrap_or_else(now_naive) - start);

    let walking_time = total_time.map(|t| t - idle_time);

    GroupDetail {
        group,
        post_scans,
        total_time,
        idle_time,
        walking_time,
    }
}

pub fn routes() -> Vec<Route> {
    routes![dashboard, group_detail_page]
}
