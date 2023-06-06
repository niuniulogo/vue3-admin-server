use axum::{routing::get, Router};

// 后端服务注册路由
pub async fn admin_route_init() -> Router {
    Router::new().route(
        "/login",
        get(|| async { "admin --init --- Hello, world! oo" }),
    )
}
