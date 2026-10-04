//! 路由装配：把处理器、中间件、状态组合成完整的应用。
//!
//! # 洋葱模型：`.layer()` 的书写顺序与执行顺序是**相反**的
//! ```text
//!   Router::new().layer(A).layer(B)
//!   ⇒ 请求先经过 B，再经过 A；响应则先出 A，再出 B
//! ```
//! 因此本文件的书写顺序是"内层先写、外层后写"，
//! 而实际请求穿过中间件的顺序见下方注释（从 1 到 6）。
//!
//! # 路由分组
//! - `/health`、`/api/health`：**不需要认证**（编排器/监控要能无凭据访问）；
//! - `/api/users/register`、`/api/users/login`：**不需要认证**（还没有令牌）；
//! - `/api/users/me`、`/api/users/admin/*`：**需要认证**（挂 `require_auth` 中间件）。
//!
//! 把"需要认证的路由"单独组成一个 [`Router`] 再统一挂中间件，
//! 是避免"新加路由忘了加认证"的最佳实践——**默认受保护，例外才显式开放**。

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};

use crate::controllers::{demo, health, users};
use crate::middleware;
use crate::state::AppState;

/// 构建完整的应用路由（含全部中间件）。
///
/// 返回的 [`Router`] 可以直接交给 `axum::serve`，也可以在测试里
/// 用 `tower::ServiceExt::oneshot` 直接调用（无需真实网络端口）。
pub fn build_router(state: AppState) -> Router {
    // ------------------------------------------------------------------
    // 第 1 组：公开路由（无需认证）
    // ------------------------------------------------------------------
    let public_routes = Router::new()
        // 存活探针：极轻量，供 K8s livenessProbe 使用
        .route("/health", get(health::liveness))
        // 就绪探针：会检查数据库
        .route("/api/health", get(health::readiness))
        // 注册与登录：此时用户还没有令牌，必须公开
        .route("/api/users/register", post(users::register))
        .route("/api/users/login", post(users::login));

    // ------------------------------------------------------------------
    // 第 2 组：需要认证的路由
    //
    // 注意：`route_layer` 只作用于**已经注册的路由**，
    // 不会影响后面 merge 进来的其它路由；同时它也不会作用于 404 兜底处理。
    // 这两个特性正好符合我们的需求。
    // ------------------------------------------------------------------
    let protected_routes = Router::new()
        .route("/api/users/me", get(users::me))
        // 管理员接口：认证之外还要在处理器里做授权判断（403）
        .route("/api/users/admin/ping", get(users::admin_ping))
        // 需要携带 Bearer 令牌才能访问
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth,
        ));

    // ------------------------------------------------------------------
    // 第 3 组：并发演示路由（同样需要认证，用于观察"谁在调用"）
    // ------------------------------------------------------------------
    let demo_routes = Router::new()
        .route("/api/demo/parallel", get(demo::parallel))
        .route("/api/demo/spawn", get(demo::spawn_background))
        .route("/api/demo/counter", get(demo::counter))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth,
        ));

    // ------------------------------------------------------------------
    // 组装：合并路由 + 挂全局中间件 + 注入状态
    // ------------------------------------------------------------------
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(demo_routes)
        // 未匹配任何路由时的统一响应（默认是空 body 的 404，前端不友好）
        .fallback(not_found)
        // 请求体大小限制：`DefaultBodyLimit` 是 axum 内建机制，
        // 与 tower-http 的 RequestBodyLimitLayer 二选一即可，这里用内建的更省一层。
        .layer(DefaultBodyLimit::max(state.config.server.max_body_bytes))
        // ============ 以下 layer 的**执行顺序从下往上** ============
        // 6. 超时（最内层：只统计"处理器 + 提取器"的时间，不含前置中间件开销）
        .layer(middleware::timeout_layer(state.config.request_timeout()))
        // 5. 自定义耗时中间件（慢请求告警）
        .layer(axum::middleware::from_fn(middleware::timing))
        // 4. 访问日志（此时请求头里已有 request-id，span 能带上它）
        .layer(middleware::trace_layer())
        // 3. 请求 ID：生成/沿用 ID，写入请求头（供第 4 层记录）与响应头
        .layer(axum::middleware::from_fn(middleware::request_id))
        // 2. CORS（必须在请求 ID 之外层：连 OPTIONS 预检与错误响应也要带 CORS 头，
        //    否则浏览器只会显示"跨域失败"，看不到真实的错误原因）
        .layer(middleware::cors_layer(&state.config.environment))
        // 1. 注入共享状态（必须最后，它是所有处理器与中间件的数据来源）
        .with_state(state)
}

/// 404 兜底处理：返回结构化 JSON，而不是空 body。
///
/// 为什么重要：前端拿到空 body 的 404 只能显示"请求失败"，
/// 而结构化错误体可以统一走与其它错误相同的解析逻辑。
async fn not_found(request: axum::extract::Request) -> crate::error::AppError {
    crate::error::AppError::NotFound(format!(
        "接口不存在：{} {}",
        request.method(),
        request.uri().path()
    ))
}

/// 仅供测试使用：构造一个内存仓储的应用（避免测试文件重复写装配代码）。
#[doc(hidden)]
pub fn build_router_for_test() -> Router {
    use crate::repository::InMemoryUserRepository;
    use crate::services::JwtService;
    use std::sync::Arc;

    let config = crate::config::AppConfig {
        environment: "test".to_string(),
        ..crate::config::AppConfig::default()
    };
    let jwt = Arc::new(JwtService::new(&config.jwt).expect("构造 JWT 服务失败"));
    let state = AppState::new(
        config,
        InMemoryUserRepository::shared() as Arc<dyn crate::repository::UserRepository>,
        jwt,
    );
    build_router(state)
}

// ============================================================================
// 单元测试：直接调用路由（不监听端口）
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::models::{LoginRequest, RegisterRequest};
    use crate::repository::{InMemoryUserRepository, UserRepository};
    use crate::services::JwtService;
    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use std::sync::Arc;
    use tower::ServiceExt; // 提供 `oneshot`

    /// 构造测试用应用（内存仓储 + 测试环境配置）。
    pub(crate) fn test_app() -> Router {
        let config = AppConfig {
            environment: "test".to_string(),
            ..AppConfig::default()
        };
        let jwt = Arc::new(JwtService::new(&config.jwt).expect("构造 JWT 服务失败"));
        let state = AppState::new(
            config,
            InMemoryUserRepository::shared() as Arc<dyn UserRepository>,
            jwt,
        );
        build_router(state)
    }

    /// 发送一个请求并返回状态码与响应体字符串。
    async fn send(app: Router, request: Request<Body>) -> (StatusCode, String) {
        let response = app.oneshot(request).await.expect("请求执行失败");
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("读取响应体失败");
        (status, String::from_utf8_lossy(&bytes).to_string())
    }

    fn json_request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap()
    }

    #[tokio::test]
    async fn liveness_returns_ok() {
        let (status, body) = send(
            test_app(),
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("ok"));
    }

    /// 注册成功要走通 201，并返回 JSON。
    #[tokio::test]
    async fn register_endpoint_returns_201() {
        let body = serde_json::to_value(RegisterRequest {
            username: "alice_01".to_string(),
            email: "alice@example.com".to_string(),
            password: "secret123".to_string(),
        })
        .unwrap();

        let (status, response) = send(
            test_app(),
            json_request("POST", "/api/users/register", body),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED, "响应体：{response}");
        assert!(response.contains("alice@example.com"));
        assert!(!response.contains("password"), "响应不应包含密码字段");
    }

    /// 受保护接口在缺少令牌时必须返回 401，且响应是结构化 JSON。
    #[tokio::test]
    async fn protected_route_requires_token() {
        let (status, body) = send(
            test_app(),
            Request::builder()
                .uri("/api/users/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(body.contains("UNAUTHORIZED"), "响应体：{body}");
    }

    /// 伪造令牌必须被拒绝。
    #[tokio::test]
    async fn forged_token_is_rejected() {
        let request = Request::builder()
            .uri("/api/users/me")
            .header(header::AUTHORIZATION, "Bearer not.a.real.token")
            .body(Body::empty())
            .unwrap();
        let (status, _) = send(test_app(), request).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    /// 完整闭环：注册 → 登录 → 带令牌访问 /me。
    #[tokio::test]
    async fn full_register_login_me_flow() {
        let app = test_app();

        // 1) 注册
        let register_body = serde_json::to_value(RegisterRequest {
            username: "bob_01".to_string(),
            email: "bob@example.com".to_string(),
            password: "secret123".to_string(),
        })
        .unwrap();
        let (status, _) = send(
            app.clone(),
            json_request("POST", "/api/users/register", register_body),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);

        // 2) 登录拿令牌
        let login_body = serde_json::to_value(LoginRequest {
            account: "bob@example.com".to_string(),
            password: "secret123".to_string(),
        })
        .unwrap();
        let (status, body) = send(
            app.clone(),
            json_request("POST", "/api/users/login", login_body),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "登录响应：{body}");

        let parsed: serde_json::Value = serde_json::from_str(&body).expect("登录响应应是合法 JSON");
        let token = parsed["data"]["access_token"]
            .as_str()
            .expect("响应应包含 access_token");

        // 3) 带令牌访问 /me
        let request = Request::builder()
            .uri("/api/users/me")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap();
        let (status, body) = send(app, request).await;
        assert_eq!(status, StatusCode::OK, "响应体：{body}");
        assert!(body.contains("bob@example.com"));
    }

    /// 未知路径应当返回结构化 404，而不是空 body。
    #[tokio::test]
    async fn unknown_route_returns_structured_404() {
        let (status, body) = send(
            test_app(),
            Request::builder()
                .uri("/api/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body.contains("NOT_FOUND"), "响应体：{body}");
    }

    /// 每个响应都应当带上 `x-request-id`（可观测性的基础）。
    #[tokio::test]
    async fn response_carries_request_id() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(
            response.headers().get("x-request-id").is_some(),
            "响应应包含 x-request-id 头"
        );
    }

    /// 非法 JSON 应当返回 400 而不是 500。
    #[tokio::test]
    async fn malformed_json_returns_400() {
        let request = Request::builder()
            .method("POST")
            .uri("/api/users/register")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{ 这不是合法 JSON }"))
            .unwrap();
        let (status, _) = send(test_app(), request).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
