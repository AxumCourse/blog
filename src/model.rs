use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

/// 分类
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="categories")]
pub struct Category {
    pub id:i32,
    pub name:String,
    pub is_del:bool,
}

/// 分类ID
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="categories")]
pub struct CategoryID {
    pub id:i32,
}
