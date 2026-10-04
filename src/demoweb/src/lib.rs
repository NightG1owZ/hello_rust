//! # demoweb —— 基于 Axum 的企业级简易 Web 应用
//!
//! 这是一个**用于学习 Rust Web 开发完整流程**的示例项目。
//! 它刻意保持了"够用但不过度"的规模：所有企业级项目该有的分层与机制都在，
//! 但每个模块都控制在几百行以内，方便通读。
//!
//! ## 分层架构
//!
//! ```text
//!                        ┌──────────────────────────────┐
//!   HTTP 请求 ──────────► │  中间件 middleware             │
//!                        │  request-id → trace → cors    │
//!                        │  → body-limit → timeout       │
//!                        │  → auth（仅受保护路由）        │
//!                        └──────────────┬───────────────┘
//!                                       ▼
//!                        ┌──────────────────────────────┐
//!                        │  控制器 controllers           │  只做 HTTP 语义转换
//!                        │  users / health / demo        │
//!                        └──────────────┬───────────────┘
//!                                       ▼
//!                        ┌──────────────────────────────┐
//!                        │  服务 services                │  业务规则唯一归属
//!                        │  users / auth(JWT) / password │
//!                        └──────────────┬───────────────┘
//!                                       ▼
//!                        ┌──────────────────────────────┐
//!                        │  仓储 repository（trait）     │  数据访问抽象
//!                        │  ├── SqliteUserRepository     │
//!                        │  └── InMemoryUserRepository   │
//!                        └──────────────┬───────────────┘
//!                                       ▼
//!                                 SQLite / 内存
//!
//!   横向支撑：config（三级配置） · error（统一错误） · logging（结构化日志）
//!             state（共享状态）  · models（领域模型与 DTO）
//! ```
//!
//! ## 快速开始
//! ```bash
//! cd src/demoweb
//! cargo run                 # 默认监听 127.0.0.1:3000，零外部依赖
//! cargo test                # 运行全部单元测试与集成测试
//! ```
//!
//! ## 从哪读起
//! 建议按这个顺序阅读源码，每一步都建立在前一步之上：
//! 1. [`config`] —— 配置从哪来、如何校验；
//! 2. [`error`] —— 领域错误如何变成 HTTP 响应；
//! 3. [`models`] —— 领域模型与 DTO 的区分；
//! 4. [`repository`] —— 数据访问抽象与两种实现；
//! 5. [`services`] —— 注册/登录的业务规则、密码哈希、JWT；
//! 6. [`controllers`] —— 控制器如何保持"薄"；
//! 7. [`middleware`] —— 中间件洋葱模型与顺序；
//! 8. [`routes`] —— 路由装配与"默认受保护"原则；
//! 9. [`state`] —— 共享状态与并发安全的计数。

// ============================================================================
// 模块声明
//
// 顺序刻意按照"从底层到上层"排列，便于对照上面的架构图阅读。
// ============================================================================

pub mod config;
pub mod error;
pub mod logging;
pub mod models;
pub mod state;

pub mod db;
pub mod repository;
pub mod services;

pub mod controllers;
pub mod middleware;
pub mod routes;

// ============================================================================
// 对外导出（re-export）
//
// 把最常用的类型提升到 crate 根，让使用者可以写 `use demoweb::AppConfig;`
// 而不是 `use demoweb::config::AppConfig;`。
// 注意：**只导出真正需要对外暴露的项**，避免把内部实现细节变成公共 API
// ——公共 API 一旦发布就难以修改（向后兼容约束）。
// ============================================================================

pub use config::AppConfig;
pub use error::{AppError, AppResult};
pub use routes::build_router;
pub use state::AppState;

use std::sync::Arc;

use crate::repository::UserRepository;
use crate::services::JwtService;

/// 从配置构建完整的应用（路由 + 依赖装配），但不启动监听。
///
/// # 为什么把"构建"与"运行"分开
/// - **可测试**：集成测试可以用它拿到 `Router`，绑定随机端口或被 `oneshot` 直接调用；
/// - **可复用**：将来要在同一进程里挂载多个应用（例如灰度双跑）也很方便；
/// - **`main` 更干净**：`main` 只负责"读配置、初始化日志、启动、停机"这四件事。
pub fn build_app(config: AppConfig) -> anyhow::Result<axum::Router> {
    // JWT 服务：密钥校验失败会在这里直接返回错误，服务不会"带病启动"。
    let jwt_service = Arc::new(JwtService::new(&config.jwt)?);

    // 仓储实现的选择：本项目用内存仓储作为演示默认值，
    // 因为它零依赖、启动即用；生产项目把这里换成 SqliteUserRepository 即可，
    // **其它代码一行都不用改**——这正是依赖倒置的价值。
    let user_repository: Arc<dyn UserRepository> =
        Arc::new(crate::repository::InMemoryUserRepository::new());

    let state = AppState::new(config, user_repository, jwt_service);
    Ok(build_router(state))
}

/// 用 SQLite 仓储构建应用（需要数据库连接池）。
///
/// 与 [`build_app`] 的唯一区别是仓储实现，展示了"替换数据访问层"有多简单。
pub fn build_app_with_repository(
    config: AppConfig,
    user_repository: Arc<dyn UserRepository>,
) -> anyhow::Result<axum::Router> {
    let jwt_service = Arc::new(JwtService::new(&config.jwt)?);
    let state = AppState::new(config, user_repository, jwt_service);
    Ok(build_router(state))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 默认配置必须能成功构建应用（保证"克隆下来就能跑"）。
    #[test]
    fn build_app_with_default_config_succeeds() {
        let config = AppConfig {
            environment: "test".to_string(),
            ..AppConfig::default()
        };
        assert!(build_app(config).is_ok());
    }

    /// 密钥过短时构建必须失败，而不是启动后才发现。
    #[test]
    fn build_app_rejects_weak_secret() {
        let config = AppConfig {
            environment: "test".to_string(),
            jwt: crate::config::JwtConfig {
                secret: "short".to_string(),
                ..Default::default()
            },
            ..AppConfig::default()
        };
        assert!(build_app(config).is_err(), "弱密钥应当在构建阶段被拒绝");
    }
}
