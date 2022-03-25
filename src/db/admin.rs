use tokio_postgres::Client;

use crate::{Result, model::Admin};

pub async fn find(client:&Client, email: &str) -> Result<Admin> {
    super::query_row(client, "SELECT id,email,password,is_del FROM admins WHERE email=$1 AND is_del=false", &[&email]).await
}
