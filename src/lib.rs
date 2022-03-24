pub mod error;
pub mod db;

pub type Result<T> = std::result::Result<T, error::AppError>;
