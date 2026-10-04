# demoweb —— 基于 Axum 的企业级简易 Web 应用

一个**从零构建、可直接运行**的 Rust Web 实践项目，用于把[《Rust 系统学习教程》](../tutorial/README.md)里的语言知识（所有权、trait、泛型、错误处理、并发……）落到一个真实的后端服务里。

> 前置建议：先完成[教程目录的 18 课](../../README.md)（至少到 05 所有权、07 枚举、10 错误处理、12 Trait，以及 14~17 的闭包/迭代器/智能指针/并发），再读本项目的源码会顺畅得多。项目整体结构见[根 README](../../README.md)。

它刻意保持"够用但不过度"的规模：**20 个 Rust 源文件、约 4260 行代码**（含测试），企业级项目该有的分层与机制全都在，但每个模块都短到可以一次读透。

| 维度 | 内容 |
| --- | --- |
| Web 框架 | **Axum 0.8**（Tokio 官方生态） |
| 异步运行时 | **Tokio 1.x**（多线程调度器） |
| 数据库 | **SQLx 0.8 + SQLite**（异步、带迁移；默认内存库，**零外部依赖**） |
| 认证 | **bcrypt** 密码哈希 + **JWT**（jsonwebtoken） |
| 可观测性 | **tracing** + **tracing-subscriber** + **tracing-appender**（请求 ID、访问日志、文件滚动） |
| 配置 | **config** crate 三级覆盖（默认值 → TOML → 环境变量）+ **dotenvy** |
| 校验 | **validator** 声明式字段校验 |
| 测试 | **102 个**（85 单元 + 2 停机流程 + 15 集成，含真实 HTTP 端到端） |
| 质量 | `cargo fmt` 通过、`cargo clippy` **零警告**、编译零警告 |

---

## 一、30 秒跑起来

```bash
cd src/demoweb

cargo run            # 首次会编译依赖，之后启动极快
```

看到下面这行就说明服务已就绪（默认监听 `127.0.0.1:3000`）：

```
INFO demoweb: HTTP 服务已就绪，按 Ctrl+C 退出 local_addr=127.0.0.1:3000
```

然后换个终端试一下：

```bash
curl http://127.0.0.1:3000/health
# {"success":true,"data":"ok"}
```

> **零依赖**：默认使用 SQLite **内存库**，不需要安装任何数据库；进程退出数据即清空，适合反复练习。
> 想持久化，把 `config/default.toml` 里的 `url` 改成 `"sqlite://data/app.db?mode=rwc"` 并把 `max_connections` 调到 5 即可。

---

## 二、目录结构与分层职责

```
src/demoweb/
├── Cargo.toml                      # 依赖清单（每一项都注释了它承担的职责）
├── config/default.toml             # 默认配置（可被环境变量覆盖）
├── .env.example                    # 环境变量样例（复制为 .env 使用）
├── migrations/                     # SQL 迁移脚本（编译期嵌入二进制）
│   └── 20260101000001_create_users.sql
├── src/
│   ├── main.rs                     # 入口：读配置 → 初始化日志 → 装配 → 启动 → 优雅停机
│   ├── lib.rs                      # 库入口：模块声明 + 装配函数（便于测试复用）
│   │
│   ├── config.rs        ← 配置   # 三级覆盖 + 启动期校验
│   ├── error.rs         ← 错误   # 领域错误 → HTTP 响应的统一映射
│   ├── logging.rs       ← 日志   # tracing 初始化、JSON/文件输出
│   ├── state.rs         ← 状态   # Arc<AppState>：连接池、仓储、配置、计数器
│   ├── models.rs        ← 模型   # 领域实体 + 入参/出参 DTO + 校验规则
│   │
│   ├── db.rs            ← 数据   # 连接池构建、迁移执行、健康探测
│   ├── repository.rs    ← 仓储   # UserRepository trait + SQLite/内存两种实现
│   ├── services/        ← 服务   # 业务规则（唯一归属地）
│   │   ├── users.rs     #   注册 / 登录 / 查询
│   │   ├── auth.rs      #   JWT 签发与校验 + AuthenticatedUser 提取器
│   │   └── password.rs  #   bcrypt 哈希（spawn_blocking 不阻塞运行时）
│   │
│   ├── controllers/     ← 控制器 # 只做 HTTP 语义转换（薄）
│   │   ├── users.rs     #   注册 / 登录 / 当前用户 / 管理员接口
│   │   ├── health.rs    #   存活探针 / 就绪探针
│   │   └── demo.rs      #   并发演示（join! / spawn / 原子计数）
│   ├── middleware.rs    ← 中间件 # 请求 ID、访问日志、CORS、超时、认证
│   └── routes.rs        ← 路由   # 路由分组、layer 顺序、状态注入
└── tests/
    └── api_integration.rs          # 集成测试：真实 TCP 端口 + 真实 HTTP 客户端
```

### 请求全链路

```
                        ┌──────────────────────────────────────┐
   HTTP 请求 ──────────►│ middleware                           │
                        │  ① CORS                 （最外层）    │
                        │  ② 请求 ID 生成/透传 → 响应头         │
                        │  ③ TraceLayer 访问日志 span          │
                        │  ④ 自定义耗时统计（慢请求告警）        │
                        │  ⑤ 超时保护                          │
                        │  ⑥ 请求体大小限制                     │
                        │  ⑦ JWT 认证（仅受保护路由）           │
                        └───────────────┬──────────────────────┘
                                        ▼
                        ┌──────────────────────────────────────┐
                        │ controllers   只做 HTTP 语义转换      │
                        └───────────────┬──────────────────────┘
                                        ▼
                        ┌──────────────────────────────────────┐
                        │ services      业务规则唯一归属        │
                        │  注册查重 · 密码哈希 · 令牌签发       │
                        └───────────────┬──────────────────────┘
                                        ▼
                        ┌──────────────────────────────────────┐
                        │ repository    UserRepository (trait)  │
                        │   ├── SqliteUserRepository            │
                        │   └── InMemoryUserRepository          │
                        └───────────────┬──────────────────────┘
                                        ▼
                                  SQLite / 内存

   横向支撑：config · error · logging · state · models
```

**判断一段代码该放哪一层**，用这个标准：

| 问题 | 归属 |
| --- | --- |
| 涉及状态码、请求头、JSON 形状？ | `controllers` |
| 涉及业务规则（密码要哈希、邮箱不能重复、令牌怎么签发）？ | `services` |
| 涉及 SQL、索引、连接？ | `repository` |

这样分层最实际的好处：将来加一个 gRPC 接口或 CLI 工具复用同一套服务层时，业务规则不用重写一遍——也就不会出现"Web 端校验了、CLI 端忘了校验"的安全缺口。

---

## 三、环境搭建

### 1. 安装 Rust（如已安装可跳过）

```bash
# Windows：下载并运行 https://www.rust-lang.org/tools/install 的 rustup-init.exe
# macOS / Linux：
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

本项目在 **rustc 1.99.0 stable / edition 2024** 上开发与验证，`Cargo.toml` 里声明了 `rust-version = "1.85"`。

### 2. 拉取依赖

```bash
cargo fetch          # 只下载依赖，不编译（首次约 1~3 分钟）
cargo build          # 完整编译
```

国内网络建议配置镜像（本项目开发机即使用 rsproxy）：

```toml
# ~/.cargo/config.toml（Windows 为 %CARGO_HOME%\config.toml）
[source.crates-io]
replace-with = "rsproxy-sparse"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
```

### 3. 本机特有的两个构建注意点（重要）

> 这两条是**本项目开发过程中真实遇到并解决的**问题，如果你使用较新的工具链可能不会碰到，但遇到时可以照此排查。

**① MinGW gcc 编译 SQLite 时崩溃（本项目已内置规避）**

现象：`cargo build` 时报

```
cargo:warning=sqlite3/sqlite3.c: In function 'fts5ColumnSizeCb':
cargo:warning=sqlite3/sqlite3.c:...: internal compiler error: in based_loc_descr, at dwarf2out.c:14264
error occurred in cc-rs: command did not execute successfully
```

原因：`sqlx` 的 `sqlite` 特性默认以 bundled 方式编译约 25 万行的 `sqlite3.c` 单文件 C 源码，而 **MinGW gcc 8.1.0（2018 年版本）在生成 DWARF 调试信息阶段会崩溃**（编译器自身缺陷）。

解决：`Cargo.toml` 中只针对该依赖关闭调试信息：

```toml
# 仅关闭 libsqlite3-sys 的调试信息，业务代码仍保留完整调试能力
[profile.dev.package.libsqlite3-sys]
debug = false
opt-level = 1
```

若你使用 MSVC 工具链或 gcc 12+，可以直接删除这段配置。

**② curl / PowerShell 在沙箱环境下 TLS 失败**

现象：`schannel: AcquireCredentialsHandle failed: SEC_E_NO_CREDENTIALS`，导致 `cargo fetch` 无法联网。

原因：受限环境中 Windows 凭据库不可用（与项目代码无关）。解决：在正常的用户会话下执行 `cargo fetch`，或改用其它网络出口。

---

## 四、配置管理

### 三级覆盖策略（优先级由低到高）

| 层级 | 来源 | 用途 |
| --- | --- | --- |
| 1 | 代码内置默认值（`config.rs` 里的 `Default` 实现） | 保证**零配置也能启动** |
| 2 | `config/default.toml`（可用 `APP_CONFIG_FILE` 指定别的文件） | 团队共享的非敏感配置 |
| 3 | **环境变量** `APP_XXX__YYY` | 容器 / CI 注入，**最高优先级** |

环境变量命名规则：前缀 `APP_` + 配置路径，层级用**双下划线**分隔：

```bash
APP_SERVER__BIND_ADDRESS=0.0.0.0:8080
APP_SERVER__REQUEST_TIMEOUT_SECS=30
APP_DATABASE__URL="sqlite://data/app.db?mode=rwc"
APP_JWT__SECRET=$(openssl rand -base64 48)
APP_LOG__LEVEL=info,demoweb=debug,tower_http=debug
```

### 启动期校验：让服务"拒绝带病启动"

配置不只是"读进来"，还要**校验**。`AppConfig::validate()` 会在启动时拦住这些情况：

- `jwt.secret` 少于 16 字符 → 拒绝启动（弱密钥可被暴力破解）；
- `environment=production` 但仍使用内置示例密钥 → 拒绝启动并提示 `APP_JWT__SECRET`；
- SQLite **内存库**却把 `max_connections` 配成 > 1 → 拒绝启动（原因见下文"踩坑记录"）。

失败即退出，而不是等第一个请求进来才发现问题。

---

## 五、API 接口

### 接口总览

| 方法 | 路径 | 认证 | 说明 |
| --- | --- | --- | --- |
| GET | `/health` | 否 | 存活探针（极轻量，供 K8s livenessProbe） |
| GET | `/api/health` | 否 | 就绪探针（会真实查询数据库） |
| POST | `/api/users/register` | 否 | 用户注册 |
| POST | `/api/users/login` | 否 | 用户登录，返回 JWT |
| GET | `/api/users/me` | ✅ Bearer | 查询当前登录用户 |
| GET | `/api/users/admin/ping` | ✅ Bearer + 管理员 | 演示基于角色的访问控制（RBAC） |
| GET | `/api/demo/parallel` | ✅ Bearer | 并发演示：`tokio::join!` |
| GET | `/api/demo/spawn` | ✅ Bearer | 并发演示：`tokio::spawn` |
| GET | `/api/demo/counter` | ✅ Bearer | 并发演示：无锁原子计数 |

> **路由设计原则：默认受保护，例外才显式开放。**
> 需要认证的路由被单独组成一个 `Router` 再统一挂中间件，避免"新加路由忘了加认证"这类事故。

### 响应约定

成功（HTTP 2xx）：

```json
{ "success": true, "data": { } }
```

失败（HTTP 4xx/5xx）：

```json
{
  "success": false,
  "code": "USER_EMAIL_EXISTS",
  "message": "该邮箱已被注册",
  "errors": [ { "field": "email", "message": "邮箱格式不正确" } ]
}
```

- `code` 是**机器可读**的业务错误码，前端据此做分支，不要去匹配中文文案；
- `errors` 仅在字段校验失败时出现，可逐字段高亮；
- 每个响应都带 `x-request-id` 响应头，用于串联日志排查问题。

### 完整调用示例（以下输出均为**真实运行结果**）

**① 注册成功 → 201**

```bash
curl -X POST http://127.0.0.1:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{"username":"carol_01","email":"carol@example.com","password":"secret123"}'
```

```json
HTTP 201
{"success":true,"data":{"id":"e6109f4e-c6c0-498f-816b-b9b3763c94d5","username":"carol_01","email":"carol@example.com","role":"user","created_at":"2026-10-03T09:25:21.922983300Z"}}
```

注意响应里**没有 `password_hash`**——这不是靠"记得别返回"，而是出参 DTO 里根本没有这个字段，编译期就杜绝了泄露。

**② 重复邮箱 → 409 Conflict**

```json
HTTP 409
{"success":false,"code":"USER_EMAIL_EXISTS","message":"该邮箱已被注册"}
```

**③ 字段校验失败 → 400，逐字段错误**

```bash
curl -X POST http://127.0.0.1:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{"username":"a","email":"not-an-email","password":"123"}'
```

```json
HTTP 400
{"success":false,"code":"VALIDATION_FAILED","message":"请求参数校验失败","errors":[
  {"field":"password","message":"长度必须在 8~72 个字符之间"},
  {"field":"password","message":"不符合规则：password_strength"},
  {"field":"username","message":"长度必须在 3~32 个字符之间"},
  {"field":"email","message":"邮箱格式不正确"}]}
```

**④ 登录成功 → 200，返回 JWT**

```bash
curl -X POST http://127.0.0.1:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{"account":"carol@example.com","password":"secret123"}'
```

```json
HTTP 200
{"success":true,"data":{
  "user":{"id":"e6109f4e-...","username":"carol_01","email":"carol@example.com","role":"user","created_at":"..."},
  "access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJlNjEwOWY0ZS0u...",
  "token_type":"Bearer","expires_in":86400}}
```

> `account` 既可以是邮箱也可以是用户名；大小写不敏感。

**⑤ 缺少令牌访问受保护接口 → 401**

```json
HTTP 401
{"success":false,"code":"UNAUTHORIZED","message":"未认证：缺少 Authorization 请求头"}
```

**⑥ 携带令牌访问 `/me` → 200**

```bash
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
curl http://127.0.0.1:3000/api/users/me -H "Authorization: Bearer $TOKEN"
```

```json
HTTP 200
{"success":true,"data":{"id":"e6109f4e-...","username":"carol_01","email":"carol@example.com","role":"user","created_at":"..."}}
```

**⑦ 普通用户访问管理员接口 → 403 Forbidden**

```json
HTTP 403
{"success":false,"code":"FORBIDDEN","message":"禁止访问：该接口仅管理员可访问"}
```

> 401 = "不知道你是谁"（应重新登录）；403 = "知道你是谁，但你没权限"（重新登录也没用）。这两个状态码不能混用。

**⑧ 并发演示 `tokio::join!` → 并发确实更快**

```json
HTTP 200
{"success":true,"data":{
  "sequential_estimate_ms":330,
  "concurrent_elapsed_ms":125,
  "results":{"users_in_db":0,"downstream_value":"下游服务响应正常","password_hash_len":60},
  "note":"三个任务由 tokio::join! 并发推进，总耗时约为最慢任务的耗时，而不是三者之和"}}
```

三个任务串行需要约 330ms，并发只用了 **125ms** —— 这就是异步并发的价值。

**⑨ 并发演示 `tokio::spawn` → 8 个后台任务**

```json
HTTP 200
{"success":true,"data":{
  "tasks":[{"task_id":0,"sum_of_squares":0},{"task_id":1,"sum_of_squares":1},
           {"task_id":2,"sum_of_squares":5},{"task_id":3,"sum_of_squares":14},
           {"task_id":4,"sum_of_squares":30},{"task_id":5,"sum_of_squares":55},
           {"task_id":6,"sum_of_squares":91},{"task_id":7,"sum_of_squares":140}],
  "elapsed_ms":1,"total_tasks_scheduled":8,
  "note":"8 个任务由 tokio::spawn 交给运行时独立调度，CPU 计算则进一步下沉到阻塞线程池"}}
```

---

## 六、核心技术点导读

### 1. 异步编程（async/await）

```rust
// services/users.rs —— 全异步链路：控制器 → 服务 → 仓储 → 数据库
pub async fn register(&self, request: RegisterRequest) -> Result<UserResponse, AppError> {
    if self.repository.find_by_email(&email).await?.is_some() {   // ← .await 让出执行权
        return Err(AppError::conflict("USER_EMAIL_EXISTS", "该邮箱已被注册"));
    }
    let password_hash = self.password_hasher.hash_password(&request.password).await?;
    let user = self.repository.create(&new_user).await?;          // ← 非阻塞的数据库 IO
    Ok(UserResponse::from(user))
}
```

关键认知：`.await` **不阻塞线程**，它把控制权交还给运行时去执行别的任务。少量线程即可支撑上万并发连接。

### 2. CPU 密集任务必须下沉到阻塞线程池

```rust
// services/password.rs
let hashed = tokio::task::spawn_blocking(move || hash(plain, cost))
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("密码哈希任务执行失败：{e}")))?;
```

bcrypt 在 cost=12 时约需 200~300ms 的纯 CPU 计算。**如果直接在 async 函数里同步调用，会占住 tokio 的工作线程**，该线程上排队的其它几百个请求全部停摆——这是 Rust 异步编程最经典的性能坑。

### 3. 并发三种模式（`controllers/demo.rs`）

| 模式 | 工具 | 适用场景 |
| --- | --- | --- |
| 并发等待多个任务 | `tokio::join!` | 多个互不依赖的 IO 一起做，开销极小（不跨线程） |
| 后台任务 | `tokio::spawn` | 交给运行时独立调度，稍后取结果；要求 `'static` |
| 跨任务共享状态 | `Arc<AtomicU64>` | 计数器、限流器；**无锁**，比 `Mutex` 更适合简单计数 |

配套的"不该做什么"：

- ❌ 异步代码里用 `std::thread::sleep`（会阻塞运行时）→ ✅ 用 `tokio::time::sleep`；
- ❌ 用 `std::sync::Mutex` 保护跨 `.await` 的状态（可能死锁 + 阻塞线程）→ ✅ 用 `tokio::sync::Mutex` / `RwLock`（本项目内存仓储即用 `tokio::sync::RwLock`）。

### 4. 统一错误处理

```rust
// 处理器里只需一个 `?`，错误自动变成规范的 JSON 响应 + 正确的状态码
async fn me(State(state): State<AppState>, user: AuthenticatedUser)
    -> AppResult<Json<ApiResponse<UserResponse>>> {
    let profile = state.user_service().find_by_id(&user.user_id).await?;
    Ok(Json(ApiResponse::ok(profile)))
}
```

两条安全原则：

1. **5xx 的内部细节绝不返回给客户端**（可能泄露 SQL、路径、密钥），只返回固定兜底文案 + 写进服务端日志；
2. 4xx 与 5xx 分开记日志级别，避免客户端错误污染告警。

### 5. 可观测性：请求 ID + 结构化 span

```rust
// middleware.rs —— 请求 ID 中间件把 ID 同时写入请求头与响应头
fn make_request_span(request: &Request<Body>) -> tracing::Span {
    tracing::info_span!(
        "http_request",
        method = %request.method(),
        uri = %request.uri(),
        request_id = tracing::field::Empty,   // 先占位，稍后 record 填充
    )
}
```

线上排查时，用户只说"我点了注册报 500"；有了 `x-request-id`，就能把**这一次请求**相关的所有日志（网关、本服务、下游）一键串起来。

### 6. 安全要点（都在代码注释里写明了原因）

| 措施 | 位置 | 为什么 |
| --- | --- | --- |
| bcrypt 哈希 + 随机盐 | `services/password.rs` | 慢哈希 + 随机盐让彩虹表和暴力破解失效 |
| 登录失败统一文案 | `services/users.rs` | 区分"账号不存在/密码错误"会被用于枚举账号 |
| 账号不存在也做一次哈希校验 | `services/users.rs` | 抹平时序差异，防时序攻击 |
| 注册接口强制 `role = user` | `services/users.rs` | 否则客户端传 `"role":"admin"` 即可提权 |
| 出参 DTO 不含密码字段 | `models.rs` | 编译期杜绝泄露 |
| SQL 全部使用绑定参数 | `repository.rs` | 防 SQL 注入 |
| 依赖数据库唯一约束兜底 | `repository.rs` | 应用层"先查后插"存在 TOCTOU 竞态 |

---

## 七、测试

```bash
cargo test                    # 全部测试（102 个）
cargo test --lib              # 仅库单元测试（85 个）
cargo test --bin demoweb      # 仅主程序测试（2 个停机流程回归测试）
cargo test --test api_integration   # 仅集成测试（15 个）
cargo test -- --nocapture     # 显示 println 输出
```

| 类型 | 数量 | 特点 |
| --- | --- | --- |
| 单元测试（lib） | 85 | 用内存仓储 / 真实 JWT 服务，毫秒级完成；覆盖配置校验、错误映射、密码哈希、JWT、注册登录业务规则、路由装配、并发正确性 |
| 停机回归测试（bin） | 2 | 确定性验证"信号未到达时服务不会自行退出"——即下文坑 1 的回归防线 |
| 集成测试 | 15 | **启动真实 HTTP 服务**（绑定 `127.0.0.1:0` 随机端口）+ 真实客户端，完整覆盖中间件链、状态码、JSON 形状 |

> 坑 1 的回归测试是整个测试套件里最有价值的一个：它用"永不完成的信号 + 100ms 超时"复现缺陷场景，
> 断言 300ms 后服务**仍在正常响应**。若将来有人"优化"停机逻辑时重新引入该缺陷，这条测试会立刻变红。

集成测试的两个设计要点：

```rust
// ① 端口写 0 → 由系统分配空闲端口，多个测试实例可并行而不抢端口
let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;

// ② 并发 20 个真实请求，验证服务端在高并发下不丢数据、不报错
for _ in 0..20 { handles.push(tokio::spawn(http.get(&url).send())); }
```

并发正确性也有专门的断言：

```rust
// repository.rs —— 50 个并发写入后，记录数必须精确等于 50
assert_eq!(repo.len(), 50, "50 次并发写入应当全部落库");
```

---

## 八、踩坑记录（真实修复过程，值得一读）

### 坑 1：`tokio::select!` 会无条件轮询所有分支

**现象**：服务启动 10 秒后自动退出，日志显示"优雅停机超时，强制退出"。

**错误写法**：

```rust
tokio::select! {
    result = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()) => { result? }
    () = tokio::time::sleep(shutdown_timeout) => tracing::warn!("超时"),   // ← 从启动就开始计时！
}
```

表面语义是"服务结束或超时，谁先到算谁"，但 `select!` **会同时在第一次 poll 就启动所有分支**：`sleep` 从服务启动那一刻开始计时，于是服务运行 N 秒后必然走进超时分支（本项目的 N 就是配置里的 10）。

**修复**：把超时的**截止时刻**推迟到信号到达之后再计算——先用 `tokio::spawn` 拿到 `JoinHandle`，`await` 完信号才 `Instant::now() + timeout`：

```rust
let mut server_task = tokio::spawn(async move { axum::serve(listener, app).await });
shutdown_signal().await;                                    // 阶段 1：无限期等信号
let deadline = tokio::time::Instant::now() + timeout;       // 此刻才开始计时
tokio::select! {
    result = &mut server_task => { result??; }
    () = tokio::time::sleep_until(deadline) => { server_task.abort(); }
}
```

### 坑 2：SQLite 内存库的每条连接都是**独立数据库**

**现象**：迁移在连接 A 上建了表，业务查询被分到连接 B，报 `no such table: users`。

**原因**：SQLite 的 `:memory:` 数据库是**绑定在连接上**的，连接池里每条新连接都是一个全新的空库。

**修复**：内存模式把 `max_connections` 限制为 1，并在 `AppConfig::validate()` 里强制检查，配置错误直接拒绝启动。生产环境请改用文件库（连接池才有意义）。

### 坑 3：`config` crate 的 `prefix_separator` 默认跟随 `separator`

**现象**：设了 `APP_SERVER__REQUEST_TIMEOUT_SECS=42`，但配置读取到的仍是默认值 15，**且不报错**。

**原因**：`Environment::with_prefix("APP").separator("__")` 中，`prefix_separator` 的默认值是"跟随 separator"，于是前缀被当成 `APP__`，`APP_SERVER__...` 匹配不上，变量被**静默忽略**。

**修复**：显式设置 `prefix_separator("_")`，并补了两个单测锁住这个行为。

### 坑 4：`tower-http` 的 `SetRequestIdLayer` + `PropagateRequestIdLayer` 并不配对

**现象**：响应头里没有 `x-request-id`。

**原因**：`PropagateRequestIdLayer` 是从**请求头**取 ID 再写入响应头，而 `SetRequestIdLayer` 只把 ID 放进**请求扩展**（extensions），它并不修改请求头。

**修复**：自己写一个语义自洽的中间件，一次做完三件事（沿用上游 ID → 缺失则生成 → 同时写入请求头与响应头），并补了"必须沿用上游 ID"的测试。

### 坑 5：`AtomicU64` 没有实现 `Clone`

**现象**：给 `AppState` 派生 `Clone`（axum 的 `State` 提取器要求）时编译失败。

**原因**：克隆一个原子变量语义上是含糊的——是复制当前值，还是共享同一个计数？

**修复**：用 `Arc<AtomicU64>` 明确表达"多个状态句柄共享同一个计数器"。

---

## 九、从哪读起（建议顺序）

1. **`config.rs`** —— 配置从哪来、怎么校验（体会"拒绝带病启动"）；
2. **`error.rs`** —— 领域错误如何变成 HTTP 响应（`IntoResponse` 的威力）；
3. **`models.rs`** —— 领域实体与 DTO 为什么要分开；
4. **`repository.rs`** —— trait 抽象 + 两种实现（依赖倒置）；
5. **`services/password.rs`** —— `spawn_blocking` 为什么必不可少；
6. **`services/auth.rs`** —— JWT 签发校验 + 自定义提取器；
7. **`services/users.rs`** —— 注册/登录的完整业务规则与安全考量；
8. **`controllers/users.rs`** —— 控制器如何保持"薄"；
9. **`middleware.rs`** —— 中间件洋葱模型与顺序；
10. **`routes.rs`** —— 路由装配与"默认受保护"原则；
11. **`controllers/demo.rs`** —— 并发三模式；
12. **`main.rs`** —— 装配与优雅停机（含上面坑 1 的完整记录）。

### 配套练习建议

1. 给 `UserRepository` 加一个 `count()` 方法，实现它，并让 `/api/health` 返回用户总数；
2. 加一个 `PUT /api/users/me` 修改昵称的接口（想清楚要不要校验旧密码）；
3. 把 `InMemoryUserRepository` 换成 SQLite 实现，观察业务代码**一行都不用改**；
4. 给登录接口加一个基于 `AtomicU64` 的简单限流（例如每 IP 每分钟 10 次）；
5. 把日志改成 JSON 输出（`APP_LOG__JSON=true`），看结构化日志长什么样。

---

## 十、常用命令速查

```bash
cargo run                      # 启动服务
cargo run --release            # 生产构建启动（性能更好）
cargo test                     # 运行全部测试
cargo fmt                      # 格式化代码
cargo fmt --all -- --check     # 只检查格式（CI 用）
cargo clippy --all-targets     # 静态检查（本项目零警告）
cargo build --release          # 生产构建
```

环境变量覆盖示例：

```bash
# 换端口 + 打开 JSON 日志 + 自定义密钥（Windows PowerShell 用 $env: 前缀）
APP_SERVER__BIND_ADDRESS=0.0.0.0:8080 \
APP_LOG__JSON=true \
APP_JWT__SECRET=please-change-me-to-a-long-random-string \
cargo run
```

---

## 十一、这个项目刻意**没有**做的事

保持示例的可读性，以下生产级能力被刻意省略，留给你作为扩展方向：

- 没有 Dockerfile / CI 配置（与代码无关，属部署话题）；
- 没有刷新令牌（refresh token）、令牌吊销黑名单；
- 没有分布式追踪（OpenTelemetry）与指标（Prometheus）导出；
- 没有数据库读写分离、分页查询、软删除；
- 没有权限模型细分（仅演示 `user` / `admin` 两级角色）。

每一项都可以在现有分层上**平滑扩展**——这正是分层架构的价值所在。
