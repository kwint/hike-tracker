use rocket::Route;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use crate::auth::Admin;
use crate::db::DbConn;
use crate::models::{Group, Post, Scan};
use crate::stats::{calculate_group_stats, format_duration};

#[derive(Serialize)]
pub struct RankedGroup {
    pub rank: usize,
    pub group: Group,
    pub total_time: Option<String>,
    pub walking_time: Option<String>,
    pub idle_time: String,
    pub total_time_secs: Option<i64>,
    pub walking_time_secs: Option<i64>,
    pub posts_visited: usize,
    pub total_posts: usize,
    pub visited_all_posts: bool,
}

#[get("/?<sort>")]
pub async fn ranking(_admin: Admin, conn: DbConn, sort: Option<String>) -> Template {
    let sort_by = sort.unwrap_or_else(|| "total".to_string());

    let groups = conn.run(Group::get_all).await.unwrap_or_default();
    let posts = conn.run(Post::get_all).await.unwrap_or_default();
    let total_posts = posts.len();

    let mut ranked_groups: Vec<RankedGroup> = Vec::new();

    for group in groups {
        // Only include groups that have finished
        if group.finish_time.is_none() {
            continue;
        }

        let group_id = group.id.clone();
        let scans = conn
            .run(move |c| Scan::get_by_group(c, &group_id))
            .await
            .unwrap_or_default();

        let posts_visited = scans.len();
        let visited_all_posts = posts_visited >= total_posts;

        let stats = calculate_group_stats(&group, &scans, posts.clone());
        let total_time = stats.total_time;
        let idle_time = stats.idle_time;
        let walking_time = stats.walking_time;

        ranked_groups.push(RankedGroup {
            rank: 0, // Will be set after sorting
            group,
            total_time: total_time.map(format_duration),
            walking_time: walking_time.map(format_duration),
            idle_time: format_duration(idle_time),
            total_time_secs: total_time.map(|t| t.num_seconds()),
            walking_time_secs: walking_time.map(|t| t.num_seconds()),
            posts_visited,
            total_posts,
            visited_all_posts,
        });
    }

    // Sort: complete groups first (by time), then incomplete groups (by time)
    match sort_by.as_str() {
        "walking" => {
            ranked_groups.sort_by(|a, b| {
                // First sort by visited_all_posts (complete groups first)
                b.visited_all_posts.cmp(&a.visited_all_posts).then_with(|| {
                    // Then by walking time
                    a.walking_time_secs
                        .unwrap_or(i64::MAX)
                        .cmp(&b.walking_time_secs.unwrap_or(i64::MAX))
                })
            });
        }
        _ => {
            // Default: sort by total time
            ranked_groups.sort_by(|a, b| {
                // First sort by visited_all_posts (complete groups first)
                b.visited_all_posts.cmp(&a.visited_all_posts).then_with(|| {
                    // Then by total time
                    a.total_time_secs
                        .unwrap_or(i64::MAX)
                        .cmp(&b.total_time_secs.unwrap_or(i64::MAX))
                })
            });
        }
    }

    // Assign ranks
    for (i, group) in ranked_groups.iter_mut().enumerate() {
        group.rank = i + 1;
    }

    Template::render(
        "ranking",
        context! {
            ranked_groups: ranked_groups,
            sort_by: sort_by,
            is_admin: true,
        },
    )
}

pub fn routes() -> Vec<Route> {
    routes![ranking]
}
