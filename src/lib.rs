pub mod error;
pub mod db;
pub mod view;
pub mod handler;
pub mod model;
pub mod form;
pub mod config;
mod md;
mod password;
mod cookie;
pub mod middleware;

pub type Result<T> = std::result::Result<T, error::AppError>;

/// 共享状态
pub struct AppState {
    /// 数据库连接
    pub pool: deadpool_postgres::Pool,
}
