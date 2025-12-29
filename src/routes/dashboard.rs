use chrono::Utc;
use rocket::http::Status;
use rocket::Route;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use crate::models::{Group, Post, Scan};
use crate::AppState;

#[derive(Serialize)]
pub struct GroupStats {
    pub group: Group,
    pub total_time_minutes: Option<i64>,
    pub idle_time_minutes: i64,
    pub walking_time_minutes: Option<i64>,
    pub posts_visited: usize,
    pub total_posts: usize,
    pub is_finished: bool,
}

#[derive(Serialize)]
pub struct PostScanInfo {
    pub post: Post,
    pub scan: Option<Scan>,
    pub idle_minutes: Option<i64>,
}

#[derive(Serialize)]
pub struct GroupDetail {
    pub group: Group,
    pub post_scans: Vec<PostScanInfo>,
    pub total_time_minutes: Option<i64>,
    pub idle_time_minutes: i64,
    pub walking_time_minutes: Option<i64>,
}

fn calculate_group_stats(group: &Group, scans: &[Scan], total_posts: usize) -> GroupStats {
    let idle_time_minutes: i64 = scans
        .iter()
        .filter_map(|s| s.departure_time.map(|d| (d - s.arrival_time).num_minutes()))
        .sum();

    let total_time_minutes = group
        .start_time
        .map(|start| group.finish_time.unwrap_or_else(Utc::now) - start)
        .map(|t| t.num_minutes());

    let walking_time_minutes = total_time_minutes.map(|t| t - idle_time_minutes);

    GroupStats {
        group: group.clone(),
        total_time_minutes,
        idle_time_minutes,
        walking_time_minutes,
        posts_visited: scans.len(),
        total_posts,
        is_finished: group.finish_time.is_some(),
    }
}

#[get("/")]
pub fn dashboard(state: &State<AppState>) -> Template {
    let db = state.db.lock().unwrap();
    let groups = Group::get_all(db.conn()).unwrap_or_default();
    let posts = Post::get_all(db.conn()).unwrap_or_default();
    let total_posts = posts.iter().filter(|p| !p.is_finish).count();

    let group_stats: Vec<GroupStats> = groups
        .iter()
        .map(|g| {
            let scans = Scan::get_by_group(db.conn(), &g.id).unwrap_or_default();
            calculate_group_stats(g, &scans, total_posts)
        })
        .collect();

    Template::render("dashboard", context! { group_stats: group_stats })
}

#[get("/group/<id>")]
pub fn group_detail(state: &State<AppState>, id: &str) -> Result<Template, Status> {
    let db = state.db.lock().unwrap();

    let group = match Group::get_by_id(db.conn(), id).ok().flatten() {
        Some(g) => g,
        None => return Err(Status::BadRequest),
    };

    let posts = Post::get_all(db.conn()).unwrap_or_default();
    let scans = Scan::get_by_group(db.conn(), id).unwrap_or_default();

    let post_scans: Vec<PostScanInfo> = posts
        .into_iter()
        .filter(|p| !p.is_finish)
        .map(|post| {
            let scan = scans.iter().find(|s| s.post_id == post.id).cloned();
            let idle_minutes = scan
                .as_ref()
                .and_then(|s| s.departure_time.map(|d| (d - s.arrival_time).num_minutes()));
            PostScanInfo {
                post,
                scan,
                idle_minutes,
            }
        })
        .collect();

    let idle_time_minutes: i64 = post_scans.iter().filter_map(|ps| ps.idle_minutes).sum();

    let total_time_minutes = group
        .start_time
        .map(|start| group.finish_time.unwrap_or_else(Utc::now) - start)
        .map(|t| t.num_minutes());

    let walking_time_minutes = total_time_minutes.map(|t| t - idle_time_minutes);

    let detail = GroupDetail {
        group,
        post_scans,
        total_time_minutes,
        idle_time_minutes,
        walking_time_minutes,
    };

    Ok(Template::render(
        "dashboard_detail",
        context! { detail: detail },
    ))
}

pub fn routes() -> Vec<Route> {
    routes![dashboard, group_detail]
}
