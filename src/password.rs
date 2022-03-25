use bcrypt::DEFAULT_COST;

use crate::{error::AppError, Result};

pub fn hash(pwd: &str) -> Result<String> {
    bcrypt::hash(pwd, DEFAULT_COST).map_err(AppError::from)
}
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool> {
    bcrypt::verify(pwd, hashed_pwd).map_err(AppError::from)
}
