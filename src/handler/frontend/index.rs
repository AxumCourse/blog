
use std::sync::Arc;

use axum::extract::{Extension, Query};

use crate::{Result, handler::{HtmlView, render, log_error, get_client}, view::frontend::index::Index, AppState, db::{topic, category}};

use super::Args;

pub async fn index(
    Extension(state):Extension<Arc<AppState>>,
    Query(args):Query<Args>
)->Result<HtmlView> {
    let page = args.page();
    let handler_name = "frontend/index/index";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = topic::list(&client, page).await.map_err(log_error(handler_name))?;
    let cats = category::list(&client).await.map_err(log_error(handler_name))?;
    let archives = topic::archive_list(&client).await.map_err(log_error(handler_name))?;
    let tmpl = Index{
        list,
        page,
        cats,
        archives,
    };
    render(tmpl).map_err(log_error(handler_name))
}
