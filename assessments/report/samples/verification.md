# 考核用例验收记录（可解性验证）

考核用例必须**可解**：每一条断言都得有人真的用正确实现跑通过。
本文件记录 18 个考核文件的「骨架 → 参考实现 → 骨架」往返验证结果。

- 验证方式：备份骨架 → 把该文件里所有 `assessment_harness::todo_exercise(...)` 占位替换为参考实现
  → 运行测试目标，期望 `test result: ok`
  → 修正过程中发现的**考核本身缺陷**（期望值写错、用例不可满足等）
  → 还原骨架并复验（重新全部报【未实现】、编译零 warning）
- 验证环境：Windows / PowerShell / `rustc 1.99.0` / edition 2024；每次验证的账本都写到临时目录，
  不污染学员进度（`report/data/`）
- 结果汇总：**18 个考核文件、157 个知识点，参考实现下全部通过（157/157）**；
  骨架态全部零 warning、且每个知识点都明确报【未实现】（没有"假通过"）

| 考核文件 | 知识点 | 参考实现通过 | 过程中修正的考核缺陷 / 特殊处理 |
| --- | --- | --- | --- |
| lesson_01_variables_mutability | 8 | 8/8 | 范例文件，由本环节设计者按同一协议验证 |
| lesson_02_data_types | 8 | 8/8 | 修正 kp_02_02 一处**断言极性与实现要求相反**的缺陷：`#[test]` 原本写 `is_false(overflow_checked, "…应为 false")`，而同一题的实现要求写 `checked_add(10).is_none()`、示例输出写 `(4, 255, 4, true)`——`is_none()` 恒为 `true`，两者不可能同时满足；已把断言改为 `is_true(...)`（消息同步改为「应为 true」）并在 `use` 里补上 `is_true`。另：kp_02_03 断言值改用 `0.0 / 0.0` 以避免学员侧 `invalid_nan_comparisons` 警告 |
| lesson_03_functions | 8 | 8/8 | kp_03_06（发散函数）加防呆：骨架态占位自身的 panic 不算通过 |
| lesson_04_control_flow | 8 | 8/8 | kp_04_02 要求显式处理 `>100`（否则 101 会被判成 A）；kp_04_08 取「前三个**带编号**的错误」（课程错误 2 是运行期死循环、无编号） |
| lesson_05_ownership_borrowing | 9 | 9/9 | kp_05_02 边界从 `i32::MIN` 改为 `1_073_741_823`（原值两倍必然溢出，用例不可解） |
| lesson_06_structs | 8 | 8/8 | kp_06_02 返回值扩为 4 元组让 `retries` 被读到（否则学员实现完会看到 `field is never read`）；kp_06_05 在实现要求里说明单元结构体的值不是 `()` |
| lesson_07_enums_pattern_matching | 9 | 9/9 | kp_07_05 第二个匹配分支去掉 `@` 绑定（否则学员实现完会看到 `unused variable` 警告）；另把顶层 `use` 列表里的 `todo_exercise` 去掉——练习体本来就是 `assessment_harness::todo_exercise(...)` 全限定调用，导入它会让「实现完的学员」看到 `unused import` 警告 |
| lesson_08_collections | 9 | 9/9 | kp_08_05 的 `+` 分支用遮蔽写法，避免 `unused_mut`；返回集合的题一律先排序（不依赖 HashMap 迭代顺序） |
| lesson_09_packages_modules | 7 | 7/7 | kp_09_05 期望值修正为 20（`"lesson_09"` 长度是 9）；函数内 `mod` 之间的 `super::` 指向 crate 根，需包一层外层模块（已写进实现要求） |
| lesson_10_error_handling | 9 | 9/9 | kp_10_04 明确要求 `.parse::<i32>()`（不写 turbofish 会报 E0282）；`AppError::Overflow` 在测试里显式构造一次，兼顾 dead_code 与变体断言 |
| lesson_11_generics | 10 | 10/10 | kp_11_01/11_06 用 `std::ptr::eq` 精确校验「相等元素取最先出现的」 |
| lesson_12_traits | 11 | 11/11 | kp_12_05（`-> impl Area`）用内层 `fn placeholder(..) -> Square` 钉住隐藏类型（`!` 不实现 trait）；kp_12_08 保留 `type Item = u32`（缺了 impl 不完整） |
| lesson_13_lifetimes | 9 | 9/9 | kp_13_08 三个场景在课程注释里都是 E0106，故同号；诊断题一律以课程注释为准 |
| lesson_14_closures | 9 | 9/9 | kp_14_06（`-> impl Fn`）占位写成 `move \|x: i32\| -> i32 { todo_exercise(...) }`（`!` 不实现 `Fn`）；kp_14_03 的 `mut f` 通过传 `&mut f` 占位避免 `unused_mut` |
| lesson_15_iterators | 9 | 9/9 | 修正 kp_15_02 负数边界的期望值；kp_15_06 用 `Cell` 计数证明适配器惰性 |
| lesson_16_smart_pointers | 9 | 9/9 | kp_16_05（`RefCell` 借用重叠必 panic）改用 `panics_real`，否则骨架态占位的 panic 会被误判成通过；`List`/`Node` 配真实实现的辅助函数以避免 dead_code |
| lesson_17_threads_channels | 9 | 9/9 | 并发用例全部「先 join / 经通道收回，再在主线程断言」，无 sleep、无顺序假设；kp_17_09 按课程注释取前三条错误条目（第一条是运行期问题、无编译编号） |
| lesson_18_macros | 8 | 8/8 | 宏与 `use` 都写在练习函数体内；kp_18_05 的 `#[test]` 不带参数（libtest 限制），在闭包内以 1 / 0 / -7 覆盖正常与边界 |
| **合计** | **157** | **157/157** | 另：18 个文件骨架态均为**零 warning**、全部知识点报【未实现】 |

## 骨架态复验（自动化）

维护者可以随时用脚本重跑「骨架是否干净 + 是否零警告编译」：

```powershell
# 由于本机执行策略限制，用 scriptblock 方式加载执行
$code = [System.IO.File]::ReadAllText("$PWD\assessments\scripts\check_assessments.ps1", [System.Text.Encoding]::UTF8)
& ([scriptblock]::Create($code))
```

脚本对 18 个目标依次：`--no-run` 编译（过滤掉 MinGW 的 `corrupt .drectve` 噪声）→
在**临时数据目录**里运行 → 校验「【未实现】条数 == 知识点条数」（这一条能抓住
panics 类用例的"骨架态假通过"）。验收时的结果：**18/18 通过，知识点合计 157 个**。

> 本次交付前发现并修好了这个脚本自身的一个**偶发漏计**问题：它原先用 PowerShell 管道
> （`& cargo test … 2>&1 | Out-String`）捕获输出，在多线程测试输出较大时 PS 5.1 会丢行，
> 曾把 lesson_16 的 9 条【未实现】数成 6 条。现在改为「子进程输出重定向到日志文件 +
> `--test-threads=1` 串行运行」，计数稳定（已重复跑通）。

## 目录合并（架构整理）后的同步复验

`src/lessons/`（01~13）与 `src/study/`（guides + 14~18）已合并为统一的 **`src/tutorial/`**：
18 门课程序号 01~18 连续，4 篇学习指南移入 `src/tutorial/guides/`。整理后重做的验证：

| 项目 | 结果 |
| --- | --- |
| 引用同步 | 考核文件头部「对应课程 / 知识点出处」、`assessment_harness` 模块登记表、知识点矩阵（`docs/04_knowledge_map.md`）均已指向 `src/tutorial/...` |
| 骨架态全量运行 | `cargo test --no-fail-fast`：18/18 目标编译通过并逐一报【未实现】，157 个知识点齐全 |
| 课程「预期输出」断言 | [.dsh/check_expected_output.ps1](../../../.dsh/check_expected_output.ps1) 重跑通过：**808 条断言、0 未命中**（1 条地址类豁免） |
| 18 门课程二进制 | `cargo build --bins` 零 error（仅 MinGW `.drectve` 链接器噪声与 lesson_18 原有的 `unused_mut` 提示） |
| 本目录样例报告 | 报告里的课程路径字段已同步为新路径（其余内容仍是当时的真实账本输出） |

## 注释优化（去掉「提示 / 难度」，补 ACM 题式示例）

- 18 个考核文件里的 `/// 提示：…`、`/// 难度：★…` 注释与分节标题里的难度标签已全部删除
  （难度分级仍由 `assess(...)` 的 `Kind` 决定，报告与知识点矩阵照旧按四类统计）；
- 原「提示」中属于**完成该练习所必需**的信息（签名约束、必须使用的 API、易踩的编译错误编号等）
  已改写为陈述句并入「实现要求」，信息没有丢失；
- 每个练习函数的「实现要求」之后新增一组 ACM 题式的 `示例输入：` / `示例输出：`
  （共 **182 组**，逐个取自该练习对应 `#[test]` 里被断言的**典型用例**，无参数练习写 `（无参数）`）；
- 变更**只涉及注释**：逐文件比对「非注释代码行」与优化前逐字一致，`#[test]` 数、
  `todo_exercise` 占位数、`assess(...)` 参数与 `eq(...)` 期望值均未变；
  `rustfmt --edition 2024 --check` 对 18 个考核文件 + 框架 + 报告工具共 20 个文件全部通过。

## 参考答案（`assessments/solutions/`）与第三次全量验证

为了让每个练习都有可对照的答案，同时把「考核用例是否可解」变成**随时可重跑的一条命令**，
本次新增了参考答案体系：

- `assessments/solutions/lesson_XX_<主题>.rs`：18 个文件，与考核文件**同名**、是它们的完整副本，
  只有练习函数体换成了真实实现（共 **182 个练习**）；
- `assessments/bin/solution_tool.rs`：`--check`（校验答案没动考核标准）/ `--list` / `--status` /
  `--apply` / `--restore` / `--verify`（应用 → cargo test → 自动还原骨架）；
- 参考答案**不会被 `cargo test` 编译**（`autotests = false` + 显式 `[[test]]` 注册），
  已用一个故意写坏的一次性探针文件实测确认：`cargo test --no-run` 完全不受影响（探针已删除）。

**全量验证结果（本次交付前实测）**：

| 项目 | 结果 |
| --- | --- |
| `solution_tool --check` | **18/18 通过**：参考答案与考核文件在「练习函数体之外」逐字节一致 |
| `solution_tool --verify` | **18/18 全绿：`157 passed / 0 failed`**（每个目标都是「应用 → 跑测试 → 自动还原」） |
| 还原后一致性 | 18 个考核文件与 `solutions/.backup/` 里的骨架**逐字节一致**，且仍逐一报【未实现】 |
| 骨架态自检 | `check_assessments.ps1`：18/18 编译零警告 + 骨架干净（157 个知识点全部【未实现】） |
| 格式 | 考核/工具 20 个文件 + 参考答案 18 个文件全部通过 `rustfmt --edition 2024 --check` |

**本轮修正的考核缺陷**（同时记在上表的「过程中修正的考核缺陷」列）：

1. `lesson_02` kp_02_02 的 `#[test]` 断言极性与该题「实现要求 / 示例输出」相反
   （`is_false(…应为 false)` vs `checked_add(10).is_none()` 恒为 `true`）——断言改为 `is_true(…)`，
   并在 `use` 里补上 `is_true`；
2. `lesson_07` / `lesson_10` 顶层 `use` 列表里导入了 `todo_exercise`（练习体本来就是全限定调用）
   ——学员把练习全部实现后会出现 `unused import` 警告，已从 `use` 列表移除。

> 这两个问题都是「参考答案让人真的把练习逐个实现了一遍」才暴露的：一次性的人工验证容易漏掉
> 「实现完之后是否还干净」，而 `solution_tool --verify` 可以反复重跑。

## 参考实现往返验证的完整协议

见 [../docs/05_maintainer_guide.md](../../docs/05_maintainer_guide.md) 第八节
（含备份骨架、隔离账本、还原与复验的命令，以及把多次验证合并成一份"全通过"样例报告的做法）。
