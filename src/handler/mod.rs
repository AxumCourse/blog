use askama::Template;
use axum::response::Html;

use crate::{Result, error::AppError};

pub mod frontend;

type HtmlView = Html<String>;

/// 渲染模板
 fn render<T>(tmpl: T) ->Result<HtmlView> where T:Template {
    let html = tmpl.render().map_err(AppError::from)?;
    Ok(Html(html))
}

fn log_error(handler_name:&str) -> Box<dyn Fn(AppError)->AppError> {
     let handler_name = handler_name.to_string();
     Box::new(move |err| {
         tracing::error!("操作失败：{:?},  {}", err, handler_name);
         err
     })
 }
