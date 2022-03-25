use axum::{Router, routing::get};
use serde::Deserialize;

use super::auth::{login_ui, login, logout};

pub mod index;

/// 前端路由
pub fn router()->Router {
    Router::new().route("/", get(index::index))
        .route("/auth", get(login_ui).post(login))
        .route("/logout", get(logout))
}
#[derive(Deserialize)]
pub struct Args {
    pub page : Option<u32>,
}
impl Args {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
