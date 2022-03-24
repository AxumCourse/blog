use askama::Template;
use axum::{response::Html, http::{StatusCode, HeaderMap, header }};
use deadpool_postgres::Client;

use crate::{Result, error::AppError, AppState};

pub mod frontend;
pub mod backend;

type HtmlView = Html<String>;
type RedirectView = (StatusCode, HeaderMap, ());

/// 渲染模板
 fn render<T>(tmpl: T) ->Result<HtmlView> where T:Template {
    let html = tmpl.render().map_err(AppError::from)?;
    Ok(Html(html))
}

/// 将错误信息记录到日志
fn log_error(handler_name:&str) -> Box<dyn Fn(AppError)->AppError> {
     let handler_name = handler_name.to_string();
     Box::new(move |err| {
         tracing::error!("操作失败：{:?},  {}", err, handler_name);
         err
     })
 }

/// 重定向
fn redirect(url:&str) -> Result<RedirectView> {
    let mut hm = HeaderMap::new();
    hm.append(header::LOCATION,url.parse().unwrap()) ;
    Ok((StatusCode::FOUND, hm, ()))
}

/// 从连接池获取连接
async fn get_client(state: &AppState) -> Result<Client> {
   state.pool.get().await.map_err(AppError::from)
}

