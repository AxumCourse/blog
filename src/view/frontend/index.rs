use askama::Template;

#[derive(Template)]
#[template(path="frontend/index.html")]
pub struct Index {}
