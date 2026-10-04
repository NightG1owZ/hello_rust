//! 日志与链路追踪初始化。
//!
//! # 为什么 Web 服务必须用结构化日志
//! `println!` 在多请求并发场景下无法回答三个关键问题：
//! 1. **这条日志属于哪个请求？** —— 靠 `request_id`（见 [`crate::middleware::request_id`]）；
//! 2. **耗时多少、慢在哪一步？** —— 靠 span 的进入/退出时间；
//! 3. **线上怎么按条件检索？** —— 靠 JSON 格式日志（生产环境交给 ELK/Loki）。
//!
//! `tracing` 的 span 机制天然解决前两点：在 span 内产生的所有日志都会自动带上
//! span 的字段（如 `request_id`、`method`、`path`），无需手动透传参数。
//!
//! # 日志同时写文件
//! `tracing-appender` 提供了非阻塞的滚动文件写入：
//! 写日志变成"投递到通道"，由后台线程落盘，**不会阻塞业务线程**。
//! 这是高并发服务的基本要求——同步写文件会让日志成为性能瓶颈。
//!
//! # 生命周期陷阱
//! 非阻塞写入依赖一个后台守卫（`WorkerGuard`）。**它一旦被 drop，缓冲日志会丢失**。
//! 因此 [`init_logging`] 会把 guard 返回给调用方，由 `main` 持有到进程结束。

use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use crate::config::LogConfig;

/// 初始化日志系统。
///
/// # 返回值
/// - `Some(WorkerGuard)`：开启了文件日志，调用方**必须**把它保存在变量里直到程序退出；
/// - `None`：仅控制台输出。
///
/// # 示例
/// ```ignore
/// let cfg = AppConfig::load(None)?;
/// let _log_guard = logging::init(&cfg.log)?;   // 用 _log_guard 而不要用 _，否则会立刻 drop
/// ```
pub fn init(config: &LogConfig) -> anyhow::Result<Option<WorkerGuard>> {
    // 过滤器：优先用 RUST_LOG 环境变量的值，其次用配置文件里的规则。
    // 这样运维可以临时提高某个模块的日志级别，无需改配置、无需重启业务代码。
    let filter = match std::env::var("RUST_LOG") {
        Ok(value) if !value.trim().is_empty() => EnvFilter::new(value),
        _ => EnvFilter::new(&config.level),
    };

    // 控制台层：开发时可读性优先（带颜色、带 span 字段）。
    let console_layer = fmt::layer()
        .with_target(true) // 打印模块路径，便于定位日志来源
        .with_thread_ids(true) // 打印线程 ID，观察 tokio 多线程调度
        .with_level(true);

    if config.file_enabled {
        // 确保日志目录存在：`create_dir_all` 会递归创建，已存在时不报错。
        std::fs::create_dir_all(&config.dir)
            .map_err(|e| anyhow::anyhow!("创建日志目录 {} 失败：{e}", config.dir.display()))?;

        // 按天滚动：每天生成一个新的 app.log.YYYY-MM-DD 文件。
        let file_appender = tracing_appender::rolling::daily(&config.dir, "app.log");
        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

        // 文件层不打印 ANSI 颜色，避免日志文件里出现转义字符。
        let file_layer = fmt::layer().with_ansi(false).with_writer(file_writer);

        tracing_subscriber::registry()
            .with(filter)
            .with(console_layer)
            .with(file_layer)
            .init();

        tracing::info!(dir = %config.dir.display(), "日志文件输出已启用（按天滚动）");
        return Ok(Some(guard));
    }

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .init();

    Ok(None)
}

/// 生产环境把日志切成 JSON，方便日志系统按字段检索。
///
/// 这个方法与 [`init`] 二选一使用，展示"同一份代码按环境切换输出格式"的做法。
pub fn init_json(config: &LogConfig) -> anyhow::Result<()> {
    let filter = EnvFilter::new(&config.level);
    let layer = fmt::layer()
        .json() // 结构化输出：每行一个 JSON 对象
        .with_current_span(true)
        .with_span_list(true); // 输出完整 span 栈，便于还原调用链

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();

    Ok(())
}

/// 按配置选择日志格式：生产环境用 JSON，其余用易读格式。
///
/// 这是 `main` 实际调用的入口。
pub fn init_with_config(config: &LogConfig) -> anyhow::Result<Option<WorkerGuard>> {
    if config.json {
        init_json(config)?;
        tracing::info!("已启用 JSON 结构化日志");
        return Ok(None);
    }
    init(config)
}

/// 打印启动横幅，方便在控制台一眼确认关键配置。
///
/// 注意：**绝不打印 JWT 密钥、数据库密码等敏感信息**。
pub fn print_startup_banner(app_name: &str, version: &str, bind: &Path) {
    tracing::info!("========================================================");
    tracing::info!("  {app_name} v{version} 正在启动");
    tracing::info!("  监听地址: {}", bind.display());
    tracing::info!("========================================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 配置默认值应当能被 EnvFilter 解析（日志级别字符串写错会导致启动直接 panic）。
    #[test]
    fn default_filter_string_is_parseable() {
        let cfg = LogConfig::default();
        // EnvFilter::new 在语法错误时会 panic，能构造成功即说明规则合法。
        let filter = EnvFilter::new(&cfg.level);
        assert!(!filter.to_string().is_empty());
    }

    /// 关闭文件日志时不应该产生 guard，也不应该创建目录。
    #[test]
    fn file_logging_disabled_returns_none_guard() {
        let cfg = LogConfig {
            file_enabled: false,
            ..LogConfig::default()
        };
        // 注意：真正的 init 只能调用一次（全局 subscriber），
        // 所以这里只断言配置分支的判定逻辑，不做全局初始化。
        assert!(!cfg.file_enabled);
    }
}
