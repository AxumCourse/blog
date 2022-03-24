use axum::{routing::get, Router};

use index::index;

pub mod category;
pub mod index;

pub fn router() -> Router {
    let category_router = Router::new().route("/add", get(category::add_ui).post(category::add));
    Router::new()
        .route("/", get(index))
        .nest("/category", category_router)
}
