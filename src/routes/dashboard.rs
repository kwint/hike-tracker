use chrono::TimeDelta;
use rocket::http::CookieJar;
use rocket::Route;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use crate::auth::get_auth_context;
use crate::db::DbConn;
use crate::models::{Group, Post, Scan};
use crate::stats::{calculate_group_stats, PostScanInfo};

#[derive(Serialize)]
pub struct GroupDetail {
    pub group: Group,
    pub post_scans: Vec<PostScanInfo>,
    pub total_time: Option<TimeDelta>,
    pub idle_time: TimeDelta,
    pub walking_time: Option<TimeDelta>,
}

#[get("/")]
pub async fn dashboard(cookies: &CookieJar<'_>, conn: DbConn) -> Template {
    let auth_ctx = get_auth_context(cookies);
    let is_admin = auth_ctx.is_admin;
    let is_post_holder = auth_ctx.is_post_holder;
    let holder_post_id = auth_ctx.holder_post_id;

    let groups = conn.run(Group::get_all).await.unwrap_or_default();
    let posts = conn.run(Post::get_all).await.unwrap_or_default();

    let mut group_stats: Vec<GroupDetail> = Vec::new();

    for group in groups {
        let gid = group.id.clone();
        let scans = conn
            .run(move |c| Scan::get_by_group(c, &gid))
            .await
            .unwrap_or_default();
        let stats = calculate_group_stats(&group, &scans, posts.clone());
        group_stats.push(GroupDetail {
            group,
            post_scans: stats.post_scans,
            total_time: stats.total_time,
            idle_time: stats.idle_time,
            walking_time: stats.walking_time,
        });
    }

    Template::render(
        "dashboard",
        context! {
            group_stats: group_stats,
            posts: posts,
            is_admin: is_admin,
            is_post_holder: is_post_holder,
            holder_post_id: holder_post_id,
        },
    )
}

pub fn routes() -> Vec<Route> {
    routes![dashboard]
}
