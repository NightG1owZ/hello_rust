//! 集成测试：启动**真实的 HTTP 服务**，用真实客户端发请求。
//!
//! # 与单元测试的分工
//! - 单元测试（`src/**` 里的 `#[cfg(test)] mod tests`）：只测一个函数/一个类型，
//!   依赖用测试替身（如内存仓储），毫秒级完成；
//! - 集成测试（本文件）：从 TCP 端口进、从 TCP 端口出，
//!   **完整覆盖**中间件链、路由、序列化、状态码等真实链路。
//!
//! 两者都不能省：单元测试定位问题快，集成测试才能证明"整体真的能跑"。
//!
//! # 为什么绑定 `127.0.0.1:0`
//! 端口号写 0 表示"由操作系统分配一个空闲端口"，
//! 这样可以并行运行多个测试实例而不会互相抢端口（CI 上尤其重要）。
//! 绑定后用 `local_addr()` 反查真实端口。

use std::sync::Arc;

use demoweb::config::AppConfig;
use demoweb::repository::{InMemoryUserRepository, UserRepository};
use demoweb::state::AppState;
use demoweb::{build_router, db};
use serde_json::json;

/// 测试用服务器句柄：持有后台任务与访问地址。
struct TestServer {
    base_url: String,
    _handle: tokio::task::JoinHandle<()>,
}

impl TestServer {
    /// 拼出完整 URL。
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

/// 启动一个测试服务器（内存仓储）。
async fn spawn_server() -> TestServer {
    spawn_server_with(|config| {
        // 测试环境：JWT 密钥固定，bcrypt 使用低代价，日志降噪
        config.environment = "test".to_string();
        config.log.level = "warn".to_string();
        config.database.url = "sqlite::memory:".to_string();
    })
    .await
}

/// 启动测试服务器，允许测试自定义配置。
///
/// 这里刻意采用"先取默认值，再交给调用方修改"的写法，
/// 因为它比"让调用方构造完整配置"更适合测试（每个用例只关心自己要改的那几项）。
/// clippy 的 `field_reassign_with_default` 在此属于误报，故显式放行并说明原因。
#[allow(clippy::field_reassign_with_default)]
async fn spawn_server_with(customize: impl FnOnce(&mut AppConfig)) -> TestServer {
    let mut config = AppConfig::default();
    customize(&mut config);

    // 与 main 一致：用 SQLite 仓储（可运行真实迁移），确保测试覆盖数据库路径
    let user_repository: Arc<dyn UserRepository> = if config.database.url.starts_with("sqlite") {
        let pool = db::init(&config).await.expect("初始化测试数据库失败");
        Arc::new(demoweb::repository::SqliteUserRepository::new(pool))
    } else {
        InMemoryUserRepository::shared()
    };

    let jwt = Arc::new(demoweb::services::JwtService::new(&config.jwt).expect("构造 JWT 失败"));
    let state = AppState::new(config, user_repository, jwt);
    let app = build_router(state);

    // 端口 0 → 由系统分配空闲端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("绑定测试端口失败");
    let addr = listener.local_addr().expect("获取测试端口失败");

    let handle = tokio::spawn(async move {
        // 测试里不需要优雅停机，直接 serve 到测试结束
        let _ = axum::serve(listener, app).await;
    });

    TestServer {
        base_url: format!("http://{addr}"),
        _handle: handle,
    }
}

/// 构造 HTTP 客户端：超时设短一些，避免测试挂死。
fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .expect("构造 HTTP 客户端失败")
}

// ============================================================================
// 健康检查
// ============================================================================

#[tokio::test]
async fn health_endpoints_are_public() {
    let server = spawn_server().await;

    // 存活探针
    let response = client()
        .get(server.url("/health"))
        .send()
        .await
        .expect("请求 /health 失败");
    assert_eq!(response.status(), 200);

    // 就绪探针（会真的查一次数据库）
    let response = client()
        .get(server.url("/api/health"))
        .send()
        .await
        .expect("请求 /api/health 失败");
    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.expect("响应应为 JSON");
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["status"], "ready");
    // 每个响应都应有 request-id，方便串联日志
    assert!(body["data"]["version"].is_string());
}

// ============================================================================
// 注册接口
// ============================================================================

#[tokio::test]
async fn register_creates_user_and_hides_password() {
    let server = spawn_server().await;

    let response = client()
        .post(server.url("/api/users/register"))
        .json(&json!({
            "username": "alice_01",
            "email": "alice@example.com",
            "password": "secret123"
        }))
        .send()
        .await
        .expect("注册请求失败");

    assert_eq!(response.status(), 201, "注册应返回 201 Created");
    // 响应头必须带 request-id（可观测性要求）
    assert!(response.headers().get("x-request-id").is_some());

    let body: serde_json::Value = response.json().await.expect("响应应为 JSON");
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["username"], "alice_01");
    assert_eq!(body["data"]["email"], "alice@example.com");
    assert_eq!(body["data"]["role"], "user");

    // 关键安全断言：响应体里不能出现任何密码相关字段
    let raw = body.to_string();
    assert!(!raw.contains("password"), "响应体不得包含密码：{raw}");
}

#[tokio::test]
async fn register_rejects_invalid_payload_with_field_errors() {
    let server = spawn_server().await;

    let response = client()
        .post(server.url("/api/users/register"))
        .json(&json!({
            "username": "a",                 // 太短
            "email": "not-an-email",         // 非法邮箱
            "password": "short"              // 太短且无数字
        }))
        .send()
        .await
        .expect("注册请求失败");

    assert_eq!(response.status(), 400);

    let body: serde_json::Value = response.json().await.expect("响应应为 JSON");
    assert_eq!(body["success"], false);
    assert_eq!(body["code"], "VALIDATION_FAILED");

    // 应逐字段返回错误，方便前端高亮
    let errors = body["errors"].as_array().expect("errors 应为数组");
    assert!(!errors.is_empty(), "应当返回字段级错误");
    let fields: Vec<&str> = errors.iter().filter_map(|e| e["field"].as_str()).collect();
    assert!(
        fields.contains(&"email"),
        "应当指出 email 字段错误：{fields:?}"
    );
    assert!(
        fields.contains(&"password"),
        "应当指出 password 字段错误：{fields:?}"
    );
}

#[tokio::test]
async fn register_rejects_duplicate_email_with_409() {
    let server = spawn_server().await;
    let payload = json!({
        "username": "bob_01",
        "email": "bob@example.com",
        "password": "secret123"
    });

    let first = client()
        .post(server.url("/api/users/register"))
        .json(&payload)
        .send()
        .await
        .unwrap();
    assert_eq!(first.status(), 201);

    // 换用户名、留同邮箱 → 冲突
    let second = client()
        .post(server.url("/api/users/register"))
        .json(&json!({
            "username": "bob_02",
            "email": "bob@example.com",
            "password": "secret123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(second.status(), 409, "重复邮箱应返回 409 Conflict");

    let body: serde_json::Value = second.json().await.unwrap();
    assert_eq!(body["code"], "USER_EMAIL_EXISTS", "应返回精确的业务错误码");
}

#[tokio::test]
async fn register_rejects_malformed_json_with_400() {
    let server = spawn_server().await;

    let response = client()
        .post(server.url("/api/users/register"))
        .header("content-type", "application/json")
        .body("{ 这可不是 JSON }")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400, "非法 JSON 应返回 400 而不是 500");
}

// ============================================================================
// 登录接口
// ============================================================================

/// 注册 → 登录 → 用令牌访问 /me 的完整闭环。
#[tokio::test]
async fn login_returns_token_and_grants_access_to_me() {
    let server = spawn_server().await;
    let http = client();

    // 1) 注册
    http.post(server.url("/api/users/register"))
        .json(&json!({
            "username": "carol_01",
            "email": "carol@example.com",
            "password": "secret123"
        }))
        .send()
        .await
        .unwrap();

    // 2) 登录（用邮箱）
    let login = http
        .post(server.url("/api/users/login"))
        .json(&json!({ "account": "carol@example.com", "password": "secret123" }))
        .send()
        .await
        .unwrap();
    assert_eq!(login.status(), 200, "登录应成功");

    let body: serde_json::Value = login.json().await.unwrap();
    let token = body["data"]["access_token"]
        .as_str()
        .expect("响应应包含 access_token")
        .to_string();
    assert_eq!(body["data"]["token_type"], "Bearer");
    assert!(body["data"]["expires_in"].as_i64().unwrap() > 0);

    // 3) 带令牌访问受保护接口
    let me = http
        .get(server.url("/api/users/me"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert_eq!(me.status(), 200, "携带有效令牌应能访问 /me");

    let me_body: serde_json::Value = me.json().await.unwrap();
    assert_eq!(me_body["data"]["email"], "carol@example.com");
}

/// 也支持用用户名登录。
#[tokio::test]
async fn login_with_username_works() {
    let server = spawn_server().await;
    let http = client();

    http.post(server.url("/api/users/register"))
        .json(&json!({
            "username": "dave_01",
            "email": "dave@example.com",
            "password": "secret123"
        }))
        .send()
        .await
        .unwrap();

    let login = http
        .post(server.url("/api/users/login"))
        .json(&json!({ "account": "dave_01", "password": "secret123" }))
        .send()
        .await
        .unwrap();
    assert_eq!(login.status(), 200);
}

#[tokio::test]
async fn login_with_wrong_password_returns_401() {
    let server = spawn_server().await;
    let http = client();

    http.post(server.url("/api/users/register"))
        .json(&json!({
            "username": "erin_01",
            "email": "erin@example.com",
            "password": "secret123"
        }))
        .send()
        .await
        .unwrap();

    let response = http
        .post(server.url("/api/users/login"))
        .json(&json!({ "account": "erin@example.com", "password": "wrong-password" }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["code"], "UNAUTHORIZED");

    // 安全要求：不能泄露"账号不存在"与"密码错误"的区别。
    // 这里只断言核心文案，不锁定前缀（前缀由错误类型的 Display 决定）。
    let message = body["message"].as_str().expect("message 应为字符串");
    assert!(
        message.contains("账号或密码错误"),
        "统一失败文案应包含「账号或密码错误」，实际：{message}"
    );
    assert!(
        !message.contains("不存在") && !message.contains("用户不存在"),
        "不得暴露账号是否存在：{message}"
    );
}

// ============================================================================
// 认证与授权
// ============================================================================

#[tokio::test]
async fn protected_route_without_token_returns_401() {
    let server = spawn_server().await;

    let response = client()
        .get(server.url("/api/users/me"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["success"], false);
    assert_eq!(body["code"], "UNAUTHORIZED");
}

#[tokio::test]
async fn protected_route_with_forged_token_returns_401() {
    let server = spawn_server().await;

    // 一个结构正确但签名无效的令牌
    let response = client()
        .get(server.url("/api/users/me"))
        .bearer_auth("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoYWNrZXIifQ.forged-signature")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn wrong_auth_scheme_returns_401() {
    let server = spawn_server().await;

    let response = client()
        .get(server.url("/api/users/me"))
        .header("authorization", "Basic dXNlcjpwYXNz")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "非 Bearer 方案应被拒绝");
}

/// 普通用户访问管理接口 → 403（已认证但无权限）。
#[tokio::test]
async fn admin_endpoint_returns_403_for_normal_user() {
    let server = spawn_server().await;
    let http = client();

    http.post(server.url("/api/users/register"))
        .json(&json!({
            "username": "frank_01",
            "email": "frank@example.com",
            "password": "secret123"
        }))
        .send()
        .await
        .unwrap();

    let login: serde_json::Value = http
        .post(server.url("/api/users/login"))
        .json(&json!({ "account": "frank@example.com", "password": "secret123" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let token = login["data"]["access_token"].as_str().unwrap().to_string();

    let response = http
        .get(server.url("/api/users/admin/ping"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();

    // 401 = "你是谁不知道"；403 = "知道你是谁，但你没权限"
    assert_eq!(response.status(), 403);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["code"], "FORBIDDEN");
}

// ============================================================================
// 并发演示接口
// ============================================================================

#[tokio::test]
async fn parallel_demo_shows_concurrency_benefit() {
    let server = spawn_server().await;
    let token = register_and_login(&server, "grace_01", "grace@example.com").await;

    let response = client()
        .get(server.url("/api/demo/parallel"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let elapsed = body["data"]["concurrent_elapsed_ms"].as_u64().unwrap();
    let sequential = body["data"]["sequential_estimate_ms"].as_u64().unwrap();

    assert!(
        elapsed < sequential,
        "并发耗时 {elapsed}ms 应小于串行估算 {sequential}ms"
    );
}

#[tokio::test]
async fn spawn_demo_returns_all_task_results() {
    let server = spawn_server().await;
    let token = register_and_login(&server, "heidi_01", "heidi@example.com").await;

    let response = client()
        .get(server.url("/api/demo/spawn"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let tasks = body["data"]["tasks"].as_array().expect("tasks 应为数组");
    assert_eq!(tasks.len(), 8, "应当返回 8 个任务的结果");

    // 校验计算结果：1..=n 的平方和
    for task in tasks {
        let id = task["task_id"].as_u64().unwrap();
        let value = task["sum_of_squares"].as_u64().unwrap();
        let expected: u64 = (1..=id).map(|n| n * n).sum();
        assert_eq!(value, expected, "任务 {id} 的结果不正确");
    }
}

/// 并发 20 个真实 HTTP 请求，验证服务端在并发下不丢计数、不报错。
#[tokio::test]
async fn concurrent_requests_are_all_served() {
    let server = spawn_server().await;
    let token = register_and_login(&server, "ivan_01", "ivan@example.com").await;
    let http = client();
    let base = server.base_url.clone();

    let mut handles = Vec::new();
    for _ in 0..20 {
        let http = http.clone();
        let token = token.clone();
        let url = format!("{base}/api/demo/counter");
        // 注意：这里用 reqwest（真实网络 IO），20 个请求会真正并发地打到服务端
        handles.push(tokio::spawn(async move {
            http.get(&url)
                .bearer_auth(&token)
                .send()
                .await
                .map(|r| r.status().as_u16())
        }));
    }

    let mut statuses = Vec::new();
    for handle in handles {
        statuses.push(handle.await.expect("任务 panic").expect("请求失败"));
    }

    assert_eq!(statuses.len(), 20);
    assert!(
        statuses.iter().all(|s| *s == 200),
        "所有并发请求都应当成功：{statuses:?}"
    );
}

// ============================================================================
// 测试辅助函数
// ============================================================================

/// 注册一个用户并登录，返回访问令牌。
async fn register_and_login(server: &TestServer, username: &str, email: &str) -> String {
    let http = client();

    let register = http
        .post(server.url("/api/users/register"))
        .json(&json!({
            "username": username,
            "email": email,
            "password": "secret123"
        }))
        .send()
        .await
        .expect("注册请求失败");
    assert!(
        register.status().is_success(),
        "注册应当成功，实际状态：{}",
        register.status()
    );

    let login: serde_json::Value = http
        .post(server.url("/api/users/login"))
        .json(&json!({ "account": email, "password": "secret123" }))
        .send()
        .await
        .expect("登录请求失败")
        .json()
        .await
        .expect("登录响应应为 JSON");

    login["data"]["access_token"]
        .as_str()
        .expect("登录响应应包含 access_token")
        .to_string()
}
