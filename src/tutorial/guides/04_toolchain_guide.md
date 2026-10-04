# 常用工具链指南

> 本文档沉淀「用什么工具、怎么配、什么时候用」。全部工具对本项目课程均非必需——
> 一条 `cargo run --bin lesson_XX_主题` 就能跑通全部课程；但用好它们，学习效率显著提升。

---

## 一、rust-analyzer：IDE 体验的核心

- 安装：VS Code 装「rust-analyzer」扩展（其余编辑器见[官网](https://rust-analyzer.github.io/)）；
- 首次打开本项目它会索引全部 18 门课程与 demoweb，状态栏跑完前少量跳转会失效，属正常现象；
- 推荐设置（settings.json）：

```json
{
  "rust-analyzer.check.command": "clippy",   // 保存时用 clippy 而非裸 check，顺手抓惯用法
  "rust-analyzer.inlayHints.parameterHints.enable": true,   // 行内类型提示
  "rust-analyzer.cargo.features": []
}
```

- 学习期最有价值的三件事：**类型悬停**（鼠标停在表达式上看推断结果，配合 lesson_11 泛型推断）、**行内错误**（不用切终端就能看到 E0382 级别的报错）、**Go to Definition**（直接跳进标准库源码，`Vec`/`HashMap` 的真实实现就在那里）。

---

## 二、clippy：官方 linter，惯用法教练

```bash
cargo clippy --bins              # 检查全部课程
cargo clippy --bins -- -D warnings   # 警告当错误（CI 标准姿势）
```

lint 分组与本项目的关系：

| 分组 | 含义 | 举例 |
| --- | --- | --- |
| correctness | 疑似真 bug | `double_parens`、`suspicious_arithmetic_impl` |
| suspicious | 大概率写错 | `almost_complete_range`（`0..5` 想写 `0..=5`） |
| style | 惯用写法 | `redundant_clone`、`needless_borrow` |
| pedantic | 严格模式（默认关闭） | `must_use_candidate`、`module_name_repetitions` |
| nursery | 实验性 | 尚不稳定，不必开启 |

学习建议：把 clippy 当「免费代码评审」。每门课学完跑一次 `cargo clippy --bins`，
对每条警告点开编号（如 `clippy::redundant_clone`）读官方解释——这套解释本身就是最好的惯用法教材。

---

## 三、cargo 子命令地图

### 内置（装好就能用）

| 命令 | 用途 | 本项目场景 |
| --- | --- | --- |
| `cargo check` | 只类型检查不生成二进制，最快 | 写代码时的快速反馈环 |
| `cargo build --bins` | 编译全部 18 门课程 | 验证整体 |
| `cargo run --bin <名字>` | 编译并运行单课 | 日常主命令 |
| `cargo test` | 跑测试（lessons 无测试，demoweb 有 102 个） | demoweb 阶段 |
| `cargo fmt` / `cargo fmt --check` | 格式化 / 只检查 | 提交前必跑 |
| `cargo clippy` | 静态检查 | 见上节 |
| `cargo doc --open` | 生成并打开文档 | 查标准库用法 |
| `cargo tree` | 依赖树 | 进入 demoweb 后看依赖从哪来 |
| `cargo add <crate>` | 添加依赖并自动选版本 | 扩展练习时 |
| `cargo metadata --format-version 1` | 机器可读的项目元数据 | 脚本化工具的输入 |

### 推荐第三方子命令（按需安装）

| 命令 | 安装 | 用途 |
| --- | --- | --- |
| `cargo audit` | `cargo install cargo-audit` | 用 RustSec 数据库扫描依赖已知漏洞（进入 demoweb 后建议跑一次） |
| `cargo outdated` | `cargo install cargo-outdated` | 列出可升级的依赖 |
| `cargo deny` | `cargo install cargo-deny` | 依赖许可证/来源/重复版本治理（团队协作必备） |
| `cargo expand` | `cargo install cargo-expand` | 查看宏展开后的真实代码——学 lesson_18 时把 `sum_all!` 展开看一眼，胜过十行解释 |
| `cargo watch` | `cargo install cargo-watch` | `cargo watch -x "run --bin lesson_14_closures"` 保存即重跑 |

---

## 四、依赖治理常识

- **semver 约定**：`0.8` 之间互相兼容，`1.x` 内不破坏；`cargo update` 只会在兼容范围内升；
- **Cargo.lock**：应用项目提交入库（保证每个人/CI 构建一致），库项目不入库；
- **最小依赖原则**：每引入一个 crate 都在为供应链安全与编译时间付费；评估一个 crate 先看下载量、维护活跃度、`cargo tree` 里的传递依赖数量；
- **安全审计**：升级依赖后跑 `cargo audit`；CI 里固定跑（见 [03_engineering_practices.md](03_engineering_practices.md) 第九节）。

---

## 五、调试手段（按成本从低到高）

1. **println! 调试**：`dbg!(&value)` 宏比 `println!` 更好用——自动带上文件名、行号、表达式文本；
2. **结构化日志**：`tracing::info!` / `tracing::debug!`（demoweb 已配置：请求 ID、耗时、级别过滤），先想日志再想断点；
3. **断点调试**：VS Code 装 CodeLLDB 扩展，`F5` 直接对 `cargo run` 的二进制断点调试；变量窗、调用栈、条件断点齐全；
4. **`rust-gdb` / `rust-lldb`**：命令行调试器包装脚本（rustup 自带），对标准库类型（`String`、`Vec`）做了美化显示；
5. **错误复现最小化**：把出问题的代码缩到 20 行内的单文件（本项目的课程文件就是天然模板），`rustc --edition 2024 xxx.rs` 单独验证。

---

## 六、进阶检测工具（学有余力再看）

| 工具 | 用途 | 与本项目的结合点 |
| --- | --- | --- |
| `miri` | 解释执行并检测未定义行为 | 对 lesson_16 的指针操作做实验（`rustup +nightly component add miri`） |
| `loom` | 并发调度的穷举模型检测 | 验证 lesson_17 写的并发逻辑在所有交错顺序下都正确 |
| sanitizers（ASan/TSan） | 运行期内存/线程错误检测 | `RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test`（demoweb 阶段） |

> 使用原则：这些工具是**验证已写逻辑**的放大镜，不是替代理解的黑盒；
> 先把课程里的所有权/借用/并发模型想通，工具才有用武之地。

---

## 七、文档与查阅资源

| 资源 | 用途 |
| --- | --- |
| `cargo doc --open` | 本地生成的标准库文档（离线可用） |
| [std 文档](https://doc.rust-lang.org/std/) | 官方标准库手册 |
| [The Book](https://doc.rust-lang.org/book/) | 官方教程，与本项目课程互为对照 |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | 按语法点查例子 |
| [Rust 编译错误索引](https://doc.rust-lang.org/error_codes/) | `rustc --explain E0382` 直接看解释（也可在 IDE 里一键打开） |
| demoweb README 的踩坑记录 | 本仓库内最贴近实战的 5 个真实问题 |
