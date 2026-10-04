# 05 · 考核体系维护者指南（新增/修改考核用例必读）

本文件面向**维护者**：讲清考核体系的结构、考核框架 `assessment_harness` 的 API、
新增一个考核模块要改哪些文件、以及验收清单。学员请先读
[../README.md](../README.md) 与 [02_how_to_answer.md](02_how_to_answer.md)。

---

## 一、目录结构（考核代码与教程内容分离，但目录结构一一对应）

```
hello/
├── Cargo.toml                      # [[lib]] assessment_harness + 18 个 [[test]] + 21 个 [[bin]]（hello + 18 门课程 + 2 个工具）
├── src/                            # ★ 教程内容（课程 + demoweb），考核不改动这里
│   ├── tutorial/lesson_01..18_*.rs
│   └── tutorial/guides/*.md
└── assessments/                    # ★ 考核内容（与 src 的课程文件同名对应）
    ├── harness/lib.rs              # 考核框架（库 crate assessment_harness）
    ├── lesson_01_variables_mutability.rs   ← 对应 src/tutorial/lesson_01_variables_mutability.rs
    ├── lesson_02_data_types.rs
    ├── ...
    ├── lesson_18_macros.rs                 ← 对应 src/tutorial/lesson_18_macros.rs
    ├── solutions/                  # ★ 参考答案（与考核文件同名；cargo test 不会编译）
    │   ├── README.md               #   索引 + 用法 + 「先自己写，再看答案」
    │   ├── lesson_01..18_*.rs      #   完整副本，只有练习函数体不同（--check 强制）
    │   └── .backup/                #   --apply 前的骨架备份（.gitignore 忽略）
    ├── bin/assessment_report.rs    # 评估报告生成器
    ├── bin/solution_tool.rs        # 参考答案工具箱（check/list/status/apply/restore/verify）
    ├── report/
    │   ├── data/ledger.tsv         # 账本（每次运行追加，可安全删除）
    │   ├── data/history.jsonl      # 每次运行的摘要（进度跟踪用）
    │   ├── assessment_report.md    # 最近一次的学习评估报告
    │   └── summary.json            # 机器可读摘要
    ├── scripts/run_assessments.ps1 / run_assessments.sh
    └── docs/                       # 本目录
```

**对应关系**：`assessments/lesson_XX_<英文主题>.rs` ↔ `src/tutorial/lesson_XX_<英文主题>.rs`。
测试目标名 = 课程文件名（去掉 `.rs`），
因此 `cargo test --test lesson_05_ownership_borrowing` 就是「考第 5 课」。

---

## 二、考核框架 API（`use assessment_harness::...`）

### 2.1 知识点登记

```rust
#[test]
fn kp_05_03_unique_mutable_borrow() {
    assess(
        M,                      // 模块 id，文件顶部的 const M: &str = "lesson_05";
        "kp_05_03",             // 知识点 id（全局唯一，格式 kp_<课号>_<序号>）
        "可变引用的唯一性",      // 知识点标题（会出现在报告里）
        Kind::Hard,             // 基础 / 核心 / 难点 / 边界
        "复习 lesson_05 示例 6 ……", // 卡住时的复习指引（会写进报告）
        || {                    // 考核本体：调用练习函数 + 断言
            let n = exercise_05_03_unique_mutable_borrow(&mut vec![1, 2]);
            eq(n, 3, "两次 &mut 借用不重叠时，最终值是 3");
        },
    );
}
```

`assess` 的动作：
1. 往账本写一条 `start`（含模块、知识点、复习指引、调用位置）；
2. 运行闭包。断言全部通过 → 写 `pass`；断言失败 → 写 `fail` 并把 panic 抛回 libtest；
3. 练习函数还是占位 → 写 `missing`（未实现）；其它 panic → 写 `panic`（附原始信息）。

> 因此 libtest 依然是执行框架与判定权威：`cargo test` 会返回非零退出码，
> 报告生成器则从账本里读出「为什么没通过」。

### 2.2 断言助手（全部带 `hint` 提示语，失败时写入报告）

| API | 用途 |
| --- | --- |
| `eq(actual, expected, hint)` | 相等（要求 `PartialEq + Debug`） |
| `eq_slice(&actual, &expected, hint)` | 切片/数组相等，报错信息带长度 |
| `is_true(cond, hint)` / `is_false(cond, hint)` | 布尔断言 |
| `ok(result, hint) -> T` / `err(result, hint) -> E` | `Result` 断言并取值 |
| `some(option, hint) -> T` / `none(option, hint)` | `Option` 断言并取值 |
| `approx(actual, expected, tolerance, hint)` | 浮点近似比较 |
| `contains(text, needle, hint)` | 子串断言（如错误信息里应含某关键字） |
| `panics(|| { ... }, hint)` | 断言这段代码会 panic（预期内的 panic 不会污染输出） |
| `panics_real(|| { ... }, hint)` | 断言会 panic，**且不是因为练习还没实现**——练习本身就是"写出会 panic 的代码"时必须用这个，否则骨架态会假通过 |

### 2.3 练习占位

```rust
fn exercise_05_03_unique_mutable_borrow(v: &mut Vec<i32>) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_05_03_unique_mutable_borrow",  // 练习函数名
        "把 v 追加两个元素，返回最终所有元素之和",   // 实现要求
        (v,),                                    // 把入参原样传入：避免骨架态 unused 警告
    )
}
```

- 返回类型是 `!`，所以在任何位置都能编译通过（骨架态**零警告**）；
- 学员实现时删除这一行即可；
- 没有入参就传 `()`；
- **请务必写成完全限定路径 `assessment_harness::todo_exercise(...)`，不要在文件顶部 `use` 它**：
  这样学员把整个文件的练习都实现完之后，顶部 `use` 不会变成 `unused import` 警告
  （骨架态、实现态都干净）。
- 个别签名有额外约束，需要特殊写法（都保持"纯占位 + 零警告"）：
  - 返回 `impl Trait`（如 `-> impl Fn(i32) -> i32`、`-> impl Area`）：`!` 不实现那些 trait，
    占位要挂在一个能推出隐藏类型的位置，例如 `move |x: i32| -> i32 { assessment_harness::todo_exercise(...) }`
    或内层 `fn placeholder(..) -> 具体类型 { assessment_harness::todo_exercise(...) }`；
  - 参数带 `mut` 绑定（如 `mut f: F`）：把 `&mut f` 传进占位参数，避免 `unused_mut`；
  - 提供的类型只被练习使用：把相关字段/函数传进占位参数，避免 `dead_code`。

---

## 三、考核文件的固定结构（新增文件请照抄）

```rust
//! assessments/lesson_XX_<主题>.rs —— 考核：<中文主题>（对应 lesson_XX）
//!
//! - 对应课程：`src/tutorial/lesson_XX_<主题>.rs`
//! - 知识点出处：`src/tutorial/README.md` <阶段>「XX <主题>」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_XX_<主题>
//!   cargo run --bin assessment_report
//!   ```
//! # 怎么用（三分钟上手）    ← 与 lesson_01 同一套说明
//! # 规则
//! # 本课常见错误速查         ← 可选：把课程的「常见错误示例」整理成表格

use assessment_harness::{Kind, assess, eq, /* 只导入真正用到的助手 */};

const M: &str = "lesson_XX";

// （需要共享类型时）===== 提供给你的类型（不要修改）=====
// 需要学员实现的方法/impl 里同样放 todo_exercise 占位。

// ===== 每个知识点：先 #[test]，紧跟对应的 exercise_* 练习函数 =====
```

硬性要求：

1. `#[test]` 函数命名 `kp_<课号>_<序号>_<snake_case 主题>`，练习函数命名
   `exercise_<课号>_<序号>_<snake_case 主题>`，两者**一 一对应**；
2. `#[test]` 里必须用 `assess(M, "kp_xx_yy", ...)` 登记，模块 id 用文件里的 `const M`；
3. 每个练习函数的函数体**只有** `todo_exercise(...)`，不得出现实现代码；
4. 每个 `#[test]` 至少覆盖：1 个正常用例 + 1 个**边界用例**（空输入、0/1 个元素、
   极值、越界、负数、多字节字符……），边界用例是本考核体系的要求之一；
5. 每个练习函数上方必须有：`【待实现】` 标题、`实现要求：`、以及 ACM 题式的一组
   `示例输入：` / `示例输出：`（取自该练习对应 `#[test]` 里被断言的那个典型用例）；
   **不要**写 `提示：`、`难度：★☆☆☆☆` 这类注释（难度由 `assess(...)` 的 `Kind` 决定，报告按类统计）；
6. 只导入用到的 API（否则出现 unused import 警告）；禁止 `#[allow(...)]`、禁止 `unsafe`、
   禁止第三方 crate；必须通过 `rustfmt --edition 2024`；
7. 骨架态（全部是 `todo_exercise`）必须**零 warning**编译通过。

---

## 四、新增一个考核模块要改的地方

1. 新建 `assessments/lesson_XX_<主题>.rs`（照第三节结构）；
2. 在根 `Cargo.toml` 追加：

   ```toml
   [[test]]
   name = "lesson_XX_<主题>"
   path = "assessments/lesson_XX_<主题>.rs"
   ```

3. 在 `assessments/harness/lib.rs` 的 `MODULES` 表里登记模块
   （id / 中文名 / 课程文件 / 复习命令 / README 出处）——报告里的「复习入口」就来自这里；
4. 运行 `cargo test --test lesson_XX_<主题>` 确认：编译零警告 + 所有知识点报「未实现」；
5. 运行 `cargo run --bin assessment_report -- --emit-map` 重新生成
   [04_knowledge_map.md](04_knowledge_map.md)；
6. 按第五节写出参考答案 `assessments/solutions/lesson_XX_<主题>.rs`，
   并用 `cargo run --bin solution_tool -- --verify --lesson XX` 确认全部用例**可以通过**；
7. 在 [../solutions/README.md](../solutions/README.md) 的文件对照表里补一行。

---

## 五、参考答案与「可解性」验证

> 考核用例必须「可解」：每一条断言都得有人真的通过过。
> 参考答案放在 `assessments/solutions/`（与考核文件同名、是完整副本，只有练习函数体不同），
> 由 `assessments/bin/solution_tool.rs` 统一校验 / 应用 / 还原 / 验证。

### 5.1 写一份参考答案

1. 复制考核文件：`Copy-Item assessments\lesson_05_ownership_borrowing.rs assessments\solutions\ -Force`；
2. 只把每个练习函数体里的 `assessment_harness::todo_exercise(...)` 换成真实实现
   （需要辅助函数就写在**函数体内部**；`#[test]`、签名、注释、`use` 一律不动）；
3. 校验 + 验证：

   ```powershell
   cargo run --bin solution_tool -- --check  --lesson 05   # 函数体之外必须逐字节一致
   cargo run --bin solution_tool -- --verify --lesson 05   # 应用 → cargo test → 自动还原
   cargo run --bin solution_tool -- --status --lesson 05   # 最后必须是「骨架」
   ```

   `--check` 会保证答案不可能改到考核标准（`#[test]` 被改动会直接报错）；
   `--verify` 通过后账本写在 `target/solution-verify/`，**不会**污染学员的进度记录。

### 5.2 手工自检（不想用工具时）

```powershell
# 1) 备份骨架
Copy-Item assessments\lesson_05_ownership_borrowing.rs $env:TEMP\skel_05.rs -Force

# 2) 把该文件里所有 todo_exercise(...) 换成参考实现，运行
cargo test --test lesson_05_ownership_borrowing

# 3) 全部通过后还原骨架，并确认骨架态仍能编译
Copy-Item $env:TEMP\skel_05.rs assessments\lesson_05_ownership_borrowing.rs -Force
cargo test --test lesson_05_ownership_borrowing --no-run
```

自检时可用环境变量把账本写到别处，避免污染学员的进度记录：

```powershell
$env:ASSESSMENT_DATA_DIR = "$env:TEMP\assess-selfcheck"
```

---

## 六、账本与报告的数据流

```
cargo test（18 个测试目标，libtest 执行）
   └─► 每个知识点 start/ok/fail/missing/panic/pass  ──► report/data/ledger.tsv（TSV 追加）
                                                            │
cargo run --bin assessment_report ◄─────────────────────────┘
   ├─► report/assessment_report.md   人读的学习评估报告（含薄弱环节与行动清单）
   ├─► report/summary.json           机器可读摘要
   ├─► report/data/history.jsonl     每次运行一行，用于「学习进度对比」
   └─► docs/04_knowledge_map.md      --emit-map 生成的知识点覆盖矩阵
```

- 账本字段（12 列，文本字段用 `assessment_harness::escape` 转义）：
  `run_id, ts_ms, event, module_id, module_name, kp_id, kp_title, kind, location, detail, hint, guide`；
- `ASSESSMENT_RUN_ID` 指定运行 id（脚本会设置），未设置时按「10 分钟会话窗口」自动归并；
- `report/data/` 可以随时删除（只影响历史记录，不影响考核）。

---

## 七、约定与红线

- 考核**不修改** `src/` 下的任何课程文件：考核是「可选但推荐」的附加环节；
- 考核文件不使用第三方依赖，保持与课程一致的「只有标准库」；
- 不写 `#[allow(...)]`、不写 `unsafe`；
- 报告里的「复习入口」必须指向**真实存在的**课程文件与命令（改文件名时要同步更新
  `MODULES` 表）；
- 新增知识点必须同时补齐：`#[test]` + `exercise_*` + 映射表（自动生成）+ 至少一个边界用例。

---

## 八、批量验证协议（怎么证明「每个知识点都能通过」）

考核用例必须**可解**：每一条断言都得有人真的通过过。整套「骨架 → 参考实现 → 骨架」往返验证
都收敛到 `solution_tool`，一条命令覆盖 18 个文件：

```powershell
# 1) 校验参考答案没有改动考核标准（18 个文件、182 个练习，函数体之外逐字节一致）
cargo run --bin solution_tool -- --check

# 2) 应用 → cargo test → 自动还原；18 个目标都必须 N passed / 0 failed
cargo run --bin solution_tool -- --verify

# 3) 确认骨架回到干净状态（每个知识点都应报【未实现】，且编译零警告）
cargo run --bin solution_tool -- --status
cargo test --no-fail-fast
```

- `--verify` 的账本写在 `target/solution-verify/ledger.tsv`，且 18 个目标共用运行 ID
  `solution-verify`，所以**不需要手工拼账本**就能生成「全部通过」的样例报告：

  ```powershell
  $env:ASSESSMENT_LOCAL_OFFSET_MINUTES = [string][int]((Get-Date) - (Get-Date).ToUniversalTime()).TotalMinutes
  cargo run --quiet --bin assessment_report -- --data-dir target/solution-verify --run solution-verify --report-dir assessments\report\samples
  # 生成 assessment_report.md，覆盖为 samples/sample_full_pass.md 即可
  ```

- 若发现**测试本身**写错了（期望值不对、用例不可满足、提示与课程不符）：修正 `#[test]`
  并记下改了哪几处（这些修正要保留），然后重跑第 1、2 步；注意同时更新对应的参考答案
  （`solution_tool -- --check` 会因为两侧不再一致而报错，这正是它该做的事）；
- 每个文件的结论（「参考实现下 N/N 通过」+ 修正过的测试问题）记到
  `assessments/report/samples/verification.md`，作为这批考核用例的验收记录。

> 账本是 UTF-8 文本；`--verify` 走的是工具内部的文件写入，不经过 PowerShell 的管道与
> `Get-Content | Set-Content`（Windows PowerShell 5.1 会按 ANSI 解码导致中文乱码）。
