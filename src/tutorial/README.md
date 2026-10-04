# Rust 系统学习教程（18 课 + 4 篇指南）

本目录是项目**唯一的教程目录**，把「语言基础」与「补充进阶」合并成一条连续的学习主线：

- **18 门课程**：`lesson_01` ~ `lesson_18`，序号即学习顺序，每个文件都是一个可独立编译运行的完整程序；
- **4 篇学习指南**：放在 [`guides/`](guides/) 下，讲学习方法、语言理论、工程实践与工具链，按需查阅。

> 历史上教程分为 `src/tutorial`（01~13）与 `src/tutorial`（guides + 14~18）两处，现已合并到本目录，
> 序号 01~18 保持连续，课程内容与规范完全不变。

- 依赖：**仅标准库**，无任何第三方 crate（含 practice 课程）；
- 版本：Rust 1.99 stable / edition 2024；
- 命名：`lesson_<两位序号>_<英文主题>.rs`，二进制名与文件名一致；
- 每课都包含：文件头学习目标 → 若干 `demo_*` 示例（带详细行内注释与 `// 预期输出：` 断言）→ 典型场景 → 常见错误对照（注释给出编译错误编号与修正方法）。

> 项目总览与完整学习路径见[根 README](../../README.md)。

---

## 一、快速开始

```bash
# 在项目根目录（含 Cargo.toml）执行
cargo run --bin lesson_01_variables_mutability   # 运行第 1 课
cargo run --bin lesson_18_macros                 # 运行第 18 课
cargo build --bins                               # 一次性编译全部 18 门课程
cargo check                                      # 只做类型检查，最快
```

也可以不经过 Cargo 直接编译单文件（每课都是自包含的）：

```bash
rustc --edition 2024 src/tutorial/lesson_05_ownership_borrowing.rs -o lesson05
./lesson05        # Windows: .\lesson05.exe
```

> 注意：lesson_10 的示例 7 会以相对路径读取自身文件，请务必在**项目根目录**运行该课。

---

## 二、课程目录（按学习顺序）

### 第一阶段：语法基础

| 课 | 文件 / 运行命令 | 主题 | 学完你能做什么 |
| --- | --- | --- | --- |
| 01 | [lesson_01_variables_mutability.rs](lesson_01_variables_mutability.rs)<br>`cargo run --bin lesson_01_variables_mutability` | 变量与可变性 | 分清 `let` / `mut` / 遮蔽 / `const` / `static`，理解 Rust"默认不可变"的设计意图 |
| 02 | [lesson_02_data_types.rs](lesson_02_data_types.rs)<br>`cargo run --bin lesson_02_data_types` | 数据类型 | 掌握标量与复合类型、溢出行为、`as` / `From` / `TryFrom` 转换的取舍 |
| 03 | [lesson_03_functions.rs](lesson_03_functions.rs)<br>`cargo run --bin lesson_03_functions` | 函数 | 分清语句与表达式（Rust 函数体的本质），会拆分函数、使用函数指针 |
| 04 | [lesson_04_control_flow.rs](lesson_04_control_flow.rs)<br>`cargo run --bin lesson_04_control_flow` | 流程控制 | 会用"if 是表达式""loop 带返回值"等 Rust 特有的控制流写法 |

### 第二阶段：Rust 的灵魂（分水岭，务必学透）

| 课 | 文件 / 运行命令 | 主题 | 学完你能做什么 |
| --- | --- | --- | --- |
| 05 | [lesson_05_ownership_borrowing.rs](lesson_05_ownership_borrowing.rs)<br>`cargo run --bin lesson_05_ownership_borrowing` | 所有权系统 | 解释所有权三规则与借用检查，看懂 `cannot borrow as mutable` / `borrow of moved value` 等报错 |
| 06 | [lesson_06_structs.rs](lesson_06_structs.rs)<br>`cargo run --bin lesson_06_structs` | 结构体 | 用结构体建模数据，掌握方法、关联函数、更新语法与 `Debug`/`Display` |
| 07 | [lesson_07_enums_pattern_matching.rs](lesson_07_enums_pattern_matching.rs)<br>`cargo run --bin lesson_07_enums_pattern_matching` | 枚举与模式匹配 | 用携带数据的枚举 + `match` 建模状态机，理解 `Option`/`Result` 的本质 |

### 第三阶段：工程化能力

| 课 | 文件 / 运行命令 | 主题 | 学完你能做什么 |
| --- | --- | --- | --- |
| 08 | [lesson_08_collections.rs](lesson_08_collections.rs)<br>`cargo run --bin lesson_08_collections` | 常见集合 | 熟练使用 `Vec` / `String` / `HashMap`，避开集合操作中的所有权与借用陷阱 |
| 09 | [lesson_09_packages_modules.rs](lesson_09_packages_modules.rs)<br>`cargo run --bin lesson_09_packages_modules` | 包和模块 | 用模块组织代码、控制可见性、划分 crate，看懂主流项目的目录结构 |
| 10 | [lesson_10_error_handling.rs](lesson_10_error_handling.rs)<br>`cargo run --bin lesson_10_error_handling` | 错误处理 | 分清 panic 与 `Result` 的适用场景，会用 `?`、自定义错误类型与 `Box<dyn Error>` |

### 第四阶段：抽象与进阶

| 课 | 文件 / 运行命令 | 主题 | 学完你能做什么 |
| --- | --- | --- | --- |
| 11 | [lesson_11_generics.rs](lesson_11_generics.rs)<br>`cargo run --bin lesson_11_generics` | 泛型 | 写泛型函数/结构体/枚举，理解单态化"零运行期开销"的原理 |
| 12 | [lesson_12_traits.rs](lesson_12_traits.rs)<br>`cargo run --bin lesson_12_traits` | Trait | 定义与实现 trait，分清静态分发与 `dyn` 动态分发，读懂标准库 trait 签名 |
| 13 | [lesson_13_lifetimes.rs](lesson_13_lifetimes.rs)<br>`cargo run --bin lesson_13_lifetimes` | 生命周期 | 看懂并写出含生命周期注解的签名，掌握省略规则与 `'static` 两种含义 |

### 第五阶段：补充与实战（衔接 Web 项目）

| 课 | 文件 / 运行命令 | 主题 | 为什么值得学（与项目的结合点） | 推荐时机 |
| --- | --- | --- | --- | --- |
| 14 | [lesson_14_closures.rs](lesson_14_closures.rs)<br>`cargo run --bin lesson_14_closures` | 闭包与 Fn/FnMut/FnOnce | 迭代器、线程、回调的语法基础；demoweb 的 `spawn_blocking(move \|\| ...)` 用到 move 闭包 | lesson 05~10 之后 |
| 15 | [lesson_15_iterators.rs](lesson_15_iterators.rs)<br>`cargo run --bin lesson_15_iterators` | 迭代器与惰性求值 | Rust 惯用写法的支柱；自定义 Iterator、零成本抽象；词频统计场景是 demoweb 业务代码的最小版 | 紧随 lesson_14 |
| 16 | [lesson_16_smart_pointers.rs](lesson_16_smart_pointers.rs)<br>`cargo run --bin lesson_16_smart_pointers` | Box / Rc / RefCell / Weak | 递归类型、共享所有权、内部可变性、循环引用——demoweb 的 `Arc<AppState>` 前置知识 | lesson 05~13 之后 |
| 17 | [lesson_17_threads_channels.rs](lesson_17_threads_channels.rs)<br>`cargo run --bin lesson_17_threads_channels` | 线程 / mpsc / Arc&lt;Mutex&gt; / Send+Sync | 标准库并发的完整拼图；与 demoweb 的 async 并发互补 | lesson_16 之后 |
| 18 | [lesson_18_macros.rs](lesson_18_macros.rs)<br>`cargo run --bin lesson_18_macros` | 声明宏 macro_rules! | `println!`/`vec!` 背后的机制；ensure! 消除校验样板的实战模式 | 任意时刻（不依赖其余补充课） |

---

## 三、理论文档（guides/，按需查阅的手册）

| 文档 | 内容 | 什么时候读 |
| --- | --- | --- |
| [01_learning_method.md](guides/01_learning_method.md) | 四步学习循环、编译错误阅读法（高频 E 编号速查）、三条路线图、自检清单、常见误区 | **最开始**读一遍；卡住时回来查错误编号表 |
| [02_language_theory.md](guides/02_language_theory.md) | 所有权决策图、静态/动态分发成本、Deref 链、Send/Sync、`'static` 两种含义、内部可变性边界、unsafe 概念课、async 执行模型 | 对应课程学完后精读；unsafe/async 两节在进入 demoweb 前必读 |
| [03_engineering_practices.md](guides/03_engineering_practices.md) | workspace、feature、错误处理库选型、测试三层、rustdoc、newtype/Builder/类型状态、性能习惯、版本纪律、CI 配置 | 写自己的第一个项目之前通读 |
| [04_toolchain_guide.md](guides/04_toolchain_guide.md) | rust-analyzer 配置、clippy 分组、cargo 子命令地图、依赖治理、调试手段、miri/loom | 装好环境后先看前两节，其余当工具书 |

---

## 四、每课怎么学（推荐流程）

1. **先读文件头注释**：每课开头都列出了本课的学习目标，先带着问题读代码；
2. **先猜后跑**：运行前先猜每个示例的输出，再与程序打印的 `// 预期输出：` 断言对照；
3. **主动制造错误**：每课的"常见错误示例"都被注释掉了，取消注释让编译器报错，记住报错编号（如 `error[E0382]`）——以后遇到能秒懂；
4. **动手改**：改类型、改常量、改分支顺序，观察编译器反馈；
5. **收尾检查**：改完用 `cargo clippy --bins` 和 `rustfmt --edition 2024 --check` 自查。

---

## 五、推荐学习顺序

```
第一阶段 01~04（语法基础）
   └► 第二阶段 05~07（所有权/结构体/枚举——务必学透）
        └► 第三阶段 08~10（集合/模块/错误处理）
             └► 第四阶段 11~13（泛型/Trait/生命周期）
                  ├► 14 闭包 ─► 15 迭代器        （语法基础的自然延伸）
                  ├► 16 智能指针 ─► 17 线程与通道 （消化所有权/生命周期/泛型后最顺）
                  └► 18 宏                        （独立，随时可插）
全部学完
   └► guides/02 的 unsafe 与 async 两节 ─► src/demoweb
```

若时间有限，先按 `05 → 07 → 10 → 12 → 14 → 15 → 16 → 17` 精学（见 [guides/01](guides/01_learning_method.md)），其余课程快读补漏。

---

## 六、注意事项

- 每课都是**自包含**的：只依赖标准库、不引用其它课程文件，可单独编译；跨课交叉引用只出现在注释里（如"见 lesson_10_error_handling"）；
- `// 预期输出：` 是**字面量断言**：期望值写死且与真实输出逐字节一致，配套校验脚本 [.dsh/check_expected_output.ps1](../../.dsh/check_expected_output.ps1) 可一键全量核对（当前 18 门课程共 **808 条**断言）；
- 错误示例代码全部**注释掉**了，编译错误信息写在注释中；不要为"让代码编译"而直接删除这些注释；
- 线程课（lesson_17）的输出经过**确定性设计**（join/通道收集后排序打印）——多线程程序的输出顺序本身不确定，这是刻意教学点，见该课示例 4 的注释；
- 理论文档中的工程实践（workspace/CI/第三方库）为**方法论指引**，未在本仓库实际执行，动手时以对应官方文档为准；
- 课程文件遵循统一编写规范（见 [.dsh/lesson-conventions.md](../../.dsh/lesson-conventions.md)），新增课程请照此执行，并在根 `Cargo.toml` 注册 `[[bin]]`；
- 本目录与 demoweb 的交叉引用全部使用相对路径链接，移动文件时请同步更新。

---

## 七、自测考核（可选但强烈推荐）

每课都配了一个**与课程文件同名**的考核文件，用来检验你是不是真的掌握了：

```bash
cargo test --test lesson_05_ownership_borrowing     # 考第 5 课
cargo test --no-fail-fast                           # 考全部 18 课
cargo run --bin assessment_report                   # 生成学习评估报告（总分/薄弱环节/进度对比）
```

- 考核文件在 `assessments/` 下，与本目录的 18 门课程**一一对应**（文件名相同）；
- 每个知识点是一个 `#[test]`，里面写着期望值与**边界用例**；你要实现的是同名的
  `exercise_*` 练习函数（骨架里只有一行 `todo_exercise(...)` 占位），每个练习的注释里都有
  「实现要求 + 示例输入/示例输出」；
- **每题都配了参考答案**，放在 `assessments/solutions/` 下（单独目录，`cargo test` 不会编译它，
  正常练习时不会"撞见"）：**先自己写，实在卡住再看**，看完关掉答案自己重写一遍。
  想对照答案或验证答案真能全绿：
  ```bash
  cargo run --bin solution_tool -- --list   --lesson 05   # 列出「练习 → 答案在第几行」
  cargo run --bin solution_tool -- --verify --lesson 05   # 应用答案 → 跑测试 → 自动还原骨架
  ```
- 失败时输出会区分【未实现】/【未通过】/运行期错误，并给出「期望 / 实际 / 提示 / 改进建议 / 复习入口」；
- 并发课（lesson_17）的考核同样做了**确定性设计**：结果一律通过 `join()` / 通道收回后才断言；
- 完全可选：不做考核不影响课程本身的运行与阅读。

上手方式、评分口径与常见问题见 **[assessments/README.md](../../assessments/README.md)**，
各知识点与本文档条目的对应关系见 **[知识点覆盖矩阵](../../assessments/docs/04_knowledge_map.md)**。

---

## 八、下一步

过完 18 课后，进入 **[src/demoweb](../demoweb/README.md)**：一个 Axum + Tokio + SQLx 的真实 Web 服务，把这里学到的所有权、错误处理、Trait、泛型与并发知识全部用上。
