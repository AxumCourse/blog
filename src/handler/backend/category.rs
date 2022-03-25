use std::sync::Arc;

use axum::extract::{Extension, Form, Query};

use crate::{Result, handler::{HtmlView, render, log_error, RedirectView, redirect, get_client}, view::backend::category::{Add, Index}, AppState, form, db::category};

use super::Args;

pub async fn add_ui()->Result<HtmlView> {
    let handler_name = "backend/category/add_ui";
    let tmpl = Add{};
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn add(
    Extension(state):Extension<Arc<AppState>>,
    Form(frm):Form<form::CreateCategory>,
) -> Result<RedirectView> {
    let handler_name = "backend/category/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::create(&client, &frm).await.map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=分类添加成功")
}

pub async fn index(
    Extension(state):Extension<Arc<AppState>>,
    Query(args):Query<Args>,
) ->Result<HtmlView> {
    let handler_name = "backend/category/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = category::list(&client).await.map_err(log_error(handler_name))?;
    let tmpl = Index { list, msg:args.msg };
    render(tmpl).map_err(log_error(handler_name))
}
