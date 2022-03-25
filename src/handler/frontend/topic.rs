use std::sync::Arc;

use axum::extract::{Extension, Path, Query};

use crate::{
    db::{category, topic},
    handler::{get_client, log_error, render, HtmlView},
    view::frontend::topic::List,
    AppState, Result,
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
