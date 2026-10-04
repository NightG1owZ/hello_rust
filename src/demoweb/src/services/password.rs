//! 密码哈希服务。
//!
//! # 为什么必须用 bcrypt 这类"慢哈希"
//! MD5/SHA-256 是为**速度**设计的，GPU 每秒能算几十亿次，
//! 拿到哈希库后爆破 8 位密码只需几分钟。
//! bcrypt 的设计目标正相反——**故意慢**，并且：
//! - 内置**随机盐**：同样的密码每次哈希结果都不同，彻底废掉彩虹表；
//! - **代价因子（cost）**可调：算力提升时增大 cost 即可保持同等安全强度。
//!
//! # 为什么必须用 `spawn_blocking`
//! bcrypt 是纯 CPU 计算，一次哈希在 cost=12 时约需 200~300 ms。
//! 如果直接在 async 函数里同步计算，会**阻塞 tokio 的工作线程**：
//! 该线程上排队的其它几百个任务全部停摆。这是 Rust 异步编程最经典的坑。
//!
//! 正确做法是把 CPU 密集任务丢给 [`tokio::task::spawn_blocking`]，
//! 它会在线程池里执行，异步运行时继续调度其它任务。
//!
//! ```text
//!   ❌ async fn hash(...) { bcrypt::hash(...) }          // 阻塞运行时
//!   ✅ async fn hash(...) { spawn_blocking(|| bcrypt::hash(...)).await }  // 正确
//! ```

use bcrypt::{DEFAULT_COST, hash, verify};
use tokio::task::spawn_blocking;

use crate::error::AppError;

/// 密码哈希器。
///
/// 保存 bcrypt 代价因子，便于在测试环境降低强度以加速测试
/// （**只在测试环境这么做**，生产环境必须用足够高的 cost）。
#[derive(Debug, Clone)]
pub struct PasswordHasher {
    cost: u32,
}

impl PasswordHasher {
    /// 使用默认代价因子（12）构造。
    pub fn new() -> Self {
        Self { cost: DEFAULT_COST }
    }

    /// 指定代价因子。
    ///
    /// cost 每 +1，计算耗时翻倍。取值建议：
    /// - 单元测试：4（毫秒级，测试套件才不会变成"分钟级"）；
    /// - 开发环境：8~10；
    /// - 生产环境：12 及以上（按服务器性能实测调整到 ~250ms）。
    pub fn with_cost(cost: u32) -> Self {
        Self { cost }
    }

    /// 按环境推荐代价因子。
    pub fn for_environment(environment: &str) -> Self {
        // 注意：这是**唯一**一处为测试降低安全强度的地方，
        // 并且有明确的日志提示，避免它被悄悄带到生产。
        if environment.eq_ignore_ascii_case("test") {
            tracing::debug!("测试环境使用低代价 bcrypt（cost=4）以加速测试");
            return Self::with_cost(4);
        }
        if environment.eq_ignore_ascii_case("development") {
            return Self::with_cost(8);
        }
        Self::new()
    }

    /// 当前代价因子。
    pub fn cost(&self) -> u32 {
        self.cost
    }

    /// 生成密码哈希（异步，内部走阻塞线程池）。
    ///
    /// # 错误
    /// 密码超过 bcrypt 的 72 字节上限时，bcrypt 会返回错误。
    /// 上游（`validator`）已经限制了长度，这里的检查是**纵深防御**。
    pub async fn hash_password(&self, plain: &str) -> Result<String, AppError> {
        // 提前校验长度：bcrypt 的 72 字节限制按**字节**而非字符计算，
        // 中文密码很容易超限，必须给出清晰提示。
        if plain.len() > 72 {
            return Err(AppError::field(
                "password",
                "密码过长（bcrypt 限制为 72 字节，中文约占 3 字节/字）",
            ));
        }

        let cost = self.cost;
        // 把明文密码的所有权移动进闭包，避免跨线程共享引用带来的生命周期麻烦。
        let plain = plain.to_string();

        let hashed = spawn_blocking(move || hash(plain, cost))
            .await
            // JoinError：阻塞任务 panic 或被取消
            .map_err(|e| AppError::Internal(anyhow::anyhow!("密码哈希任务执行失败：{e}")))?
            // BcryptError：算法层面的失败
            .map_err(|e| AppError::Internal(anyhow::anyhow!("密码哈希失败：{e}")))?;

        Ok(hashed)
    }

    /// 校验明文密码是否匹配哈希。
    ///
    /// **注意返回值的语义**：`Ok(false)` 表示密码错误（正常的业务失败），
    /// `Err` 才表示校验过程本身出错（比如数据库里存的哈希被截断）。
    /// 上层据此决定返回 401 还是 500。
    pub async fn verify_password(&self, plain: &str, hashed: &str) -> Result<bool, AppError> {
        if plain.len() > 72 {
            // 超长输入直接判为不匹配，不做无谓的计算（也防止被当作 DoS 手段）。
            return Ok(false);
        }

        let plain = plain.to_string();
        let hashed = hashed.to_string();

        let matched = spawn_blocking(move || verify(plain, &hashed))
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("密码校验任务执行失败：{e}")))?
            .map_err(|e| AppError::Internal(anyhow::anyhow!("密码哈希格式非法：{e}")))?;

        Ok(matched)
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 用低代价因子，让测试在毫秒级完成。
    fn fast_hasher() -> PasswordHasher {
        PasswordHasher::with_cost(4)
    }

    #[tokio::test]
    async fn hash_then_verify_succeeds() {
        let hasher = fast_hasher();
        let hashed = hasher.hash_password("secret123").await.unwrap();
        assert!(hasher.verify_password("secret123", &hashed).await.unwrap());
    }

    #[tokio::test]
    async fn wrong_password_fails_without_error() {
        let hasher = fast_hasher();
        let hashed = hasher.hash_password("secret123").await.unwrap();
        // 密码错误是"业务失败"，应当是 Ok(false) 而不是 Err
        assert!(!hasher.verify_password("secret124", &hashed).await.unwrap());
    }

    /// 同一密码两次哈希结果必须不同（证明加了随机盐）。
    #[tokio::test]
    async fn hashes_are_salted_and_differ() {
        let hasher = fast_hasher();
        let first = hasher.hash_password("secret123").await.unwrap();
        let second = hasher.hash_password("secret123").await.unwrap();
        assert_ne!(first, second, "随机盐应当让每次哈希结果不同");
        assert!(first.starts_with("$2"), "bcrypt 哈希应以 $2 开头：{first}");
    }

    /// 明文绝不能出现在哈希结果里。
    #[tokio::test]
    async fn hash_does_not_contain_plaintext() {
        let hasher = fast_hasher();
        let hashed = hasher.hash_password("MySecret123").await.unwrap();
        assert!(!hashed.contains("MySecret123"));
    }

    #[tokio::test]
    async fn oversized_password_is_rejected() {
        let hasher = fast_hasher();
        let long = "a".repeat(73);
        assert!(hasher.hash_password(&long).await.is_err());
        // 校验时超长输入直接判否，不报错
        let hashed = hasher.hash_password("secret123").await.unwrap();
        assert!(!hasher.verify_password(&long, &hashed).await.unwrap());
    }

    /// 数据库里的哈希损坏时，应当报 500 而不是误判为密码错误。
    #[tokio::test]
    async fn corrupted_hash_reports_internal_error() {
        let hasher = fast_hasher();
        let err = hasher
            .verify_password("secret123", "这不是一个合法的 bcrypt 哈希")
            .await
            .expect_err("损坏的哈希应当报错");
        assert_eq!(
            err.status_code(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    /// 环境到代价因子的映射必须正确（防止测试环境意外使用生产强度）。
    #[test]
    fn environment_selects_cost() {
        assert_eq!(PasswordHasher::for_environment("test").cost(), 4);
        assert_eq!(PasswordHasher::for_environment("development").cost(), 8);
        assert_eq!(
            PasswordHasher::for_environment("production").cost(),
            DEFAULT_COST
        );
    }
}
