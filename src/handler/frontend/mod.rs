use axum::{Router, routing::get};

pub mod index;

/// 前端路由
pub fn router()->Router {
     Router::new().route("/", get(index::index))
}
