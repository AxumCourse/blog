use std::time;

use tokio_postgres::Client;

use crate::{form, Result, model::{TopicID, TopicList}};

use super::Paginate;

pub async fn create(client:&Client, frm:&form::CreateTopic) -> Result<TopicID> {
    let html = "html";
    let dateline = time::SystemTime::now();
    super::insert(client, "INSERT INTO topics (title,category_id, summary, markdown, html, hit, dateline, is_del) VALUES ($1, $2, $3, $4, $5, 0, $6, false) RETURNING id", &[&frm.title, &frm.category_id, &frm.summary, &frm.markdown, &html,  &dateline ], "添加文章失败").await
}


pub async fn list(client:&Client, page:u32)->Result<Paginate< Vec<TopicList>>> {
    let sql="SELECT id,title,category_id,summary,hit,dateline,is_del,category_name FROM v_topic_cat_list WHERE is_del=false";
    let count_sql="SELECT COUNT(*) FROM v_topic_cat_list WHERE is_del=false";
    super::pagination(client, sql, count_sql, &[],page ).await
}
