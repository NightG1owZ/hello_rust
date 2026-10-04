//! 应用共享状态。
//!
//! # 什么是"状态"
//! 处理器（handler）是无状态的函数，它们通过 `State<Arc<AppState>>` 提取器拿到
//! 需要的一切外部依赖：连接池、配置、以及正在运行的后台任务句柄等。
//!
//! # 为什么用 `Arc<AppState>` 而不是直接 `AppState`
//! axum 会把状态克隆给每个请求。若直接克隆 `AppState`，
//! 里面的连接池、配置、指标都会被深拷贝——既昂贵又会破坏共享语义。
//! `Arc`（原子引用计数）让克隆只增加一个计数，所有请求共享同一份数据。
//!
//! # 字段为什么是 `Arc<dyn Trait>` 而不是具体类型
//! 仓储（Repository）以 trait 对象形式保存，好处是：
//! - 单元测试可以注入"内存仓储"，无需真实数据库；
//! - 将来换成 PostgreSQL 实现，业务代码零改动（依赖倒置原则）。
//!
//! 注意：`sqlx::SqlitePool` 内部本身就是 `Arc`，克隆它同样很便宜，
//! 因此连接池无需再包一层 `Arc`。

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crate::config::AppConfig;
use crate::repository::UserRepository;
use crate::services::JwtService;

/// 全局应用状态。
///
/// 所有需要跨请求共享的东西都放在这里，避免使用全局可变变量（那是线程安全隐患）。
///
/// # 为什么需要 `Clone`
/// axum 在装配路由（`with_state`）以及每个请求提取 `State<T>` 时都会**克隆**状态。
/// 由于内部字段全是 `Arc` / 原子类型 / `Instant`，克隆只增加引用计数，
/// 代价极低——这正是"用 `Arc` 而不是深拷贝"的意义。
/// 注意：`Clone` 只克隆"句柄"，所有请求共享同一份底层数据。
#[derive(Clone)]
pub struct AppState {
    /// 应用配置（只读，克隆便宜：内部都是字符串与数值）。
    pub config: Arc<AppConfig>,
    /// 用户仓储（数据访问层）。
    ///
    /// `Arc<dyn UserRepository>` 是**依赖倒置**的体现：上层（服务层）依赖抽象，
    /// 下层（SQLite 实现）依赖同一抽象，两者可独立替换。
    pub user_repository: Arc<dyn UserRepository>,
    /// JWT 服务（认证中间件需要它来验签）。
    ///
    /// 放在状态里而不是每个路由各自构造：密钥解析与校验规则配置只做一次，
    /// 且保证全局使用同一套规则。
    pub jwt_service: Arc<JwtService>,
    /// 进程启动时刻，用于计算运行时长（健康检查接口会用到）。
    pub started_at: Instant,
    /// 已处理请求计数。
    ///
    /// 用 `AtomicU64` 而不是 `Mutex<u64>`：计数是"读-改-写"的原子操作，
    /// 用原子类型无锁且不会成为高并发下的瓶颈。
    ///
    /// # 为什么还要包一层 `Arc`
    /// `AtomicU64` 本身实现了 `Send + Sync`，但**没有实现 `Clone`**
    /// （克隆一个原子变量在语义上是含糊的：是复制当前值，还是共享同一个计数？）。
    /// 而 `AppState` 需要 `Clone`，因此用 `Arc` 明确表达"多个状态句柄共享同一个计数器"。
    pub request_counter: Arc<AtomicU64>,
}

impl AppState {
    /// 构造应用状态。
    pub fn new(
        config: AppConfig,
        user_repository: Arc<dyn UserRepository>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            config: Arc::new(config),
            user_repository,
            jwt_service,
            started_at: Instant::now(),
            request_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// 记录一次请求并返回当前累计值（供访问日志使用）。
    ///
    /// `Ordering::Relaxed` 足够：我们只要求计数不丢失，
    /// 不要求它与其它内存操作之间有顺序关系——这是原子类型用法的常见误区，
    /// 无脑用 `SeqCst` 会白白牺牲性能。
    pub fn record_request(&self) -> u64 {
        self.request_counter.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// 当前累计请求数。
    pub fn total_requests(&self) -> u64 {
        self.request_counter.load(Ordering::Relaxed)
    }

    /// 运行时长（秒）。
    pub fn uptime_secs(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    /// 便捷访问配置中的运行环境。
    pub fn environment(&self) -> &str {
        &self.config.environment
    }

    /// 构造用户服务。
    ///
    /// # 为什么在这里"按需构造"而不是把服务存进状态
    /// [`crate::services::UserService`] 内部只是三个 `Arc` 的克隆，
    /// 克隆一个 `Arc` 是原子的加一操作，开销可忽略；
    /// 而把它存进状态会让"状态 → 服务 → 仓储 → 状态"形成引用环的隐患。
    ///
    /// 另外，密码哈希器的代价因子需要依据运行环境决定（测试环境用低成本），
    /// 在构造时计算比在启动时硬编码更灵活。
    pub fn user_service(&self) -> crate::services::UserService {
        crate::services::UserService::new(
            Arc::clone(&self.user_repository),
            crate::services::PasswordHasher::for_environment(&self.config.environment),
            Arc::clone(&self.jwt_service),
        )
    }
}

impl std::fmt::Debug for AppState {
    /// 手写 `Debug`：`dyn UserRepository` 不满足 `Debug` 派生约束，
    /// 且我们也不想把配置里的密钥打印出来。只输出安全的摘要信息。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("environment", &self.config.environment)
            .field("uptime_secs", &self.uptime_secs())
            .field("total_requests", &self.total_requests())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 用一个最小的内存仓储来测试状态本身，避免依赖数据库。
    struct NullRepository;

    #[async_trait::async_trait]
    impl UserRepository for NullRepository {
        async fn find_by_email(
            &self,
            _email: &str,
        ) -> Result<Option<crate::models::User>, crate::error::AppError> {
            Ok(None)
        }
        async fn find_by_username(
            &self,
            _username: &str,
        ) -> Result<Option<crate::models::User>, crate::error::AppError> {
            Ok(None)
        }
        async fn find_by_id(
            &self,
            _id: &str,
        ) -> Result<Option<crate::models::User>, crate::error::AppError> {
            Ok(None)
        }
        async fn create(
            &self,
            _new_user: &crate::repository::NewUser,
        ) -> Result<crate::models::User, crate::error::AppError> {
            unimplemented!("本测试不使用")
        }
    }

    #[test]
    fn counters_and_uptime_work() {
        let jwt = Arc::new(
            JwtService::new(&AppConfig::default().jwt).expect("默认配置应能构造 JwtService"),
        );
        let state = AppState::new(AppConfig::default(), Arc::new(NullRepository), jwt);
        assert_eq!(state.total_requests(), 0);
        assert_eq!(state.record_request(), 1, "首次计数应为 1");
        assert_eq!(state.record_request(), 2);
        assert_eq!(state.total_requests(), 2);
        assert!(state.uptime_secs() < 60);
        let debug = format!("{state:?}");
        assert!(debug.contains("development"));
    }

    /// `Debug` 输出不能泄露密钥（防手滑打日志）。
    #[test]
    fn debug_does_not_leak_secret() {
        let config = AppConfig::default();
        let jwt = Arc::new(JwtService::new(&config.jwt).expect("默认配置应能构造 JwtService"));
        let state = AppState::new(config.clone(), Arc::new(NullRepository), jwt);
        let debug = format!("{state:?}");
        assert!(!debug.contains(&config.jwt.secret));
    }
}
