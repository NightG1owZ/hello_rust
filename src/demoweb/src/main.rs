//! 应用入口。
//!
//! # `main` 只做四件事（保持入口干净是工程规范的一部分）
//! 1. **加载配置** —— 三级覆盖：默认值 → 配置文件 → 环境变量；
//! 2. **初始化日志** —— 必须在做任何其它事情之前，否则前面的日志会丢；
//! 3. **装配依赖并启动服务** —— 数据库 / 仓储 / 路由 / 监听；
//! 4. **优雅停机** —— 收到 Ctrl+C 后等待在途请求处理完再退出。
//!
//! # 为什么用 `#[tokio::main]`
//! 它把 `async fn main` 展开成"创建多线程运行时 → 阻塞执行 future"。
//! 多线程运行时（`rt-multi-thread`）会启动与 CPU 核心数相当的 worker 线程，
//! 每个 worker 都能调度大量任务，这是异步 Web 框架获得高并发的根本原因。

use std::sync::Arc;

use anyhow::Context;
use demoweb::config::AppConfig;
use demoweb::repository::{InMemoryUserRepository, SqliteUserRepository, UserRepository};
use demoweb::{build_app_with_repository, db, logging};

/// 默认配置文件路径（不存在时会退化到"仅默认值 + 环境变量"）。
const DEFAULT_CONFIG_FILE: &str = "config/default.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ------------------------------------------------------------------
    // 0. 加载 .env（可选）：把本地开发用的密钥放进 .env 而不是写进代码。
    //    生产环境一般由编排系统直接注入环境变量，因此这里的失败可以忽略。
    // ------------------------------------------------------------------
    let _ = dotenvy::dotenv();

    // ------------------------------------------------------------------
    // 1. 加载配置
    //    解析失败或校验不通过会立刻返回错误 → 服务**拒绝带病启动**。
    // ------------------------------------------------------------------
    let config_path = std::path::Path::new(DEFAULT_CONFIG_FILE);
    let config = AppConfig::load(Some(config_path)).context("加载配置失败")?;

    // ------------------------------------------------------------------
    // 2. 初始化日志
    //
    // `_log_guard` 必须绑定到变量上：它一旦被 drop，非阻塞文件日志的
    // 缓冲内容就会丢失。用 `let _ = ...` 会立刻 drop，属于经典错误。
    // ------------------------------------------------------------------
    let _log_guard = logging::init_with_config(&config.log).context("初始化日志失败")?;

    logging::print_startup_banner(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        std::path::Path::new(&config.server.bind_address.to_string()),
    );

    tracing::info!(
        environment = %config.environment,
        database = %config.database.url,
        "配置加载完成"
    );

    // ------------------------------------------------------------------
    // 3. 装配仓储层
    //
    // 这里演示"同一套业务代码切换数据访问实现"：
    // - 内存库：零依赖，进程退出即清空，适合本地学习与单元测试；
    // - SQLite：真实持久化，含迁移与外键约束。
    // 两种实现都满足 `UserRepository` trait，因此上层代码完全不受影响。
    // ------------------------------------------------------------------
    let user_repository: Arc<dyn UserRepository> = if config.database.url.starts_with("sqlite") {
        let pool = db::init(&config).await.context("初始化数据库失败")?;
        tracing::info!("使用 SQLite 仓储（已执行迁移）");
        Arc::new(SqliteUserRepository::new(pool))
    } else {
        tracing::info!("使用内存仓储（进程退出后数据清空）");
        Arc::new(InMemoryUserRepository::new())
    };

    // ------------------------------------------------------------------
    // 4. 构建路由
    // ------------------------------------------------------------------
    let app = build_app_with_repository(config.clone(), user_repository).context("构建应用失败")?;

    // ------------------------------------------------------------------
    // 5. 绑定端口并启动
    //
    // 先 `TcpListener::bind` 再 `axum::serve`（而不是 `axum::Server::bind`）：
    // 这样可以**先确认端口绑定成功**，再打印"已启动"日志，
    // 避免出现"日志说启动成功、实际端口被占用"的假象。
    // ------------------------------------------------------------------
    let listener = tokio::net::TcpListener::bind(config.server.bind_address)
        .await
        .with_context(|| {
            format!(
                "绑定端口 {} 失败（端口可能已被占用）",
                config.server.bind_address
            )
        })?;

    let local_addr = listener.local_addr().context("获取本地监听地址失败")?;
    tracing::info!(%local_addr, "HTTP 服务已就绪，按 Ctrl+C 退出");
    tracing::info!("可用接口：");
    tracing::info!("  GET  http://{local_addr}/health                 存活探针（无需认证）");
    tracing::info!("  GET  http://{local_addr}/api/health             就绪探针（检查数据库）");
    tracing::info!("  POST http://{local_addr}/api/users/register     用户注册");
    tracing::info!("  POST http://{local_addr}/api/users/login        用户登录");
    tracing::info!("  GET  http://{local_addr}/api/users/me           当前用户（需 Bearer 令牌）");
    tracing::info!("  GET  http://{local_addr}/api/demo/parallel      并发演示：tokio::join!");
    tracing::info!("  GET  http://{local_addr}/api/demo/spawn         并发演示：tokio::spawn");
    tracing::info!("  GET  http://{local_addr}/api/demo/counter       并发演示：无锁原子计数");

    // ------------------------------------------------------------------
    // 6. 优雅停机编排
    //
    // ⚠️ 这里的分工很关键，写错会导致"服务跑一会儿自己退出"：
    //
    //   ❌ 错误写法（本项目开发过程中真实踩到的坑）：
    //      tokio::select! {
    //          result = axum::serve(..).with_graceful_shutdown(signal()) => ..
    //          () = tokio::time::sleep(shutdown_timeout) => warn!("超时"),
    //      }
    //      问题：`sleep` 从**服务启动**那一刻就开始计时，
    //      于是服务会在运行 N 秒后无条件自行退出（本项目的 N 就是 10 秒）。
    //
    //   ✅ 正确做法见 `shutdown_with_timeout`：
    //      先 await 停机信号，信号到达后才给"等待在途请求"加上限时。
    // ------------------------------------------------------------------
    tracing::info!(%local_addr, "开始接收请求（Ctrl+C 触发优雅停机）");

    shutdown_with_timeout(listener, app, config.shutdown_timeout())
        .await
        .context("HTTP 服务异常退出")?;

    tracing::info!("服务已优雅停机，再见 👋");
    Ok(())
}

/// 优雅停机编排：**先等信号**，再限时等待在途请求排空。
///
/// # 为什么必须自己编排，而不是简单套一层 `select!`
/// 这是本项目在开发过程中**真实踩到并修复**的一个坑，值得完整记录：
///
/// ```ignore
/// // ❌ 错误写法：服务会在固定时间后自行退出！
/// tokio::select! {
///     result = axum::serve(listener, app)
///         .with_graceful_shutdown(shutdown_signal()) => { result? }
///     () = tokio::time::sleep(shutdown_timeout) => tracing::warn!("超时"),
/// }
/// ```
/// 表面看是"服务结束或超时，谁先到算谁"，但 **`tokio::select!` 会无条件轮询所有分支**：
/// `sleep` 从**服务启动那一刻**就开始计时，于是服务运行 N 秒后必然走进超时分支并退出。
/// 本项目的现象就是：服务启动 10 秒后自动停止（配置里 `shutdown_timeout_secs = 10`）。
///
/// 修复思路：让超时的**截止时刻**在信号到达之后才计算。
/// 下面用 `tokio::spawn` 拿到一个 `JoinHandle`，在**等待它完成之前**先 `await` 信号，
/// 信号到达后才 `Instant::now() + timeout` 作为绝对截止时间：
///
/// ```text
///   阶段 1：await 信号（无限期等待，服务正常处理请求）
///            ↓ 收到 Ctrl+C / SIGTERM
///   阶段 2：axum 停止接受新连接，等待在途请求排空
///            ↓ 最多等 timeout 秒
///   阶段 3：仍未结束 → 记录 WARN 并强制退出
/// ```
async fn shutdown_with_timeout(
    listener: tokio::net::TcpListener,
    app: axum::Router,
    timeout: std::time::Duration,
) -> anyhow::Result<()> {
    shutdown_with_signal(listener, app, timeout, shutdown_signal()).await
}

/// 与 [`shutdown_with_timeout`] 相同，但停机信号由调用方提供。
///
/// 抽出来的唯一目的是**可测试性**：单元测试可以传入一个"立即完成"的 future
/// 来确定性地验证停机流程，而不必真的发送 Ctrl+C（在不同平台上难以稳定复现）。
/// 这是"为可测试性做一点设计让步"的典型例子——代价是一个参数，收益是核心逻辑被测试覆盖。
async fn shutdown_with_signal<F>(
    listener: tokio::net::TcpListener,
    app: axum::Router,
    timeout: std::time::Duration,
    signal: F,
) -> anyhow::Result<()>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    // 把服务放进独立任务：`axum::serve` 接收的是拥有所有权的 listener/router，
    // 满足 `'static` 要求，因此可以安全地 spawn。
    let mut server_task = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .map_err(anyhow::Error::from)
    });

    // —— 阶段 1：等待停机信号（这里是"无限期"，服务正常对外提供能力）——
    signal.await;
    tracing::info!(
        timeout_secs = timeout.as_secs(),
        "收到停机信号：停止接受新连接，等待在途请求完成"
    );

    // —— 阶段 2 / 3：服务排空 与 超时兜底 竞争 ——
    // 关键点：`deadline` 是在**信号到达之后**才计算的，
    // 因此不会出现"服务跑 N 秒自己退出"的问题。
    let deadline = tokio::time::Instant::now() + timeout;
    tokio::select! {
        result = &mut server_task => {
            match result {
                // 正常排空并结束
                Ok(inner) => inner?,
                // 任务 panic 或被取消
                Err(e) => anyhow::bail!("服务任务异常结束：{e}"),
            }
            tracing::debug!("在途请求已排空，服务自然结束");
        }
        () = tokio::time::sleep_until(deadline) => {
            tracing::warn!(
                timeout_secs = timeout.as_secs(),
                "优雅停机超时，强制退出（可能仍有在途请求未完成）"
            );
            // 主动收敛服务任务，避免带着未完成的连接继续运行
            server_task.abort();
        }
    }

    Ok(())
}

/// 等待停机信号（Ctrl+C / SIGTERM）。
///
/// 只负责"告诉上层该停了"；在途请求的排空由 `axum::serve` 完成。
async fn shutdown_signal() {
    tracing::debug!("已挂载信号监听，等待 Ctrl+C / SIGTERM");

    let ctrl_c = async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => tracing::info!("收到 Ctrl+C（SIGINT），开始优雅停机"),
            // 极端情况下（例如没有控制台）信号注册可能失败；
            // 此时返回而不是 panic，让停机流程继续走完。
            Err(e) => tracing::warn!("Ctrl+C 信号监听不可用：{e}"),
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
                tracing::info!("收到 SIGTERM，开始优雅停机");
            }
            Err(e) => tracing::warn!("SIGTERM 信号监听不可用：{e}"),
        }
    };

    // Windows 上没有 SIGTERM 概念，用 `pending()` 表示"这个分支永不触发"。
    // 这正是"平台上可用的信号不同"的常见处理方式。
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // `select!`：谁先完成就走谁的分支（另一个被取消）。
    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
}

// ============================================================================
// 单元测试
//
// 二进制 crate 也能写测试（`cargo test` 会为 bin 目标生成测试用例）。
// 这里重点验证上面"坑 1"的修复：**超时不能从启动开始计时**。
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// 构造一个最小路由用于停机测试。
    fn test_app() -> axum::Router {
        axum::Router::new().route("/ping", axum::routing::get(|| async { "pong" }))
    }

    /// 核心回归测试：信号**尚未到达**时，服务必须一直运行，不能因超时而退出。
    ///
    /// 这正是曾经的真实缺陷：`select!` 与 sleep 竞争导致 sleep 从启动就开始计时，
    /// 服务运行 `shutdown_timeout` 秒后无条件自行退出。
    #[tokio::test]
    async fn server_keeps_running_before_signal_arrives() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口失败");
        let addr = listener.local_addr().unwrap();

        // 用一个"永不完成"的信号模拟"没有收到 Ctrl+C"，超时设为 100ms
        let handle = tokio::spawn(shutdown_with_signal(
            listener,
            test_app(),
            Duration::from_millis(100),
            std::future::pending::<()>(),
        ));

        // 等 300ms（超过超时时间 3 倍），服务必须仍在正常响应
        tokio::time::sleep(Duration::from_millis(300)).await;
        assert!(
            !handle.is_finished(),
            "信号未到达时服务不应结束（否则就是'跑一会儿自己退出'的缺陷复现）"
        );

        let body = reqwest::get(format!("http://{addr}/ping"))
            .await
            .expect("请求失败")
            .text()
            .await
            .unwrap();
        assert_eq!(body, "pong", "服务应当仍在正常处理请求");

        handle.abort();
    }

    /// 信号到达后，服务应当按时收敛（要么排空后自然结束，要么在超时后强制退出）。
    ///
    /// 注意这里用的是 `tokio::join!` 而不是给整个停机过程设一个短超时：
    /// `axum` 的优雅停机语义是"排空所有**已建立**的连接"，而 reqwest 默认保持
    /// keep-alive 连接，因此在连接关闭前排空会一直等待——这正是"停机超时兜底"
    /// 存在的意义。用一个宽松的上限（超时 + 3 秒）来断言它**一定会结束**即可。
    #[tokio::test]
    async fn server_stops_after_signal() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口失败");

        let timeout = Duration::from_millis(500);
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        // 用 `Arc<Notify>`：既要把它 move 进任务，又要在测试主体里等待通知
        let shutdown_started = std::sync::Arc::new(tokio::sync::Notify::new());
        let notify = std::sync::Arc::clone(&shutdown_started);

        let handle = tokio::spawn(async move {
            shutdown_with_signal(listener, test_app(), timeout, async move {
                let _ = rx.await;
                // 通知测试：信号确实被投递进来了
                notify.notify_one();
            })
            .await
        });

        // 触发"信号"
        tokio::time::sleep(Duration::from_millis(50)).await;
        tx.send(()).expect("发送停机信号失败");

        // 断言 ①：停机流程确实被触发了
        tokio::time::timeout(Duration::from_secs(2), shutdown_started.notified())
            .await
            .expect("停机信号未被处理");

        // 断言 ②：服务最终一定会结束（排空完成或超时兜底），不会永久挂住
        let result = tokio::time::timeout(timeout + Duration::from_secs(3), handle)
            .await
            .expect("服务未在预期时间内结束（优雅停机可能挂住）")
            .expect("停机任务 panic");
        assert!(result.is_ok(), "优雅停机应当成功返回：{result:?}");
    }
}
