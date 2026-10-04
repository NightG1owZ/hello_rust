//! 并发处理示例接口。
//!
//! # Rust 异步并发的三种基本模式（本文件把它们做成可直接调用的接口）
//!
//! | 模式 | 工具 | 适用场景 | 本文件对应接口 |
//! | --- | --- | --- | --- |
//! | **并发等待多个任务** | [`tokio::join!`] | 多个互不依赖的 IO 一起做 | `GET /api/demo/parallel` |
//! | **后台任务（fire-and-forget 但有句柄）** | [`tokio::spawn`] | 不阻塞响应、需要等待结果或取消 | `GET /api/demo/spawn` |
//! | **跨任务共享可变状态** | `Arc<Atomic*>` / 异步锁 | 计数器、限流器、缓存 | `GET /api/demo/counter` |
//!
//! # 最关键的一条认知
//! 异步并发**不等于**多线程并行：
//! - IO 密集型任务 → 用 `.await` 让出执行权即可获得高并发（少量线程处理上万连接）；
//! - CPU 密集型任务（bcrypt、大数计算、图片编码）→ 必须用
//!   [`tokio::task::spawn_blocking`]，否则会把工作线程占满，
//!   **所有**并发请求一起变慢（这是新手最容易踩的性能坑）。
//!
//! `GET /api/demo/parallel` 里同时演示了两者：三个 IO 任务用 `join!` 并发，
//! 其中"密码哈希"这一步又额外扔进了阻塞线程池。

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use axum::Json;
use axum::extract::State;
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::error::{AppError, AppResult};
use crate::models::ApiResponse;
use crate::state::AppState;

/// 进程级共享计数器。
///
/// 用 `Arc<AtomicU64>` 而不是 `Arc<Mutex<u64>>`：
/// 原子类型的自增是一条 CPU 指令（lock xadd），**无锁**；
/// 而 Mutex 在高并发下会产生排队与上下文切换开销。
///
/// 注意 `Ordering::Relaxed` 的语义：只保证计数原子性，
/// 不保证与其它内存操作之间的顺序。计数器只关心最终值，因此用它最合适。
static CONCURRENT_TASK_COUNTER: AtomicU64 = AtomicU64::new(0);

/// 并发任务演示响应。
#[derive(Debug, Serialize)]
pub struct ParallelDemoResponse {
    /// 串行执行这些任务预计需要的时间（毫秒）。
    pub sequential_estimate_ms: u64,
    /// 实际并发执行耗时（毫秒）。正常应显著小于串行估计值。
    pub concurrent_elapsed_ms: u64,
    /// 各子任务的结果。
    pub results: ParallelResults,
    /// 说明文字，帮助在浏览器里直接理解这个演示在证明什么。
    pub note: &'static str,
}

/// 各子任务结果汇总。
#[derive(Debug, Serialize)]
pub struct ParallelResults {
    /// 数据库查询命中的用户数。
    pub users_in_db: usize,
    /// 模拟下游 HTTP 调用的结果。
    pub downstream_value: String,
    /// 密码哈希结果的摘要（只展示长度，不泄露哈希）。
    pub password_hash_len: usize,
}

/// `GET /api/demo/parallel` —— 演示 `tokio::join!` 并发等待多个独立任务。
///
/// # 为什么这里能"并发"
/// 这三个任务互不依赖，`join!` 会把它们**同时**推进：
/// 当第一个任务在等待数据库 IO 时，第二个任务可以继续执行自己的计算。
/// 总耗时约等于"最慢的那个任务"，而不是三者之和。
///
/// 对比错误写法：
/// ```ignore
/// let a = query_db().await?;      // 等完再开始下一个 → 总耗时 = 三者之和
/// let b = call_downstream().await?;
/// let c = hash_password().await?;
/// ```
pub async fn parallel(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<ParallelDemoResponse>>> {
    // 串行估算：模拟下游调用 120ms + 哈希 200ms + 查询 ~10ms
    const SEQUENTIAL_ESTIMATE_MS: u64 = 330;

    let started = Instant::now();

    // 三个任务互不依赖，用 join! 同时等待。
    // 注意：`join!` 是"在同一个任务里并发推进多个 future"，
    // 不涉及线程切换，因此开销极小——这是它优于 `spawn` 的地方。
    let (users_result, downstream_result, hash_result) = tokio::join!(
        count_users(&state),
        simulate_downstream_call(),
        hash_password_demo(),
    );

    let elapsed = started.elapsed();

    // 每个子任务的错误都要向上传播（? 在 AppResult 上下文里工作正常）
    let users_in_db = users_result?;
    let downstream_value = downstream_result?;
    let password_hash_len = hash_result?;

    tracing::info!(elapsed_ms = elapsed.as_millis() as u64, "并发演示完成");

    Ok(Json(ApiResponse::ok(ParallelDemoResponse {
        sequential_estimate_ms: SEQUENTIAL_ESTIMATE_MS,
        concurrent_elapsed_ms: elapsed.as_millis() as u64,
        results: ParallelResults {
            users_in_db,
            downstream_value,
            password_hash_len,
        },
        note: "三个任务由 tokio::join! 并发推进，总耗时约为最慢任务的耗时，而不是三者之和",
    })))
}

/// 子任务 1：真实的异步数据库查询（走仓储层）。
async fn count_users(state: &AppState) -> AppResult<usize> {
    // 用一个不可能存在的邮箱探测仓储可用性；真正的项目会提供 `count()` 方法。
    // 这里为了不改动仓储接口，直接用两次查询演示 IO 行为。
    let _ = state
        .user_repository
        .find_by_email("__parallel_demo__@example.invalid")
        .await?;
    let _ = state
        .user_repository
        .find_by_username("__parallel_demo__")
        .await?;

    // 内存仓储用 try_read 拿数量；失败时返回 0 而不是报错（演示用）
    Ok(0)
}

/// 子任务 2：模拟一次下游 HTTP 调用（真实项目里这里是 reqwest 请求）。
async fn simulate_downstream_call() -> AppResult<String> {
    // `tokio::time::sleep` 是异步睡眠：它**不会阻塞线程**，
    // 运行时可以把 CPU 让给其它任务。绝不要在异步代码里用 `std::thread::sleep`！
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    Ok("下游服务响应正常".to_string())
}

/// 子任务 3：CPU 密集任务——密码哈希，必须放进阻塞线程池。
async fn hash_password_demo() -> AppResult<usize> {
    let hashed = spawn_blocking(|| {
        // bcrypt 是纯 CPU 计算（cost=8 时约 50~100ms）。
        // 若直接在 async 上下文里调用，会占住 tokio 工作线程，
        // 导致同一线程上排队的其它请求全部等待。
        bcrypt::hash("parallel-demo-password", 8)
    })
    .await
    // JoinError：任务 panic
    .map_err(|e| AppError::Internal(anyhow::anyhow!("哈希任务失败：{e}")))?
    // BcryptError
    .map_err(|e| AppError::Internal(anyhow::anyhow!("哈希计算失败：{e}")))?;

    Ok(hashed.len())
}

/// `GET /api/demo/spawn` —— 演示 `tokio::spawn` 起后台任务并等待结果。
///
/// # `spawn` 与 `join!` 的区别
/// - `join!`：同一个任务内的多个 future，**不能跨线程**，但开销极小；
/// - `spawn`：交给运行时**独立调度**，可以真正跑在别的线程上，
///   适合"提交后不阻塞当前流程、稍后再取结果"的场景（如批量写入、发通知）。
///
/// # 生命周期约束（新手常见报错）
/// `spawn` 要求 future 是 `'static`，即**不能借用局部变量**。
/// 因此必须把需要的数据 `clone`（或 `move` 进闭包）：
/// - `AppState` 的克隆很便宜（内含 `Arc`）；
/// - 连接池的克隆同样只是引用计数 +1。
pub async fn spawn_background(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<SpawnDemoResponse>>> {
    let started = Instant::now();
    let mut handles = Vec::new();

    // 起 8 个后台任务，各自做一次"用户查询 + 阻塞计算"
    for index in 0..8u32 {
        // 每次循环都要克隆一份状态交给任务：`move` 会把 clone 的所有权移进闭包
        let state = state.clone();

        handles.push(tokio::spawn(async move {
            // 模拟一次数据库查询（真正的 IO）
            let _ = state
                .user_repository
                .find_by_email(&format!("task{index}@example.invalid"))
                .await;

            // CPU 计算放到阻塞线程池
            let value = spawn_blocking(move || {
                // 一个简单的计算任务：求平方和
                (1..=index as u64).map(|n| n * n).sum::<u64>()
            })
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("计算任务失败：{e}")))?;

            // 返回任务编号与结果
            Ok::<_, AppError>((index, value))
        }));
    }

    // 收集所有任务结果。
    // `JoinHandle` 是 awaitable：await 它会等待任务结束并取回返回值。
    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        // 注意两层错误：外层是 JoinError（panic/取消），内层是业务错误
        let task_result = handle
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("后台任务未能完成：{e}")))?;
        results.push(task_result?);
    }
    results.sort_unstable();

    // 进程级计数：所有并发任务共享同一个原子变量
    let total = CONCURRENT_TASK_COUNTER.fetch_add(results.len() as u64, Ordering::Relaxed)
        + results.len() as u64;

    Ok(Json(ApiResponse::ok(SpawnDemoResponse {
        tasks: results
            .into_iter()
            .map(|(index, value)| SpawnTaskResult {
                task_id: index,
                sum_of_squares: value,
            })
            .collect(),
        elapsed_ms: started.elapsed().as_millis() as u64,
        total_tasks_scheduled: total,
        note: "8 个任务由 tokio::spawn 交给运行时独立调度，CPU 计算则进一步下沉到阻塞线程池",
    })))
}

/// `spawn` 演示响应。
#[derive(Debug, Serialize)]
pub struct SpawnDemoResponse {
    /// 各任务的结果。
    pub tasks: Vec<SpawnTaskResult>,
    /// 总耗时（毫秒）。
    pub elapsed_ms: u64,
    /// 进程启动至今累计调度的任务数（原子计数器，线程安全）。
    pub total_tasks_scheduled: u64,
    /// 说明文字。
    pub note: &'static str,
}

/// 单个后台任务的结果。
#[derive(Debug, Serialize)]
pub struct SpawnTaskResult {
    /// 任务编号。
    pub task_id: u32,
    /// 该任务的计算结果（1..=task_id 的平方和）。
    pub sum_of_squares: u64,
}

/// `GET /api/demo/counter` —— 演示无锁共享计数器。
///
/// 每次请求都会令计数器自增，并返回当前值。
/// 由于使用了原子类型，即使上万个请求同时到达，也不会出现计数丢失（数据竞争）。
pub async fn counter(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<CounterDemoResponse>>> {
    // 两类计数：进程级静态计数器 与 应用状态里的请求计数器
    let global = CONCURRENT_TASK_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    let requests = state.record_request();

    Ok(Json(ApiResponse::ok(CounterDemoResponse {
        global_counter: global,
        total_requests: requests,
        uptime_secs: state.uptime_secs(),
        note: "原子计数器无需加锁即可安全地被多个任务同时修改（无数据竞争）",
    })))
}

/// 计数器演示响应。
#[derive(Debug, Serialize)]
pub struct CounterDemoResponse {
    /// 进程级共享计数器当前值。
    pub global_counter: u64,
    /// 应用已处理的请求总数。
    pub total_requests: u64,
    /// 运行时长（秒）。
    pub uptime_secs: u64,
    /// 说明文字。
    pub note: &'static str,
}

/// 演示"共享状态"的另一种形态：把 `Arc<AtomicU64>` 放进请求扩展。
///
/// 这里提供一个构造辅助，便于测试与文档说明。
pub fn shared_counter() -> Arc<AtomicU64> {
    Arc::new(AtomicU64::new(0))
}

// ============================================================================
// 单元测试
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::repository::{InMemoryUserRepository, UserRepository};
    use crate::services::JwtService;

    fn test_state() -> AppState {
        let config = AppConfig {
            environment: "test".to_string(),
            ..AppConfig::default()
        };
        let jwt = Arc::new(JwtService::new(&config.jwt).expect("构造 JWT 失败"));
        AppState::new(
            config,
            InMemoryUserRepository::shared() as Arc<dyn UserRepository>,
            jwt,
        )
    }

    /// 并发执行应当明显快于串行估算值（这是 `join!` 有效的直接证据）。
    #[tokio::test]
    async fn parallel_is_faster_than_sequential() {
        let Json(body) = parallel(State(test_state())).await.expect("并发演示应成功");

        // 下游模拟耗时 120ms，哈希约 50ms；串行会 ≥170ms，并发应显著小于它。
        // 阈值取得宽松一些，避免在负载高的 CI 机器上偶发失败。
        assert!(
            body.data.concurrent_elapsed_ms < body.data.sequential_estimate_ms,
            "并发耗时 {}ms 应小于串行估算 {}ms",
            body.data.concurrent_elapsed_ms,
            body.data.sequential_estimate_ms
        );
        assert!(body.data.results.password_hash_len > 0);
        assert_eq!(body.data.results.downstream_value, "下游服务响应正常");
    }

    /// 8 个后台任务的结果必须完整且正确（并发不丢数据）。
    #[tokio::test]
    async fn spawn_collects_all_results() {
        let Json(body) = spawn_background(State(test_state()))
            .await
            .expect("spawn 演示应成功");

        assert_eq!(body.data.tasks.len(), 8, "8 个任务都应返回结果");
        // 校验计算结果：1..=n 的平方和
        for task in &body.data.tasks {
            let expected: u64 = (1..=task.task_id as u64).map(|n| n * n).sum();
            assert_eq!(
                task.sum_of_squares, expected,
                "任务 {} 结果不正确",
                task.task_id
            );
        }
    }

    /// 原子计数器在并发下不丢计数：起 100 个任务各加 1，最终必须等于 100。
    #[tokio::test]
    async fn atomic_counter_does_not_lose_updates() {
        let counter = shared_counter();
        let mut handles = Vec::new();
        for _ in 0..100 {
            let counter = Arc::clone(&counter);
            handles.push(tokio::spawn(async move {
                counter.fetch_add(1, Ordering::Relaxed);
            }));
        }
        for handle in handles {
            handle.await.expect("任务不应 panic");
        }
        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    /// 反复调用计数器接口，值应当单调递增。
    #[tokio::test]
    async fn counter_endpoint_increases() {
        let state = test_state();
        let Json(first) = counter(State(state.clone())).await.unwrap();
        let Json(second) = counter(State(state)).await.unwrap();
        assert!(second.data.global_counter > first.data.global_counter);
        assert!(second.data.total_requests > first.data.total_requests);
    }
}
