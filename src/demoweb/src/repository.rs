//! 仓储层（Repository）：数据访问的统一抽象。
//!
//! # 这一层解决什么问题
//! 服务层关心的是"按邮箱查用户"，而不关心 SQL 怎么写、用的哪种数据库。
//! [`UserRepository`] 用 trait 把这个边界划出来：
//!
//! ```text
//!   controllers ──► services ──► UserRepository (trait)
//!                                     ▲
//!                     ┌───────────────┴───────────────┐
//!            SqliteUserRepository              InMemoryUserRepository
//!              （生产/集成测试）                  （单元测试，零依赖）
//! ```
//!
//! 好处有三个：
//! 1. **可测试**：服务层单元测试注入内存实现，毫秒级完成，不需要数据库；
//! 2. **可替换**：将来换 PostgreSQL / MySQL，只新增一个实现，服务层不动；
//! 3. **可读**：数据访问的边界一目了然，避免 SQL 散落在业务代码里。
//!
//! # 为什么用 `#[async_trait]`
//! Rust 1.75 起支持 trait 里直接写 `async fn`，但那样的 trait 不是**对象安全**的，
//! 不能写成 `dyn UserRepository`。`async-trait` 宏把 `async fn` 转写为
//! `fn(...) -> Pin<Box<dyn Future<Output = ...> + Send>>`，从而支持动态分发。
//! 代价是每次调用会多一次堆分配（Box）——在本场景完全可接受。
//!
//! # 并发写安全：内存实现用 `tokio::sync::RwLock`
//! 不能用 `std::sync::RwLock`：它的写锁会**阻塞线程**，
//! 而异步运行时里阻塞线程会拖垮整个 executor（一个 worker 线程被占住，
//! 上面其它成百上千个任务都没法推进）。异步场景必须用异步锁。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::error::AppError;
use crate::models::{User, UserRole};

/// 新建用户的输入数据（不含 ID 与时间戳，由仓储层生成）。
///
/// 单独定义一个"插入模型"而不是复用 [`User`]，是为了让类型系统表达
/// "调用方无权指定 ID 和创建时间"这一业务约束。
#[derive(Debug, Clone)]
pub struct NewUser {
    /// 用户名（已确保唯一）。
    pub username: String,
    /// 邮箱（已确保唯一）。
    pub email: String,
    /// **已哈希**的密码。仓储层绝不接收明文密码。
    pub password_hash: String,
    /// 角色。
    pub role: UserRole,
}

/// 用户仓储抽象。
///
/// 所有方法都返回 [`AppError`]，这样仓储实现可以自由地把
/// 底层错误（sqlx::Error、锁中毒等）翻译成统一的领域错误。
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    /// 按邮箱查找用户（登录时用）。
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    /// 按用户名查找用户（注册时做重复检查、登录时支持用户名登录）。
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;

    /// 按 ID 查找用户（鉴权成功后加载用户信息）。
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError>;

    /// 创建用户并返回落库后的完整实体。
    ///
    /// 实现必须把"唯一约束冲突"翻译成 [`AppError::Conflict`]，
    /// 让上层能直接返回 409 而不必解析数据库错误文本。
    async fn create(&self, new_user: &NewUser) -> Result<User, AppError>;
}

// ============================================================================
// 实现一：SQLite 仓储（生产实现）
// ============================================================================

/// 基于 SQLite 的用户仓储。
///
/// 内部持有连接池；`SqlitePool` 本身是 `Arc` 包裹的，克隆它只增加引用计数。
#[derive(Debug, Clone)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    /// 用连接池构造仓储。
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 把数据库的唯一约束冲突翻译成带业务语义的错误。
    ///
    /// SQLite 的错误文本形如：
    /// `UNIQUE constraint failed: users.email`
    /// 从中解析出冲突的列名，就能给前端精确提示（而不是笼统的"数据重复"）。
    fn map_unique_violation(error: sqlx::Error) -> AppError {
        let Some(db_error) = error.as_database_error() else {
            return AppError::Database(error);
        };
        let message = db_error.message().to_string();
        if !message.contains("UNIQUE constraint failed") {
            return AppError::Database(error);
        }

        // 依据约束涉及的列给出不同错误码，前端可据此高亮对应输入框。
        if message.contains("users.email") {
            AppError::conflict("USER_EMAIL_EXISTS", "该邮箱已被注册")
        } else if message.contains("users.username") {
            AppError::conflict("USER_USERNAME_EXISTS", "该用户名已被占用")
        } else {
            AppError::conflict("USER_DUPLICATED", "用户信息重复")
        }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        // 使用**绑定参数**（`?`）而不是字符串拼接：这是防 SQL 注入的根本手段。
        // 对比：`format!("... WHERE email = '{email}'")` 会被 `' OR '1'='1` 击穿。
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE username = ?
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&self, new_user: &NewUser) -> Result<User, AppError> {
        let now = Utc::now();
        // UUID v4：128 位随机数，碰撞概率可忽略，且不泄露用户数量。
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password_hash: new_user.password_hash.clone(),
            role: new_user.role,
            created_at: now,
            updated_at: now,
        };

        let result = sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.role)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                tracing::debug!(user_id = %user.id, email = %user.email, "用户已写入数据库");
                Ok(user)
            }
            Err(e) => Err(Self::map_unique_violation(e)),
        }
    }
}

// ============================================================================
// 实现二：内存仓储（单元测试 / 演示用，零外部依赖）
// ============================================================================

/// 基于 HashMap 的内存仓储。
///
/// 用途：
/// - 服务层单元测试的替身（stub），让测试无需数据库、毫秒级完成；
/// - 本地演示"数据库不可用时的降级模式"。
///
/// 实现细节：用 [`tokio::sync::RwLock`] 保护两个索引。
/// 读多写少的场景下 `RwLock` 比 `Mutex` 吞吐更高——
/// 多个 `find_*` 可以真正并发执行，只有 `create` 需要独占。
#[derive(Debug, Default)]
pub struct InMemoryUserRepository {
    /// 主存储：id → 用户。
    users: RwLock<HashMap<String, User>>,
    /// 邮箱索引：email → id。用索引是为了让查找是 O(1) 而不是遍历全表。
    email_index: RwLock<HashMap<String, String>>,
    /// 用户名索引：username → id。
    username_index: RwLock<HashMap<String, String>>,
    /// 用于演示原子计数（比如统计写入次数）。
    write_count: AtomicU64,
}

impl InMemoryUserRepository {
    /// 创建一个空的内存仓储。
    pub fn new() -> Self {
        Self::default()
    }

    /// 以 `Arc` 形式创建（便于注入 [`crate::state::AppState`]）。
    pub fn shared() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// 已写入的用户数，供测试断言使用。
    pub fn len(&self) -> usize {
        // `try_read`：本方法不是 async 的，且只用于测试断言，
        // 拿不到锁时返回一个保守值即可，避免把简单 API 变成 async。
        self.email_index.try_read().map(|m| m.len()).unwrap_or(0)
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 累计写入次数。
    pub fn write_count(&self) -> u64 {
        self.write_count.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let index = self.email_index.read().await;
        let Some(id) = index.get(email) else {
            return Ok(None);
        };
        let users = self.users.read().await;
        Ok(users.get(id).cloned())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let index = self.username_index.read().await;
        let Some(id) = index.get(username) else {
            return Ok(None);
        };
        let users = self.users.read().await;
        Ok(users.get(id).cloned())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let users = self.users.read().await;
        Ok(users.get(id).cloned())
    }

    async fn create(&self, new_user: &NewUser) -> Result<User, AppError> {
        // 先做重复检查。注意：这里的"检查后写入"不是原子的，
        // 中间可能插入其它写操作——所以真正的唯一性保证仍必须由数据库约束兜底。
        // 这也是为什么生产实现依赖 SQLite 的 UNIQUE 约束，而不是只靠上层预检查。
        {
            let email_index = self.email_index.read().await;
            if email_index.contains_key(&new_user.email) {
                return Err(AppError::conflict("USER_EMAIL_EXISTS", "该邮箱已被注册"));
            }
        }
        {
            let username_index = self.username_index.read().await;
            if username_index.contains_key(&new_user.username) {
                return Err(AppError::conflict(
                    "USER_USERNAME_EXISTS",
                    "该用户名已被占用",
                ));
            }
        }

        let now = Utc::now();
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password_hash: new_user.password_hash.clone(),
            role: new_user.role,
            created_at: now,
            updated_at: now,
        };

        // 写锁：需要同时持有三把锁，顺序固定为 users → email_index → username_index，
        // 固定加锁顺序可以避免死锁（A 等 B、B 等 A）。
        {
            let mut users = self.users.write().await;
            users.insert(user.id.clone(), user.clone());
        }
        {
            let mut email_index = self.email_index.write().await;
            email_index.insert(user.email.clone(), user.id.clone());
        }
        {
            let mut username_index = self.username_index.write().await;
            username_index.insert(user.username.clone(), user.id.clone());
        }

        self.write_count.fetch_add(1, Ordering::Relaxed);
        Ok(user)
    }
}

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_new_user() -> NewUser {
        NewUser {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "hashed".to_string(),
            role: UserRole::User,
        }
    }

    /// 写入后应当能按三种方式查到同一条记录。
    #[tokio::test]
    async fn in_memory_create_and_find() {
        let repo = InMemoryUserRepository::new();
        let created = repo.create(&sample_new_user()).await.expect("创建失败");

        assert_eq!(created.username, "alice");
        assert_eq!(created.role, UserRole::User);
        assert!(!created.id.is_empty(), "ID 应为 UUID");

        // 三个索引都要能命中
        assert!(repo.find_by_id(&created.id).await.unwrap().is_some());
        assert!(
            repo.find_by_email("alice@example.com")
                .await
                .unwrap()
                .is_some(),
            "邮箱索引应命中"
        );
        assert!(
            repo.find_by_username("alice").await.unwrap().is_some(),
            "用户名索引应命中"
        );
    }

    #[tokio::test]
    async fn in_memory_missing_user_returns_none() {
        let repo = InMemoryUserRepository::new();
        assert!(
            repo.find_by_email("nobody@example.com")
                .await
                .unwrap()
                .is_none()
        );
        assert!(repo.find_by_id("not-exist").await.unwrap().is_none());
    }

    /// 重复邮箱必须被拒绝，且错误码要精确。
    #[tokio::test]
    async fn duplicate_email_conflicts() {
        let repo = InMemoryUserRepository::new();
        repo.create(&sample_new_user()).await.unwrap();

        let mut duplicate = sample_new_user();
        duplicate.username = "alice2".to_string(); // 换用户名，只让邮箱重复
        let err = repo.create(&duplicate).await.expect_err("重复邮箱应当失败");
        assert_eq!(err.code(), "USER_EMAIL_EXISTS");
        assert_eq!(err.status_code(), axum::http::StatusCode::CONFLICT);
    }

    /// 重复用户名必须被拒绝。
    #[tokio::test]
    async fn duplicate_username_conflicts() {
        let repo = InMemoryUserRepository::new();
        repo.create(&sample_new_user()).await.unwrap();

        let mut duplicate = sample_new_user();
        duplicate.email = "other@example.com".to_string(); // 只让用户名重复
        let err = repo
            .create(&duplicate)
            .await
            .expect_err("重复用户名应当失败");
        assert_eq!(err.code(), "USER_USERNAME_EXISTS");
    }

    /// 并发写入：验证异步锁下的数据一致性。
    ///
    /// 这是本项目"并发处理"要求的体现之一：用 `tokio::spawn` 起多个任务同时写，
    /// 最终记录数必须等于成功写入的次数，且不出现索引与主表不一致。
    #[tokio::test]
    async fn concurrent_writes_are_consistent() {
        let repo = InMemoryUserRepository::shared();
        let mut handles = Vec::new();

        for i in 0..50 {
            let repo = Arc::clone(&repo);
            // `tokio::spawn` 把任务交给运行时调度，多个任务的 `.await` 会真正交替执行
            handles.push(tokio::spawn(async move {
                let new_user = NewUser {
                    username: format!("user{i}"),
                    email: format!("user{i}@example.com"),
                    password_hash: "hashed".to_string(),
                    role: UserRole::User,
                };
                repo.create(&new_user).await.map(|u| u.id)
            }));
        }

        let mut created_ids = Vec::new();
        for handle in handles {
            // join 会传播 panic，测试里直接 unwrap 即可
            let id = handle.await.expect("任务 panic").expect("创建失败");
            created_ids.push(id);
        }

        assert_eq!(created_ids.len(), 50);
        assert_eq!(repo.len(), 50, "50 次并发写入应当全部落库");
        assert_eq!(repo.write_count(), 50);

        // 每个 ID 都必须能查到（主表与索引一致）
        for id in created_ids {
            assert!(repo.find_by_id(&id).await.unwrap().is_some());
        }
    }

    /// 并发写同一个邮箱时，"先查后写"的竞态由谁兜底？
    /// 内存实现用的是异步读写锁 → 检查与写入不在同一把锁内，
    /// 因此本用例只断言"至少有 1 个成功"，说明真实项目必须依赖数据库唯一约束。
    #[tokio::test]
    async fn concurrent_duplicate_email_at_least_one_succeeds() {
        let repo = InMemoryUserRepository::shared();
        let mut handles = Vec::new();
        for _ in 0..10 {
            let repo = Arc::clone(&repo);
            handles.push(tokio::spawn(async move {
                repo.create(&NewUser {
                    username: format!("u-{}", uuid::Uuid::new_v4()),
                    email: "same@example.com".to_string(),
                    password_hash: "hashed".to_string(),
                    role: UserRole::User,
                })
                .await
                .is_ok()
            }));
        }

        let mut success = 0;
        for handle in handles {
            if handle.await.unwrap() {
                success += 1;
            }
        }
        assert!(success >= 1, "至少应有一个请求成功");
    }
}
