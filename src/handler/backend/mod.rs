use axum::{routing::get, Router};

use index::index;
use serde::Deserialize;

pub mod category;
pub mod index;
pub mod topic;

pub fn router() -> Router {
    let category_router = Router::new()
        .route("/", get(category::index))
        .route("/add", get(category::add_ui).post(category::add))
        .route("/del/:id", get(category::del))
        .route("/edit/:id", get(category::edit_ui).post(category::edit))
        ;
    let topic_router = Router::new()
        .route("/", get(topic::index))
        .route("/add", get(topic::add_ui).post(topic::add))
        ;
    Router::new()
        .route("/", get(index))
        .nest("/category", category_router)
        .nest("/topic", topic_router)
}

#[derive(Deserialize)]
pub struct Args {
    pub msg: Option<String>,
    pub page: Option<u32>,
}
impl Args {
    pub fn msg(&self) -> String {
        self.msg.clone().unwrap_or("".to_string())
    }
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
