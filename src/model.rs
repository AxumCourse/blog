use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

/// 分类
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="categies")]
pub struct Category {
    pub id:i32,
    pub name:String,
    pub is_del:bool,
}
