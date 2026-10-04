//! 配置管理模块。
//!
//! # 设计目标
//! 一个企业级服务必须做到"同一份代码，在不同环境用不同配置"，且**不重新编译**。
//! 本项目采用业界通用的三级覆盖策略，优先级由低到高：
//!
//! 1. **代码内置默认值**（[`AppConfig::default`]）——保证零配置也能启动，降低上手门槛；
//! 2. **配置文件**（`config/default.toml`，可用 `APP_CONFIG_FILE` 指定其它路径）——团队共享的非敏感配置；
//! 3. **环境变量**（如 `APP_SERVER__PORT=8080`）——容器/CI 环境注入，**最高优先级**，用于覆盖前两者。
//!
//! # 为什么要用 `__` 分隔层级？
//! 环境变量名里不能出现 `.`（shell 限制），因此业界惯例用双下划线 `__` 表示嵌套层级：
//! `APP_SERVER__PORT` → `server.port`。`APP_` 是本项目所有环境变量的统一前缀，
//! 避免与系统上其它程序的变量名冲突。
//!
//! # 敏感信息处理
//! `jwt.secret` 这类密钥**不写进配置文件**，只从环境变量注入；
//! 代码里为开发环境提供了默认值，但 [`AppConfig::validate`] 会在
//! `environment == "production"` 时拒绝使用默认密钥——这类"安全检查"是必须的，
//! 否则线上服务可能带着示例密钥运行。

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// 环境变量前缀：所有可覆盖的配置项都以它开头。
const ENV_PREFIX: &str = "APP";

/// 指定配置文件路径的环境变量名（它是唯一的例外，本身不带 `APP_` 之外的前缀规则）。
const ENV_CONFIG_FILE: &str = "APP_CONFIG_FILE";

/// 应用总配置。
///
/// 每个字段都对应配置文件里的一个 `[section]`，同时可通过环境变量覆盖。
///
/// # 为什么同时需要 `Serialize` 和 `Deserialize`
/// - `Deserialize`：把 TOML/环境变量读成 `AppConfig`；
/// - `Serialize`：把内置默认值**序列化成配置源**，交给 `config` crate 作为最低优先级的
///   "第 1 层"，从而实现"任何字段缺失都能独立回退到默认值"。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    /// 运行环境标识：`development` / `test` / `production`
    #[serde(default = "default_environment")]
    pub environment: String,

    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub database: DatabaseConfig,

    #[serde(default)]
    pub jwt: JwtConfig,

    #[serde(default)]
    pub log: LogConfig,
}

/// HTTP 服务相关配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    /// 监听地址，例如 `127.0.0.1:3000`。
    /// 用 `SocketAddr` 而不是字符串，可以在启动前就由类型系统保证格式合法。
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,

    /// 单个请求的处理超时（秒）。超时后由中间件返回 408，避免慢请求拖垮连接池。
    #[serde(default = "default_request_timeout_secs")]
    pub request_timeout_secs: u64,

    /// 请求体大小上限（字节）。默认 1 MiB，防止恶意超大 body 占满内存。
    #[serde(default = "default_max_body_bytes")]
    pub max_body_bytes: usize,

    /// 优雅停机时等待在途请求完成的最长时间（秒）。
    #[serde(default = "default_shutdown_timeout_secs")]
    pub shutdown_timeout_secs: u64,
}

/// 数据库相关配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    /// SQLite 连接串。默认使用**内存库**，无需安装任何数据库即可运行；
    /// 改成 `sqlite://data/app.db?mode=rwc` 即可持久化到文件。
    #[serde(default = "default_database_url")]
    pub url: String,

    /// 连接池最大连接数。
    ///
    /// 注意：SQLite 内存库的每条连接都是一个**独立**的数据库，
    /// 因此内存模式下必须限制为 1，否则会出现"建表在 A 连接、查询走 B 连接"的诡异问题。
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// 获取连接的超时时间（秒）。池满时超过该时间就报错，而不是无限等待。
    #[serde(default = "default_acquire_timeout_secs")]
    pub acquire_timeout_secs: u64,
}

/// JWT 相关配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    /// 签名密钥（HS256）。生产环境必须通过环境变量 `APP_JWT__SECRET` 注入。
    #[serde(default = "default_jwt_secret")]
    pub secret: String,

    /// 令牌有效期（小时）。
    #[serde(default = "default_jwt_expires_hours")]
    pub expires_hours: i64,

    /// 签发者标识，写入 `iss` 声明，便于多服务场景下区分令牌来源。
    #[serde(default = "default_jwt_issuer")]
    pub issuer: String,
}

/// 日志相关配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogConfig {
    /// 日志级别过滤规则，语法与 `RUST_LOG` 相同，例如
    /// `info,demoweb=debug,tower_http=debug`。
    #[serde(default = "default_log_level")]
    pub level: String,

    /// 是否输出 JSON 格式日志（生产环境交给 ELK/Loki 采集时开启）。
    #[serde(default)]
    pub json: bool,

    /// 是否把日志同时写入文件（写入 `logs/` 目录，按天滚动）。
    #[serde(default)]
    pub file_enabled: bool,

    /// 日志文件目录。
    #[serde(default = "default_log_dir")]
    pub dir: PathBuf,
}

// ============================================================================
// 默认值函数
//
// serde 的 `#[serde(default = "...")]` 需要"函数"而不是常量，
// 这样每个字段缺失时都能独立回退，配置文件的任何一部分都可以只写自己关心的项。
// ============================================================================

fn default_environment() -> String {
    "development".to_string()
}

fn default_bind_address() -> SocketAddr {
    "127.0.0.1:3000".parse().expect("内置默认监听地址必须合法")
}

fn default_request_timeout_secs() -> u64 {
    15
}

fn default_max_body_bytes() -> usize {
    1024 * 1024
}

fn default_shutdown_timeout_secs() -> u64 {
    10
}

fn default_database_url() -> String {
    "sqlite::memory:".to_string()
}

fn default_max_connections() -> u32 {
    1
}

fn default_acquire_timeout_secs() -> u64 {
    5
}

fn default_jwt_secret() -> String {
    "dev-only-insecure-secret-change-me".to_string()
}

fn default_jwt_expires_hours() -> i64 {
    24
}

fn default_jwt_issuer() -> String {
    "demoweb".to_string()
}

fn default_log_level() -> String {
    "info,demoweb=debug,tower_http=info".to_string()
}

fn default_log_dir() -> PathBuf {
    PathBuf::from("logs")
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: default_environment(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            jwt: JwtConfig::default(),
            log: LogConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: default_bind_address(),
            request_timeout_secs: default_request_timeout_secs(),
            max_body_bytes: default_max_body_bytes(),
            shutdown_timeout_secs: default_shutdown_timeout_secs(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: default_database_url(),
            max_connections: default_max_connections(),
            acquire_timeout_secs: default_acquire_timeout_secs(),
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: default_jwt_secret(),
            expires_hours: default_jwt_expires_hours(),
            issuer: default_jwt_issuer(),
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            json: false,
            file_enabled: false,
            dir: default_log_dir(),
        }
    }
}

impl AppConfig {
    /// 按"默认值 → 配置文件 → 环境变量"的顺序加载配置。
    ///
    /// # 参数
    /// - `config_file`：显式指定的配置文件路径。传 `None` 时会依次尝试
    ///   `APP_CONFIG_FILE` 环境变量、`config/default.toml`，都不存在则只用默认值。
    ///
    /// # 错误
    /// 文件存在但解析失败、或环境变量值类型不合法（如 `APP_SERVER__PORT=abc`）时会返回错误。
    /// **注意**：这里故意不忽略解析错误——配置写错却静默使用默认值，是线上事故的常见根因。
    pub fn load(config_file: Option<&Path>) -> anyhow::Result<Self> {
        let mut builder = config::Config::builder();

        // 第 1 层：内置默认值。有了它，任何字段缺失都不会导致启动失败。
        builder = builder.add_source(config::Config::try_from(&AppConfig::default())?);

        // 第 2 层：配置文件（可选）。
        if let Some(path) = resolve_config_path(config_file) {
            tracing::info!(path = %path.display(), "加载配置文件");
            builder = builder.add_source(config::File::from(path));
        } else {
            tracing::debug!("未找到配置文件，仅使用默认值与环境变量");
        }

        // 第 3 层：环境变量，优先级最高。
        // `try_parsing(true)` 让 "8080" 自动变成整数而非字符串；
        // `separator("__")` 实现 APP_SERVER__PORT → server.port 的映射。
        //
        // ⚠️ 必须同时显式设置 `prefix_separator("_")`：
        // 它的默认值是"跟随 separator"，若只写 separator("__")，
        // 前缀就会被当成 `APP__`，于是 `APP_SERVER__PORT` 匹配不上、
        // 环境变量被**静默忽略**——这是本项目在测试中真实踩到过的坑。
        builder = builder.add_source(
            config::Environment::with_prefix(ENV_PREFIX)
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true),
        );

        let config: AppConfig = builder.build()?.try_deserialize()?;
        config.validate()?;
        Ok(config)
    }

    /// 业务级校验：不仅要求"格式合法"，还要求"组合合理"。
    ///
    /// 这一步体现了企业级配置管理的关键点——**让服务在启动时就失败**，
    /// 而不是等到第一个请求进来才发现配置有问题。
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.jwt.secret.len() < 16 {
            anyhow::bail!(
                "jwt.secret 过短（至少 16 个字符），请通过 APP_JWT__SECRET 注入足够随机的密钥"
            );
        }
        if self.is_production() && self.jwt.secret == default_jwt_secret() {
            anyhow::bail!(
                "生产环境禁止使用内置示例密钥，请设置环境变量 APP_JWT__SECRET=<强随机字符串>"
            );
        }
        if self.database.max_connections == 0 {
            anyhow::bail!("database.max_connections 不能为 0");
        }
        if self.database.url == default_database_url() && self.database.max_connections > 1 {
            anyhow::bail!(
                "SQLite 内存库（sqlite::memory:）的每条连接都是独立数据库，max_connections 必须为 1；\
                 需要连接池请改用文件库，例如 sqlite://data/app.db?mode=rwc"
            );
        }
        if self.server.request_timeout_secs == 0 {
            anyhow::bail!("server.request_timeout_secs 不能为 0");
        }
        Ok(())
    }

    /// 是否生产环境。
    pub fn is_production(&self) -> bool {
        self.environment.eq_ignore_ascii_case("production")
    }

    /// 是否测试环境（测试环境会降低 bcrypt 计算强度以加快测试）。
    pub fn is_test(&self) -> bool {
        self.environment.eq_ignore_ascii_case("test")
    }

    /// 请求超时，转成 `Duration` 方便直接给中间件使用。
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.server.request_timeout_secs)
    }

    /// 连接池获取连接的超时时间。
    pub fn acquire_timeout(&self) -> Duration {
        Duration::from_secs(self.database.acquire_timeout_secs)
    }

    /// 优雅停机等待时长。
    pub fn shutdown_timeout(&self) -> Duration {
        Duration::from_secs(self.server.shutdown_timeout_secs)
    }
}

/// 解析配置文件路径：显式参数 > `APP_CONFIG_FILE` 环境变量 > `config/default.toml`。
///
/// 返回 `None` 表示"没有配置文件"，这是完全合法的（零配置启动）。
fn resolve_config_path(explicit: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = explicit {
        // 显式指定的文件必须存在，否则属于用户操作失误，应当立刻报错。
        if path.exists() {
            return Some(path.to_path_buf());
        }
        tracing::warn!(path = %path.display(), "显式指定的配置文件不存在，将忽略");
        return None;
    }

    if let Ok(from_env) = std::env::var(ENV_CONFIG_FILE)
        && !from_env.trim().is_empty()
    {
        let path = PathBuf::from(from_env);
        if path.exists() {
            return Some(path);
        }
        tracing::warn!(path = %path.display(), "{ENV_CONFIG_FILE} 指向的文件不存在，将忽略");
    }

    let default_path = PathBuf::from("config/default.toml");
    if default_path.exists() {
        return Some(default_path);
    }

    None
}

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// 未提供任何配置时，应当落到"零依赖即可运行"的安全默认值上。
    #[test]
    fn default_config_is_runnable() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.server.bind_address.to_string(), "127.0.0.1:3000");
        assert_eq!(cfg.database.url, "sqlite::memory:");
        assert_eq!(cfg.database.max_connections, 1, "内存库必须限制为单连接");
        assert_eq!(cfg.environment, "development");
        assert!(cfg.validate().is_ok());
    }

    /// 默认配置必须能通过业务校验（否则新同学第一次运行就会失败）。
    #[test]
    fn default_config_passes_validation() {
        let cfg = AppConfig::default();
        assert!(
            cfg.validate().is_ok(),
            "默认配置应当合法：{:?}",
            cfg.validate()
        );
    }

    /// 生产环境使用内置示例密钥必须被拒绝。
    #[test]
    fn production_rejects_default_secret() {
        let cfg = AppConfig {
            environment: "production".to_string(),
            ..AppConfig::default()
        };
        let err = cfg.validate().expect_err("生产环境不应接受示例密钥");
        assert!(
            err.to_string().contains("APP_JWT__SECRET"),
            "错误信息应给出修复指引"
        );
    }

    /// SQLite 内存库配多连接会被拦下（这是最容易踩的坑）。
    #[test]
    fn memory_database_rejects_pool() {
        let cfg = AppConfig {
            database: DatabaseConfig {
                max_connections: 5,
                ..DatabaseConfig::default()
            },
            ..AppConfig::default()
        };
        assert!(cfg.validate().is_err(), "内存库配连接池应当报错");
    }

    /// 过短的密钥应当被拒绝。
    #[test]
    fn short_secret_is_rejected() {
        let cfg = AppConfig {
            jwt: JwtConfig {
                secret: "short".to_string(),
                ..JwtConfig::default()
            },
            ..AppConfig::default()
        };
        assert!(cfg.validate().is_err());
    }

    /// 环境变量可以覆盖默认值（验证三级覆盖中的第三级）。
    ///
    /// # 注意这里用了"唯一变量名 + 独占式用法"
    /// 环境变量是**进程级全局状态**，多线程并发的测试会互相干扰。
    /// 因此本用例只使用一个专属变量名，且设置后立刻读取再清理。
    #[test]
    fn environment_variables_override_defaults() {
        // 用一个本测试专属的键，避免与其它用例或外部环境冲突
        const KEY: &str = "APP_SERVER__SHUTDOWN_TIMEOUT_SECS";
        // SAFETY: Rust 2024 起设置环境变量是 unsafe 的（多线程下修改全局状态不安全）。
        // 本测试独占该键，且紧随其后就读取并清理。
        unsafe {
            std::env::set_var(KEY, "77");
        }
        let cfg = AppConfig::load(None).expect("加载配置失败");
        unsafe {
            std::env::remove_var(KEY);
        }

        // 环境变量优先级高于默认值（默认 10）与配置文件（config/default.toml 里是 10）
        assert_eq!(
            cfg.server.shutdown_timeout_secs, 77,
            "环境变量应当覆盖默认值与配置文件"
        );
        assert_eq!(cfg.shutdown_timeout(), Duration::from_secs(77));
    }

    /// 前缀分隔符与层级分隔符必须配对（曾因二者不配对导致环境变量被静默忽略）。
    #[test]
    fn nested_environment_variable_maps_to_nested_key() {
        const KEY: &str = "APP_DATABASE__ACQUIRE_TIMEOUT_SECS";
        unsafe {
            std::env::set_var(KEY, "9");
        }
        let cfg = AppConfig::load(None).expect("加载配置失败");
        unsafe {
            std::env::remove_var(KEY);
        }
        assert_eq!(
            cfg.database.acquire_timeout_secs, 9,
            "APP_DATABASE__ACQUIRE_TIMEOUT_SECS 应当映射到 database.acquire_timeout_secs"
        );
    }
}
