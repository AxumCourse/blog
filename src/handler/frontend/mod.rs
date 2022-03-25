use axum::{Router, routing::get};

use super::auth::{login_ui, login, logout};

pub mod index;

/// 前端路由
pub fn router()->Router {
    Router::new().route("/", get(index::index))
        .route("/auth", get(login_ui).post(login))
        .route("/logout", get(logout))
}
