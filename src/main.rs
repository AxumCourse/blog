use axum::Router;
use axum_rs_blog::handler::{backend, frontend};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_rs_blog=debug");
    }
    tracing_subscriber::fmt::init();

    tracing::info!("服务已启动");

    let frentend_routers = frontend::router();
    let backend_routers = backend::router();
    let app = Router::new()
        .nest("/", frentend_routers)
        .nest("/admin", backend_routers);

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
