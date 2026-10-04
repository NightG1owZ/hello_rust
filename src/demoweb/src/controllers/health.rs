//! 健康检查与可观测性接口。
//!
//! # 为什么需要两种健康检查（这是容器编排的硬要求）
//! - **存活探针（liveness）**：进程还活着吗？返回失败时编排器会**重启**容器。
//!   因此它必须极轻量，且**绝不能**依赖外部系统——否则数据库抖动会导致服务被反复重启，
//!   形成"雪崩"：明明只是下游慢，却把自己的实例全杀了一遍。
//! - **就绪探针（readiness）**：现在能接流量吗？返回失败时编排器只把实例**摘出负载均衡**，
//!   不重启。因此它可以（也应该）检查数据库等关键依赖。
//!
//! 把这两者混为一谈，是生产事故的常见原因。

use axum::Json;
use axum::extract::State;

use crate::error::AppResult;
use crate::models::{ApiResponse, HealthResponse};
use crate::state::AppState;

/// `GET /health` —— 存活探针：只证明"进程能响应 HTTP"。
///
/// 刻意不做任何 IO：它必须永远是毫秒级返回。
pub async fn liveness() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::ok("ok"))
}

/// `GET /api/health` —— 就绪探针：检查关键依赖（数据库）是否可用。
///
/// 这里直接走仓储层做一次真实查询，而不是只看连接池是否存在——
/// "池子有连接"不等于"数据库能响应"。
pub async fn readiness(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<HealthResponse>>> {
    // 用一次"查不存在的用户"来验证数据库真的可用：
    // 它走的是真实 SQL 路径，但不会因为数据为空而误判。
    state
        .user_repository
        .find_by_email("__health_check_probe__@example.invalid")
        .await?;

    Ok(Json(ApiResponse::ok(HealthResponse {
        status: "ready",
        version: env!("CARGO_PKG_VERSION"),
        environment: state.config.environment.clone(),
        uptime_secs: state.uptime_secs(),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::repository::{InMemoryUserRepository, UserRepository};
    use crate::services::JwtService;
    use std::sync::Arc;

    fn test_state() -> AppState {
        let config = AppConfig {
            environment: "test".to_string(),
            ..AppConfig::default()
        };
        let jwt = Arc::new(JwtService::new(&config.jwt).expect("构造 JWT 服务失败"));
        AppState::new(
            config,
            InMemoryUserRepository::shared() as Arc<dyn UserRepository>,
            jwt,
        )
    }

    /// 存活探针必须永远成功（不依赖任何外部系统）。
    #[tokio::test]
    async fn liveness_always_ok() {
        let Json(body) = liveness().await;
        assert_eq!(body.data, "ok");
    }

    /// 就绪探针在依赖可用时返回 ready，并带上版本与运行时长。
    #[tokio::test]
    async fn readiness_reports_ready() {
        let Json(body) = readiness(State(test_state()))
            .await
            .expect("就绪检查应通过");
        assert_eq!(body.data.status, "ready");
        assert_eq!(body.data.environment, "test");
        assert!(!body.data.version.is_empty());
    }
}
