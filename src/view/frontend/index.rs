use askama::Template;

use crate::{db::Paginate, model::{TopicList, Category, TopicArchive}};

#[derive(Template)]
#[template(path="frontend/index.html")]
pub struct Index {
    pub list: Paginate<Vec<TopicList>>,
    pub page : u32,
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
}
