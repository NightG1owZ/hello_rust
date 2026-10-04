//! 用户服务：注册与登录的业务规则。
//!
//! # 这一层要回答的问题
//! - 注册时先查重还是直接插？（答案是**两者都要**，见 [`UserService::register`] 的注释）
//! - 密码什么时候哈希？——**进仓储之前**，仓储永远拿不到明文密码；
//! - 登录失败该返回什么？——统一文案，防止账号枚举攻击；
//! - 令牌谁来签发？——服务层，控制器不碰密钥。

use std::sync::Arc;

use crate::error::AppError;
use crate::models::{LoginRequest, LoginResponse, RegisterRequest, User, UserResponse, UserRole};
use crate::repository::{NewUser, UserRepository};
use crate::services::auth::JwtService;
use crate::services::password::PasswordHasher;

/// 用户服务。
///
/// 持有三个依赖（仓储、哈希器、JWT 服务），全部通过构造函数注入，
/// 便于测试时替换为测试替身。
#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
    password_hasher: PasswordHasher,
    jwt_service: Arc<JwtService>,
}

impl UserService {
    /// 构造服务。
    pub fn new(
        repository: Arc<dyn UserRepository>,
        password_hasher: PasswordHasher,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            repository,
            password_hasher,
            jwt_service,
        }
    }

    /// 用户注册。
    ///
    /// # 流程（注意每一步的顺序都是有讲究的）
    /// 1. **先查重**：给出精确的错误提示（"该邮箱已被注册"比"插入失败"友好得多）；
    /// 2. **哈希密码**：明文到此为止，后面的环节只见哈希；
    /// 3. **写库**：仍然依赖数据库唯一约束兜底。
    ///
    /// # 为什么"查重"之后还要依赖数据库约束
    /// 两个请求可能同时通过第 1 步检查（TOCTOU 竞态），
    /// 然后双双执行插入。此时只有数据库的 `UNIQUE` 约束能拦住重复数据。
    /// 这是"不能把并发正确性寄托在应用层检查上"的经典案例。
    pub async fn register(&self, request: RegisterRequest) -> Result<UserResponse, AppError> {
        // 统一做输入规整：邮箱大小写不敏感（Alice@X.com 与 alice@x.com 是同一个人），
        // 用户名也统一小写，避免出现 "Alice" 和 "alice" 两个账号。
        let email = request.email.trim().to_lowercase();
        let username = request.username.trim().to_lowercase();

        if self.repository.find_by_email(&email).await?.is_some() {
            return Err(AppError::conflict("USER_EMAIL_EXISTS", "该邮箱已被注册"));
        }
        if self.repository.find_by_username(&username).await?.is_some() {
            return Err(AppError::conflict(
                "USER_USERNAME_EXISTS",
                "该用户名已被占用",
            ));
        }

        // 哈希在这里发生：内部走 spawn_blocking，不会阻塞运行时。
        let password_hash = self
            .password_hasher
            .hash_password(&request.password)
            .await?;

        let new_user = NewUser {
            username,
            email,
            password_hash,
            // 注册接口一律创建普通用户：**绝不允许客户端指定角色**，
            // 否则任何人 POST 一个 "role": "admin" 就能提权。这是真实世界的高危漏洞。
            role: UserRole::User,
        };

        let user = self.repository.create(&new_user).await?;
        tracing::info!(user_id = %user.id, username = %user.username, "新用户注册成功");

        Ok(UserResponse::from(user))
    }

    /// 用户登录。
    ///
    /// # 安全要点
    /// 1. **账号不存在与密码错误返回同一条消息**：否则攻击者可以用登录接口
    ///    逐个探测哪些邮箱已注册（账号枚举）；
    /// 2. **账号不存在时也要做一次哈希校验**（时序攻击防护）：
    ///    若"账号不存在"立即返回，而"密码错误"要等 200ms 的 bcrypt 计算，
    ///    攻击者仅凭响应时间就能判断账号是否存在。本项目用一次"空校验"抹平差异。
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AppError> {
        let account = request.account.trim().to_lowercase();

        // 账号既可能是邮箱也可能是用户名：包含 `@` 就按邮箱查，否则按用户名查。
        // 这样对用户更友好，而实现只多了一个分支。
        let user = if account.contains('@') {
            self.repository.find_by_email(&account).await?
        } else {
            self.repository.find_by_username(&account).await?
        };

        let Some(user) = user else {
            // 时序攻击防护：即使账号不存在，也消耗一次同等量级的哈希计算时间。
            // 用一个固定的假哈希，保证耗时形态与真实校验一致。
            const DUMMY_HASH: &str = "$2b$04$abcdefghijklmnopqrstuu6BpX4XPR5nMYNdYQ4n4X7bRvUc8gWnS";
            let _ = self
                .password_hasher
                .verify_password(&request.password, DUMMY_HASH)
                .await;
            tracing::warn!(account = %account, "登录失败：账号不存在");
            return Err(Self::invalid_credentials());
        };

        if !self
            .password_hasher
            .verify_password(&request.password, &user.password_hash)
            .await?
        {
            tracing::warn!(user_id = %user.id, "登录失败：密码错误");
            return Err(Self::invalid_credentials());
        }

        // 鉴权通过，签发令牌。
        let access_token = self
            .jwt_service
            .issue_token(&user.id, &user.username, user.role)?;

        tracing::info!(user_id = %user.id, username = %user.username, "用户登录成功");

        Ok(LoginResponse {
            user: UserResponse::from(&user),
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_service.expires_in_secs(),
        })
    }

    /// 按 ID 加载用户（`GET /api/users/me` 用）。
    pub async fn find_by_id(&self, user_id: &str) -> Result<UserResponse, AppError> {
        let user = self
            .repository
            .find_by_id(user_id)
            .await?
            // 令牌有效但用户已被删除：属于认证信息过期，返回 401 让客户端重新登录，
            // 比 404 更贴合语义。
            .ok_or_else(|| AppError::Unauthorized("令牌对应的用户不存在".to_string()))?;

        Ok(UserResponse::from(&user))
    }

    /// 加载完整的领域实体（供需要 `password_hash` 等内部字段的场景使用）。
    pub async fn find_entity(&self, user_id: &str) -> Result<Option<User>, AppError> {
        self.repository.find_by_id(user_id).await
    }

    /// 统一的登录失败错误。
    ///
    /// 刻意不区分"账号不存在"与"密码错误"——这是安全设计的**有意为之**，
    /// 不要"好心"改成更具体的提示。
    fn invalid_credentials() -> AppError {
        AppError::Unauthorized("账号或密码错误".to_string())
    }
}

// ============================================================================
// 单元测试（用内存仓储，无需数据库，毫秒级完成）
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::JwtConfig;
    use crate::repository::InMemoryUserRepository;

    /// 构造一个使用内存仓储的服务实例。
    fn test_service() -> (UserService, Arc<InMemoryUserRepository>) {
        let repository = InMemoryUserRepository::shared();
        let jwt = Arc::new(
            JwtService::new(&JwtConfig {
                secret: "unit-test-secret-key-0123456789".to_string(),
                expires_hours: 1,
                issuer: "demoweb-test".to_string(),
            })
            .unwrap(),
        );
        let service = UserService::new(
            Arc::clone(&repository) as Arc<dyn UserRepository>,
            // 测试环境用最低代价因子，让整个测试套件保持秒级
            PasswordHasher::with_cost(4),
            jwt,
        );
        (service, repository)
    }

    fn register_request() -> RegisterRequest {
        RegisterRequest {
            username: "Alice_01".to_string(),
            email: "Alice@Example.COM".to_string(),
            password: "secret123".to_string(),
        }
    }

    #[tokio::test]
    async fn register_succeeds_and_normalizes_input() {
        let (service, repository) = test_service();
        let response = service
            .register(register_request())
            .await
            .expect("注册应当成功");

        // 输入做了规整：邮箱、用户名统一小写
        assert_eq!(response.email, "alice@example.com");
        assert_eq!(response.username, "alice_01");
        // 注册接口强制普通用户，防止提权
        assert_eq!(response.role, UserRole::User);
        assert_eq!(repository.len(), 1);
    }

    /// 落库的必须是哈希，绝不能是明文。
    #[tokio::test]
    async fn password_is_stored_hashed() {
        let (service, _repository) = test_service();
        let response = service.register(register_request()).await.unwrap();

        let stored = service.find_entity(&response.id).await.unwrap().unwrap();
        assert_ne!(stored.password_hash, "secret123", "不能存明文密码");
        assert!(stored.password_hash.starts_with("$2"), "应为 bcrypt 哈希");
    }

    #[tokio::test]
    async fn duplicate_email_is_rejected() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let mut second = register_request();
        second.username = "bob_01".to_string(); // 只改用户名
        let err = service.register(second).await.expect_err("邮箱重复应失败");
        assert_eq!(err.code(), "USER_EMAIL_EXISTS");
    }

    #[tokio::test]
    async fn duplicate_username_is_rejected() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let mut second = register_request();
        second.email = "bob@example.com".to_string(); // 只改邮箱
        let err = service
            .register(second)
            .await
            .expect_err("用户名重复应失败");
        assert_eq!(err.code(), "USER_USERNAME_EXISTS");
    }

    #[tokio::test]
    async fn login_with_email_succeeds() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let response = service
            .login(LoginRequest {
                account: "alice@example.com".to_string(),
                password: "secret123".to_string(),
            })
            .await
            .expect("登录应当成功");

        assert_eq!(response.token_type, "Bearer");
        assert!(!response.access_token.is_empty());
        assert!(response.expires_in > 0);
        assert_eq!(response.user.username, "alice_01");
    }

    /// 也支持用用户名登录（同一套接口，两种入口）。
    #[tokio::test]
    async fn login_with_username_succeeds() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let response = service
            .login(LoginRequest {
                account: "alice_01".to_string(),
                password: "secret123".to_string(),
            })
            .await
            .expect("用用户名登录应当成功");
        assert_eq!(response.user.email, "alice@example.com");
    }

    /// 大小写不敏感：注册用大写邮箱，登录用小写也能成功。
    #[tokio::test]
    async fn login_is_case_insensitive_for_account() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let response = service
            .login(LoginRequest {
                account: "ALICE@EXAMPLE.COM".to_string(),
                password: "secret123".to_string(),
            })
            .await
            .expect("账号应大小写不敏感");
        assert_eq!(response.user.username, "alice_01");
    }

    #[tokio::test]
    async fn wrong_password_is_rejected() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let err = service
            .login(LoginRequest {
                account: "alice@example.com".to_string(),
                password: "wrong-password".to_string(),
            })
            .await
            .expect_err("密码错误应当失败");
        assert_eq!(err.status_code(), axum::http::StatusCode::UNAUTHORIZED);
    }

    /// 账号不存在与密码错误必须返回**完全相同**的消息（防账号枚举）。
    #[tokio::test]
    async fn nonexistent_account_and_wrong_password_are_indistinguishable() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let wrong_password = service
            .login(LoginRequest {
                account: "alice@example.com".to_string(),
                password: "wrong-password".to_string(),
            })
            .await
            .unwrap_err();

        let no_such_account = service
            .login(LoginRequest {
                account: "nobody@example.com".to_string(),
                password: "secret123".to_string(),
            })
            .await
            .unwrap_err();

        assert_eq!(wrong_password.code(), no_such_account.code());
        assert_eq!(
            wrong_password.to_string(),
            no_such_account.to_string(),
            "两种失败的文案必须一致，否则可被用于枚举账号"
        );
    }

    /// 注册 → 登录 → 用令牌反查用户，走通完整闭环。
    #[tokio::test]
    async fn issued_token_resolves_to_registered_user() {
        let (service, _) = test_service();
        service.register(register_request()).await.unwrap();

        let login = service
            .login(LoginRequest {
                account: "alice@example.com".to_string(),
                password: "secret123".to_string(),
            })
            .await
            .unwrap();

        // 这里直接复用服务里持有的 JWT 服务来验签（控制器层也走同一条路径）
        let jwt = JwtService::new(&JwtConfig {
            secret: "unit-test-secret-key-0123456789".to_string(),
            expires_hours: 1,
            issuer: "demoweb-test".to_string(),
        })
        .unwrap();
        let claims = jwt.verify_token(&login.access_token).expect("令牌应可验签");
        let me = service.find_by_id(&claims.sub).await.expect("应能查到本人");
        assert_eq!(me.email, "alice@example.com");
    }

    /// 令牌有效但用户已不存在 → 401，而不是 404 或 500。
    #[tokio::test]
    async fn missing_user_reports_unauthorized() {
        let (service, _) = test_service();
        let err = service.find_by_id("not-exist").await.expect_err("应当报错");
        assert_eq!(err.status_code(), axum::http::StatusCode::UNAUTHORIZED);
    }
}
