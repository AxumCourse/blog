use askama::Template;

#[derive(Template)]
#[template(path="backend/category/add.html")]
pub struct Add {}
