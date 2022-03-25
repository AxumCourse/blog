use askama::Template;

#[derive(Template)]
#[template(path="backend/admin/login.html")]
pub struct Login{}
