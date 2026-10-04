# 02 · 答题指南（怎么练、怎么写、怎么自测）

本文件面向**正在做考核练习的新人**：讲清考核环节是什么、每课怎么走三步、失败输出怎么读、
怎么自己写测试验证想法、以及每课练完的自评标准。

> 环境还没搭好？请先读 [01_environment_setup.md](01_environment_setup.md)。

---

## 一、这个环节是什么

- **可选，但强烈推荐**：不做考核也能学完课程，但做完考核你才知道自己**真的**会了；
- `src/tutorial/` 下的 **18 门课讲知识**（01~13 语言基础 + 14~18 补充进阶），`assessments/` 下的
  **18 个考核文件检验掌握程度**；
- **一一对应**：`assessments/lesson_XX_<主题>.rs` ↔ `src/tutorial/lesson_XX_<主题>.rs`；
- **测试目标名 = 课程文件名（去掉 `.rs`）**，所以「考第 5 课」就是：

  ```powershell
  cargo test --test lesson_05_ownership_borrowing
  ```

### 考核与课程的对应关系（示例）

| 课号 | 考核文件（学员改这个） | 对应课程（复习看这个） | 单课运行命令 |
| --- | --- | --- | --- |
| 01 | [../lesson_01_variables_mutability.rs](../lesson_01_variables_mutability.rs) | [../../src/tutorial/lesson_01_variables_mutability.rs](../../src/tutorial/lesson_01_variables_mutability.rs) | `cargo test --test lesson_01_variables_mutability` |
| 05 | `assessments/lesson_05_ownership_borrowing.rs` | [../../src/tutorial/lesson_05_ownership_borrowing.rs](../../src/tutorial/lesson_05_ownership_borrowing.rs) | `cargo test --test lesson_05_ownership_borrowing` |
| 18 | `assessments/lesson_18_macros.rs` | [../../src/tutorial/lesson_18_macros.rs](../../src/tutorial/lesson_18_macros.rs) | `cargo test --test lesson_18_macros` |

全部 18 个课号与主题见 [../../src/tutorial/README.md](../../src/tutorial/README.md)「二、课程目录」。

---

## 二、每课三步流程（10~30 分钟）

### 第 1 步：读 `#[test]` —— 先搞清「它会怎么检查你」

每个知识点都是一个 `#[test]` 函数，函数体里是一次 `assess(...)` 调用。
你**不需要**看懂 `assess` 的每个参数，只要抓住三件事：

1. **它会传什么进去**（调用 `exercise_XX_YY_...(...)` 时给的入参）；
2. **它期望什么出来**（`eq(...)` 里的期望值、`hint` 里那句中文说明）；
3. **它考了哪些用例**（一个知识点通常有 1 个正常用例 + 若干**边界用例**，
   比如空串、0 次、负数、多字节中文）。

以 lesson_01 的 `kp_01_01` 为例，它检查 3 个用例：

```rust
// 正常用例：从 0 开始累加 5 次、每次 +2 → 10
eq(exercise_01_01_mut_counter(0, 5), 10, "循环 5 次、每次加 2，最终值应为 10");
// 边界：一次都不加，结果应等于初始值
eq(exercise_01_01_mut_counter(7, 0), 7, "times 为 0 时不应改变初始值（循环体一次都不执行）");
// 边界：负数初始值
eq(exercise_01_01_mut_counter(-3, 2), 1, "初始 -3 加上两次 2 应得到 1");
```

> 读 `#[test]` 的目的不是「抄答案」，而是**明确接口与验收标准**。
> 你的实现必须同时满足正常用例和边界用例。

### 第 2 步：实现 `exercise_*` —— 删掉 `todo_exercise(...)` 那一行

每个 `#[test]` 的下方，紧跟一个同名同主题的练习函数，函数体里现在只有一行占位：

```rust
/// 【待实现】用 `mut` 累加器累加。
///
/// 实现要求：
///   - 入参 `start` 是初始值，`times` 是要累加的**次数**；
///   - 用 `let mut` 声明一个累加器，循环 `times` 次，每次 `+= 2`；
///   - 返回累加后的值；`times` 为 0 时直接返回 `start`。
///
/// 示例输入：
/// ```text
/// start = 0, times = 5
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_01_01_mut_counter(start: i32, times: i32) -> i32 {
    todo_exercise(
        "exercise_01_01_mut_counter",
        "用 let mut 累加器循环 times 次、每次 +2，返回结果",
        (start, times),
    )
}
```

**做法**：把 `todo_exercise(...)` 这一整行**删掉**，按上面的「实现要求」写代码：

```rust
fn exercise_01_01_mut_counter(start: i32, times: i32) -> i32 {
    let mut total = start;
    for _ in 0..times {
        total += 2;
    }
    total
}
```

每个练习函数上方固定有三样东西，按顺序读即可：

| 标注 | 含义 |
| --- | --- |
| `【待实现】` 标题 | 这一题的主题 |
| `实现要求：` | 你要写出的行为（**验收合同**） |
| `示例输入：` / `示例输出：` | 一组典型的输入与它应当产出的结果（ACM 题式样例，取自该题在 `#[test]` 里被断言的那个典型用例）；照着它自测最快 |

### 第 3 步：跑测试，再看评估报告

```powershell
# 只考这一课（推荐：每次改完就跑，反馈几秒就回来）
cargo test --test lesson_01_variables_mutability

# 看学习评估报告（薄弱环节、行动清单、进度对比）
cargo run --bin assessment_report
```

全部知识点变绿之后，报告里这一课的覆盖率应达到 **100%**。
一次想跑完 18 课：

```powershell
cargo test --no-fail-fast
```

### 命令速查

| 目的 | 命令 |
| --- | --- |
| 只考某一课 | `cargo test --test lesson_XX_<主题>` |
| 只看是否编译通过（不运行） | `cargo test --test lesson_XX_<主题> --no-run` |
| 顺序执行（失败原因更好读） | `cargo test --test lesson_XX_<主题> -- --test-threads=1` |
| 只跑某一个知识点 | `cargo test --test lesson_XX_<主题> -- kp_XX_YY` |
| 只跑自己写的沙箱测试 | `cargo test --test lesson_XX_<主题> -- my_sandbox_xxx --nocapture` |
| 全部 18 课，不因失败中断 | `cargo test --no-fail-fast` |
| 生成学习评估报告 | `cargo run --bin assessment_report` |
| 生成知识点覆盖矩阵 | `cargo run --bin assessment_report -- --emit-map` |

> 目标名写错会报 `error: no test target named ...`：正确的目标名 = 考核文件名去掉 `.rs`，
> 与根 `Cargo.toml` 里 `[[test]] name` 完全一致（也等于课程文件名）。
> 跑之前请确认自己在**项目根目录**。

---

## 三、函数命名规则：`kp_*` 与 `exercise_*` 一一对应

| 角色 | 命名格式 | 例子 | 谁写 |
| --- | --- | --- | --- |
| 考核题（检查你） | `kp_<课号>_<序号>_<主题>` | `kp_01_01_mut_counter` | 出题人（你**不要改**） |
| 练习（你来实现） | `exercise_<课号>_<序号>_<主题>` | `exercise_01_01_mut_counter` | 你（**只改函数体**） |

规律：

- 去掉前缀后，`kp_01_01_mut_counter` 与 `exercise_01_01_mut_counter` 的
  **`<课号>_<序号>_<主题>` 完全相同**；
- 所以看到失败的 `kp_XX_YY_...`，直接往下找 `exercise_XX_YY_...` 就是你要改的函数
  （框架的失败信息也会把「待实现函数名」直接打出来）；
- 一个文件里的知识点按序号顺序排列，序号从 `01` 开始。

---

## 四、只改练习函数：可以改什么，不可以改什么

| 可以改 ✅ | 不可以改 ❌ |
| --- | --- |
| `exercise_*` 函数的**函数体**（删掉 `todo_exercise(...)`，写你的实现） | `#[test]` 函数（包括 `assess(...)` 的入参、`eq(...)` 的期望值与顺序） |
| 在函数体内新增局部变量、`if` / `for` / `match` | 练习函数的**名字**与**签名**（参数、返回类型） |
| 在文件末尾新增你自己的 `#[test] fn my_sandbox_xxx()`（见第六节） | 文件顶部的 `const M` 与 `use assessment_harness::{...}` 导入（除非你确实需要额外导入，且不留 unused 警告） |
| 在文件里新增**仅供你调用**的辅助函数 | 其它课程文件、`Cargo.toml`、`assessments/harness/lib.rs` |
| 调整你自己的代码风格（保持 `rustfmt` 默认格式） | 在练习函数体里留下 `todo_exercise(...)`（会一直判「未实现」） |

三条红线：

1. **不要改 `#[test]`**：改了就改变了验收标准，测出来的只是你改过的标准；
2. **不要为了过测试写死返回值**：考核要求你实现**通用**逻辑，写死会被边界用例抓住（见第九节）；
3. **不要引入第三方 crate**：考核体系零依赖，只用标准库 + `assessment_harness`。

---

## 五、怎么用测试输出定位问题：逐字段解读

### 5.1 情形 A：练习还没写（骨架态）

```powershell
cargo test --test lesson_01_variables_mutability
```

真实输出（已在本机核对）：

```text
running 8 tests
test kp_01_01_mut_counter ... FAILED
...

thread 'kp_01_01_mut_counter' (40208) panicked at assessments\lesson_01_variables_mutability.rs:99:5:
【未实现·基础】变量与可变性 · kp_01_01「let 绑定与 mut：默认不可变，需要改值就写 mut」
  待实现函数：exercise_01_01_mut_counter
  实现要求：用 let mut 累加器循环 times 次、每次 +2，返回结果
  位置：assessments\lesson_01_variables_mutability.rs:99
  复习：复习 lesson_01 示例 1（immutable_by_default）与示例 2（mut）：`let x = 1; x = 2;` 会报 error[E0384]，必须写成 `let mut x = 1;`。

test result: FAILED. 0 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

逐行读：

| 字段 | 含义 | 你要做什么 |
| --- | --- | --- |
| `【未实现·基础】` | 这一类失败是**未实现**；`·基础` 是知识点分类（基础/核心/难点/边界） | 去实现，不用怀疑代码写错 |
| `变量与可变性 · kp_01_01` | 模块中文名 + 知识点 id | 定位是哪个知识点 |
| `「let 绑定与 mut：…」` | 知识点标题 | 知道这一题考什么 |
| `待实现函数：exercise_01_01_mut_counter` | **要你实现的函数名** | 在考核文件里搜索这个名字 |
| `实现要求：…` | 一句话合同（与练习函数上方注释一致） | 按它写 |
| `位置：assessments\lesson_01_variables_mutability.rs:99` | 练习函数 `todo_exercise(...)` 所在行（**改哪一行**） | 跳到该行 |
| `复习：复习 lesson_01 示例 1 …` | 卡住时该去看课程的哪个示例 | 先看课程，再回来改 |

### 5.2 情形 B：实现了，但结果不对（断言失败）

如果实现逻辑写错（或只覆盖了正常用例、没考虑边界），框架给出的是
`【未通过·<分类>】`，字段更全（各字段的拼接格式由考核框架
[../harness/lib.rs](../harness/lib.rs) 里的 `fail_now` 固定，可自行核对）：

```text
【未通过·边界】变量与可变性 · kp_01_02「变量遮蔽（shadowing）：同名 let 覆盖旧绑定」
  值不相等（期望 6，实际 2）
  期望：6
  实际：2
  提示：String::len() 返回的是字节数："中文" 在 UTF-8 下是 6 字节
  改进建议：边界条件：检查空输入、0/1 个元素、首尾元素、类型极值（如 u8::MAX）、整除与负数，以及 UTF-8 多字节字符。
  位置：assessments\lesson_01_variables_mutability.rs:140
  复习：复习 lesson_01 示例 3（shadowing）：遮蔽是「新建一个同名绑定」，不是修改旧值，所以不需要 mut，也可以换类型。
```

> 上面是**模拟输出**：它是你自己写出「只 trim 不按字节算」的错误实现时，`kp_01_02`
> 那条「中文」边界用例（`exercise_01_02_shadowing_chain("中文")` 期望 6）会给出的形状。
> 字段名与措辞与框架实现一致，行号只是示例（真实行号以你那次运行打印的为准）。

逐字段含义：

| 字段 | 含义 | 怎么用 |
| --- | --- | --- |
| `【未通过·边界】` | 断言失败（不是「未实现」）；`边界` 说明这个是边界用例 | 优先检查边界逻辑 |
| 第一行摘要 | 哪个断言、哪个知识点 | 定位 |
| **期望 / 实际** | 框架要的值 vs 你的实现返回的值 | 对比两者，反推逻辑错在哪 |
| **提示** | 出题人为这条断言写的中文说明 | 往往直接点出考点 |
| **改进建议** | 按知识点分类给出的**通用**改进方向 | 分类为「边界」时按建议逐个自查 |
| **位置** | 出错的**那一行断言**（`file:line`） | 对照 `#[test]` 该行看入参 |
| **复习** | 对应课程的示例与知识点 | 回课程复习该示例 |

> 注意：`位置` 指向的是**断言那一行**（情形 A 里指向的是 `todo_exercise` 那一行）。
> 两处行号都在**考核文件**里，不是框架文件里——框架用 `#[track_caller]` 保证这一点。

### 5.3 情形 C：实现里 panic 了

如果越界、`unwrap()` 失败、除零等，框架记为 `panic` 并附原始信息，
输出里会出现 libtest 自己的 panic 行（如 `index out of bounds`）。
**这类失败要先修「为什么会 panic」，不要急着改断言**——断言通常没错。

---

## 六、怎么自己写一个测试来验证想法（沙箱测试）

考核文件是**普通 Rust 测试文件**，你完全可以在末尾加自己的测试当「草稿纸」。

**示例**（把它贴到任意考核文件的**最末尾**）：

```rust
// ===== 我的沙箱测试（不属于考核，不会被计入评估报告）=====

#[test]
fn my_sandbox_trim_and_len() {
    // 直接验证我的猜想，不调用 assessment_harness::assess，也不调用 todo_exercise
    let text = "  rust  ";
    let text = text.trim();
    println!("trim 之后 = {text:?}");
    let text = text.len();
    assert_eq!(text, 4, "trim 后 \"rust\" 的字节长度应为 4");

    // 顺便确认一个容易记错的点：中文在 UTF-8 下占几个字节
    println!("\"中文\".len() = {}", "中文".len());
    assert_eq!("中文".len(), 6, "UTF-8 下每个汉字 3 字节");
}
```

两条**重要规则**：

1. **自己写的测试不要调用 `assessment_harness::assess`**（也不要调用 `assessment_harness::todo_exercise`）。
   原因：只有 `assess(...)` 里的断言才会写进账本、才会计入
   `assessment_report` 的掌握率与得分统计。不调用它，你的沙箱测试就**只是你自己的草稿纸**，
   不会让评估报告的数字变好看，也不会污染它。
   （顺带一提：在没有 `assess` 上下文时调用断言助手，框架会直接以
   「【考核框架误用】」panic，所以请老老实实用标准库的 `assert_eq!` / `assert!`。）
2. **不要改 `#[test]` 和 `exercise_*`**，沙箱测试独立命名（建议统一加 `my_sandbox_` 前缀），
   做完实验可以整段删掉。

### 只运行你的沙箱测试

```powershell
cargo test --test lesson_01_variables_mutability -- my_sandbox_trim_and_len --nocapture
```

说明：

- `--` 之后的参数交给测试执行器 libtest：`my_sandbox_trim_and_len` 是**按名字过滤**，
  只会运行名字里含该字符串的测试；
- `--nocapture` 让你写的 `println!` **打印到终端**（默认会被 libtest 吞掉，
  失败时才显示）；
- 想看全部测试的打印（包括考核题的输出）就去掉过滤名字：

  ```powershell
  cargo test --test lesson_01_variables_mutability -- --nocapture
  ```

### 沙箱测试能做什么

| 用途 | 例子 |
| --- | --- |
| 验证一个 API 的行为 | `assert_eq!("中文".len(), 6)` |
| 演示借用/所有权是否编译得过 | 写两行会冲突的代码，看 `cargo check` 报什么 |
| 打印中间值 | `println!("{v:?}")` + `--nocapture` |
| 试出边界行为 | `assert_eq!((0..0).count(), 0, "空区间的元素个数是 0")` |

---

## 七、卡住时的正确顺序

不要一卡住就到处搜答案。按这个顺序走，稳定且省时间：

1. **读课程文件头的学习目标**：明确这一课到底要你掌握什么；
2. **跑一遍课程看「预期输出」**：

   ```powershell
   cargo run --bin lesson_XX_<主题>        # 例如 cargo run --bin lesson_01_variables_mutability
   ```

   课程文件里 `// 预期输出：` 是字面量断言，和真实输出逐字节一致，是你最好的对照物；
3. **读失败信息里的 `复习：` 指引**：它会直接告诉你「复习 lesson_XX 示例 N」，去看那个示例；
4. **只改一处，再跑一次**：一次只动一个地方，才能知道是哪个改动起了作用；
5. **超过 20 分钟还卡住**：去看**该课示例里的同类写法**——对照是允许的，
   但要求你能用自己的话说清「**为什么必须这样写**」。说不清就等于没学会，
   换一个例子你还是会卡。
6. **还是过不去**：看**参考答案** `assessments/solutions/lesson_XX_<主题>.rs`
   （索引与用法见 [solutions/README.md](../solutions/README.md)）。
   规则是：**先自己写**，只在真的卡住时才看；看完**关掉答案、自己重写一遍**。
   想快速确认某一课的答案是否真能全绿：

   ```powershell
   cargo run --bin solution_tool -- --verify --lesson 05   # 应用答案 → 跑测试 → 自动还原骨架
   cargo run --bin solution_tool -- --list   --lesson 05   # 列出「练习 → 答案在第几行」
   ```

> 卡住时**不要**做的事：乱试类型标注、把 `clone()` 洒满代码、直接删断言、
> 大量搜「Rust 怎么过测试」。这些只能让你暂时变绿，学不到东西。

---

## 八、边界用例要求：正常用例 + 边界用例都要过

考核体系的硬性要求：**每个知识点至少覆盖 1 个正常用例 + 1 个边界用例**。
也就是说，一个「正常输入能跑对」的实现，很可能仍然是不合格的。

常见的边界类型（都会真实出现在考核里）：

| 边界类型 | 典型例子 |
| --- | --- |
| 空输入 | 空字符串 `""`、空 `Vec`、空 `HashMap`、`0..0` |
| 0 / 1 个元素 | `times = 0`、只有一个元素的切片、`n = 1` |
| 极值 | `u8::MAX`、`i32::MIN`、超长文本、`usize` 与 `i32` 转换的边界 |
| 越界 / 不存在 | 索引超出长度、查不到 key、`find` 返回 `None` |
| 负数 | 负初始值、负偏差、负数参与累加与比较 |
| UTF-8 多字节 | 中文/emoji 的**字节数**与**字符数**不同（`"中文".len() == 6`，`.chars().count() == 2`） |
| 整除与取余 | 除数为 0、不能整除、负数取余 |
| 类型与精度 | 整数溢出、浮点不能用 `==`（框架提供 `approx` 近似比较） |
| 顺序与重复 | 重复 key、相等元素的排序稳定性、首尾元素 |

### 边界自查清单（实现完一个练习函数后逐条问自己）

- [ ] **空**：输入为空时我的代码会 panic 吗？返回什么才合理？
- [ ] **0 / 1**：次数为 0、只有一个元素时，循环体一次都不执行/只执行一次，结果对吗？
- [ ] **极值**：用 `u8::MAX` / `i32::MIN` 这类极值代入，会溢出吗？该用更宽的类型吗？
- [ ] **越界**：索引/`get` 越界时我用的是 `[]`（会 panic）还是 `get()`（返回 `Option`）？考试期望哪个？
- [ ] **负数**：负数参与时，`abs`、取余、比较、累加的结果还对吗？
- [ ] **UTF-8**：题目要的是**字节数**（`.len()`）还是**字符数**（`.chars().count()`）？
- [ ] **整除**：会不会除以 0？整数除法会不会把小数部分丢掉？
- [ ] **浮点**：我是不是在用 `==` 比浮点？（考核里应该用 `approx(actual, expected, tolerance, hint)`）
- [ ] **首尾**：首元素、末元素、相邻相等元素这几处特判对吗？
- [ ] **顺序**：`HashMap` / `HashSet` 的遍历顺序是不确定的——我的实现依赖顺序了吗？

---

## 九、常见误区（踩过就长记性）

1. **为了过测试硬编码返回值**
   *症状*：`fn exercise_01_01_mut_counter(start: i32, times: i32) -> i32 { 10 }`，正常用例过了，
   边界用例全挂。
   *后果*：评估报告会体现为**「只覆盖部分用例」**（一个知识点下的 `ok` 断言不全、
   且有 `fail` 记录），覆盖率上不去。
   *正确做法*：实现通用逻辑，让所有入参都对。
2. **改 `#[test]` 来「让测试通过」**
   *症状*：把期望值改成自己算出来的错值，或用 `#[ignore]` 跳过。
   *后果*：验收标准被你改掉了，学到的是错的东西；维护者对比原始文件能立刻发现。
   *正确做法*：`#[test]` 是标准，只改 `exercise_*`。
3. **`todo_exercise` 还留着，却以为自己写完了**
   *症状*：在函数上方加了实现代码，但占位那一行没删——`todo_exercise` 的返回类型是 `!`
   （发散类型），写成函数最后一个表达式时它会**永远 panic**，测试一直报「未实现」；
   若你把代码写在它前面，编译器还会警告 `unreachable expression`。
   *正确做法*：删掉整行，确认函数体最后一行是你要返回的表达式。
4. **用 `unwrap()` 掩盖错误**
   *症状*：`let n = input.parse().unwrap();` 遇到非法输入直接 panic，
   而题目要求的是「返回 `Result` / `Option` 让调用方处理」。
   *正确做法*：先看练习函数的**返回类型**——是 `Result` 就必须返回 `Err`，
   是 `Option` 就必须返回 `None`（多数考核题就是考这个）。
   *例外*：题目注释里明确写了「考核只会传入合法数字字符串，这里 `unwrap()` 可以接受」时，
   那是题目给出的许可。
5. **忽略类型标注提示**
   *症状*：`.parse()`、`Vec::new()` 报 `error[E0282]: type annotations needed`，
   于是开始瞎改逻辑。
   *正确做法*：报 E0282 时**只加类型**，不改逻辑：`let n = "7".parse::<i32>().unwrap();`
   或 `let v: Vec<i32> = Vec::new();`。
6. **把 `String` 与 `&str` 混用**
   *症状*：函数要 `&str`，你传了 `String` 的**值**（所有权被 move 走，后面用不了）；
   或者函数要返回 `String`，你返回了 `&str`，触发借用/生命周期错误。
   *正确做法*：要「只读一个字符串」就收 `&str`，用 `&s` 或 `s.as_str()`；
   要「造一个新字符串」才用 `String`（`String::from(...)` / `to_string()` / `format!`）。
7. **用 `clone()` 到处糊住借用错误**
   *症状*：编译器一报 `cannot borrow ... as mutable`，就在每个变量后面加 `.clone()`。
   *后果*：编译过了，但你没理解借用规则；后面一考借用/切片/生命周期立刻暴露；
   而且 `clone()` 有真实运行期开销，课程里强调的「零开销」就没意义了。
   *正确做法*：先读报错里的 `help:`，判断到底该用 `&` 借用、缩小作用域，
   还是改变量的先后顺序；clone 只在确实需要独立副本时用。
8. **跳过 `#[test]` 直接看「实现要求」**
   *症状*：实现完了才跑测试，一次挂 3 条边界用例，然后反复猜。
   *正确做法*：先读 `#[test]`（入参、期望、边界），再写代码——这一步通常只要 1 分钟。
9. **一次只跑不看报告**
   *症状*：只顾着让测试变绿，从不看 `cargo run --bin assessment_report`
   的「薄弱环节」，同类型的错误反复犯。
   *正确做法*：每完成 2~3 课就跑一次报告，按它给的行动清单补课。
10. **在考核文件里写 `unwrap()` 之外的 `panic!` / `unsafe` / `#[allow(...)]`**
    *说明*：本仓库约定不使用 `unsafe`、不写 `#[allow(...)]`。
    练习函数该 panic 的时候，题目会明确要求（这时按题目写 `panic!` / `assert!` 即可）。

---

## 十、每课练完之后的自评清单

做完一课，四条都能打勾才算「过了」：

- [ ] **能一句话解释知识点**：不看代码，用一句中文说清这一课的核心概念
      （例如：「遮蔽是新建同名绑定，所以能换类型；`mut` 只能改值不能换类型」）。
- [ ] **能默写出签名**：不看文件，凭记忆写出本课 `exercise_*` 的函数签名
      （参数类型、返回类型）；这一步检验你是否真的看懂了 `#[test]` 的接口。
- [ ] **能复现该课的预期输出**：

  ```powershell
  cargo run --bin lesson_XX_<主题>
  ```

  输出与文件里 `// 预期输出：` 的断言逐字节一致（不一致就是你没读透这一课）。
- [ ] **报告里该课 100%**：

  ```powershell
  cargo test --test lesson_XX_<主题>
  cargo run --bin assessment_report
  ```

  `assessments/report/assessment_report.md` 中该模块的全部知识点均为通过（覆盖率 100%），
  且没有 `missing` / `fail` / `panic` 记录。

四条全中，就可以进入下一课了。18 课全部 100% 之后，可以：

- 用 `cargo run --bin assessment_report -- --emit-map` 生成知识点覆盖矩阵
  （生成物 `assessments/docs/04_knowledge_map.md`，由报告生成器写出，不要手写），
  检查有没有整块跳过的阶段；
- 回到根 [README.md](../../README.md) 的第四、五节，按学习路径继续推进到
  [src/demoweb](../../src/demoweb/README.md)。

想了解考核体系内部结构与考核框架 API，见 [05_maintainer_guide.md](05_maintainer_guide.md)；
环境问题见 [01_environment_setup.md](01_environment_setup.md)。
