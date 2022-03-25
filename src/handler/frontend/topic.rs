use std::sync::Arc;

use axum::extract::{Extension, Path, Query};
use chrono::{Local, TimeZone};

use crate::{
    db::{category, topic},
    handler::{get_client, log_error, render, HtmlView},
    view::frontend::topic::{List, Detail, ArchiveList},
    AppState, Result, error::AppError,
};

use super::Args;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let page = args.page();
    let handler_name = "frontend/topic/list";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = topic::list_by_cat(&client, page, id)
        .await
        .map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let archives = topic::archive_list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let cat = category::find(&client, id)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = List {
        list,
        cats,
        archives,
        category_name: cat.name.clone(),
        page,
    };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<HtmlView> {
    let handler_name = "frontend/topic/list";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let archives = topic::archive_list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let item = topic::detail(&client, id).await.map_err(log_error(handler_name))?;
    let tmpl = Detail {
        cats,
        archives,
        item,
    };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn archive(
    Extension(state): Extension<Arc<AppState>>,
    Path(dt): Path<String>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let handler_name = "frontend/topic/archive";
    let dt_str = dt.clone();
    let dt = format!("{}01 00:00:00", dt);
    let dt = Local.datetime_from_str(&dt, "%Y年%m月%d %H:%M:%S").map_err(AppError::from).map_err(log_error(handler_name))?;
    let dt = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    let page = args.page();
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = topic::list_by_arch(&client, page, dt)
        .await
        .map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let archives = topic::archive_list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = ArchiveList {
        list,
        cats,
        archives,
        page,
        dt:dt_str,
    };
    render(tmpl).map_err(log_error(handler_name))
}
