// 这个是公共资源 例如定义 返回统一格式的数据格式

use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

// 处理跨域
pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}
