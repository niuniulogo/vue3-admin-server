# vue3-admin 后台服务

#Cargo.toml: Rust项目配置文件，包含依赖项和其他配置。
#src/: 项目源代码目录。
#src/main.rs: Web服务入口点，启动HTTP服务器并将请求路由到处理程序。

#src/handlers/: 请求处理程序，负责处理HTTP请求并与数据库交互。
#src/*.middleware.rs: 中间件，用于在响应发送之前或之后拦截请求。
#src/*.models.rs: 数据库模型，定义表格和行的结构。
#src/*.routes.rs: 定义HTTP路由和它们对应的处理程序。
#src/schema.sql: 数据库模式，定义数据库表格结构。

db.env: 包含连接PostgreSQL数据库所需的环境变量。



# todo！token
# todo！redis


项目运行
 cargo watch -x run


macos系统 项目打包

 TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl