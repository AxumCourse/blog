use tokio_postgres::Client;

use crate::{
    model::{Category, CategoryID},
    Result,
};

/// 创建新分类
pub async fn create(client: &Client, name: String) -> Result<CategoryID> {
    super::insert(
        client,
        "INSERT INTO categories (name, is_del) VALUES ($1, false) RETUNING id",
        &[&name],
        "创建分类失败",
    )
    .await
}

/// 获取所有分类
pub async fn list(client: &Client) -> Result<Vec<Category>> {
    super::query(
        client,
        "SELECT id,name,is_del FROM categories WHERE is_del=false ORDER BY id ASC",
        &[],
    )
    .await
}

/// 删除或恢复记录
pub async fn del_or_restore(client: &Client, id: i32, is_del: bool) -> Result<bool> {
    let n = super::del_or_restore(client, "categories", &id, is_del).await?;
    Ok(n > 0)
}
