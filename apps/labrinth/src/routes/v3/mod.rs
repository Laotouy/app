pub use super::ApiError;
use crate::util::cors::default_cors;
use actix_web::{HttpResponse, web};
use serde_json::json;

pub mod analytics_get;
pub mod bans;
pub mod collections;
pub mod forum;
pub mod images;
pub mod notifications;
pub mod organizations;
pub mod payouts;
pub mod project_creation;
pub mod projects;
pub mod reports;
pub mod statistics;
pub mod tags;
pub mod teams;
pub mod threads;
pub mod users;
pub mod version_creation;
pub mod version_file;
pub mod versions;

pub mod creator;
pub mod issues;
pub mod oauth_clients;
pub mod payment_merchant;
#[allow(clippy::unnecessary_unwrap, clippy::explicit_auto_deref)]
mod wikis;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("v3")
            .wrap(default_cors())
            .configure(analytics_get::config)
            .configure(collections::config)
            .configure(images::config)
            .configure(notifications::config)
            .configure(organizations::config)
            .configure(project_creation::config)
            .configure(projects::config)
            .configure(reports::config)
            .configure(statistics::config)
            .configure(tags::config)
            .configure(teams::config)
            .configure(threads::config)
            // creator 和 payment_merchant 必须在 users 之前，
            // 因为 users 中的 user/{id} 会匹配 user/creator 和 user/payment
            .configure(creator::config)
            .configure(payment_merchant::config)
            .configure(users::config)
            .configure(version_file::config)
            .configure(payouts::config)
            .configure(versions::config)
            .configure(forum::config)
            .configure(issues::config)
            .configure(bans::config),
    );
}

pub async fn hello_world() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(json!({
        "hello": "world",
    })))
}
