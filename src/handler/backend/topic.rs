use std::{sync::Arc, };

use axum::extract::{Extension, Form, Query};

use crate::{Result, handler::{HtmlView, render, log_error, RedirectView, redirect, get_client}, view::backend::topic::{Add, Index}, AppState, form::CreateTopic, db::{topic, category}};

use super::Args;

pub async fn add_ui(
    Extension(state):Extension<Arc<AppState>>,
) -> Result<HtmlView> {
    let handler_name = "backend/topic/add_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let cats = category::list(&client).await.map_err(log_error(handler_name))?;
    let tmpl = Add { cats };
    render(tmpl).map_err(log_error(handler_name))
}
pub async fn add(
    Extension(state):Extension<Arc<AppState>>,
    Form(frm):Form<CreateTopic>,
)->Result<RedirectView> {
    let handler_name = "backend/topic/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    topic::create(&client, &frm).await.map_err(log_error(handler_name))?;
    redirect("/admin/topic?msg=文章添加成功")
}
pub async fn index(
    Extension(state):Extension<Arc<AppState>>,
    Query(args):Query<Args>,
)->Result<HtmlView> {
    let handler_name = "backend/topic/index";
    let page = args.page();
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = topic::list(&client, page).await.map_err(log_error(handler_name))?;
    let tmpl = Index {
        msg: args.msg.clone(),
        page,
        list,
    };
    render(tmpl).map_err(log_error(handler_name))
}
