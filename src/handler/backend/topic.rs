use std::sync::Arc;

use axum::extract::{Extension, Form, Path, Query};

use crate::{
    db::{category, topic},
    form::{CreateTopic, EditTopic},
    handler::{get_client, log_error, redirect, render, HtmlView, RedirectView},
    view::backend::topic::{Add, Edit, Index},
    AppState, Result,
};

use super::Args;

pub async fn add_ui(Extension(state): Extension<Arc<AppState>>) -> Result<HtmlView> {
    let handler_name = "backend/topic/add_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = Add { cats };
    render(tmpl).map_err(log_error(handler_name))
}
pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<CreateTopic>,
) -> Result<RedirectView> {
    let handler_name = "backend/topic/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    topic::create(&client, &frm)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/topic?msg=文章添加成功")
}
pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let handler_name = "backend/topic/index";
    let page = args.page();
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = topic::list(&client, page)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = Index {
        msg: args.msg.clone(),
        page,
        list,
    };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn edit_ui(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<HtmlView> {
    let handler_name = "backend/topic/edit_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let item = topic::find2edit(&client, id)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = Edit { cats, item };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<EditTopic>,
    Path(id): Path<i64>,
) -> Result<RedirectView> {
    let handler_name = "backend/topic/edit";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    topic::update(&client, &frm, id)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/topic?msg=文章修改成功")
}
pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<RedirectView> {
    let handler_name = "backend/topic/del";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    topic::del_or_restore(&client, id, true).await.map_err(log_error(handler_name))?;
    redirect("/admin/topic?msg=文章删除成功")
}
