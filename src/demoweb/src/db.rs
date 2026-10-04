//! 数据库层：连接池构建与迁移执行。
//!
//! # 为什么用连接池而不是"每次请求新建连接"
//! TCP 连接 + 认证的开销远大于一次查询本身。连接池复用物理连接，
//! 把"获取连接"变成从队列里取一个句柄，是 Web 服务的标配。
//!
//! # SQLite 内存库的陷阱（本项目默认使用内存库，务必读懂）
//! SQLite 的 `:memory:` 数据库是**绑定在连接上**的：每条新连接都是一个全新的空库。
//! 因此如果连接池里放了多条连接，就会出现经典的诡异现象：
//! - 迁移在连接 A 上执行（建了表）；
//! - 业务查询被分配到连接 B（表不存在，报 `no such table`）。
//!
//! 两种正确做法：
//! 1. **限制池大小为 1**（本项目默认，简单可靠，配置校验里也强制了这一点）；
//! 2. 使用共享缓存 + 具名内存库（`file:memdb1?mode=memory&cache=shared`），
//!    这样多条连接看到同一个库，但需要保证有一条"常驻连接"存活，否则库会被销毁。
//!
//! 生产环境请改用文件库：`sqlite://data/app.db?mode=rwc`。

use std::str::FromStr;
use std::time::Duration;

use sqlx::SqlitePool;
// `ConnectOptions` 是提供 `disable_statement_logging()` 等通用连接选项的 trait，
// 必须导入它才能在 `SqliteConnectOptions` 上调用这些方法——Rust 的 trait 方法
// 要求 trait 本身在作用域内，这是新手常见的"方法不存在"报错原因。
use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};

use crate::config::AppConfig;

/// 创建数据库连接池。
///
/// 参数里的超时来自配置，避免写死魔法数字。
pub async fn create_pool(config: &AppConfig) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(&config.database.url)
        .map_err(|e| anyhow::anyhow!("数据库连接串非法（{}）：{e}", config.database.url))?
        // 外键约束：SQLite 默认**关闭**外键检查，必须显式打开，
        // 否则建表时写的 FOREIGN KEY 形同虚设。
        .foreign_keys(true)
        // WAL 模式：读写并发更好（内存库会忽略该设置，文件库有效）。
        .journal_mode(SqliteJournalMode::Wal)
        // 内存库无法使用 WAL，回退为 NORMAL 同步级别以兼顾性能与安全。
        .synchronous(SqliteSynchronous::Normal)
        // 忙等待：写锁被占用时最多等待 2 秒再报错，减少"database is locked"。
        .busy_timeout(Duration::from_secs(2))
        // 关闭 sqlx 的语句级日志（它很吵），我们用自己的 tracing 记录慢查询。
        .disable_statement_logging();

    let pool = SqlitePoolOptions::new()
        .max_connections(config.database.max_connections)
        // 池满时的等待上限：宁可快速失败返回 503，也不要无限堆积请求。
        .acquire_timeout(config.acquire_timeout())
        .connect_with(options)
        .await
        .map_err(|e| anyhow::anyhow!("连接数据库失败：{e}"))?;

    tracing::debug!(
        url = %config.database.url,
        max_connections = config.database.max_connections,
        "数据库连接池已创建"
    );

    Ok(pool)
}

/// 执行数据库迁移。
///
/// `sqlx::migrate!` 是**编译期**宏：它把 `migrations/` 目录下的 SQL 文件
/// 嵌入到二进制里。好处是部署时不需要额外拷贝 SQL 文件，
/// 且"代码与迁移脚本版本不一致"这类事故从根上被消除。
///
/// 迁移是有状态的：sqlx 会在库里维护 `_sqlx_migrations` 表，
/// 已执行过的迁移不会重复执行。
pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| anyhow::anyhow!("数据库迁移失败：{e}"))?;

    tracing::info!("数据库迁移执行完成");
    Ok(())
}

/// 健康检查：用一条极轻量的查询确认连接池真的可用。
///
/// 为什么不用 `pool.acquire()`？——因为它只能证明"拿到了连接"，
/// 不能证明"数据库能响应查询"。`SELECT 1` 才是完整的端到端探测。
pub async fn health_check(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let _: i64 = sqlx::query_scalar("SELECT 1").fetch_one(pool).await?;
    Ok(())
}

/// 初始化数据库：建池 + 迁移，一步到位。
pub async fn init(config: &AppConfig) -> anyhow::Result<SqlitePool> {
    let pool = create_pool(config).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    // `DatabaseConfig` 仅在测试里用于构造局部覆盖的配置
    use crate::config::DatabaseConfig;

    /// 测试专用配置：内存库 + 单连接 + 强制走测试环境。
    fn test_config() -> AppConfig {
        // 用结构体更新语法一次性给出需要覆盖的字段，其余走默认值
        AppConfig {
            environment: "test".to_string(),
            database: DatabaseConfig {
                url: "sqlite::memory:".to_string(),
                max_connections: 1,
                ..DatabaseConfig::default()
            },
            ..AppConfig::default()
        }
    }

    #[tokio::test]
    async fn pool_and_migrations_work() {
        let pool = init(&test_config()).await.expect("初始化数据库失败");
        health_check(&pool).await.expect("健康检查应通过");

        // 迁移建好的 users 表应当存在且可查询。
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .expect("users 表应存在");
        assert_eq!(count, 0);
    }

    /// 迁移应当是幂等的：重复执行不会报错（靠 _sqlx_migrations 表记录状态）。
    #[tokio::test]
    async fn migrations_are_idempotent() {
        let pool = init(&test_config()).await.expect("首次迁移失败");
        run_migrations(&pool).await.expect("重复迁移不应报错");
    }

    /// 非法连接串应当在创建池时报错，而不是等到第一次查询。
    #[tokio::test]
    async fn invalid_url_fails_fast() {
        let mut cfg = test_config();
        cfg.database.url = "这不是一个合法的连接串".to_string();
        assert!(create_pool(&cfg).await.is_err(), "非法连接串应当快速失败");
    }
}
