use askama::Template;

use crate::{model::{Category, TopicList, TopicEditData}, db::Paginate};

#[derive(Template)]
#[template(path="backend/topic/add.html")]
pub struct Add {
    pub cats : Vec<Category>,
}
#[derive(Template)]
#[template(path="backend/topic/index.html")]
pub struct Index {
    pub msg:Option<String>,
    pub page: u32,
    pub list:Paginate<Vec<TopicList>>,
}

#[derive(Template)]
#[template(path="backend/topic/edit.html")]
pub struct Edit {
    pub cats : Vec<Category>,
    pub item: TopicEditData,
}
