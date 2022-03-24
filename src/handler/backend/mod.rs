use axum::{Router, routing::get};

use index::index;

pub mod index;

pub fn router() -> Router {
    Router::new().route("/", get(index))
}
