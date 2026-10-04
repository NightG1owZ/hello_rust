# 工程实践指南：从「能跑」到「能维护」

> 本文档沉淀「怎么组织、怎么协作、怎么演进」。主题大多需要第三方 crate 或仓库结构变更，
> 因此以文档形式落地（实践课 lesson_14~18 保持零依赖）。示例均以本项目现状为基准。

---

## 一、Cargo workspace：多 crate 仓库组织

**本项目现状**：根目录 `hello`（13+5 门课程）与 `src/demoweb`（Web Demo）是**两个独立的 Cargo 项目**，各自有 Cargo.toml 与 target 目录，互不感知。

**何时值得升级为 workspace**：多个 crate 共享依赖、希望统一 `cargo test`/`cargo build`、或开始拆分公共库时。改造方法（示范，未在本仓库执行）：

```toml
# 根 Cargo.toml 增加
[workspace]
members = [".", "src/demoweb"]
resolver = "2"
```

改造后：所有成员共享一个 `Cargo.lock` 与 target 缓存，根目录一条 `cargo test --workspace` 跑完全部测试；各成员仍保持独立版本号。

**何时不必**：成员间无依赖共享、构建隔离反而清晰（本项目教学场景即属此类，故保持现状）。

---

## 二、feature flags：可选能力的开关

```toml
[features]
default = []
json = ["dep:serde_json"]   # cargo build --features json 时才启用
```

- 用途：同一份代码按需编译出不同能力组合（CLI 的可选导出格式、库的可选异步运行时）；
- 习惯：`default` 保持最小；feature 应当**可叠加**（任意组合都能编译）；
- 本项目的教学策略相反——刻意零依赖零 feature，先让语言特性显形，工程化开关进入 demoweb 再学。

---

## 三、错误处理选型：手写 / thiserror / anyhow

实践课程：lesson_10（手写错误类型的完整套路）。

| 方案 | 适用 | 理由 |
| --- | --- | --- |
| 手写（lesson_10 的做法） | 教学、超简单项目 | 看清 `Display` + `Error` + `From` 的全部机制 |
| `thiserror` | **库**（别人要 match 你的错误） | derive 生成 Display/From，错误类型仍是精确枚举 |
| `anyhow` | **应用/二进制**（只需把错误传到顶层打印） | `anyhow::Error` 万能包装 + `Context` 附加现场信息 |

决策口诀：**对外接口用 thiserror 保精确，应用主体用 anyhow 求省事**。demoweb 的 `error.rs` 展示了 anyhow + 统一 HTTP 错误响应的落地。

---

## 四、测试策略：三层防线

demoweb 的 102 个测试就是按这三层组织的：

| 层 | 位置 | 测什么 | 示例 |
| --- | --- | --- | --- |
| 单元测试 | 源码文件内 `#[cfg(test)] mod tests` | 单个函数/结构的逻辑 | 密码哈希 roundtrip、JWT 过期判断 |
| 集成测试 | `tests/` 目录，独立 crate | 走真实 HTTP 的端到端行为 | 健康检查、未授权 401、分页查询 |
| 文档测试 | 文档注释里的 ```` ```rust ```` 块 | 示例代码本身可运行 | 库 API 的用法示例 |

起步顺序：先给纯函数写单元测试（成本最低）；接口稳定后补集成测试；库代码随手写 doctest。
运行：`cargo test`（全部）、`cargo test --bin lesson_10_error_handling`（按目标过滤）。

---

## 五、文档即代码：rustdoc 与 doctest

- `///` 写给**使用者**（函数/结构体文档），`//!` 写给**读者**（模块级说明）——本仓库课程文件头部即 `//!`；
- `cargo doc --open` 生成并浏览全项目文档；公开 API 没有 `///` 会被 clippy 提醒；
- 文档中的代码块默认会被 `cargo test` **编译并执行**（doctest）：示例代码永远不会悄悄失效；
- README 驱动：像本仓库一样，把「30 秒跑起来」放在 README 顶部，新环境按文档从零跑通一次胜过十次口头交接。

---

## 六、API 设计模式三件套

### 1. newtype：用零成本包装表达业务语义

```rust
struct UserId(u64);          // 编译期杜绝「把订单 id 当用户 id」的混用
struct Port(u16);            // 同时收窄取值语义，构造函数里做范围校验
```

运行期零开销（编译后就是 u64/u16），却把一类低级错误从 code review 层挪到了编译期。demoweb 的 `model` 层大量使用此模式。

### 2. Builder：解决多可选参数的构造爆炸

```rust
let config = ServerConfig::builder()
    .host("127.0.0.1")
    .port(3000)
    .workers(4)
    .build()?;
```

reqwest/axum 的配置入口几乎全是 Builder：链式、可选、类型安全。手写要领：`builder()` 关联函数起头，每个 setter 消耗 `self` 返回 `Self`，`build()` 做最终校验。

### 3. 类型状态：把非法状态编码进类型

```rust
struct Empty;               // 未配置状态
struct Ready;               // 已配置状态
struct Client<S> { state: S }
impl Client<Ready>  { fn send(self) {...} }   // 只有 Ready 才有 send
```

「未初始化就调用」这类错误直接编译失败。与 lesson_07 的状态机枚举一脉相承：**让非法状态无法被表示**。

---

## 七、性能习惯：先测量，再优化

1. 建立基线：`cargo build --release`（永远用 release 测性能，debug 与之相差 10~50 倍）；
2. 微基准：`criterion` crate 写基准测试，统计置信区间而非单次计时；
3. 火焰图：`cargo flamegraph` 定位热点，眼睛盯着最宽的栈条优化；
4. 常见免费优化：迭代器替代手动循环（lesson_15 的零成本抽象）、`&str` 替代 `String` 参数（lesson_16 Deref 强转）、避免热路径上的多余 `clone()`；
5. 心法：Rust 的抽象大多是零成本的，**先写清晰代码，用数据证明慢了才优化**。

---

## 八、版本与发布纪律

- 遵循 [语义化版本](https://semver.org/)：破坏 API 才升主版本；Rust 生态默认「次要版本内不破坏」；
- `Cargo.lock`：应用提交入库（构建可复现），库不提交；
- 每次破坏性变更写 CHANGELOG（一行为什么改 + 怎么迁移）；
- 发布到 crates.io：`cargo publish --dry-run` 先演练，`cargo package` 检查打包内容里没有多余文件与密钥。

---

## 九、CI 最小配置（GitHub Actions）

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --all -- --check          # 格式不一致直接失败
      - run: cargo clippy --bins -- -D warnings  # 警告当错误
      - run: cargo build --bins
      - run: cargo test --workspace
```

四条流水线的含义：格式统一 → 惯用法统一 → 可编译 → 行为正确。本仓库的
[.dsh/check_expected_output.ps1](../../../.dsh/check_expected_output.ps1) 可作为第五步接入，
保证课程「预期输出」断言持续有效。
