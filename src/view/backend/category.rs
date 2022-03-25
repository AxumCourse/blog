use askama::Template;

use crate::model::Category;

#[derive(Template)]
#[template(path="backend/category/add.html")]
pub struct Add {}

#[derive(Template)]
#[template(path="backend/category/index.html")]
pub struct Index{
    pub list: Vec<Category>,
    pub msg: Option<String>,
}
