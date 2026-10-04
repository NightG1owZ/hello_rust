# 考核环节（assessments）—— 用测试检验你对每一课的掌握程度

> **这是可选但强烈推荐的环节。** `src/` 下的 18 门课教你知识，`assessments/` 下的 18 个考核文件
> 用「测试驱动」的方式检验你是不是真的掌握了——不强制，但做完你会清楚知道**哪一课学扎实了、
> 哪一课只是看懂了**。

- 考核文件与课程**一一对应**：`assessments/lesson_05_ownership_borrowing.rs` ↔ `src/tutorial/lesson_05_ownership_borrowing.rs`
- 测试目标名 = 课程文件名（去掉 `.rs`）：`cargo test --test lesson_05_ownership_borrowing`
- 测试内容与各模块 README 标注的**核心知识点逐条对应**，完整映射表见 [docs/04_knowledge_map.md](docs/04_knowledge_map.md)
- **零第三方依赖**（只用 Rust 标准库 + 本项目自带的考核框架），完全离线可用
- 考核**不修改**任何课程文件：`src/` 保持原样，考核代码全部在 `assessments/` 下

---

## 一、30 秒上手

```bash
# 0) 前提：已装好 Rust（见 docs/01_environment_setup.md）

# 1) 挑一课开始（例：第 5 课 所有权系统）
cargo test --test lesson_05_ownership_borrowing
#    骨架态会看到一堆 FAILED，并在输出里明确写着【未实现】——这就是你的任务清单

# 2) 打开对应考核文件，实现里面所有 exercise_* 练习函数
#    （删掉函数体里的 todo_exercise(...) 那一行，按“实现要求”写你的代码）

# 3) 反复运行，直到变绿
cargo test --test lesson_05_ownership_borrowing

# 4) 生成学习评估报告：知识点掌握情况 + 薄弱环节 + 改进建议 + 学习进度
cargo run --bin assessment_report
#    报告文件：assessments/report/assessment_report.md
```

> **卡住了怎么办**：每一题都配了参考答案（`assessments/solutions/lesson_XX_<主题>.rs`，
> 索引与用法见 [solutions/README.md](solutions/README.md)）。
> **请先自己写**——实在过不去再看答案，而且看完要关掉答案自己重写一遍。
> 想确认答案真能让这一课全绿：
> `cargo run --bin solution_tool -- --verify --lesson 05`（应用答案 → 跑测试 → 自动还原骨架）。

一次考全部 18 课（会先编译 18 个课程二进制，第一次稍慢）：

```bash
cargo test --no-fail-fast
cargo run --bin assessment_report
```

Windows 用户可以直接用脚本（自动设置运行 ID、跑测试、出报告）：

```powershell
.\assessments\scripts\run_assessments.ps1              # 全部 18 课
.\assessments\scripts\run_assessments.ps1 -Lesson 05   # 只考第 5 课
```

---

## 二、这个环节是怎么设计的

### 2.1 每个知识点 = 一个 `#[test]` + 一个待实现的练习函数

以第 5 课为例（节选自 `assessments/lesson_05_ownership_borrowing.rs`）：

```rust
/// 知识点考核：返回最长单词的切片
#[test]
fn kp_05_05_longest_word() {
    assess(M, "kp_05_05", "切片 &str：返回最长单词", Kind::Core, "复习 lesson_05 示例 8/9 ……", || {
        eq(exercise_05_05_longest_word("hello rustacean"), "rustacean", "长度优先，不是字典序");
        eq(exercise_05_05_longest_word(""), "", "空串必须返回空串，不能 panic");   // 边界用例
    });
}

/// 【待实现】返回输入中最长的单词（切片）
///
/// 实现要求：……
///
/// 示例输入：
/// ```text
/// text = "hello rustacean"
/// ```
/// 示例输出：
/// ```text
/// "rustacean"
/// ```
fn exercise_05_05_longest_word(text: &str) -> &str {
    todo_exercise("exercise_05_05_longest_word", "返回最长单词的切片，并列取第一个", (text,))
}
```

- **测试函数（`kp_*`）是考核标准**，由本环节提供，你**不要修改**；
- **练习函数（`exercise_*`）是需要你实现的**：删掉 `todo_exercise(...)` 那一行，写你的实现；
- 每个知识点都包含**至少一个边界用例**（空输入、0/1 个元素、类型极值、越界、负数、多字节字符……），
  这是本考核体系刻意设计的部分：语言细节往往就藏在边界上。

### 2.2 知识点分四类（报告里按类统计掌握率）

| 类型 | 含义 | 报告里的建议口径 |
| --- | --- | --- |
| 基础 `Basic` | 本课最先要求掌握的概念 | 重跑该课 demo 1~4，把示例输出逐个猜对 |
| 核心 `Core` | README 表格里的核心知识点 | 回 README 对应条目精读，先写伪代码 |
| 难点 `Hard` | 新人最容易卡住的地方 | 允许对照示例改写，但要能讲清「为什么必须这样写」 |
| 边界 `Edge` | 极值、空输入、类型细节 | 检查空输入/极值/整除与负数/UTF-8 |

### 2.3 三类失败会被区分开（这是本环节的关键）

| 报告里的状态 | 含义 | 你要做的事 |
| --- | --- | --- |
| **通过** | 该知识点所有断言都过了 | 继续下一个 |
| **未实现** | 练习函数还是 `todo_exercise(...)` 占位 | 去实现它（报告会告诉你是哪个函数、在哪一行） |
| **未通过** | 实现了，但结果不符合预期 | 报告给出「期望 / 实际 / 提示 / 改进建议 / 复习入口」 |
| **运行期错误** | 实现里发生了 panic（越界、unwrap 失败等） | 报告附原始 panic 信息 |
| **未运行或编译失败** | 该课的考核目标根本没跑起来 | 先按编译器报错修好语法/类型错误 |

---

## 三、目录结构

```
assessments/
├── README.md                     # 本文件
├── harness/lib.rs                # 考核框架（库 crate assessment_harness，零第三方依赖）
├── lesson_01_variables_mutability.rs
├── lesson_02_data_types.rs
├── ...
├── lesson_13_lifetimes.rs        # ↑ 与 src/tutorial/ 的 01~13 课一一对应
├── lesson_14_closures.rs
├── ...
├── lesson_18_macros.rs           # ↑ 与 src/tutorial/ 的 14~18 课一一对应
├── bin/assessment_report.rs      # 学习评估报告生成器
├── bin/solution_tool.rs          # 参考答案工具箱（校验 / 应用 / 还原 / 验证）
├── solutions/                    # ★ 参考答案：每课一个文件，与考核文件同名（182 个练习）
│   ├── README.md                 #   索引 + 用法 + 「先自己写，再看答案」
│   └── lesson_01..18_*.rs        #   完整副本：只把练习函数体换成真实实现
├── scripts/
│   ├── run_assessments.ps1       # Windows：一键跑考核 + 出报告
│   ├── run_assessments.sh        # macOS / Linux：同上
│   └── check_assessments.ps1     # 维护者自检：骨架是否干净 + 是否零警告编译
├── report/
│   ├── assessment_report.md      # 最近一次的学习评估报告（人读）
│   ├── summary.json              # 机器可读摘要
│   ├── data/                     # 账本与历史记录（可随时删除，只影响进度对比）
│   └── samples/                  # 样例报告 + 验收记录（对照着看"报告应该长什么样"）
└── docs/
    ├── 01_environment_setup.md   # 测试环境配置指南（含离线说明与排错）
    ├── 02_how_to_answer.md       # 答题指南：怎么写、怎么自测、常见误区
    ├── 03_reading_reports.md     # 报告解读：状态、评分口径、怎么定位薄弱点
    ├── 04_knowledge_map.md       # 知识点覆盖矩阵（考核 ↔ 课程 README，自动生成）
    └── 05_maintainer_guide.md    # 维护者指南（框架 API、如何新增考核模块、验证协议）
```

---

## 四、知识点覆盖（与各模块 README 的对应关系）

考核内容不是自由发挥的，而是**逐条对齐**三个 README 里标注的学习内容：

| 考核范围 | 知识点出处 | 覆盖的核心概念 |
| --- | --- | --- |
| lesson_01 ~ 04 | `src/tutorial/README.md` 第一阶段 | 变量与可变性、数据类型（含溢出/转换）、函数（语句与表达式、函数指针、发散函数）、流程控制（if 表达式、三种循环、break 带值、循环标签） |
| lesson_05 ~ 07 | `src/tutorial/README.md` 第二阶段 | **所有权系统**（move/Copy/Clone、借用规则、可变引用唯一性、NLL、切片、Drop 顺序）、结构体（方法/关联函数/更新语法/Debug 与 Display）、枚举与模式匹配（Option/Result、match 穷尽、守卫与 `@`、if let/while let/let else、状态机解析） |
| lesson_08 ~ 10 | `src/tutorial/README.md` 第三阶段 | Vec/String/HashMap（含 `entry` API 与借用陷阱、UTF-8 字节与字符）、包与模块（`pub` 层级、`use`/`as`、`pub use`、三种路径）、**错误处理**（panic 与 Result、`?`、自定义错误类型、`From` 转换、`Box<dyn Error>`、配置解析） |
| lesson_11 ~ 13 | `src/tutorial/README.md` 第四阶段 | 泛型（函数/结构体/枚举/方法、trait bound 与 where、多参数、const 泛型、单态化）、Trait（默认方法、`impl Trait` 与泛型参数、返回 `impl Trait`、`Box<dyn Trait>`、关联类型、派生与手写）、生命周期（函数注解、多参数、结构体持有引用、省略规则、`'static`、与 trait bound 组合） |
| lesson_14 ~ 18 | `src/tutorial/README.md` 第五阶段 | 闭包（Fn/FnMut/FnOnce、`move`、作为参数与返回值、`Box<dyn Fn>`）、迭代器（三种迭代模式、适配器与消费器、惰性求值、自定义 Iterator、词频统计）、智能指针（Box/Rc/RefCell/Weak、内部可变性与 panic 边界）、**并发**（spawn/join、`move` 所有权、mpsc 多生产者、`Arc<Mutex>`、Send/Sync、scoped threads、并行求和）、声明宏（片段说明符、重复模式、`stringify!`/`concat!`、`ensure!`、DSL、卫生性） |

完整的「知识点 ↔ 课程 demo ↔ README 出处」矩阵由报告生成器产出：

```bash
cargo run --bin assessment_report -- --emit-map     # 重新生成 docs/04_knowledge_map.md
cargo run --bin assessment_report -- --check-map    # 校验矩阵与考核代码是否一致
```

---

## 五、评分与学习评估报告

```bash
cargo run --bin assessment_report                  # 分析最近一次运行
cargo run --bin assessment_report -- --list        # 列出历史运行
cargo run --bin assessment_report -- --run <id>    # 分析指定运行
```

- **总分** = 通过的知识点数 ÷ 应掌握的知识点总数 × 100；
- **等级**：≥90 优秀 / ≥75 良好 / ≥60 合格 / <60 待加强；
- 报告还会给出：**按模块得分表**、**四类知识点的掌握率**（看「难点」「边界」是否薄弱）、
  **逐条明细**（期望/实际/提示/位置/复习入口）、**需要加强的领域**（Top 5 + 具体建议）、
  **学习进度对比**（与上一次运行相比，哪些知识点新通过、哪些退步）、**下一步行动清单**。
- 详细解读见 [docs/03_reading_reports.md](docs/03_reading_reports.md)；
  想看「报告应该长什么样」，直接看 [report/samples/](report/samples/README.md) 里的样例报告与
  [验收记录](report/samples/verification.md)（每个考核文件在参考实现下的通过数）。

---

## 六、自动化脚本

```powershell
# Windows（PowerShell）
.\assessments\scripts\run_assessments.ps1                  # 全部课程
.\assessments\scripts\run_assessments.ps1 -Lesson 05       # 只跑第 5 课（支持 05 或 lesson_05_ownership_borrowing）
.\assessments\scripts\run_assessments.ps1 -Open            # 跑完直接打开报告
```

```bash
# macOS / Linux
./assessments/scripts/run_assessments.sh
./assessments/scripts/run_assessments.sh 05
```

脚本做的事：设置本次运行 ID → 运行 `cargo test`（`--no-fail-fast`，保证一次拿到完整结果）
→ 调用报告生成器 → 打印总分与薄弱环节摘要。

除了一键脚本，还有两个 `[[bin]]` 工具：

```bash
cargo run --bin assessment_report -- --list            # 列出账本里已登记的模块与知识点
cargo run --bin solution_tool     -- --list   --lesson 05   # 练习 → 参考答案在第几行
cargo run --bin solution_tool     -- --verify --lesson 05   # 应用参考答案 → 跑测试 → 自动还原骨架
cargo run --bin solution_tool     -- --status              # 看每个考核文件是「骨架」还是「已应用」
```

参考答案放在 `assessments/solutions/`，用法、"先自己写再看"的规矩与可解性验证见
**[solutions/README.md](solutions/README.md)**。

> 如果 PowerShell 因执行策略不允许直接运行脚本，用根 README 第六节给出的
> `[scriptblock]::Create((Get-Content -Raw ...))` 方式加载，或改用 `-ExecutionPolicy Bypass`。

---

## 七、常见问题

**Q1：不完成考核会影响我学课程吗？**
不会。考核是**独立**的附加环节，`cargo run --bin lesson_XX_...` 与 `cargo build --bins` 完全不受影响。
`cargo test` 才是考核入口。

**Q2：`cargo test` 一次跑 18 课报了一大堆 FAILED，正常吗？**
正常。骨架态每个知识点都会报【未实现】，那就是你的任务清单。想只看一课就用 `--test lesson_XX_...`。

**Q3：我可以改 `#[test]` 函数吗？**
不建议。`#[test]` 里写的是考核标准（期望值与边界用例），改了就等于自己改分数线。
如果你想验证自己的想法，可以在文件末尾自己加一个 `#[test] fn my_sandbox_xxx()`——
只要它不调用 `assessment_harness::assess`，就不会计入评估报告。写法见 [docs/02_how_to_answer.md](docs/02_how_to_answer.md)。

**Q4：报告里的分数是怎么算的？某一课编译不过会怎样？**
分数 = 通过知识点 ÷ 应掌握知识点。若某课的考核文件因为你的实现有语法/类型错误而编译不过，
该课会显示为「未运行或编译失败」，报告会提示你先修编译错误（这一类不计入通过）。

**Q5：我的进度记录在哪？能重来吗？**
在 `assessments/report/data/`（账本 `ledger.tsv` + 历史 `history.jsonl`）。
删除整个 `report/data/` 目录即可清空进度，不会影响考核本身。

**Q6：为什么考核不使用第三方的测试库（如 Criterion）？**
课程体系刻意保持零第三方依赖、离线可用。基准测试（benchmark）属于进阶话题，
需要时可参考 [docs/05_maintainer_guide.md](docs/05_maintainer_guide.md) 自行引入 Criterion；
本环节的考核目标是「掌握语言知识点」，libtest + 评估 harness 已经足够。

**Q7：Web 实战模块（src/demoweb）怎么考核？**
demoweb 是独立的 Cargo 项目，自带 102 个单元/集成测试，且需要联网下载依赖：

```bash
cd src/demoweb
cargo test
```

它是「学完之后用语言」的阶段，考核入口就是它自己的测试套件。

**Q8：有参考答案吗？会不会不小心看到？**
有，放在 `assessments/solutions/`（每课一个文件，与考核文件同名，共 182 个练习的答案），
索引与用法见 [solutions/README.md](solutions/README.md)。它**不会被 `cargo test` 编译**，
也不在考核文件里，所以正常练习时不会"撞见"答案；只有你显式运行
`cargo run --bin solution_tool -- --apply/--verify --lesson NN` 才会临时写进考核文件（并自动还原）。
还是那句话：**先自己写，再看答案。**
