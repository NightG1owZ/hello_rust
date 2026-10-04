//! 领域模型与数据传输对象（DTO）。
//!
//! # 为什么要把"领域模型"与"接口 DTO"分开？
//! 初学者常把数据库实体直接当接口返回体，这会带来三个真实问题：
//! 1. **泄露敏感字段**：`User` 里有 `password_hash`，一旦直接序列化返回就是安全事故；
//! 2. **接口与表结构强耦合**：一旦加字段，所有客户端都受影响；
//! 3. **无法表达字段级约束**：注册请求需要"用户名 3~32 字符"这类校验，数据库实体不需要。
//!
//! 因此本模块同时定义了：
//! - [`User`]：领域实体（与 `users` 表一一对应）；
//! - [`RegisterRequest`] / [`LoginRequest`]：入参 DTO，带声明式校验规则；
//! - [`UserResponse`] / [`LoginResponse`]：出参 DTO，**绝不包含密码哈希**。
//!
//! # 命名约定
//! JSON 字段统一用 `snake_case`（`#[serde(rename_all = "snake_case")]`），
//! 这样与 Rust 命名一致，无需在两端做心智转换；若对接的前端约定是驼峰，
//! 只需把该属性改成 `"camelCase"`，其余代码不动——这正是 DTO 层的价值。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// ============================================================================
// 领域实体
// ============================================================================

/// 用户领域实体，与 `users` 表结构一一对应。
///
/// `FromRow` 让 sqlx 能直接把查询结果行映射成本结构体（字段名需与列名一致）。
#[derive(Debug, Clone, FromRow)]
pub struct User {
    /// 用户 ID（UUID v4 字符串）。
    pub id: String,
    /// 唯一用户名。
    pub username: String,
    /// 唯一邮箱。
    pub email: String,
    /// 密码哈希（bcrypt）。**任何情况下都不得返回给客户端**。
    pub password_hash: String,
    /// 角色。
    pub role: UserRole,
    /// 创建时间。
    pub created_at: DateTime<Utc>,
    /// 最后更新时间。
    pub updated_at: DateTime<Utc>,
}

/// 用户角色。
///
/// # 为什么枚举要手写 `Type` 实现？
/// sqlx 不知道如何把 `UserRole` 存进 SQLite。有两种做法：
/// - `#[derive(sqlx::Type)]` + `#[sqlx(rename_all = "lowercase")]`：适合简单枚举；
/// - 手写 `Type`/`Encode`/`Decode`（本文件的做法）：**更值得学习**，
///   因为它把"字符串 → 枚举"的转换逻辑显式写了出来，非法值会报错而不是被静默接受。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// 普通用户
    User,
    /// 管理员
    Admin,
}

impl UserRole {
    /// 转成数据库/JWT 里使用的字符串。
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
        }
    }

    /// 从字符串解析。
    ///
    /// 返回 `Result` 而不是默认值：数据库里出现未知角色时必须报错，
    /// 否则一个 `"superuser"` 会被静默降级成普通用户，掩盖真实问题。
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "user" => Ok(Self::User),
            "admin" => Ok(Self::Admin),
            other => Err(format!("未知的用户角色：{other}")),
        }
    }

    /// 该角色是否拥有管理权限。业务代码用方法而不是到处写 `== UserRole::Admin`，
    /// 将来增加角色时只需改这一处。
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 手写 sqlx 类型映射：`UserRole` ↔ SQLite `TEXT`。
impl sqlx::Type<sqlx::Sqlite> for UserRole {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }

    /// 声明"本类型可无损地当作 TEXT 使用"，
    /// 这样 sqlx 才敢于把 TEXT 列解码成 `UserRole`。
    fn compatible(ty: &<sqlx::Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for UserRole {
    fn decode(
        value: <sqlx::Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let raw = <String as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Self::parse(&raw).map_err(|e| e.into())
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode(self.as_str().to_string(), buf)
    }
}

// ============================================================================
// 入参 DTO（带声明式校验）
// ============================================================================

/// 用户注册请求体。
///
/// 校验规则写在字段上，处理器里只要调用 `.validate()?` 即可——
/// 这就是"声明式校验"相比手写 `if` 的价值：规则集中、可读、不会漏。
///
/// 注意这里同时派生了 `Serialize`：虽然接口只**反序列化**它，
/// 但单元测试要用 `serde_json::to_value` 构造请求体，
/// 集成测试也常需要序列化，加上它能让测试代码更简洁。
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    /// 用户名：3~32 字符，只允许字母/数字/下划线/连字符。
    #[validate(length(min = 3, max = 32, message = "用户名长度必须在 3~32 个字符之间"))]
    #[validate(custom(function = "validate_username_charset"))]
    pub username: String,

    /// 邮箱：必须是合法格式（`validator` 内置规则）。
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 密码：8~72 字符（bcrypt 上限为 72 字节，超过部分会被忽略，因此必须限制）。
    #[validate(length(min = 8, max = 72, message = "密码长度必须在 8~72 个字符之间"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
}

/// 用户登录请求体。
///
/// 登录失败的提示必须**模糊**（统一"用户名或密码错误"），
/// 否则攻击者可以借此枚举出系统里存在哪些账号。
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    /// 账号：可以是用户名或邮箱，由服务层统一解析（提升使用体验）。
    #[validate(length(min = 3, max = 254, message = "账号长度不合法"))]
    pub account: String,

    /// 密码：登录时不校验强度，只要求非空——强度是注册时的事。
    #[validate(length(min = 1, max = 72, message = "密码不能为空"))]
    pub password: String,
}

// ============================================================================
// 出参 DTO
// ============================================================================

/// 用户信息响应体。
///
/// **注意这里没有 `password_hash` 字段**——编译期就杜绝了泄露可能。
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role,
            created_at: user.created_at,
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self::from(&user)
    }
}

/// 登录成功响应体：用户信息 + 访问令牌。
#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    /// JWT 访问令牌，客户端后续请求放在 `Authorization: Bearer <token>` 头里。
    pub access_token: String,
    /// 令牌类型，固定 `Bearer`（OAuth2 规范字段）。
    pub token_type: String,
    /// 令牌有效期（秒），客户端据此决定何时刷新。
    pub expires_in: i64,
}

/// 统一成功响应包装。
///
/// 与 [`crate::error::ErrorResponse`] 的字段风格保持一致（都有 `success`），
/// 前端只需要判断一个字段就能分流成功/失败分支。
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> ApiResponse<T> {
    /// 包装成功数据。
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

/// 健康检查响应体（用于探针/负载均衡健康检查）。
#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub environment: String,
    /// 运行时长（秒），便于确认服务是否发生过重启。
    pub uptime_secs: u64,
}

// ============================================================================
// 自定义校验函数
// ============================================================================

/// 用户名只允许字母、数字、下划线、连字符。
///
/// 为什么不用正则？——避免为了一个简单规则引入 `regex` 依赖；
/// 实际项目中用 `regex` 或 `validator` 的 `regex` 规则更清晰。
fn validate_username_charset(username: &str) -> Result<(), validator::ValidationError> {
    let valid = username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if valid {
        Ok(())
    } else {
        Err(validator::ValidationError::new("username_charset")
            .with_message("用户名只能包含字母、数字、下划线和连字符".into()))
    }
}

/// 密码强度：必须同时包含字母和数字。
///
/// 真实生产环境还应检查"是否在常见弱密码字典中"（如 `12345678`），
/// 这里演示自定义校验函数的写法。
fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    if has_letter && has_digit {
        Ok(())
    } else {
        Err(validator::ValidationError::new("password_strength")
            .with_message("密码必须同时包含字母和数字".into()))
    }
}

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    fn valid_register() -> RegisterRequest {
        RegisterRequest {
            username: "alice_01".to_string(),
            email: "alice@example.com".to_string(),
            password: "secret123".to_string(),
        }
    }

    #[test]
    fn valid_register_request_passes() {
        assert!(valid_register().validate().is_ok());
    }

    #[test]
    fn invalid_email_is_rejected() {
        let mut req = valid_register();
        req.email = "not-an-email".to_string();
        let errors = req.validate().expect_err("非法邮箱应当校验失败");
        assert!(errors.field_errors().contains_key("email"));
    }

    #[test]
    fn short_password_is_rejected() {
        let mut req = valid_register();
        req.password = "a1".to_string();
        assert!(req.validate().is_err());
    }

    /// 纯字母或纯数字的密码都应被拒绝。
    #[test]
    fn weak_password_is_rejected() {
        let mut req = valid_register();
        req.password = "abcdefgh".to_string();
        assert!(req.validate().is_err(), "纯字母密码应当被拒绝");

        req.password = "12345678".to_string();
        assert!(req.validate().is_err(), "纯数字密码应当被拒绝");
    }

    /// 用户名里的特殊字符必须被拦下。
    #[test]
    fn username_with_special_chars_is_rejected() {
        let mut req = valid_register();
        req.username = "alice admin".to_string();
        assert!(req.validate().is_err());
    }

    #[test]
    fn role_round_trip() {
        for role in [UserRole::User, UserRole::Admin] {
            assert_eq!(UserRole::parse(role.as_str()).unwrap(), role);
        }
        assert!(
            UserRole::parse("SUPERUSER").is_err(),
            "未知角色必须报错而不是降级"
        );
        assert!(UserRole::Admin.is_admin());
        assert!(!UserRole::User.is_admin());
    }

    /// 响应 DTO 的字段列表里绝不能出现密码哈希。
    #[test]
    fn user_response_never_contains_password_hash() {
        let user = User {
            id: "u-1".into(),
            username: "alice".into(),
            email: "alice@example.com".into(),
            password_hash: "$2b$12$topsecret".into(),
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let json = serde_json::to_string(&UserResponse::from(&user)).unwrap();
        assert!(
            !json.contains("password"),
            "响应体不应该包含任何密码字段：{json}"
        );
        assert!(!json.contains("topsecret"));
    }
}
