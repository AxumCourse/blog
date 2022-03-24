use crate::{Result, handler::{HtmlView, render, log_error}, view::backend::index::Index};

pub async fn index()->Result<HtmlView>{
let handler_name="backend/index/index";
    let tmpl = Index{};
    render(tmpl).map_err(log_error(handler_name))
}
