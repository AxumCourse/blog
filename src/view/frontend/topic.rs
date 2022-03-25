use askama::Template;

use crate::{db::Paginate, model::{TopicList, Category, TopicArchive, TopicDetail}};

#[derive(Template)]
#[template(path="frontend/topic_list.html")]
pub struct List {
    pub list: Paginate<Vec<TopicList>>,
    pub page : u32,
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
    pub category_name: String,
}
#[derive(Template)]
#[template(path="frontend/topic_detail.html")]
pub struct Detail {
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
    pub item: TopicDetail,
}
