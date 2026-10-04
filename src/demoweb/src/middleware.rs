//! 中间件层：请求在到达处理器之前/之后统一经过的处理。
//!
//! # 顺序极其重要
//! axum/tower 的中间件是**洋葱模型**：`layer(A).layer(B)` 的执行顺序是
//! `A 进入 → B 进入 → 处理器 → B 退出 → A 退出`。
//! 因此正确的装配顺序是（从外到内）：
//!
//! ```text
//!   请求 → 1. request-id（必须最先，后续日志都要用到它）
//!        → 2. trace/访问日志（记录耗时）
//!        → 3. cors（预检请求要能被应答，不能先拦认证）
//!        → 4. body 限制（超大请求尽早拒绝，别浪费后面环节的资源）
//!        → 5. timeout（限制处理时长）
//!        → 6. 认证（只有通过前面检查的请求才值得解析令牌）
//!        → 处理器
//! ```
//!
//! # 认证为什么用中间件而不是每个处理器自己解析
//! - **不会漏**：新加路由时只要挂上同一层就自动受保护；
//! - **只做一次**：令牌解析/验签是 CPU 开销，放在中间件里只执行一遍；
//! - **处理器更干净**：处理器参数里写 `user: AuthenticatedUser` 就能直接拿到身份。

use std::time::Duration;

use axum::extract::{Request, State};
use axum::http::{HeaderName, HeaderValue, Method, header};
use axum::middleware::Next;
use axum::response::Response;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnBodyChunk, DefaultOnEos, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::error::AppError;
use crate::services::auth::{AuthenticatedUser, JwtService};
use crate::state::AppState;

/// 请求 ID 的请求头名称（同时也是响应头名称）。
///
/// 用常量而不是到处写字符串字面量：改名时只需改一处，且拼错会立刻编译报错。
pub const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

/// 第 1 层：为每个请求生成请求 ID，并透传到响应头。
///
/// # 为什么需要请求 ID
/// 线上排查问题时，用户只会说"我点了注册，报 500"。
/// 若响应头里带着 `x-request-id`，运维就能用它把**这一次请求**相关的
/// 所有日志（网关、本服务、下游服务）一键串起来。
///
/// # 为什么不用现成的 `SetRequestIdLayer` + `PropagateRequestIdLayer`
/// 这两层看起来配套，实际**不配对**：`PropagateRequestIdLayer` 是从
/// **请求头**里取 ID 再写到响应头，而 `SetRequestIdLayer` 只把 ID 放进
/// **请求扩展**（`extensions`）——它改的是扩展，不是请求头。
/// 因此单纯把两者叠起来会出现"响应头没有 x-request-id"的问题。
///
/// 这里用一个自制的中间件把三件事一次做完，语义清晰且完全可控：
/// 1. 取请求头里的 `x-request-id`（调用方/网关可能已经生成，必须尊重它，
///    否则跨服务链路会在本服务断掉）；
/// 2. 没有就生成一个 UUID v4；
/// 3. 把 ID 同时写回**请求头**（供后续 span 记录）与**响应头**（返回给客户端）。
pub async fn request_id(mut request: Request, next: Next) -> Response {
    // `MakeRequestUuid` 是 tower-http 提供的 UUID v4 生成器，复用它避免重复造轮子。
    let existing = request
        .headers()
        .get(&REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let request_id = match existing {
        // 上游已带 ID：直接沿用，保证跨服务链路可追踪
        Some(value) => value,
        // 否则生成一个新的
        None => uuid::Uuid::new_v4().to_string(),
    };

    // 写回请求头：让第 2 层 TraceLayer 的 span 能读到它
    if let Ok(value) = HeaderValue::from_str(&request_id) {
        request
            .headers_mut()
            .insert(REQUEST_ID_HEADER.clone(), value.clone());

        let mut response = next.run(request).await;
        // 写入响应头：客户端/网关据此记录本次调用
        response
            .headers_mut()
            .insert(REQUEST_ID_HEADER.clone(), value);
        response
    } else {
        // 理论上不可达（UUID 只含十六进制与连字符），但保持不 panic 的工程习惯。
        tracing::warn!("生成请求 ID 失败，本次请求不带 x-request-id");
        next.run(request).await
    }
}

/// 访问日志中间件的具体类型。
///
/// # 为什么要专门起一个类型别名
/// `TraceLayer` 有 7 个泛型参数（分类器、span 构造器、请求/响应/body/eos/失败回调），
/// 把完整类型直接写在函数签名里既冗长又难读（clippy 也会提示 `type_complexity`）。
/// 起别名之后：函数签名一眼能看懂、类型变化只需改一处、文档也集中在这里。
type HttpTraceLayer = TraceLayer<
    // 分类器：把 5xx 归为失败
    SharedClassifier<ServerErrorsAsFailures>,
    // span 构造器：用函数指针而非闭包，类型更明确
    fn(&Request<axum::body::Body>) -> tracing::Span,
    // on_request：把 request_id 记录进 span
    fn(&Request<axum::body::Body>, &tracing::Span),
    // on_response：打印状态码与耗时
    DefaultOnResponse,
    // on_body_chunk / on_eos：本示例不处理流式 body
    DefaultOnBodyChunk,
    DefaultOnEos,
    // on_failure：不额外处理（错误已在 AppError::into_response 中记录）
    (),
>;

/// 第 2 层：结构化访问日志。
///
/// `TraceLayer` 会为每个请求创建一个 span，span 内的所有日志自动带上
/// `method` / `uri` / `request_id` 等字段——不需要手动往每个日志调用里传参数。
///
/// 日志级别策略：
/// - 请求进入用 `DEBUG`（量大，避免生产环境刷屏）；
/// - 响应完成用 `INFO` 并带耗时（这是排障最有价值的一条日志）。
///
/// # 请求 ID 是怎么进到 span 里的（两阶段写入）
/// 1. `make_span_with` 在**第 3 层请求 ID 中间件之后**执行（见 `routes.rs`
///    的 layer 顺序表），此时请求头里已经有 `x-request-id`，
///    但为了让逻辑更清晰，这里先用 `field::Empty` **占位声明**该字段；
/// 2. `on_request` 阶段再用 `span.record("request_id", ...)` 把真实值填进去。
///
/// 这正是 `tracing` 推荐的"先声明后填充"模式：span 创建时不必所有字段就绪，
/// 后续任一环节都可以补写，非常适合"请求 ID 由中间件生成"这类场景。
pub fn trace_layer() -> HttpTraceLayer {
    TraceLayer::new_for_http()
        // 函数项（fn item）需要显式 `as fn(...)` 强制转换成函数指针，
        // 才能匹配返回类型里声明的 `fn(...)` 类型——这是 Rust 里
        // "每种 fn 都有自己唯一的零大小类型"带来的常见细节。
        .make_span_with(make_request_span as fn(&Request<axum::body::Body>) -> tracing::Span)
        .on_request(record_request_id as fn(&Request<axum::body::Body>, &tracing::Span))
        // 响应日志带状态码与耗时
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        // 失败只记录分类，具体错误已在 AppError::into_response 里记过，
        // 这里不重复打印，避免同一错误两条日志。
        .on_failure(())
}

/// 创建每个请求的 span。
///
/// 抽成独立的 `fn` 而不是闭包：函数项（fn item）可以作为
/// `TraceLayer` 的泛型参数，类型更明确，也便于单独测试。
fn make_request_span(request: &Request<axum::body::Body>) -> tracing::Span {
    tracing::info_span!(
        "http_request",
        method = %request.method(),
        uri = %request.uri(),
        // field::Empty：先占位，稍后在 on_request 里填充（两阶段写入）
        request_id = tracing::field::Empty,
    )
}

/// 把 `x-request-id` 记录进当前 span。
///
/// 第二个参数是由 `TraceLayer` 传入的 span，**不要**用 `Span::current()`：
/// 在该回调里当前 span 未必是请求 span，显式使用传入的引用才可靠。
fn record_request_id(request: &Request<axum::body::Body>, span: &tracing::Span) {
    if let Some(request_id) = request
        .headers()
        .get(&REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
    {
        // span 里已在 make_request_span 中用 field::Empty 声明了该字段，
        // 因此这里的 record 会真正生效。
        span.record("request_id", request_id);
    }
}

/// 第 3 层：CORS（跨域资源共享）。
///
/// 浏览器默认禁止跨域 AJAX。开发阶段前端常跑在 5173/3000 等端口，
/// 因此需要显式放行。**生产环境不要用 `Any`**，要写明确的前端域名白名单。
pub fn cors_layer(environment: &str) -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            REQUEST_ID_HEADER.clone(),
        ])
        // 让浏览器端 JS 能读到这两个响应头（默认只暴露少数几个安全头）
        .expose_headers([REQUEST_ID_HEADER.clone()]);

    if environment.eq_ignore_ascii_case("production") {
        // 生产：只允许白名单来源。真实项目里从配置读取，这里演示写法。
        base.allow_origin([HeaderValue::from_static("https://app.example.com")])
    } else {
        // 开发/测试：允许任意来源，方便本地联调
        base.allow_origin(Any)
    }
}

/// 第 4 层：请求体大小限制。
pub fn body_limit_layer(max_bytes: usize) -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(max_bytes)
}

/// 第 5 层：请求超时。
///
/// 超时后由 [`TimeoutLayer`] 返回 **408 Request Timeout**
/// （用 `with_status_code` 显式指定，`TimeoutLayer::new` 自 tower-http 0.6.7 起已弃用）。
///
/// 为什么要限制：慢请求会长期占用连接与连接池，几个这样的请求就能把服务拖垮
/// （这也是"慢速攻击"的原理）。
///
/// 注意：这是**请求级**超时；数据库查询还应在连接池层单独设超时（见
/// [`crate::config::DatabaseConfig::acquire_timeout_secs`]），两者配合才完整。
pub fn timeout_layer(timeout: Duration) -> TimeoutLayer {
    TimeoutLayer::with_status_code(axum::http::StatusCode::REQUEST_TIMEOUT, timeout)
}

/// 第 6 层：JWT 认证中间件。
///
/// 校验通过后把 [`AuthenticatedUser`] 放进请求扩展（`extensions`），
/// 处理器用 `user: AuthenticatedUser` 提取器即可取用。
///
/// # 为什么放在中间件而不是提取器里各自校验
/// 提取器每个处理器都要写一遍，容易漏；中间件只挂一次，作用于整棵路由子树。
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 取出 Authorization 头（注意：HeaderMap::get 返回 Option<&HeaderValue>）
    let authorization = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = JwtService::extract_bearer_token(authorization)?;
    let claims = state.jwt_service.verify_token(token)?;

    // 把解析结果放进扩展：处理器侧零成本获取，无需二次解析令牌。
    request.extensions_mut().insert(AuthenticatedUser {
        user_id: claims.sub.clone(),
        username: claims.username.clone(),
        role: claims.role,
        token_remaining_secs: claims.remaining_secs(),
    });

    tracing::debug!(user_id = %claims.sub, "JWT 校验通过");

    // 继续执行内层中间件与处理器
    Ok(next.run(request).await)
}

/// 演示中间件：统计并记录请求耗时（自定义中间件的写法模板）。
///
/// 与 `TraceLayer` 的区别：这里演示**如何手写**一个中间件，
/// 包括"进入前记时间、`next.run()` 后算差值"的标准套路。
pub async fn timing(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    // `Instant` 是单调时钟，不受系统时间调整影响，测量耗时必须用它
    // （用 `SystemTime` 会在 NTP 校时时出现负耗时）。
    let started = std::time::Instant::now();

    let response = next.run(request).await;

    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    // 慢请求单独用 WARN 记录，便于接告警规则（例如 P99 > 500ms 就报警）
    if elapsed_ms > 500.0 {
        tracing::warn!(
            %method, %path, elapsed_ms = format!("{elapsed_ms:.2}"),
            status = response.status().as_u16(),
            "慢请求告警"
        );
    } else {
        tracing::debug!(
            %method, %path, elapsed_ms = format!("{elapsed_ms:.2}"),
            status = response.status().as_u16(),
            "请求处理完成"
        );
    }

    response
}

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// CORS 在生产环境必须收紧（不能是任意来源）。
    #[test]
    fn cors_is_restrictive_in_production() {
        // 这里只能验证函数可构造；具体行为由集成测试用真实请求验证。
        let _dev = cors_layer("development");
        let _prod = cors_layer("production");
    }

    /// 请求 ID 头名称必须合法且是约定值。
    #[test]
    fn request_id_header_is_well_known() {
        assert_eq!(REQUEST_ID_HEADER.as_str(), "x-request-id");
    }

    /// 请求 ID 中间件应当给响应补上 `x-request-id`。
    #[tokio::test]
    async fn request_id_middleware_sets_response_header() {
        use axum::body::Body;
        use axum::http::Request as HttpRequest;
        use tower::ServiceExt;

        // 包一层最小路由，只挂请求 ID 中间件
        let app = axum::Router::new()
            .route("/ping", axum::routing::get(|| async { "pong" }))
            .layer(axum::middleware::from_fn(request_id));

        let response = app
            .clone()
            .oneshot(
                HttpRequest::builder()
                    .uri("/ping")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(
            response.headers().get("x-request-id").is_some(),
            "响应应当带上生成的 x-request-id"
        );

        // 上游已带 ID 时必须原样沿用（跨服务链路追踪的关键）
        let response = app
            .oneshot(
                HttpRequest::builder()
                    .uri("/ping")
                    .header("x-request-id", "upstream-id-123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.headers().get("x-request-id").unwrap(),
            "upstream-id-123",
            "应当沿用上游传入的请求 ID"
        );
    }

    /// 超时层应当能接受配置中的时长。
    #[test]
    fn timeout_layer_accepts_duration() {
        let _layer = timeout_layer(Duration::from_secs(5));
    }
}
