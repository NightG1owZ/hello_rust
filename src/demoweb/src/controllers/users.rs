//! 用户相关接口：注册、登录、查询当前用户。
//!
//! # 提取器（Extractor）是怎么工作的
//! axum 处理器参数里的每个类型都是一次"提取"，例如：
//! - [`State<AppState>`]：从路由状态里取出共享状态（克隆 `Arc`）；
//! - [`Json<RegisterRequest>`]：读取请求体并按 JSON 反序列化；
//! - [`AuthenticatedUser`]：从请求扩展里取认证信息（由认证中间件写入）。
//!
//! 提取失败会**短路**返回错误（例如 JSON 格式错误），处理器函数体根本不会执行。
//! 顺序也有讲究：**消耗请求体的提取器（`Json`）必须放在最后**，
//! 因为请求体只能被读取一次。

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::{ApiResponse, LoginRequest, LoginResponse, RegisterRequest, UserResponse};
use crate::services::auth::AuthenticatedUser;
use crate::state::AppState;

/// `POST /api/users/register` —— 用户注册。
///
/// # 请求
/// ```json
/// { "username": "alice_01", "email": "alice@example.com", "password": "secret123" }
/// ```
///
/// # 响应
/// - `201 Created`：注册成功，返回用户公开信息（**不含密码哈希**）；
/// - `400 Bad Request`：字段校验失败，`errors` 数组逐字段说明原因；
/// - `409 Conflict`：邮箱或用户名已被占用（业务错误码区分二者）；
/// - `413/408/500`：由中间件或内部错误产生。
///
/// # 并发安全提醒
/// 本接口的"查重"与"插入"之间存在时间窗口，
/// 最终一致性由数据库的 `UNIQUE` 约束保证（见 `repository::map_unique_violation`）。
pub async fn register(
    State(state): State<AppState>,
    // `Json` 放在最后一个参数：它会消耗请求体
    Json(payload): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UserResponse>>)> {
    // 声明式校验：规则写在 DTO 上，这里一行搞定；
    // `?` 会把 ValidationErrors 自动转成 400 + 逐字段错误。
    payload.validate()?;

    // 服务层的构造在 main 中完成一次，这里复用同一套依赖（仓储/哈希器/JWT）。
    let service = state.user_service();
    let user = service.register(payload).await?;

    // 201 表示"资源已创建"，并在语义上区别于 200（仅成功处理，未创建资源）。
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(user))))
}

/// `POST /api/users/login` —— 用户登录。
///
/// # 请求
/// ```json
/// { "account": "alice@example.com", "password": "secret123" }
/// ```
/// `account` 既可以是邮箱也可以是用户名。
///
/// # 响应
/// - `200 OK`：返回用户信息 + `access_token`；
/// - `400 Bad Request`：字段缺失或格式错误；
/// - `401 Unauthorized`：账号或密码错误（**故意不区分两者**，防账号枚举）。
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<LoginResponse>>> {
    payload.validate()?;

    let service = state.user_service();
    let login = service.login(payload).await?;

    Ok(Json(ApiResponse::ok(login)))
}

/// `GET /api/users/me` —— 查询当前登录用户。
///
/// # 认证
/// 需要请求头 `Authorization: Bearer <token>`。
/// 这里的 `user: AuthenticatedUser` 参数之所以能直接取到值，
/// 是因为该路由挂了认证中间件（见 `routes::build_router`）。
pub async fn me(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let service = state.user_service();
    let profile = service.find_by_id(&user.user_id).await?;

    Ok(Json(ApiResponse::ok(profile)))
}

/// `GET /api/users/admin/ping` —— 演示基于角色的访问控制（RBAC）。
///
/// 认证（Authentication，"你是谁"）由中间件完成；
/// **授权**（Authorization，"你能做什么"）必须在业务代码里判断——
/// 这是两件不同的事，初学者最容易混淆。
pub async fn admin_ping(
    user: AuthenticatedUser,
) -> AppResult<Json<ApiResponse<AdminPingResponse>>> {
    if !user.is_admin() {
        // 已登录但权限不足 → 403（不是 401：401 表示"未认证"，
        // 客户端收到 401 会去重新登录，而重新登录并不能解决权限问题）。
        return Err(AppError::Forbidden("该接口仅管理员可访问".to_string()));
    }

    Ok(Json(ApiResponse::ok(AdminPingResponse {
        message: "管理员你好，认证与授权都已通过",
        operator: user.username,
        token_remaining_secs: user.token_remaining_secs,
    })))
}

/// 管理员接口响应体。
#[derive(Debug, serde::Serialize)]
pub struct AdminPingResponse {
    /// 提示信息。
    pub message: &'static str,
    /// 操作人用户名（来自 JWT 声明，无需再查库）。
    pub operator: String,
    /// 令牌剩余有效期，方便客户端决定何时续期。
    pub token_remaining_secs: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, JwtConfig};
    use crate::models::UserRole;
    use crate::repository::{InMemoryUserRepository, UserRepository};
    use crate::services::{JwtService, PasswordHasher};

    /// 构造一个不依赖数据库的测试状态。
    fn test_state() -> AppState {
        let config = AppConfig {
            environment: "test".to_string(),
            ..AppConfig::default()
        };
        let jwt = std::sync::Arc::new(
            JwtService::new(&JwtConfig {
                secret: "unit-test-secret-key-0123456789".to_string(),
                expires_hours: 1,
                issuer: "demoweb-test".to_string(),
            })
            .unwrap(),
        );
        AppState::new(
            config,
            InMemoryUserRepository::shared() as std::sync::Arc<dyn UserRepository>,
            jwt,
        )
    }

    /// 注册接口应当返回 201，并且响应的 JSON 里没有密码字段。
    #[tokio::test]
    async fn register_returns_created_without_password() {
        let state = test_state();
        let (status, Json(body)) = register(
            State(state),
            Json(RegisterRequest {
                username: "alice_01".to_string(),
                email: "alice@example.com".to_string(),
                password: "secret123".to_string(),
            }),
        )
        .await
        .expect("注册应当成功");

        assert_eq!(status, StatusCode::CREATED);
        assert!(body.success);

        let json = serde_json::to_string(&body).unwrap();
        assert!(!json.contains("password"), "响应不能包含密码字段：{json}");
    }

    /// 非法邮箱应当在控制器层就被拦下（返回 400），不会进入服务层。
    #[tokio::test]
    async fn register_rejects_invalid_email_with_400() {
        let state = test_state();
        let err = register(
            State(state),
            Json(RegisterRequest {
                username: "alice_01".to_string(),
                email: "not-an-email".to_string(),
                password: "secret123".to_string(),
            }),
        )
        .await
        .expect_err("非法邮箱应当被拒绝");

        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(err.code(), "VALIDATION_FAILED");
    }

    /// 重复注册同一个邮箱应当得到 409 与精确的业务错误码。
    #[tokio::test]
    async fn register_conflict_returns_409() {
        let state = test_state();
        let payload = RegisterRequest {
            username: "alice_01".to_string(),
            email: "alice@example.com".to_string(),
            password: "secret123".to_string(),
        };
        // `Json<T>` 被标注了 `#[must_use]`，因此返回值必须显式接收；
        // 顺便把响应体也断言一下，测试更有价值。
        let (status, Json(created)) = register(State(state.clone()), Json(payload.clone()))
            .await
            .expect("首次注册应成功");
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(created.data.username, "alice_01");

        let err = register(State(state), Json(payload))
            .await
            .expect_err("重复注册应当失败");
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
        assert_eq!(err.code(), "USER_EMAIL_EXISTS");
    }

    /// 登录成功后再调用 `/me`，应当能取到同一个人（闭环验证）。
    #[tokio::test]
    async fn login_then_me_round_trip() {
        let state = test_state();
        let (status, _) = register(
            State(state.clone()),
            Json(RegisterRequest {
                username: "alice_01".to_string(),
                email: "alice@example.com".to_string(),
                password: "secret123".to_string(),
            }),
        )
        .await
        .unwrap();
        assert_eq!(status, StatusCode::CREATED);

        let Json(login_body) = login(
            State(state.clone()),
            Json(LoginRequest {
                account: "alice@example.com".to_string(),
                password: "secret123".to_string(),
            }),
        )
        .await
        .expect("登录应成功");

        // 手工模拟认证中间件的产物（真实流程里由中间件写入 extensions）
        let claims = state
            .jwt_service
            .verify_token(&login_body.data.access_token)
            .expect("令牌应可验签");
        let authenticated = AuthenticatedUser {
            user_id: claims.sub.clone(),
            username: claims.username.clone(),
            role: claims.role,
            token_remaining_secs: claims.remaining_secs(),
        };

        let Json(me_body) = me(State(state), authenticated).await.expect("/me 应成功");
        assert_eq!(me_body.data.email, "alice@example.com");
        assert_eq!(me_body.data.role, UserRole::User);
    }

    /// 非管理员访问管理接口必须得到 403。
    #[tokio::test]
    async fn admin_endpoint_forbids_normal_user() {
        let err = admin_ping(AuthenticatedUser {
            user_id: "u-1".to_string(),
            username: "alice".to_string(),
            role: UserRole::User,
            token_remaining_secs: 3600,
        })
        .await
        .expect_err("普通用户不应通过");

        assert_eq!(err.status_code(), StatusCode::FORBIDDEN);
    }

    /// 管理员身份应当通过。
    #[tokio::test]
    async fn admin_endpoint_allows_admin() {
        let Json(body) = admin_ping(AuthenticatedUser {
            user_id: "u-1".to_string(),
            username: "root".to_string(),
            role: UserRole::Admin,
            token_remaining_secs: 3600,
        })
        .await
        .expect("管理员应通过");
        assert_eq!(body.data.operator, "root");
    }

    /// 测试辅助：验证 `PasswordHasher` 在本模块可用（防止测试配置漂移）。
    #[tokio::test]
    async fn password_hasher_is_usable() {
        let hasher = PasswordHasher::for_environment("test");
        let hashed = hasher.hash_password("secret123").await.unwrap();
        assert!(hasher.verify_password("secret123", &hashed).await.unwrap());
    }
}
