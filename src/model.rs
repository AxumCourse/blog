use std::time;

use chrono::{Local, TimeZone};
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

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="v_topic_cat_list")]
pub struct TopicList {
    pub id:i64,
    pub title: String,
    pub category_id:i32,
    pub summary:String,
    pub hit:i32,
    pub dateline:time::SystemTime,
    pub is_del:bool,
    pub category_name:String,
}
impl TopicList {
    pub fn dateline(&self) ->String {
        let ts = self.dateline.clone().duration_since(time::UNIX_EPOCH).unwrap_or(time::Duration::from_secs(0)).as_secs() as i64;
        Local.timestamp(ts, 0).format("%Y/%m/%d %H:%M:%S").to_string()
    }
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="topics")]
pub struct TopicID {
    pub id:i64,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="topics")]
pub struct TopicEditData {
    pub id:i64,
    pub title: String,
    pub category_id: i32,
    pub summary: String,
    pub markdown: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="topics")]
pub struct TopicArchive {
    pub dateline: String,
}
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="admins")]
pub struct Admin {
    pub id:i32,
    pub email:String,
    pub password:String,
    pub is_del:bool,
}
