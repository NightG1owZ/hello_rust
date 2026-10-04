//! 认证服务：JWT 令牌的签发与校验，以及"已认证用户"提取器。
//!
//! # 为什么用 JWT（JSON Web Token）
//! 传统 Session 需要在服务端存会话状态，多实例部署时要额外引入 Redis 共享。
//! JWT 把"用户身份 + 过期时间"用服务端密钥签名后交给客户端保存，
//! 服务端**无状态**即可校验——这也是它适合水平扩展的原因。
//!
//! # 签名 vs 加密（关键认知）
//! JWT 默认只做**签名**（保证内容没被篡改），**不做加密**。
//! 任何人都能 Base64 解出 payload 内容。所以：
//! - ✅ 可以放：用户 ID、用户名、角色、过期时间；
//! - ❌ 绝不能放：密码、手机号、身份证号、密钥。
//!
//! # 令牌校验必须校验什么
//! 1. **签名**——否则攻击者可自造令牌（[`Validation::algorithms`] 限定算法，
//!    还能防"算法混淆攻击"：把 `alg` 改成 `none` 试图绕过校验）；
//! 2. **过期时间 `exp`**——否则令牌永久有效；
//! 3. **签发者 `iss`**——多服务共用密钥时，防止 A 服务的令牌被拿去访问 B 服务。

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::config::JwtConfig;
use crate::error::AppError;
use crate::models::UserRole;

/// JWT 载荷（payload）。
///
/// 字段名遵循 JWT 规范中的**注册声明**（registered claims）：
/// `sub`(subject) / `iss`(issuer) / `exp`(expiration) / `iat`(issued at)，
/// 自定义字段（`username`、`role`）与它们并列存放。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    /// 主体：用户 ID（规范字段 `sub`）。
    pub sub: String,
    /// 签发者（规范字段 `iss`），校验时要求完全匹配。
    pub iss: String,
    /// 签发时间（Unix 秒）。
    pub iat: i64,
    /// 过期时间（Unix 秒）。
    pub exp: i64,
    /// 用户名（自定义声明，前端可直接展示，省一次查询）。
    pub username: String,
    /// 角色（自定义声明），用于接口级鉴权。
    pub role: UserRole,
}

impl TokenClaims {
    /// 令牌剩余有效秒数（已过期则为 0）。
    pub fn remaining_secs(&self) -> i64 {
        (self.exp - Utc::now().timestamp()).max(0)
    }

    /// 是否已过期。
    pub fn is_expired(&self) -> bool {
        self.exp <= Utc::now().timestamp()
    }
}

/// 经过认证的用户信息。
///
/// 该类型实现 [`FromRequestParts`]，因此处理器只要在参数里写
/// `user: AuthenticatedUser`，axum 就会自动从请求扩展（extensions）里取出它。
/// **令牌校验在中间件里完成一次**，处理器不再重复解析——既省 CPU，也避免遗漏。
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// 用户 ID。
    pub user_id: String,
    /// 用户名。
    pub username: String,
    /// 角色。
    pub role: UserRole,
    /// 令牌剩余有效期（秒），便于在响应头里回传 `X-Token-Remaining`。
    pub token_remaining_secs: i64,
}

impl AuthenticatedUser {
    /// 是否管理员。
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }
}

/// 注意：这里**没有**使用 `#[async_trait]`。
///
/// axum 0.8 的 [`FromRequestParts`] 采用 Rust 原生的 `async fn in trait`（AFIT），
/// 其签名是 `fn from_request_parts(...) -> impl Future<Output = ...> + Send`。
/// 直接写 `async fn` 就能满足要求——这是 Rust 1.75 之后的重要变化：
/// **只在需要 `dyn Trait` 动态分发时才需要 `async_trait`**（如本项目里的仓储 trait）。
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    /// 用具体错误类型而不是 `Infallible`，这样令牌缺失/非法时能直接返回 401。
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            // 走到这里说明认证中间件没有挂到该路由上，属于**装配错误**而非用户错误。
            // 返回 500 并把原因写进日志，能让问题在开发阶段立刻暴露。
            .ok_or_else(|| {
                AppError::Internal(anyhow::anyhow!(
                    "路由缺少认证中间件：本次请求未经过 JWT 校验层"
                ))
            })
    }
}

/// JWT 服务：持有签名/验签密钥。
///
/// 同一个密钥既用于签发（EncodingKey）也用于校验（DecodingKey），
/// 启动时构造一次，之后以 `Arc<JwtService>` 共享（构造密钥有微小开销，且应复用）。
#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    /// 校验配置（算法白名单、签发者、是否校验过期等）。
    validation: Validation,
    /// 令牌有效期。
    expires_in: Duration,
    /// 签发者。
    issuer: String,
}

impl std::fmt::Debug for JwtService {
    /// 手写 `Debug`：密钥绝不能出现在日志里。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("algorithm", &"HS256")
            .field("issuer", &self.issuer)
            .field("expires_in_hours", &self.expires_in.num_hours())
            .finish_non_exhaustive()
    }
}

impl JwtService {
    /// 依据配置构造服务。
    ///
    /// 这里会做一次启动期检查：密钥太短直接拒绝启动，
    /// 避免用弱密钥签发可被暴力破解的令牌。
    pub fn new(config: &JwtConfig) -> anyhow::Result<Self> {
        if config.secret.len() < 16 {
            anyhow::bail!("JWT 密钥过短（至少 16 字符），请在配置或环境变量中提供强随机密钥");
        }

        // 显式构造校验规则。**不要**用 `Validation::default()` 后只改一处，
        // 那样容易忘记关掉或打开某个校验项；这里逐项写明意图。
        let mut validation = Validation::new(Algorithm::HS256);
        // 要求令牌必须带 exp 并校验过期
        validation.validate_exp = true;
        // 校验签发者，避免跨服务的令牌被误用
        validation.set_issuer(&[config.issuer.as_str()]);
        // 允许 60 秒时钟偏移：多实例部署时服务器时钟可能略有差异
        validation.leeway = 60;

        Ok(Self {
            encoding_key: EncodingKey::from_secret(config.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.secret.as_bytes()),
            validation,
            expires_in: Duration::hours(config.expires_hours),
            issuer: config.issuer.clone(),
        })
    }

    /// 令牌有效期（秒）——登录响应里的 `expires_in` 用它。
    pub fn expires_in_secs(&self) -> i64 {
        self.expires_in.num_seconds()
    }

    /// 为用户签发访问令牌。
    pub fn issue_token(
        &self,
        user_id: &str,
        username: &str,
        role: UserRole,
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = TokenClaims {
            sub: user_id.to_string(),
            iss: self.issuer.clone(),
            iat: now.timestamp(),
            exp: (now + self.expires_in).timestamp(),
            username: username.to_string(),
            role,
        };

        encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("签发令牌失败：{e}")))
    }

    /// 校验令牌并返回载荷。
    ///
    /// # 错误分类（决定客户端该"重新登录"还是"稍后重试"）
    /// - 签名错误 / 格式错误 / 已过期 → [`AppError::Unauthorized`]（401，客户端应清除本地令牌）；
    /// - 其它异常 → 内部错误（500）。
    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, AppError> {
        decode::<TokenClaims>(token, &self.decoding_key, &self.validation)
            .map(|data| data.claims)
            .map_err(|e| {
                use jsonwebtoken::errors::ErrorKind;
                match e.kind() {
                    ErrorKind::ExpiredSignature => {
                        AppError::Unauthorized("令牌已过期，请重新登录".to_string())
                    }
                    ErrorKind::InvalidSignature => {
                        AppError::Unauthorized("令牌签名无效".to_string())
                    }
                    ErrorKind::InvalidToken => AppError::Unauthorized("令牌格式非法".to_string()),
                    ErrorKind::InvalidIssuer => {
                        AppError::Unauthorized("令牌签发者不匹配".to_string())
                    }
                    _ => AppError::Unauthorized(format!("令牌校验失败：{e}")),
                }
            })
    }

    /// 从 `Authorization: Bearer <token>` 头里取出令牌原文。
    ///
    /// 兼容大小写（`Bearer` / `bearer`），并把"缺少头/格式错误"翻译成明确的 401 文案，
    /// 而不是笼统的"认证失败"——客户端调试时能少走弯路。
    pub fn extract_bearer_token(authorization: Option<&str>) -> Result<&str, AppError> {
        let Some(header_value) = authorization else {
            return Err(AppError::Unauthorized(
                "缺少 Authorization 请求头".to_string(),
            ));
        };

        let mut parts = header_value.splitn(2, ' ');
        let scheme = parts.next().unwrap_or_default();
        let token = parts.next().unwrap_or_default().trim();

        if !scheme.eq_ignore_ascii_case("bearer") {
            return Err(AppError::Unauthorized(
                "Authorization 头格式应为 `Bearer <token>`".to_string(),
            ));
        }
        if token.is_empty() {
            return Err(AppError::Unauthorized("Bearer 令牌为空".to_string()));
        }

        Ok(token)
    }
}

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig {
            secret: "unit-test-secret-key-0123456789".to_string(),
            expires_hours: 1,
            issuer: "demoweb-test".to_string(),
        }
    }

    fn service() -> JwtService {
        JwtService::new(&test_config()).expect("构造 JwtService 失败")
    }

    #[test]
    fn issue_then_verify_round_trip() {
        let service = service();
        let token = service
            .issue_token("user-1", "alice", UserRole::Admin)
            .unwrap();
        let claims = service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, "user-1");
        assert_eq!(claims.username, "alice");
        assert_eq!(claims.role, UserRole::Admin);
        assert_eq!(claims.iss, "demoweb-test");
        assert!(!claims.is_expired());
        assert!(claims.remaining_secs() > 3500);
    }

    /// 换一个密钥签发的令牌必须被拒绝（签名校验的意义）。
    #[test]
    fn token_signed_by_other_secret_is_rejected() {
        let mut other = test_config();
        other.secret = "another-secret-key-9876543210".to_string();
        let forged = JwtService::new(&other)
            .unwrap()
            .issue_token("user-1", "alice", UserRole::User)
            .unwrap();

        let err = service()
            .verify_token(&forged)
            .expect_err("伪造令牌应当被拒绝");
        assert_eq!(err.status_code(), axum::http::StatusCode::UNAUTHORIZED);
        assert!(
            err.to_string().contains("签名"),
            "错误信息应指出签名问题：{err}"
        );
    }

    /// 篡改 payload 后签名失效。
    #[test]
    fn tampered_payload_is_rejected() {
        let service = service();
        let token = service
            .issue_token("user-1", "alice", UserRole::User)
            .unwrap();
        let mut parts: Vec<&str> = token.split('.').collect();
        // 替换 payload 段，签名不再匹配
        let forged_payload = "eyJzdWIiOiJhZG1pbiJ9";
        parts[1] = forged_payload;
        let tampered = parts.join(".");

        assert!(
            service.verify_token(&tampered).is_err(),
            "篡改的令牌必须被拒绝"
        );
    }

    /// 过期令牌必须被拒绝，且错误文案提示"重新登录"。
    #[test]
    fn expired_token_is_rejected() {
        let mut config = test_config();
        config.expires_hours = -1; // 负数 → 立即过期（测试用技巧）
        let service = JwtService::new(&config).unwrap();
        let token = service.issue_token("u", "alice", UserRole::User).unwrap();

        let err = service
            .verify_token(&token)
            .expect_err("过期令牌应当被拒绝");
        // leeway 为 60 秒，因此刚过期的令牌可能仍在宽限内；
        // 用 -1 小时构造确保超出宽限期。
        assert!(
            err.to_string().contains("过期") || err.to_string().contains("校验失败"),
            "{err}"
        );
    }

    #[test]
    fn issuer_mismatch_is_rejected() {
        let mut config = test_config();
        config.issuer = "other-service".to_string();
        let foreign = JwtService::new(&config)
            .unwrap()
            .issue_token("u", "a", UserRole::User)
            .unwrap();
        let err = service()
            .verify_token(&foreign)
            .expect_err("签发者不匹配应当被拒绝");
        assert_eq!(err.status_code(), axum::http::StatusCode::UNAUTHORIZED);
    }

    /// Bearer 头解析的各种边界情况。
    #[test]
    fn bearer_header_parsing() {
        assert_eq!(
            JwtService::extract_bearer_token(Some("Bearer abc.def.ghi")).unwrap(),
            "abc.def.ghi"
        );
        // 大小写不敏感
        assert_eq!(
            JwtService::extract_bearer_token(Some("bearer xyz")).unwrap(),
            "xyz"
        );
        // 缺少头
        assert!(JwtService::extract_bearer_token(None).is_err());
        // 方案错误
        assert!(JwtService::extract_bearer_token(Some("Basic abc")).is_err());
        // 令牌为空
        assert!(JwtService::extract_bearer_token(Some("Bearer ")).is_err());
        // 缺少空格分隔
        assert!(JwtService::extract_bearer_token(Some("Bearer")).is_err());
    }

    /// 密钥过短必须在构造阶段就被拒绝（启动即失败优于运行期才发现）。
    #[test]
    fn short_secret_is_rejected_at_construction() {
        let mut config = test_config();
        config.secret = "short".to_string();
        assert!(JwtService::new(&config).is_err());
    }

    /// `Debug` 输出不得包含密钥。
    #[test]
    fn debug_hides_secret() {
        let debug = format!("{:?}", service());
        assert!(!debug.contains("unit-test-secret"));
    }
}
