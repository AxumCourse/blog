pub mod error;
pub mod db;
pub mod view;
pub mod handler;
pub mod model;

pub type Result<T> = std::result::Result<T, error::AppError>;
