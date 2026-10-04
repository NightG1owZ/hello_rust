# Rust 系统学习项目（RustPros / hello）

一套面向**零基础到进阶**的 Rust 中文学习项目，由三个部分组成：

- **[src/tutorial](src/tutorial/README.md)** —— **统一教程目录**：18 门可独立编译运行的核心课程，序号 01~18 即学习顺序（01~13 语言基础，14~18 闭包/迭代器/智能指针/线程与通道/宏），外加 4 篇学习指南（学习方法论/语言原理/工程实践/工具链）；每课都配大量行内注释、预期输出说明、典型场景与常见错误对照；
- **[src/demoweb](src/demoweb/README.md)** —— 一个基于 Axum 的**企业级简易 Web Demo**：把课程里学的所有权、trait、错误处理、异步并发落到一个真实可运行的后端服务里；
- **[assessments](assessments/README.md)** —— **考核环节（可选但强烈推荐）**：18 个与课程一一对应的测试驱动自测文件（157 个知识点、182 个练习）+ 每题的**参考答案**（[assessments/solutions](assessments/solutions/README.md)，单独存放、不会被 `cargo test` 编译）+ 自动化学情评估报告，检验你「是真的掌握了，还是只是看懂了」。

- 语言版本：**Rust 1.99 stable**（edition 2024）
- 课程模块依赖：**仅标准库**，无任何第三方 crate（demoweb 除外）
- 命名规范：课程文件统一为 `lesson_<两位序号>_<英文主题>.rs`，**二进制名与文件名一致**，看到文件名就知道主题、顺序和运行命令
- **验证状态**：18 个课程文件（13 基础 + 5 补充）全部通过编译（0 error / 0 warning）、全部运行退出码 0、全部符合 `rustfmt --edition 2024` 默认风格；共 **808 条**「预期输出」断言全部核对通过

---

## 一、目录结构

```
hello/
├── Cargo.toml                              # 显式注册 21 个 [[bin]]（hello + 18 门课程 + 报告生成器 + 参考答案工具箱）与 18 个测试目标
├── README.md                               # 本文件（项目总览）
├── src/
│   ├── main.rs                             # 原始 hello 示例（保留）
│   ├── tutorial/                           # ★ 教程目录：18 门课程 + 4 篇学习指南
│   │   ├── README.md                       #   课程目录、学习顺序与各课说明
│   │   ├── lesson_01_variables_mutability.rs   # 01 变量与可变性
│   │   ├── lesson_02_data_types.rs             # 02 数据类型
│   │   ├── lesson_03_functions.rs              # 03 函数
│   │   ├── lesson_04_control_flow.rs           # 04 流程控制
│   │   ├── lesson_05_ownership_borrowing.rs    # 05 所有权系统
│   │   ├── lesson_06_structs.rs                # 06 结构体
│   │   ├── lesson_07_enums_pattern_matching.rs # 07 枚举与模式匹配
│   │   ├── lesson_08_collections.rs            # 08 常见集合
│   │   ├── lesson_09_packages_modules.rs       # 09 包和模块
│   │   ├── lesson_10_error_handling.rs         # 10 错误处理
│   │   ├── lesson_11_generics.rs               # 11 泛型
│   │   ├── lesson_12_traits.rs                 # 12 Trait
│   │   ├── lesson_13_lifetimes.rs              # 13 生命周期
│   │   ├── lesson_14_closures.rs               # 14 闭包
│   │   ├── lesson_15_iterators.rs              # 15 迭代器
│   │   ├── lesson_16_smart_pointers.rs         # 16 智能指针
│   │   ├── lesson_17_threads_channels.rs       # 17 线程与通道
│   │   ├── lesson_18_macros.rs                 # 18 声明宏
│   │   └── guides/                         #   学习指南（Markdown 手册，按需查阅）
│   │       ├── 01_learning_method.md       #     学习方法论：四步循环、错误阅读法、路线图
│   │       ├── 02_language_theory.md       #     语言原理补遗：所有权模型/Send-Sync/unsafe/async
│   │       ├── 03_engineering_practices.md #     工程实践：workspace/测试/设计模式/CI
│   │       └── 04_toolchain_guide.md       #     工具链：rust-analyzer/clippy/调试/依赖治理
│   └── demoweb/                            # ★ Axum Web 实战 Demo（独立 Cargo 项目）
│       ├── README.md                       #   Demo 的完整文档（架构 / API / 踩坑记录）
│       └── ...                             #   20 个源文件 + 102 个测试
├── assessments/                            # ★ 考核环节（可选但推荐）：测试驱动自测 + 学情评估报告
│   ├── README.md                           #   考核体系说明（30 秒上手 / 评分口径 / 常见问题）
│   ├── harness/lib.rs                      #   考核框架 assessment_harness（零第三方依赖）
│   ├── lesson_01..18_*.rs                  #   18 个考核文件（与课程文件同名、一一对应）
│   ├── bin/assessment_report.rs            #   学习评估报告生成器（总分/薄弱环节/进度对比）
│   ├── bin/solution_tool.rs                #   参考答案工具箱（校验/应用/还原/验证）
│   ├── solutions/                          #   ★ 参考答案（每课一个文件，182 个练习；cargo test 不编译）
│   ├── scripts/                            #   一键脚本（run_assessments.ps1 / .sh）
│   └── docs/                               #   环境配置 / 答题指南 / 报告解读 / 知识点矩阵 / 维护者指南
└── .dsh/lesson-conventions.md              # 课程文件统一编写规范（维护者参考）
```

> 说明：根 `Cargo.toml` 中设置了 `autobins = false`，课程文件通过显式 `[[bin]]` 注册，保证 18 门课程可以各自拥有独立的 `fn main()` 并存。

---

## 二、环境搭建指南

### 1. 安装 Rust（Windows / macOS / Linux）

推荐使用官方工具链管理器 **rustup**。

**Windows**

1. 访问 <https://www.rust-lang.org/tools/install> 下载 `rustup-init.exe`；
2. 双击运行，安装程序会提示安装 **MSVC 构建工具**（Visual Studio C++ Build Tools），
   选择默认选项 `1) Proceed with standard installation` 即可；
3. 安装完成后**重开一个终端**，执行：

```powershell
rustc --version
cargo --version
rustup --version
```

能打印版本号即安装成功（本项目在 `rustc 1.99.0` 上验证通过）。

**macOS / Linux**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"     # 或重开终端
rustc --version
```

### 2. 更新到最新稳定版

```bash
rustup update stable          # 更新 stable 工具链
rustup default stable         # 确保默认使用 stable
rustup component add rustfmt  # 代码格式化工具
rustup component add clippy   # 官方 lint 工具
```

本项目不使用任何 nightly / 实验性特性，stable 即可完整运行。

### 3. 常用命令速查

| 命令 | 作用 |
| --- | --- |
| `cargo run --bin lesson_05_ownership_borrowing` | 编译并运行第 5 课 |
| `cargo build --bins` | 一次性编译全部课程 |
| `cargo check` | 只做类型检查，速度最快 |
| `cargo test --test lesson_05_ownership_borrowing` | 考第 5 课（考核环节，见 [assessments](assessments/README.md)） |
| `cargo test --no-fail-fast` | 考核全部 18 课（一次拿到完整结果） |
| `cargo run --bin assessment_report` | 生成学习评估报告（总分 / 薄弱环节 / 进度对比） |
| `rustfmt --edition 2024 src/tutorial/lesson_01_variables_mutability.rs` | 格式化单个文件 |
| `cargo clippy --bins` | 静态检查，学习惯用法 |
| `rustc --edition 2024 src/tutorial/lesson_01_variables_mutability.rs -o lesson01.exe` | 不使用 Cargo，直接编译单文件 |

---

## 三、模块说明

### 模块一：src/tutorial —— 统一教程（18 课 + 4 篇学习指南）

每课是一个**自包含**的可运行程序：只依赖标准库，不引入其它课程文件，单独拿去 `rustc` 编译也能通过。课程按五个阶段组织：

| 阶段 | 课程 | 目标 |
| --- | --- | --- |
| 第一阶段：语法基础 | 01 变量与可变性 · 02 数据类型 · 03 函数 · 04 流程控制 | 掌握"表达式"思维，能读懂并改写简单 Rust 程序 |
| 第二阶段：Rust 的灵魂 | 05 所有权系统 · 06 结构体 · 07 枚举与模式匹配 | 理解 Rust 区别于其它语言的核心设计 |
| 第三阶段：工程化能力 | 08 常见集合 · 09 包和模块 · 10 错误处理 | 写出组织良好、错误处理规范的工程代码 |
| 第四阶段：抽象与进阶 | 11 泛型 · 12 Trait · 13 生命周期 | 进入抽象层，能读懂标准库与主流 crate 的签名 |
| 第五阶段：补充与实战 | 14 闭包 · 15 迭代器 · 16 智能指针 · 17 线程与通道 · 18 声明宏 | 补齐 demoweb 真正需要的进阶特性，打好并发与异步的前置知识 |

`guides/` 下的 4 篇学习指南回答「为什么这样设计」「怎么学得快」「用什么工具」：学习方法论、语言原理补遗（所有权模型 / Send-Sync / unsafe / async）、工程实践（workspace / 测试 / 设计模式 / CI）、工具链（rust-analyzer / clippy / 调试 / 依赖治理）。

各课详细的知识点清单、运行命令与推荐学习顺序见 **[src/tutorial/README.md](src/tutorial/README.md)**。

### 模块二：src/demoweb —— Axum Web 实战 Demo

一个**从零构建、可直接运行**的企业级风格 Web 服务（**Axum 0.8 + Tokio + SQLx/SQLite + JWT + bcrypt**，102 个测试），用于把课程知识落到真实后端场景：

- 分层架构：controllers / services / repository / models，职责边界清晰；
- 全异步链路：`async/await`、`tokio::join!`、`spawn_blocking`、优雅停机；
- 生产级机制：三级配置、统一错误响应、请求 ID 追踪、JWT 认证与 RBAC；
- 质量保障：单元测试 + 集成测试（真实 HTTP 端到端）、5 条真实踩坑记录。

详细文档（30 秒跑起来、API 示例、阅读顺序、踩坑记录）见 **[src/demoweb/README.md](src/demoweb/README.md)**。

### 模块三：assessments —— 考核环节（可选但强烈推荐）

`src/tutorial/` 下的 18 门课解决「知识讲清楚」，`assessments/` 解决「你到底掌握了没有」：

- **与课程一一对应**：`assessments/lesson_05_ownership_borrowing.rs` 对应 `src/tutorial/lesson_05_ownership_borrowing.rs`，
  测试目标名就是课程文件名：`cargo test --test lesson_05_ownership_borrowing`；
- **测试驱动**：每个知识点是一个 `#[test]`，里面写着期望值与边界用例；
  需要你实现的是同名的 `exercise_*` 练习函数（骨架里只有一行 `todo_exercise(...)` 占位，
  注释里给了「实现要求 + 示例输入/示例输出」——照着示例自测最快）；
- **每题都有参考答案**：`assessments/solutions/` 里放着 **182 个练习的参考答案**（每课一个文件、与考核文件同名，
  只有练习函数体换成了真实实现）。这些文件**不会被 `cargo test` 编译**，也不出现在测试输出里，
  所以正常练习时不会"不小心看到答案"；想对照请看
  [assessments/solutions/README.md](assessments/solutions/README.md)：`--list` 列出「练习 → 答案行号」，
  `--verify --lesson 05` 则是「应用答案 → 跑测试 → 自动还原骨架」的一条命令。**还是那句话：先自己写，再看答案**；
- **覆盖与 README 一致**：18 个考核文件、**157 个知识点**，逐条对齐 `src/tutorial/README.md` 五个阶段表格里标注的核心知识点（含**难点专项**与**边界条件**用例），
  完整矩阵见 [assessments/docs/04_knowledge_map.md](assessments/docs/04_knowledge_map.md)；
- **每个用例都验证过「可解」**：18 个文件都经过「骨架 → 参考实现 → 骨架」往返验证，
  参考实现下 157/157 全部通过，骨架态零 warning 且每个知识点都明确报【未实现】，
  验收记录见 [assessments/report/samples/verification.md](assessments/report/samples/verification.md)；
- **有反馈、有报告、有进度**：测试输出会区分「未实现 / 未通过 / 运行期错误」并给出**期望、实际、提示、复习入口**；
  `cargo run --bin assessment_report` 生成学习评估报告（总分、按模块得分、四类知识点掌握率、薄弱环节与行动清单、与上次对比的进度）；
- **零依赖、纯离线**：只用标准库 + 本项目自带的考核框架 `assessment_harness`，不引入任何第三方 crate；
- **完全可选**：考核是独立目标，不影响 `cargo run --bin lesson_XX_...` 与 `cargo build --bins`。

快速上手、评分口径、常见问题见 **[assessments/README.md](assessments/README.md)**。

### 三个部分的关系

```
src/tutorial（学语言：01~13 打基础，14~18 补缺口，guides 讲方法） ──► src/demoweb（用语言）
  单文件、零依赖                                                    多 crate 协作、真实业务
        │                                                                    │
        └────────────────► assessments（考核：每课一个测试驱动自测） ◄─────────┘
                            可选但强烈推荐：用测试证明自己真的掌握了
```

建议先把 01~13 过完（至少到第 10 课），再按教程目录的推荐顺序补齐 14~18，最后进入 demoweb——那里的每一层代码都是课程知识点的综合运用。

---

## 四、学习路径指南

### 路线 A：完整系统学习（推荐，约 3~4 周）

```
第 1 周  01 → 02 → 03 → 04          语法基础，重点是"表达式"思维
第 2 周  05 → 06 → 07               所有权/结构体/枚举，Rust 的分水岭
第 3 周  08 → 09 → 10               集合/模块/错误处理，写出工程化代码
第 4 周  11 → 12 → 13               泛型/Trait/生命周期，进入抽象层
第 5 周  14 → 18                    闭包/迭代器/智能指针/线程/宏，补齐关键缺口
之后     src/demoweb                把语言知识放到真实 Web 项目里检验

每课学完（可选但推荐）：cargo test --test lesson_XX_主题  →  cargo run --bin assessment_report
```

### 路线 B：快速通读（有其它语言基础，3~5 天）

先读 `05 → 07 → 10 → 12`，这四课覆盖 Rust 区别于其它语言的关键设计（所有权、模式匹配、错误处理、Trait），再用 `01 → 02 → 03 → 04 → 06 → 08 → 09 → 11 → 13` 补齐细节，最后按需加入第五阶段的 14~17 课（闭包/迭代器/智能指针/并发是 demoweb 的前置）。

### 路线 C：按痛点查阅

| 遇到问题 | 直接看 |
| --- | --- |
| `cannot borrow as mutable` / `borrow of moved value` | lesson_05_ownership_borrowing |
| `missing lifetime specifier` | lesson_13_lifetimes |
| `the trait bound ... is not satisfied` | lesson_11_generics、lesson_12_traits |
| `the size for values of type ... cannot be known` | lesson_12_traits（`dyn` / `Box`） |
| 不知道该 panic 还是返回 `Result` | lesson_10_error_handling |
| 不知道模块怎么拆、`pub` 加在哪 | lesson_09_packages_modules |

### 每课的学习方法（重要）

1. **先读文件头注释**，明确本课学习目标；
2. **先自己猜输出**，再运行 `cargo run --bin lesson_XX_主题` 对照；
3. **把 demo 函数逐个注释掉再打开**，观察编译器在你删改代码后给出的报错——每课都附了"常见错误示例"，那是刻意保留的路标；
4. **动手改**：把示例里的类型、常量、分支顺序改掉，看编译器如何提示；
5. 用 `cargo clippy` 检查自己的写法，逐步形成 Rust 惯用法直觉；
6. **做个自测**（可选但强烈推荐）：`cargo test --test lesson_XX_主题`，把考核文件里的 `exercise_*` 练习函数实现到全部变绿，
   再用 `cargo run --bin assessment_report` 看评估报告——报告会告诉你这一课哪些知识点是真掌握、哪些只是看懂了。

> 学习流程上，「读课 → 跑示例 → 做考核自测」是闭环：考核失败会直接指向该课的示例编号与 README 条目，
> 你不用猜自己漏了什么。详见 [assessments/README.md](assessments/README.md)。

---

## 五、课程文件清单（按学习顺序）

### 第一阶段：语法基础（必须先掌握）

| 编号 | 文件 | 核心知识点 | 典型场景 |
| --- | --- | --- | --- |
| 01 | [lesson_01_variables_mutability.rs](src/tutorial/lesson_01_variables_mutability.rs) | `let` 绑定、`mut` 可变性、变量遮蔽（shadowing）、`const` 与 `static`、类型推断与显式标注 | 用常量描述系统上限、用可变计数器累加状态 |
| 02 | [lesson_02_data_types.rs](src/tutorial/lesson_02_data_types.rs) | 整数/浮点/布尔/字符、整数字面量与溢出、元组与数组、`as` 转换、`From`/`TryFrom` | 解析用户输入并安全转换为数值 |
| 03 | [lesson_03_functions.rs](src/tutorial/lesson_03_functions.rs) | 函数定义、参数、返回值、**语句与表达式的区别**、函数指针、发散函数 `!` | 把一大段逻辑拆成若干可复用的小函数 |
| 04 | [lesson_04_control_flow.rs](src/tutorial/lesson_04_control_flow.rs) | `if` 是表达式、`loop`/`while`/`for`、`break` 带值、`continue`、循环标签 | 表格遍历、状态机、提前跳出嵌套循环 |

### 第二阶段：Rust 的灵魂

| 编号 | 文件 | 核心知识点 | 典型场景 |
| --- | --- | --- | --- |
| 05 | [lesson_05_ownership_borrowing.rs](src/tutorial/lesson_05_ownership_borrowing.rs) | 所有权三规则、`move`/`Copy`/`Clone`、栈与堆、引用与借用规则、可变引用唯一性、切片 `&str`/`&[T]`、`Drop` | 写一个"取最长单词"函数并解释借用关系 |
| 06 | [lesson_06_structs.rs](src/tutorial/lesson_06_structs.rs) | 结构体定义与实例化、字段初始化简写、结构体更新语法、元组结构体、方法（`&self`/`&mut self`/`self`）、关联函数、`Debug`/`Display` | 用结构体建模矩形、订单并实现面积/总价计算 |
| 07 | [lesson_07_enums_pattern_matching.rs](src/tutorial/lesson_07_enums_pattern_matching.rs) | 枚举变体携带数据、`Option<T>`、`Result<T, E>`、`match`、通配 `_`、匹配守卫、`@` 绑定、`if let`、`while let`、`let else` | 用枚举实现状态机与命令行解析 |

### 第三阶段：工程化能力

| 编号 | 文件 | 核心知识点 | 典型场景 |
| --- | --- | --- | --- |
| 08 | [lesson_08_collections.rs](src/tutorial/lesson_08_collections.rs) | `Vec<T>` 增删改查、`String` 与 `&str` 与 UTF-8、字符串拼接与格式化、`HashMap` 增查改与 `entry` API、集合的借用陷阱 | 统计一段文本的词频并排序输出 |
| 09 | [lesson_09_packages_modules.rs](src/tutorial/lesson_09_packages_modules.rs) | crate / package / 模块、`pub` 与可见性层级、`use` 与 `as`、`pub use` 重导出、`crate`/`self`/`super` 路径、文件与目录模块拆分 | 用嵌套模块组织一个迷你库并暴露公开 API |
| 10 | [lesson_10_error_handling.rs](src/tutorial/lesson_10_error_handling.rs) | `panic!`/`unwrap`/`expect`、`Result`、`?` 运算符、`From` 自动错误转换、自定义错误类型（`Display` + `Error`）、`Box<dyn Error>`、`main` 返回 `Result` | 解析 `key=value` 配置串并在出错时给出清晰错误 |

### 第四阶段：抽象与进阶

| 编号 | 文件 | 核心知识点 | 典型场景 |
| --- | --- | --- | --- |
| 11 | [lesson_11_generics.rs](src/tutorial/lesson_11_generics.rs) | 泛型函数/结构体/枚举/方法、单态化、`trait bound`、`where` 子句、多泛型参数、`const` 泛型 | 泛型坐标点、泛型求最大值、定长缓存 |
| 12 | [lesson_12_traits.rs](src/tutorial/lesson_12_traits.rs) | `trait` 定义与实现、默认方法、`impl Trait` 参数、泛型 + bound、返回 `impl Trait`、`+` 组合约束、关联类型、`dyn` trait 对象与动态分发 | 抽象"可绘制图形"集合并按面积排序 |
| 13 | [lesson_13_lifetimes.rs](src/tutorial/lesson_13_lifetimes.rs) | 为什么需要生命周期、函数签名生命周期注解、多生命周期参数、结构体持有引用、`impl` 块中的生命周期、省略三规则、`'static`、生命周期与 `trait bound` | 返回较长字符串引用、结构体持有 `&str` 配置 |

### 第五阶段：补充与实战

| 编号 | 文件 | 核心知识点 | 典型场景 |
| --- | --- | --- | --- |
| 14 | [lesson_14_closures.rs](src/tutorial/lesson_14_closures.rs) | 闭包语法与类型推断、捕获定义处的变量、`Fn`/`FnMut`/`FnOnce` 的差别、`move` 捕获、闭包作参数（`impl Fn`）与返回值、`Box<dyn Fn>` 异构集合 | 迭代器适配器、回调注册、任务队列 |
| 15 | [lesson_15_iterators.rs](src/tutorial/lesson_15_iterators.rs) | `Iterator` 与 `next()` 协议、`iter`/`iter_mut`/`into_iter` 的所有权差别、适配器链（`enumerate`/`filter`/`map`）、消费器（`sum`/`max`/`any`）、`fold`/`collect`、惰性与短路、自定义迭代器 | 词频统计、数据清洗管道 |
| 16 | [lesson_16_smart_pointers.rs](src/tutorial/lesson_16_smart_pointers.rs) | `Box<T>` 与递归类型、堆分配与解引用、`Rc<T>` 共享所有权与引用计数、`RefCell<T>` 内部可变性与**运行期**借用规则、`Rc<RefCell<T>>` 组合、`Weak<T>` 打破循环引用 | 树/链表等递归结构、共享可变缓存 |
| 17 | [lesson_17_threads_channels.rs](src/tutorial/lesson_17_threads_channels.rs) | `thread::spawn` + `join` 回收返回值、`move` 闭包转移所有权、`mpsc` 通道、多生产者 `tx.clone()`、`Arc<Mutex<T>>`、`Send`/`Sync`、`thread::scope` 借用局部数据、分块并行求和 | 并行计算、工作池、共享状态 |
| 18 | [lesson_18_macros.rs](src/tutorial/lesson_18_macros.rs) | `macro_rules!` 可变参数与重复模式、片段说明符（`$x:ident`/`expr`/`ty`）、`$(...),+` 与可选尾逗号、`stringify!`/`concat!`、`ensure!` 提前返回宏、小型 DSL、宏卫生性、典型报错 | 消除校验样板、自定义断言宏 |

### 验证结果一览（本机 Rust 1.99.0 / edition 2024）

| 文件 | 行数 | demo 示例数 | 编译 | 运行退出码 | rustfmt |
| --- | --- | --- | --- | --- | --- |
| lesson_01_variables_mutability.rs | 340 | 7 | 通过 0 warning | 0 | 合规 |
| lesson_02_data_types.rs | 465 | 8 | 通过 0 warning | 0 | 合规 |
| lesson_03_functions.rs | 444 | 7 | 通过 0 warning | 0 | 合规 |
| lesson_04_control_flow.rs | 502 | 7 | 通过 0 warning | 0 | 合规 |
| lesson_05_ownership_borrowing.rs | 487 | 10 | 通过 0 warning | 0 | 合规 |
| lesson_06_structs.rs | 776 | 10 | 通过 0 warning | 0 | 合规 |
| lesson_07_enums_pattern_matching.rs | 819 | 9 | 通过 0 warning | 0 | 合规 |
| lesson_08_collections.rs | 857 | 10 | 通过 0 warning | 0 | 合规 |
| lesson_09_packages_modules.rs | 783 | 8 | 通过 0 warning | 0 | 合规 |
| lesson_10_error_handling.rs | 1077 | 10 | 通过 0 warning | 0 | 合规 |
| lesson_11_generics.rs | 776 | 12 | 通过 0 warning | 0 | 合规 |
| lesson_12_traits.rs | 910 | 13 | 通过 0 warning | 0 | 合规 |
| lesson_13_lifetimes.rs | 596 | 11 | 通过 0 warning | 0 | 合规 |
| **基础课程合计（01~13）** | **8832** | **122** | — | — | — |

第五阶段（14~18，见 [课程文件清单](#第五阶段补充与实战)）的 5 门进阶课同样通过上述全部验收：lesson_14（431 行/27 断言）、lesson_15（433 行/42 断言）、lesson_16（333 行/17 断言）、lesson_17（354 行/21 断言）、lesson_18（316 行/22 断言），合计 1867 行、129 条断言全部核对通过。

验收方式与结果：

1. **编译**：18 个文件逐个用 `rustc --edition 2024 --crate-type bin -D warnings`（把警告当错误）编译，全部 0 error / 0 warning；
2. **运行**：18 个可执行文件各连续运行两次，退出码均为 0，两次输出完全一致（无 HashMap 随机顺序等不确定输出）；
3. **格式化**：18 个文件全部通过 `rustfmt --edition 2024 --check`；
4. **内容**：808 条 `// 预期输出：` 断言逐条与紧邻上方的真实输出行**逐字节比对一致**（仅 1 条 `{:p}` 地址因 ASLR 每次不同而合理豁免）。校验脚本见 [.dsh/check_expected_output.ps1](.dsh/check_expected_output.ps1)；
5. **纯度**：无 `#[allow(...)]`、无 `unsafe`、无 `#![feature]`、无第三方依赖。

---

## 六、如何运行

### 运行基础课程（在项目根目录执行）

```bash
cargo run --bin lesson_01_variables_mutability      # 运行第 1 课
cargo run --bin lesson_13_lifetimes                 # 运行第 13 课
cargo build --bins                                  # 一次性编译全部课程
```

> 每个课程文件都是**自包含**的：只依赖标准库，不 `mod` 引入其它课程文件，因此单独拿去 `rustc` 编译也能通过：

不使用 Cargo 时，也可以直接编译单个文件：

```bash
rustc --edition 2024 src/tutorial/lesson_01_variables_mutability.rs -o lesson01
./lesson01          # Windows: .\lesson01.exe
```

> **注意**：lesson_10_error_handling 的示例 7 会以**相对路径** `src/tutorial/lesson_10_error_handling.rs` 读取自身文件，因此该课必须在**项目根目录**下通过 `cargo run` 运行；换目录运行会走到"预期内的读取失败"分支（这是它刻意演示的错误处理，不算 bug）。

### 运行 Web Demo（独立 Cargo 项目）

```bash
cd src/demoweb
cargo run                 # 启动后访问 http://127.0.0.1:3000/health
```

demoweb 的依赖、配置、API 与测试详见 **[src/demoweb/README.md](src/demoweb/README.md)**。

### 运行考核环节（可选但推荐）

```bash
cargo test --test lesson_05_ownership_borrowing      # 考第 5 课（测试目标名 = 课程文件名）
cargo test --no-fail-fast                            # 考全部 18 课（一次拿到完整结果）
cargo run --bin assessment_report                    # 生成学习评估报告
cargo run --bin solution_tool -- --list --lesson 05   # 卡住了？列出第 5 课「练习 → 参考答案行号」
cargo run --bin solution_tool -- --verify --lesson 05 # 应用参考答案 → 跑测试 → 自动还原骨架
```

> 参考答案在 `assessments/solutions/`（单独目录，`cargo test` 不会编译它，正常练习时不会"撞见"）。
> **请先自己写**，实在卡住再看；用法与索引见 [assessments/solutions/README.md](assessments/solutions/README.md)。

Windows 一键脚本：`.\assessments\scripts\run_assessments.ps1`（跑测试 + 出报告）；
macOS/Linux：`./assessments/scripts/run_assessments.sh`。

### 一键校验全部课程（可选）

仓库提供了辅助脚本，用于验证课程文件是否仍然正确（改动示例代码后建议跑一遍）：

```powershell
# 全部课程编译 + 运行 + 期望输出比对
#   由于本机执行策略限制，用下面这种方式加载执行：
$code = [System.IO.File]::ReadAllText("$PWD\.dsh\check_expected_output.ps1", [System.Text.Encoding]::UTF8)
& ([scriptblock]::Create($code))
```

校验脚本做的事：把每个 lesson 编译运行，逐条检查文件里 `// 预期输出：` 声明的值与紧邻上方的真实输出是否**逐字节一致**（这是本课程"预期输出"注释的写法约定，详见 [.dsh/lesson-conventions.md](.dsh/lesson-conventions.md)）。

---

## 七、约定与规范

- 文件名：`lesson_<两位序号>_<英文主题>.rs`，序号即学习顺序，主题名即内容；**二进制名 = 文件名（去掉 .rs）**；
- 每个文件：`//!` 文件头注释（主题 / 学习目标 / 运行方式）+ 唯一 `fn main()` 调度 + 若干 `demo_*` 函数；
- 预期输出统一写成 `println!("// 预期输出：...")` 或紧邻的 `// 预期输出：` 注释，便于边读边核对；
- 常见错误示例均以注释形式给出，并标注编译错误编号（如 `error[E0382]`）与修正方法；
- 代码风格遵循 `rustfmt` 默认配置（4 空格缩进，`snake_case` 函数、`UpperCamelCase` 类型、`SCREAMING_SNAKE_CASE` 常量）。

完整编写规范见 [.dsh/lesson-conventions.md](.dsh/lesson-conventions.md)。

---

## 八、常见问题（FAQ）

**Q1：`cargo run --bin lesson_05_ownership_borrowing` 报 `no bin target named ...`？**
确认你在含 `Cargo.toml` 的**项目根目录**执行，且 `Cargo.toml` 中已注册对应 `[[bin]]`（二进制名与文件名一致）。

**Q2：为什么 18 门课程都有 `fn main()` 却不会冲突？**
因为它们是 18 个**独立的二进制目标**，每个 `[[bin]]` 单独编译链接，互不影响。

**Q3：示例里的错误代码为什么编译不过？**
它们被刻意注释掉了，仅作为反例展示。取消注释即可亲眼看到编译器给出的错误编号。

**Q3-1：`cargo build` 时出现 `warning: linker stderr: corrupt .drectve at end of def file`？**
这**不是代码问题**，而是 Windows + `x86_64-pc-windows-gnu`（MinGW）工具链下链接器对 cargo 传入的 `.drectve` 段给出的噪声提示：

- 本机 `rustc -vV` 显示 `host: x86_64-pc-windows-gnu`，链接器是 MinGW 的 `ld`；
- 用 `rustc` 直接编译**零诊断**，`cargo check --bins` 同样**零诊断**，可执行文件也能正常生成并运行（退出码 0）；
- 连本仓库原有的 `src/main.rs`（`cargo build --bin hello`）也会报同一条警告，可确认与课程代码无关。

如需彻底消除该噪声，可切换到 MSVC 工具链：

```bash
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
```

**Q4：能用 nightly 吗？**
不需要。全部代码在 stable 上验证通过，也刻意避开了实验性特性；edition 使用 2024。

**Q5：想加第三方 crate（如 `serde`）怎么办？**
在 `Cargo.toml` 的 `[dependencies]` 中添加即可。基础课程为了突出语言本身，刻意保持零依赖；想看真实第三方生态的用法，请直接进入 [src/demoweb](src/demoweb/README.md)。

**Q6：lesson_10 运行时打印"读取失败（预期内的失败）"是出错了吗？**
不是。示例 7 刻意演示"读一个不存在的文件 + 读自身文件"两种分支，前者是**预期内的错误演示**；只要最后看到 `成功读到本文件，字节数 = ...` 就说明一切正常。注意在项目根目录运行。
