use askama::Template;

#[derive(Template)]
#[template(path="backend/index.html")]
pub struct Index {}
