use tokio_postgres::Client;

use crate::{
    error::AppError,
    form,
    model::{Category, CategoryID},
    Result,
};

/// 创建新分类
pub async fn create(client: &Client, frm: &form::CreateCategory) -> Result<CategoryID> {
    // 名称是否存在
    let n = super::count(
        client,
        "SELECT COUNT(*) FROM categories WHERE name=$1",
        &[&frm.name],
    )
    .await?;
    if n > 0 {
        return Err(AppError::duplicate("同名的分类已存在"));
    }

    super::insert(
        client,
        "INSERT INTO categories (name, is_del) VALUES ($1, false) RETURNING id",
        &[&frm.name],
        "创建分类失败",
    )
    .await
}

/// 获取所有分类
pub async fn list(client: &Client) -> Result<Vec<Category>> {
    super::query(
        client,
        "SELECT id,name,is_del FROM categories WHERE is_del=false ORDER BY id ASC LIMIT 1000",
        &[],
    )
    .await
}

/// 删除或恢复分类
pub async fn del_or_restore(client: &Client, id: i32, is_del: bool) -> Result<bool> {
    let n = super::del_or_restore(client, "categories", &id, is_del).await?;
    Ok(n > 0)
}

/// 修改分类
pub async fn edit(client: &Client, frm: &form::EditCategory) -> Result<bool> {
    // 名称是否存在
    let n = super::count(
        client,
        "SELECT COUNT(*) FROM categories WHERE name=$1 AND id<>$2",
        &[&frm.name, &frm.id],
    )
    .await?;
    if n > 0 {
        return Err(AppError::duplicate("同名的分类已存在"));
    }

    let n = super::execute(
        client,
        "UPDATE categories SET name=$1 WHERE id=$2",
        &[&frm.name, &frm.id],
    )
    .await?;
    Ok(n > 0)
}

/// 根据ID查找分类
pub async fn find(client: &Client, id:i32) ->Result<Category> {
    super::query_row(client, "SELECT id, name, is_del FROM categories WHERE id=$1 LIMIT 1", &[&id]).await
}
