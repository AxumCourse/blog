use askama::Template;
use axum::{
    http::{header, HeaderMap, StatusCode},
    response::Html,
};
use deadpool_postgres::Client;

use crate::{error::AppError, AppState, Result, cookie};

pub mod auth;
pub mod backend;
pub mod frontend;

type HtmlView = Html<String>;
type RedirectView = (StatusCode, HeaderMap, ());

/// 渲染模板
fn render<T>(tmpl: T) -> Result<HtmlView>
where
    T: Template,
{
    let html = tmpl.render().map_err(AppError::from)?;
    Ok(Html(html))
}

/// 将错误信息记录到日志
fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("操作失败：{:?},  {}", err, handler_name);
        err
    })
}

/// 重定向
fn redirect(url: &str) -> Result<RedirectView> {
    redirect_with_cookie(url, None)
}

/// 从连接池获取连接
async fn get_client(state: &AppState) -> Result<Client> {
    state.pool.get().await.map_err(AppError::from)
}

/// 重定向
fn redirect_with_cookie(url: &str, c:Option<&str>) -> Result<RedirectView> {
    let mut hm = match c {
        Some(s) => cookie::set_cookie(s),
        None => HeaderMap::new(),
    };
    hm.insert(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, hm, ()))
}
