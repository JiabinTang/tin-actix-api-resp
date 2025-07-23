# tin-actix-api-resp

基于 Actix-Web 的统一 API 响应封装库

## 简介

tin-actix-api-resp 提供了统一的 API 响应结构体 `ApiRes<T>`，支持标准 HTTP 状态码、业务自定义 code、消息 message 及数据 data，便于前后端协作和接口规范化。

- 支持所有常见 HTTP 状态码的响应类型
- 兼容 actix-web 的 Responder/ResponseError
- 响应体自动序列化为 JSON
- 便于自定义 message/code/data

## 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tin-actix-api-resp = { path = "." }
actix-web = "4"
serde = { version = "1", features = ["derive"] }
```

## 快速开始

```rust
use tin_actix_api_resp::{ApiRes, ApiResult};
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> ApiResult<String> {
    Ok(ApiRes::Ok("Hello, world!".to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

## 响应结构

成功响应：
```json
{
  "code": 200,
  "message": "Success",
  "data": "Hello, world!"
}
```

错误响应：
```json
{
  "code": 404,
  "message": "Not Found"
}
```

## API 说明

- `ApiRes<T>`：统一响应类型，支持 Ok/OkWith/Custom 及所有常见 HTTP 错误类型
- `ApiResult<T>`：类型别名，等价于 `Result<ApiRes<T>, ApiErr>`
- `ApiErr`：类型别名，等价于 `ApiRes<()>`

### 常用构造方法

```rust
ApiRes::Ok(data) // 200 成功
ApiRes::OkWith(201, "Created".into(), data) // 200+自定义 code/message
ApiRes::NotFound("资源不存在".into()) // 404
ApiRes::Custom(400, 10001, "参数错误".into(), None) // 自定义
```

## License

MIT
