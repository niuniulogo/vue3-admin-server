use std::{net::SocketAddr, time::Duration};

use admin_server::admin_routes::admin_route_init;
use axum::{Extension, Router};
use common::cors_middleware;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod admin_server;
mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //读取本地 env文件 数据库连接地址
    dotenv().ok();

    //  数据库初始化
    let db_connection_str = env::var("DATABASE_URL").expect("---没有配置数据库env---");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .idle_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("------数据库连接失败---");

    let admin_route_init = admin_route_init(); // 后台路由注册
                                               //前台路由注册  todo!()

    // 初始化 前端 后台 路由器
    let app = Router::new()
        .nest("/admin", admin_route_init.await)
        .layer(Extension(pool)) //注入全局共享的 数据库pool
        .layer(cors_middleware()); // 处理跨域

    // 启动HTTP服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 9981));

    println!(
        "-------- 🦀️🦀️ 🦀️🦀️  服务器启动 {} 🦀️🦀️ 🦀️🦀️ -----------",
        addr
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
