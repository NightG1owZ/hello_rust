# 学习评估报告（assessment_report）

- 生成时间：2026-10-04 18:24:32（本地时间）
- 运行 id：`sample-new-learner`
- 本次结束时间：2026-10-04 18:24:30（本地时间）
- 数据来源（账本）：`target\ledger-sample-new\ledger.tsv`
- 报告文件：`target\sample-new\assessment_report.md`
- 知识点矩阵：`assessments\docs\04_knowledge_map.md`

**计分口径**
- 预期知识点集合来源：**知识点矩阵**；知识点矩阵与账本并集一致。
- 预期知识点总数：**157**（账本中所有运行出现过的知识点并集；矩阵存在时以矩阵为准）
- **分母口径**：总分 = 通过知识点数 ÷ 预期知识点总数 × 100（四舍五入）= 0 ÷ 157 → **0 分**；某个模块本轮**完全没跑**时，它的知识点**仍然计入分母**（本轮记作「未运行或编译失败」），这样跨次运行比较分数才有意义。
- 等级线：≥90 优秀 / ≥75 良好 / ≥60 合格 / <60 待加强
- 判定口径：同一次运行内同一知识点只取**最后一组**（最后一个 `start` 及其之后的事件），这样重复运行同一测试目标不会重复计数；
  有 `pass` → 通过，否则依次看 `missing`（未实现）/ `fail`（未通过）/ `panic`（运行期错误），有 `start` 但没有终态事件的算「未完成或中断」；本轮一条记录都没有的算「未运行或编译失败」。

## 一、总览

| 指标 | 数值 |
| --- | --- |
| 总分 | 0 分 |
| 等级 | 待加强 |
| 通过 | 0 |
| 未实现 | 157 |
| 未通过 | 0 |
| 运行期错误 | 0 |
| 未完成或中断（本次有记录但没有终态事件） | 0 |
| 本轮未运行或编译失败（计入分母） | 0 |
| 预期知识点总数 | 157 |

**一句话诊断**：157 个知识点还没动手实现；当前 0 分 · 待加强。建议先集中攻 `lesson_12`（Trait）——把「未实现」变成「通过」是最快的提分方式。

## 二、模块得分表（按课号顺序）

| 模块 | 模块名 | 知识点数 | 通过 | 未实现 | 未通过 | 得分 | 等级 | 本次是否运行 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| lesson_01 | 变量与可变性 | 8 | 0 | 8 | 0 | 0 | 待加强 | 已运行 |
| lesson_02 | 数据类型 | 8 | 0 | 8 | 0 | 0 | 待加强 | 已运行 |
| lesson_03 | 函数 | 8 | 0 | 8 | 0 | 0 | 待加强 | 已运行 |
| lesson_04 | 流程控制 | 8 | 0 | 8 | 0 | 0 | 待加强 | 已运行 |
| lesson_05 | 所有权系统 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_06 | 结构体 | 8 | 0 | 8 | 0 | 0 | 待加强 | 已运行 |
| lesson_07 | 枚举与模式匹配 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_08 | 常见集合 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_09 | 包和模块 | 7 | 0 | 7 | 0 | 0 | 待加强 | 已运行 |
| lesson_10 | 错误处理 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_11 | 泛型 | 10 | 0 | 10 | 0 | 0 | 待加强 | 已运行 |
| lesson_12 | Trait | 11 | 0 | 11 | 0 | 0 | 待加强 | 已运行 |
| lesson_13 | 生命周期 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_14 | 闭包 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_15 | 迭代器 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_16 | 智能指针 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_17 | 线程与通道 | 9 | 0 | 9 | 0 | 0 | 待加强 | 已运行 |
| lesson_18 | 声明宏 | 8 | 0 | 8 | 0 | 0 | 待加强 | 已运行 |
## 三、分类掌握率

| 类型 | 通过 | 总数 | 掌握率 |
| --- | --- | --- | --- |
| 基础 | 0 | 25 | 0/25（0%） |
| 核心 | 0 | 75 | 0/75（0%） |
| 难点 | 0 | 45 | 0/45（0%） |
| 边界 | 0 | 12 | 0/12（0%） |

**核心**掌握率偏低：核心知识点是后续课程的地基，建议回到对应 README 条目精读，先用注释写出思路（伪代码）再写代码。

**难点**薄弱：难点允许对照本课示例改写，但每改一次都要能用自己的话说清「为什么必须这样写」；卡住超过 20 分钟就去看示例注释，不要乱改类型。

**边界**薄弱：边界题考的是空输入、0/1 个元素、首尾元素、类型极值（如 `u8::MAX`）、整除与负数、以及中文等多字节字符——写完先自己举三个极端输入试一遍。

## 四、逐条明细（按模块分组）

### lesson_01 · 变量与可变性（通过 0/8，得分 0 · 待加强）

- `kp_01_01` **let 绑定与 mut：默认不可变，需要改值就写 mut**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:50`
  - 待实现函数：`exercise_01_01_mut_counter`
  - 实现要求：用 let mut 累加器循环 times 次、每次 +2，返回结果
  - 作者复习指引：复习 lesson_01 示例 1（immutable_by_default）与示例 2（mut）：`let x = 1; x = 2;` 会报 error[E0384]，必须写成 `let mut x = 1;`。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_02` **变量遮蔽（shadowing）：同名 let 覆盖旧绑定**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:113`
  - 待实现函数：`exercise_01_02_shadowing_chain`
  - 实现要求：用两次 let 遮蔽：先 trim()，再取 len()，返回字节长度
  - 作者复习指引：复习 lesson_01 示例 3（shadowing）：遮蔽是「新建一个同名绑定」，不是修改旧值，所以不需要 mut，也可以换类型。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_03` **遮蔽可以改变类型，mut 不能（这一条是两者的本质区别）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:180`
  - 待实现函数：`exercise_01_03_shadowing_changes_type`
  - 实现要求：返回 (原始字节长度, trim 后 parse 成 i32 再乘 2)
  - 作者复习指引：复习 lesson_01 示例 3 与示例 7 的错误 2：`let mut v = 5; v = "hi";` 报 E0308，而 `let v = 5; let v = "hi";` 合法——遮蔽新建绑定，允许类型变化。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_04` **const 与 static：编译期常量 vs 静态变量（命名用 SCREAMING_SNAKE_CASE）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:243`
  - 待实现函数：`exercise_01_04_const_and_static`
  - 实现要求：函数内声明 const MAX_RETRY: u32 = 3 与 static APP_NAME: &str = "assessment"，返回 (MAX_RETRY * 2, APP_NAME, APP_NAME.len())
  - 作者复习指引：复习 lesson_01 示例 5（const_and_static）：const 在编译期求值、可写在任何作用域；static 有固定内存地址、全程只有一个实例；两者都必须显式写类型。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_05` **类型推断与显式标注：能推断就推断，推断不了必须标注**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:294`
  - 待实现函数：`exercise_01_05_inference_and_annotation`
  - 实现要求：返回 (40 + 2 + "7".parse::<i32>().unwrap(), 19.9f64, 3usize)
  - 作者复习指引：复习 lesson_01 示例 4（type_inference_and_annotation）与示例 7 的错误 5：`.parse()` 与 `Vec::new()` 这类无法从上下文推断目标类型的地方，必须显式标注。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_06` **作用域与遮蔽：内层 let 遮蔽不影响外层绑定的值**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:348`
  - 待实现函数：`exercise_01_06_scope_shadowing`
  - 实现要求：外层 x=10，块内遮蔽 x=x*2，返回 (内层 20, 外层 10)
  - 作者复习指引：复习 lesson_01 示例 3 的作用域部分：遮蔽是「新绑定」，内层块结束时旧绑定重新可见。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_07` **const 表达式：单位换算在编译期完成，零运行期开销**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:395`
  - 待实现函数：`exercise_01_07_const_expression`
  - 实现要求：声明 KB / MB（= KB * 1024）/ SECONDS_PER_DAY，返回 (MB, SECONDS_PER_DAY)
  - 作者复习指引：复习 lesson_01 示例 5：`const KB: usize = 1024; const MB: usize = KB * 1024;` 这类写法在编译期就求值，运行期只是一个立即数。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_08` **常见错误诊断：E0384 / E0308 / E0381 分别对应哪类错误**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_01_variables_mutability.rs:445`
  - 待实现函数：`exercise_01_08_diagnose_errors`
  - 实现要求：返回 ["E0384", "E0308", "E0381"]（二次赋值 / 类型不匹配 / 未初始化）
  - 作者复习指引：复习 lesson_01 示例 7（common_mistakes）：本课列的 5 个坑里，前三个分别是「二次赋值给不可变变量」「mut 变量赋了不同类型」「未初始化就使用」。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`

### lesson_02 · 数据类型（通过 0/8，得分 0 · 待加强）

- `kp_02_01` **整数类型与字面量：位宽决定取值范围，字面量用后缀指定类型**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:53`
  - 待实现函数：`exercise_02_01_integer_literals`
  - 实现要求：返回 (255u8, -32768i16, 1_000_000u32, 9_000_000_000u64)
  - 作者复习指引：复习 lesson_02 示例 1（integer_types）：整数默认是 i32；u8 的范围是 0..=255，i16 的范围是 -32768..=32767；下划线只是可读性分隔符，后缀（u8/i16/u32/u64）才决定类型。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_02` **整数溢出：wrapping_add / saturating_add / overflowing_add / checked_add 的区别**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:126`
  - 待实现函数：`exercise_02_02_overflow_handling`
  - 实现要求：以 250u8 加 10，返回 (wrapping_add, saturating_add, overflowing_add().0, checked_add().is_none())
  - 作者复习指引：复习 lesson_02 示例 8 的错误 1（common_mistakes）：debug 构建下 `250u8 + 10` 会直接 panic（attempt to add with overflow），必须改用四个显式方法之一；示例 1 里的 `u8::MAX` 是理解「溢出边界」的起点。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_03` **浮点基础：默认 f64、0.1 + 0.2 != 0.3、NaN 不等于任何值（包括自身）**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:200`
  - 待实现函数：`exercise_02_03_float_basics`
  - 实现要求：返回 (7.0 / 2.0, 0.1 + 0.2, (0.1 + 0.2) == 0.3, f64::NAN == f64::NAN)
  - 作者复习指引：复习 lesson_02 示例 2（float_types）与示例 8 的错误 5：浮点数不能用 `==` 判断「相等」，要比差值 `(a - b).abs() < 1e-9`；f64 的 NaN 与任何值比较都是 false。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_04` **布尔与字符：bool 只有 true/false，char 是 4 字节的 Unicode 标量值**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:269`
  - 待实现函数：`exercise_02_04_bool_and_char`
  - 实现要求：返回 (true && !false, '中', '中' as u32, '中'.len_utf8())
  - 作者复习指引：复习 lesson_02 示例 3（bool_and_char_literals）：char 用单引号书写、占 4 字节；`as u32` 得到 Unicode 码点，`len_utf8()` 得到这个字符在 UTF-8 里占几个字节。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_05` **元组解构：一次把元组拆成多个变量，再参与运算**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:337`
  - 待实现函数：`exercise_02_05_tuple_destructure`
  - 实现要求：用 let (a, b, c) = (3, -5, 8); 解构，返回 (a + b + c, a * b, c - a)
  - 作者复习指引：复习 lesson_02 示例 4（tuples）：解构是 `let (a, b, c) = 元组;`；也可以用 `.0/.1/.2` 按位置访问，两种写法等价，但解构更适合「一次拿到全部字段」。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_06` **数组安全访问：get 返回 Option，比 [] 索引更安全（越界不 panic）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:401`
  - 待实现函数：`exercise_02_06_array_get`
  - 实现要求：对 [10, 20, 30] 用 .get(index).copied() 安全取值，越界返回 None
  - 作者复习指引：复习 lesson_02 示例 5（arrays）与示例 8 的错误 2：`数组[index]` 越界会 panic（index out of bounds），而 `数组.get(index)` 返回 `Option`，越界只是 `None`。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_07` **类型转换：`as` 静默截断，`u8::try_from` 做范围检查并返回 Result**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:474`
  - 待实现函数：`exercise_02_07_conversions`
  - 实现要求：返回 (value as u32, u8::try_from(value).ok())
  - 作者复习指引：复习 lesson_02 示例 6（type_conversion）：`300i32 as u8` 得到 44（二进制截断）；`u8::try_from(300)` 得到 `Err`；`From` 只用于「一定成功」的加宽转换，如 `u32::from(1000u16)`。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_08` **元组比较与数组长度：元组按字典序逐元素比较，数组长度是类型的一部分**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_02_data_types.rs:571`
  - 待实现函数：`exercise_02_08_tuple_compare`
  - 实现要求：返回 ((1, 2) < (1, 3), (1, 2) < (0, 9), [0u8; 4].len())
  - 作者复习指引：复习 lesson_02 示例 4（tuples）与示例 5（arrays）：元组实现了 `PartialOrd`，从第 0 个元素开始逐个比较、第一个不同的元素决定结果；`[u8; 4]` 里的 4 是类型的一部分。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`

### lesson_03 · 函数（通过 0/8，得分 0 · 待加强）

- `kp_03_01` **函数定义：参数逐个标注类型，尾表达式就是返回值**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:60`
  - 待实现函数：`exercise_03_01_add`
  - 实现要求：用尾表达式返回 a + b（不加分号、不写 return）
  - 作者复习指引：复习 lesson_03 示例 1（define_and_call）与示例 2（multiple_params_and_return）：签名写成 `fn add(a: i32, b: i32) -> i32`，函数体最后一行 `a + b` 不带分号；示例 1 还演示了「调用时机与定义顺序无关」。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_02` **多返回值：用一个元组同时带回面积与周长**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:128`
  - 待实现函数：`exercise_03_02_rect`
  - 实现要求：返回 (面积 width * height, 周长 2 * (width + height))
  - 作者复习指引：复习 lesson_03 示例 2（multiple_params_and_return）：`divide_with_remainder` 用 `-> (i32, i32)` 一次返回商和余数，调用方用 `let (q, r) = ...` 解构接收；参数类型也必须逐个写全，不能省略。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_03` **语句与表达式：`if` 是表达式，放在函数体末尾就是返回值**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:198`
  - 待实现函数：`exercise_03_03_tail_expression`
  - 实现要求：用 if 尾表达式：n > 0 时返回 n * 2，否则返回 0
  - 作者复习指引：复习 lesson_03 示例 3（statement_vs_expression）：`let sign_label = if x > 0 { ... } else { ... };` 说明 if 会产生值；示例 7 的错误 3 演示了尾表达式多写分号会得到 `()`（E0308）。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_04` **提前 return + 尾表达式：先挡掉非法输入，再走正常路径**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:267`
  - 待实现函数：`exercise_03_04_early_return`
  - 实现要求：n < 0 时提前 return -1，否则用尾表达式返回 n * n
  - 作者复习指引：复习 lesson_03 示例 3 的 `positive_only`：`if value <= 0 { return 0; }` 提前返回，末尾直接写 `value` 作为尾表达式；示例 6 的 `safe_divide` 也用了同样的「守卫」写法。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_05` **函数指针：`fn(i32) -> i32` 是类型，函数名与不捕获环境的闭包都能当值传**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:336`
  - 待实现函数：`exercise_03_05_apply`
  - 实现要求：用尾表达式返回 f(value)
  - 作者复习指引：复习 lesson_03 示例 4（function_pointer）：`let operation: fn(i32, i32) -> i32 = add;` 说明函数名可以直接赋给函数指针；`let increment: fn(i32) -> i32 = \|value\| value + 1;` 说明不捕获外部变量的闭包能强制转换成 fn 指针（示例 1 把折扣率写成常量，正是为此）。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_06` **发散函数（`!`）：永不返回的函数可被强制转换成任意类型**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:403`
  - 失败原因：实际：false
  - 提示：exercise_03_06_diverge 仍是未实现的占位（todo_exercise） · 骨架态下它本来就会 panic：先删掉 `assessment_harness::todo_exercise(...)` 写上实现，再谈「必然 panic」
  - 待实现函数：`exercise_03_06_diverge`
  - 实现要求：函数内定义 fn fail(msg: &str) -> ! { panic!("{msg}") }，调用它让本函数永不返回
  - 作者复习指引：复习 lesson_03 示例 5（diverging_function）：`fn exit_with_error(message: &str) -> !` 能放在需要 i32 的 match 分支里，因为 `!` 可以强转成任何类型；示例 7 的错误 6 提醒：发散函数之后的语句不可达，编译器会给出 unreachable_code 警告。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_07` **函数指针数组：按顺序把每个函数作用到 value 上（空数组原样返回）**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:496`
  - 待实现函数：`exercise_03_07_pipeline`
  - 实现要求：按顺序把 ops 里的每个函数作用到 value 上并返回结果；空切片原样返回 value
  - 作者复习指引：复习 lesson_03 示例 4（function_pointer）的「策略表」：`let strategies: [fn(i32, i32) -> i32; 3] = [add, subtract, max_of];`；示例 6 的 `pipeline` 演示了把函数指针放进数组依次执行、组合小函数完成完整计算。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_08` **常见错误诊断：函数课前三个坑分别报什么（两个无编号解析错误 + E0308）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_03_functions.rs:579`
  - 待实现函数：`exercise_03_08_diagnose_errors`
  - 实现要求：返回 ["无编号", "E0308", "E0308"]（参数漏类型 / 漏返回类型 / 尾表达式多分号）
  - 作者复习指引：复习 lesson_03 示例 7（common_mistakes）：错误 1「参数漏写类型」报的是解析阶段的 `expected one of `:`, `@`, or `\|``，课程注释明确写了**没有 E 编号**；错误 2「漏写返回类型却用表达式返回值」报 E0308（expected `()`, found `i32`）；错误 3「尾表达式多写分号」同样报 E0308（expected `i32`, found `()`）。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`

### lesson_04 · 流程控制（通过 0/8，得分 0 · 待加强）

- `kp_04_01` **if 是表达式：分支的值可以直接做函数尾表达式**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:53`
  - 待实现函数：`exercise_04_01_if_expression`
  - 实现要求：用 if / else if / else 表达式分别返回 "正数" / "负数" / "零"，并直接作为函数尾表达式
  - 作者复习指引：复习 lesson_04 示例 1（if_expression）：`if` 有值、两个分支必须同类型，所以 `if n > 0 { "正数" } else { "负数" }` 可以直接当返回值，末尾**不要**写分号，也**不要**写 `return`。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_02` **if / else if 链：条件自上而下判断，命中第一个为真的分支**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:128`
  - 待实现函数：`exercise_04_02_grade`
  - 实现要求：用 if / else if 链实现：>=90 得 'A'，>=80 得 'B'，>=60 得 'C'，其余（含 >100）得 'D'
  - 作者复习指引：复习 lesson_04 示例 2（if_else_chain）：多分支按**从高到低**的顺序比较，顺序写反会让高分先落进低档；最后的 `else` 是兜底分支，本题约定分数 > 100 这类越界输入也落到 `'D'`。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_03` **loop 与 break 带值：把循环当成有返回值的表达式**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:220`
  - 待实现函数：`exercise_04_03_first_square_above`
  - 实现要求：用 loop 递增 n 并用 break n 带值返回第一个满足 n * n > limit 的 n
  - 作者复习指引：复习 lesson_04 示例 4（break_value_and_continue）：`let x = loop { … break 值; };` 是 Rust 特有的写法——`loop` 是表达式，`break 值` 就是它的值；只有 `loop` 支持 break 带值，`while` / `for` 不行（会报 E0571）。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_04` **while 循环与 continue：跳过奇数，只累加偶数**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:290`
  - 待实现函数：`exercise_04_04_sum_even`
  - 实现要求：用 while + continue 累加 1..=limit 中的所有偶数并返回
  - 作者复习指引：复习 lesson_04 示例 3（loop_while_for）与示例 4 的 continue 部分：`while` 先判断条件再执行循环体，因此必须保证循环变量趋向结束；`continue` 立即进入下一轮，它后面的累加语句本轮不会执行。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_05` **for 遍历切片：元素个数由迭代器决定，不需要手写下标**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:366`
  - 待实现函数：`exercise_04_05_for_sum`
  - 实现要求：用 for 遍历切片累加所有元素并返回（空切片返回 0）
  - 作者复习指引：复习 lesson_04 示例 3 的 for 部分：`for value in items { … }` 直接遍历 `&[i32]`，因为切片实现了 `IntoIterator`；`for index in 0..items.len()` 也能写，但一旦下标写错就越界 panic，所以优先用 for-each。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_06` **循环标签（'outer:）：break 'outer 一次跳出两层嵌套循环**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:442`
  - 待实现函数：`exercise_04_06_find_in_matrix`
  - 实现要求：用 'outer 标签 + break 'outer 做二维查找，返回 Some((行, 列))，找不到返回 None
  - 作者复习指引：复习 lesson_04 示例 5（loop_labels）：给外层循环写 `'outer: for row in …`，内层用 `break 'outer;` 跳出外层——这比用布尔标志位「绕圈」退出清晰得多；找不到目标时两个循环都自然跑完，函数尾部返回 `None`。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_07` **循环实现 Collatz 步数：偶数减半、奇数 3n+1，数到 1 为止**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:519`
  - 待实现函数：`exercise_04_07_collatz_steps`
  - 实现要求：用循环统计 Collatz 步数：偶数 /2、奇数 3n+1，直到 1；n = 1 时返回 0
  - 作者复习指引：复习 lesson_04 示例 3 与示例 4：这一题考的是「循环 + 计数器 + 终止条件」三件套；关键是**先判断是否已经等于 1**——n 本来就是 1 时一步都不该走，返回 0。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_08` **常见错误诊断：E0308（loop 的值）/ E0571（while 里 break 带值）/ E0425（循环外用了循环变量）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_04_control_flow.rs:590`
  - 待实现函数：`exercise_04_08_diagnose_errors`
  - 实现要求：返回 ["E0308", "E0571", "E0425"]（loop 的值 / while 里 break 带值 / 循环外用了循环变量）
  - 作者复习指引：复习 lesson_04 示例 7（common_mistakes）：本课列了 7 个坑，其中**有编译器错误编号**的第一个是「loop 体最后是表达式却没写 break 带值」（E0308），第二个是「while 循环里 break 带值」（E0571），第三个是「循环变量在循环外被使用」（E0425）；注意「while 条件永远不变」属于运行期死循环，没有错误编号。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`

### lesson_05 · 所有权系统（通过 0/9，得分 0 · 待加强）

- `kp_05_01` **move 语义：String 传参即移交所有权，函数通过返回值把所有权交还**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:81`
  - 待实现函数：`exercise_05_01_move_semantics`
  - 实现要求：按值接收 String，返回 (同一个 String, 它的字节长度)
  - 作者复习指引：复习 lesson_05 示例 1（three_rules）与示例 2（move_vs_copy）：`String` 没有实现 Copy，所以 `let t = s;` 或把 `s` 传进函数都是 move；move 之后原变量**不可再使用**，否则报 `error[E0382]: borrow of moved value`。本题的练习函数按值接收 String、再把同一个 String 装进元组返回，所有权于是回到调用方。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_02` **Copy 与 Clone：i32 赋值是复制、String 必须显式 .clone()**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:167`
  - 待实现函数：`exercise_05_02_copy_vs_clone`
  - 实现要求：返回 (2 * n, text.clone() 得到的字符串, 原 text 的字节长度)
  - 作者复习指引：复习 lesson_05 示例 2（move_vs_copy）与示例 3（clone_explicit）：`i32` 实现了 `Copy`，赋值/传参只是复制 4 个字节，原变量照样能用，所以 `n` 可以在同一个表达式里出现两次；`String` 没有 Copy，想要「一份留着、一份交出去」只能写 `.clone()`，它在堆上真分配、真拷贝。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_03` **共享借用：同一时刻可以存在任意多个 &T（只读共享是安全的）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:277`
  - 待实现函数：`exercise_05_03_shared_borrows`
  - 实现要求：在同一作用域持有两个不可变借用，返回 (len1, len2, len1 + len2)
  - 作者复习指引：复习 lesson_05 示例 5（borrow_rules）：`let a: &String = &text; let b: &String = &text;` 两个不可变借用同时存在完全合法，因为只读、不会有人看到「写了一半」的数据；反过来说，只要出现一个 `&mut`，就要求「没有其它任何借用同时活着」。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_04` **可变借用：两次 &mut 只要不重叠就合法（每次借用用完即结束）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:349`
  - 待实现函数：`exercise_05_04_mutable_borrow`
  - 实现要求：往 v 里 push 1 和 2，返回全部元素之和
  - 作者复习指引：复习 lesson_05 示例 6（mutable_borrow）与示例 7（nll）：`v.push(1); v.push(2);` 看起来是两次可变借用，但第一次借用在 `push` 返回时就结束了，所以第二次借用不冲突——这就是 NLL（非词法生命周期）；反过来，`let r1 = &mut v; let r2 = &mut v;` 两个引用同时活着就报 `error[E0499]`。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_05` **返回切片：最长单词是原句的一部分，返回 &str 而不是 String**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:431`
  - 待实现函数：`exercise_05_05_longest_word`
  - 实现要求：用 split_whitespace + 严格大于比较返回最长单词的切片；空串与纯空格返回 ""
  - 作者复习指引：复习 lesson_05 示例 9（typical_scenario_longest_word）：返回值写成 `&str` 并在省略生命周期的情况下由编译器绑定到唯一的引用参数 `text` 上；实现上用 `longest.len()` 做「严格大于」比较，长度相同时保留**先出现**的那个单词，而 `longest` 的初值必须是 `""`，这样空串才能安全返回空切片。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_06` **len 与 capacity：len 是已有元素个数，capacity 是已分配的坑位数**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:528`
  - 待实现函数：`exercise_05_06_vec_len_capacity`
  - 实现要求：返回 (with_capacity(10) 的 len, 其 capacity, push(1) 后的 len, 其 capacity) = (0, 10, 1, 10)
  - 作者复习指引：复习 lesson_05 示例 4（stack_and_heap）：`Vec::with_capacity(10)` 只预留坑位、不放元素，所以 `len() == 0` 而 `capacity() == 10`；`push(1)` 放了一个元素，`len()` 变成 1，而预留的容量足够，`capacity()` 仍是 10（不会被无故放大）。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_07` **NLL 边界：先把值取出来（借用结束），再可变借用修改**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:580`
  - 待实现函数：`exercise_05_07_nll_boundary`
  - 实现要求：先解引用取出最后一个元素（借用结束），再 push(99)，返回 (取到的值, 新长度)
  - 作者复习指引：复习 lesson_05 示例 7（nll_non_lexical_lifetime）：借用的有效范围由**最后一次使用**决定；本题中 `last` 是一个 `i32`（Copy 类型），`*v.last().unwrap()` 一取值，不可变借用就结束了，所以后面 `v.push(99)` 完全不冲突；如果写成 `let last = v.last().unwrap();`（保留 `&i32`）再 push，就会报 `error[E0502]`。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_08` **常见错误诊断：E0382（move 后使用）/ E0499（两个可变借用）/ E0502（不可变与可变重叠）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:675`
  - 待实现函数：`exercise_05_08_diagnose_errors`
  - 实现要求：返回 ["E0382", "E0499", "E0502"]（move 后使用 / 两个可变借用 / 不可变与可变重叠）
  - 作者复习指引：复习 lesson_05 示例 10（common_mistakes）：错误 1 是「move 之后继续用原变量」（E0382），错误 2 是「同一时刻两个 `&mut`」（E0499），错误 3 是「不可变借用与可变借用重叠」（E0502）；这三个编号覆盖了初学者 90% 的所有权编译失败。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_09` **Drop 顺序：作用域结束时后声明的先 drop，内层作用域先于外层**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:729`
  - 待实现函数：`exercise_05_09_drop_order`
  - 实现要求：用 Tracer 安排作用域（外层 a；内层块 b；之后 c），返回 take_drop_log()，期望 ["b", "c", "a"]
  - 作者复习指引：复习 lesson_05 示例 1（three_rules）的规则三与示例 4（stack_and_heap）：值离开作用域时自动 drop；同一个作用域内**后声明的先 drop**（逆序），内层块整体先于外层块结束。本题期望的顺序是 ["b", "c", "a"]。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`

### lesson_06 · 结构体（通过 0/8，得分 0 · 待加强）

- `kp_06_01` **结构体定义与实例化：字段写全、用点号访问**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:112`
  - 待实现函数：`exercise_06_01_define_and_instantiate`
  - 实现要求：在函数内定义 struct Rectangle { width: u32, height: u32 }，实例化 30×50，用点号读取字段并返回面积 1500
  - 作者复习指引：复习 lesson_06 示例 1（define_and_instantiate）：先 `struct Rectangle { width: u32, height: u32 }` 定义形状，再 `Rectangle { width: 30, height: 50 }` 实例化，然后用 `rect.width` / `rect.height` 点号读字段——实例化时字段必须写全（漏了会报 E0063）。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_02` **字段初始化简写与结构体更新语法（..other）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:166`
  - 待实现函数：`exercise_06_02_shorthand_and_update`
  - 实现要求：函数内定义 struct Config { host: String, port: u16, retries: u8 }：用简写造 first（retries = 3），用 `Config { port: 9090, ..first }` 造 second，返回 (first.port, second.port, second.host, second.retries)
  - 作者复习指引：复习 lesson_06 示例 2（field_init_shorthand）与示例 3（struct_update_syntax）：字段名与变量名相同时可以只写一次名字；`Config { port: 9090, ..first }` 只覆盖写出来的字段，其余字段从 `first` 补齐，非 Copy 字段（String）会被 move。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_03` **方法接收者：&self 读、&mut self 改、self 消费**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:265`
  - 待实现函数：`Counter::add`
  - 实现要求：用 &mut self 接收者把 delta 累加到 value 字段上
  - 作者复习指引：复习 lesson_06 示例 6（methods_and_receivers）：`&self` 只读借用、`&mut self` 可写借用、`self` 按值接收会把调用者的所有权 move 进方法（调用后原变量失效）。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_04` **关联函数与 Self：用 `类型名::函数名` 调用，不依赖实例**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:383`
  - 待实现函数：`Point::origin`
  - 实现要求：关联函数：返回原点 (0, 0)
  - 作者复习指引：复习 lesson_06 示例 7（associated_functions）：`impl` 块里不带 self 的函数是关联函数，用 `Point::origin()` 调用；返回类型写 `Self` 就是当前类型的别名，等价于写 `Point`。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_05` **元组结构体（.0 位置访问）与单元结构体（零大小标记）**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:492`
  - 待实现函数：`exercise_06_05_tuple_and_unit_struct`
  - 实现要求：函数内定义元组结构体 Meters(i32) / Named(String) 与单元结构体 Marker，构造 Marker 值，返回 (Meters(5).0, Named(String::from("ab")).0.len(), ())
  - 作者复习指引：复习 lesson_06 示例 4（tuple_struct）与示例 5（unit_struct）：元组结构体字段没有名字，只能用 `.0` / `.1` 按位置访问；单元结构体没有任何字段，值就是类型名本身，占 0 字节。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_06` **手动实现 Display（{}）与派生 Debug（{:?}）的分工**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:555`
  - 待实现函数：`Temperature::fmt`
  - 实现要求：实现 Display：用 write!(f, "{:.1}°C", self.celsius) 输出，例如 25.0 -> 25.0°C
  - 作者复习指引：复习 lesson_06 示例 8（debug_and_display）：`#[derive(Debug)]` 给的是面向开发者的 `{:?}`，而 `{}` 必须手动 `impl std::fmt::Display`；Display 负责把内部数据翻译成用户语言。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_07` **链式调用：构造器方法消费 self 并返回 Self**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:625`
  - 待实现函数：`QueryBuilder::new`
  - 实现要求：用 table.to_string() 与 limit: None 构造 QueryBuilder 并返回 Self
  - 作者复习指引：复习 lesson_06 示例 6 的消耗型方法（`self` 接收者）与示例 7 的构造函数惯例：链式调用的每一步都要把 `self` 交出去（返回 `Self`），最后一步 `build` 才产出结果；中间任何一步返回 `&mut Self` 或 `()` 都会让链断掉。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_08` **部分移动：move 出 String 字段的同时仍可读取 Copy 字段**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_06_structs.rs:747`
  - 待实现函数：`exercise_06_08_partial_move`
  - 实现要求：把非 Copy 的 order.note 移动出来、同时读取 Copy 的 order.id，返回 (id, note)
  - 作者复习指引：复习 lesson_06 示例 3 与示例 10 的错误 7：从结构体里把非 Copy 字段（String）move 出来之后，该实例不能再用作整体，但**没被移走**的字段（如 u32 的 id）仍然可以读。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`

### lesson_07 · 枚举与模式匹配（通过 0/9，得分 0 · 待加强）

- `kp_07_01` **match 的穷尽性：每个变体都要有分支**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:85`
  - 待实现函数：`exercise_07_01_area`
  - 实现要求：用 match 覆盖 Shape 的三个变体：Circle(r) 返回 PI * r * r，Rectangle { width, height } 返回 width * height，Point 返回 0.0
  - 作者复习指引：复习 lesson_07 示例 4（match_basics_and_wildcard）：`match` 必须覆盖所有可能，少写一个变体会报 `error[E0004]: non-exhaustive patterns`；元组变体用 `Shape::Circle(r)` 取值，结构体变体用 `Shape::Rectangle { width, height }` 按字段名绑定。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_02` **Option<T>：把「可能没有值」写进类型里**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:171`
  - 待实现函数：`exercise_07_02_option_sum`
  - 实现要求：两个都是 Some 时返回 Some(x + y)，任意一边是 None 时返回 None
  - 作者复习指引：复习 lesson_07 示例 2（option）：`Option` 只有 `Some(值)` 与 `None` 两个变体，处理时必须显式应对 `None`；`match (a, b)` 一次匹配两个 Option 是最直观的写法。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_03` **Result<T, E>：成功取 Ok，失败给出可读原因**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:245`
  - 待实现函数：`exercise_07_03_parse_port`
  - 实现要求：trim 之后 parse::<u16>()：成功返回 Ok，失败返回 Err(String)，且错误信息里必须包含「端口」二字
  - 作者复习指引：复习 lesson_07 示例 3（result）：`Result` 的 `Err` 分支携带「为什么失败」，比 `Option` 更适合解析类操作；`.parse::<u16>()` 的 `Err` 类型是 `ParseIntError`，要转成自己的 `String` 说明。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_04` **或模式 `\|` 与通配 `_`：合并同结果分支、兜住其余取值**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:334`
  - 待实现函数：`exercise_07_04_classify`
  - 实现要求：match n：1 => "一"，2 \| 3 => "二或三"，_ => "其它"
  - 作者复习指引：复习 lesson_07 示例 4 的 `\|` 与 `_` 部分：`2 \| 3` 把两个模式合并到同一个分支；`_` 兜住所有剩余取值，但它必须写在**最后一个**分支。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_05` **匹配守卫（if）与 @ 绑定：范围判断 + 取到原值**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:415`
  - 待实现函数：`exercise_07_05_guarded`
  - 实现要求：match n：x @ 1..=9 if x % 2 == 0 => "小偶数"，1..=9 => "小奇数"，_ => "超范围"
  - 作者复习指引：复习 lesson_07 示例 5（guards_and_bindings）：守卫写在模式之后、`=>` 之前，例如 `n if n % 2 == 0`；`@` 绑定既做范围判断又把值绑给变量，例如 `s @ 0..=59`。分支顺序很重要：带守卫的分支必须写在同范围的裸模式之前。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_06` **while let：不断取出直到 None，None 用 continue 跳过**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:501`
  - 待实现函数：`exercise_07_06_if_let_while_let`
  - 实现要求：用 while let Some(item) = iter.next() 遍历：Some(v) 累加，None 用 continue 跳过，返回总和
  - 作者复习指引：复习 lesson_07 示例 6（if_let_and_while_let）：`while let Some(v) = iter.next()` 每次循环都尝试取出一个值，取出 `None` 时循环自然结束，因此特别适合「消耗式遍历」。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_07` **let else：失败分支发散，成功路径无需再嵌套**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:573`
  - 待实现函数：`exercise_07_07_let_else`
  - 实现要求：用 let Some(first) = text.lines().next() else { return 0; }，返回 first.len()（字节长度）
  - 作者复习指引：复习 lesson_07 示例 7（let_else）：`let ... else { ... }` 的 else 块必须发散（`return` / `continue` / `break` / `panic!`），因此后面的代码可以直接使用解包后的值。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_08` **枚举 + match 的典型用法：文本 → 结构化命令**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:645`
  - 待实现函数：`exercise_07_08_parse_command`
  - 实现要求："move x y" -> Move { x, y }、"say 内容" -> Say(内容)、"quit" -> Quit，参数缺失 / 非法 / 空串 / 未知命令一律返回 None
  - 作者复习指引：复习 lesson_07 示例 8（typical_scenario_command_parser）：`split_whitespace()` 天然处理多余空格，`parts.next()` 逐个取词，取不到参数就返回 `None`；解析成功后用枚举变体（元组变体 / 结构体变体 / 无数据变体）把结果结构化。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_09` **常见错误诊断：E0004 / E0308 / E0005 分别对应哪类错误**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:741`
  - 待实现函数：`exercise_07_09_diagnose_errors`
  - 实现要求：返回 ["E0004", "E0308", "E0005"]（match 不穷尽 / 模式类型不匹配 / let 用了可反驳模式）
  - 作者复习指引：复习 lesson_07 示例 9（common_mistakes）的错误 1~3：错误 1 是 match 分支没写全（non-exhaustive patterns），错误 2 是模式与值的类型不一致（mismatched types: expected integer, found `&str`），错误 3 是把可反驳模式用在 `let` 位置（refutable pattern in local binding）。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`

### lesson_08 · 常见集合（通过 0/9，得分 0 · 待加强）

- `kp_08_01` **Vec 的创建与 push：`to_vec()` 复制出拥有所有权的向量，`push` 在尾部追加**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:64`
  - 待实现函数：`exercise_08_01_vec_create_and_push`
  - 实现要求：用 input.to_vec() 构造 Vec<i32>，push(extra) 后返回
  - 作者复习指引：复习 lesson_08 示例 1（vec_create_and_push）：`Vec::new` / `vec![]` / `Vec::with_capacity` 三种创建方式，以及 `push` 把元素追加到末尾（必要时扩容）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_02` **Vec 的索引与安全访问：`get` / `first` 返回 `Option`，`[]` 越界会 panic**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:134`
  - 待实现函数：`exercise_08_02_vec_access`
  - 实现要求：返回 (input.get(index).copied(), input.first().copied().unwrap_or(0))
  - 作者复习指引：复习 lesson_08 示例 2（vec_access_and_modify）：`v[i]` 越界在运行期 panic （index out of bounds），`v.get(i)` 把「可能越界」表达成 `Option`，`v.first()` 同理。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_03` **Vec 的原地修改：`retain` 按条件删除、`iter_mut` 解引用修改元素**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:205`
  - 待实现函数：`exercise_08_03_vec_mutate`
  - 实现要求：用 retain 删掉负数、iter_mut 翻倍，返回 (len, 元素之和)
  - 作者复习指引：复习 lesson_08 示例 1 与示例 3（vec_iterate_and_capacity）：`retain` 就地保留满足条件的元素（比倒序 `remove` 直观），`for value in v.iter_mut() { *value *= 2; }` 才能修改元素。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_04` **String 与 UTF-8：`len()` 是字节数，`chars().count()` 是字符数**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:275`
  - 待实现函数：`exercise_08_04_string_utf8`
  - 实现要求：返回 (text.len(), text.chars().count())
  - 作者复习指引：复习 lesson_08 示例 4（string_basics_and_utf8）：中文一个字占 3 字节，emoji 占 4 字节，所以 `len()` 常大于 `chars().count()`；按字节下标切分不在字符边界上会 panic。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_05` **字符串拼接三种方式：`push_str` 原地追加、`+` 消耗左侧、`format!` 不消耗任何操作数**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:351`
  - 待实现函数：`exercise_08_05_concat_three_ways`
  - 实现要求：分别用 push_str 循环、`+` 运算符、format! 拼出同一个字符串，返回三个结果
  - 作者复习指引：复习 lesson_08 示例 5（string_concat_and_format）：`push_str` 不转移所有权；`a + &b` 里 `a` 的所有权被消耗（之后不能再使用 `a`）；`format!` 返回新 `String`，参数都还可用。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_06` **HashMap 基础：`insert` 覆盖旧值并返回旧值；遍历顺序不固定，输出前必须排序**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:448`
  - 待实现函数：`exercise_08_06_hashmap_basics`
  - 实现要求：用 HashMap 插入（重复键覆盖），收集成 Vec 后按键字典序排序返回
  - 作者复习指引：复习 lesson_08 示例 6（hashmap_basics）：`HashMap` 不保证遍历顺序，所以「需要确定输出」时必须先排序；`insert` 对已存在的键会覆盖旧值（并返回被覆盖的旧值）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_07` **entry API 统计词频：`entry(word).or_insert(0)` 一次查找完成「查 + 改」，输出前排序**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:522`
  - 待实现函数：`exercise_08_07_word_frequency`
  - 实现要求：用 entry API 统计词频（小写、按空白切分），按次数降序、同次数按字典序返回
  - 作者复习指引：复习 lesson_08 示例 7 与示例 8（hashmap_entry_api / word_frequency_scenario）：用 `*counts.entry(word).or_insert(0) += 1;` 统计；排序规则是`b.1.cmp(&a.1).then_with(\|\| a.0.cmp(&b.0))`（次数多的在前，次数相同按字典序）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_08` **借用陷阱：`get` 拿到引用期间不能 `insert`（E0502），先把值拷贝出来即可**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:609`
  - 待实现函数：`exercise_08_08_borrow_trap`
  - 实现要求：先 get("a").copied().unwrap_or(0)，再 insert("b", v + 1)，返回 (v, map.len())
  - 作者复习指引：复习 lesson_08 示例 9（ownership_and_borrow_traps）陷阱三：`let value = counter.get("hits").unwrap(); counter.insert("misses", 1);` 会报`error[E0502]: cannot borrow as mutable because it is also borrowed as immutable`；修正就是先 `copied()` 把值取出来，让不可变借用立刻结束。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_09` **常见错误诊断：E0596 / E0282 / E0277 分别对应哪类集合错误**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_08_collections.rs:691`
  - 待实现函数：`exercise_08_09_diagnose_errors`
  - 实现要求：返回 ["E0596", "E0282", "E0277"]（忘写 mut / 推断不出元素类型 / 字符串整数下标）
  - 作者复习指引：复习 lesson_08 示例 10（common_mistakes）：错误 1 是「`Vec::new()` 忘了 mut 就 push」（E0596），错误 2 是「元素类型完全没有来源」的 `Vec::new()`（E0282），错误 3 是「用整数下标访问 `String`」（E0277：`str` 不能用 `{integer}` 索引）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`

### lesson_09 · 包和模块（通过 0/7，得分 0 · 待加强）

- `kp_09_01` **模块基础：`mod math { pub fn ... }` + 路径 `math::add` 调用**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:64`
  - 待实现函数：`exercise_09_01_module_basics`
  - 实现要求：函数内定义 mod math { pub fn add, pub fn double }，返回 add(1, 2) + double(3) = 9
  - 作者复习指引：复习 lesson_09 示例 1（module_basics）：模块用 `mod 名字 { ... }` 声明，里面的项默认私有，必须写 `pub fn` 才能从模块外用 `模块名::函数名` 调用。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_02` **可见性层级：`pub(crate)`（本 crate 可见）/ `pub(super)`（父模块可见）/ 私有（仅本模块可见）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:121`
  - 待实现函数：`exercise_09_02_visibility_levels`
  - 实现要求：用 pub(crate) + pub(super) + 私有 fn 三层可见性组合出 5（如 1 + 3 + 1）
  - 作者复习指引：复习 lesson_09 示例 2（visibility_levels）：四种可见性分别是 `pub`、`pub(crate)`、`pub(super)`、不加修饰（私有，只有本模块及其子模块可见）；访问范围只能缩小不能放大。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_03` **`use` 引入与 `as` 重命名：路径别名不改变可见性，只改调用时写的名字**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:193`
  - 待实现函数：`exercise_09_03_use_and_rename`
  - 实现要求：用 use ... as ... 重命名两条同名路径，返回 (calc(2), other_calc(2)) = (200, 7)
  - 作者复习指引：复习 lesson_09 示例 3（use_and_rename）：`use 路径::项 as 别名;` 之后就能用别名调用；别名可以解决同名冲突，也可以让调用点更短。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_04` **`pub use` 重导出：门面模块让调用方只记住一条短路径**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:253`
  - 待实现函数：`exercise_09_04_pub_use_reexport`
  - 实现要求：定义 inner::Api 与 facade 的 pub(crate) use 重导出，经 facade::Api 调用 run() 返回 42
  - 作者复习指引：复习 lesson_09 示例 4（pub_use_reexport）：`pub use` 把内部的项「搬」到当前模块名下，内部结构怎么改都不影响调用方；重导出的可见性不能放大（`pub(crate)` 项不能被 `pub use` 到 crate 外，否则报 `error[E0364]` / `error[E0365]`）。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_05` **模块路径：`super::` 父模块、`self::` 当前模块、`crate::` 绝对路径（crate 根）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:324`
  - 待实现函数：`exercise_09_05_module_paths`
  - 实现要求：嵌套模块里用 super::(6) + self::(5) + crate::M.len()(9) 三种路径求和，返回 20
  - 作者复习指引：复习 lesson_09 示例 5（module_paths）：`crate::` 从 crate 根出发，任何模块里都能用；`self::foo` 与 `foo` 等价，强调「在本模块里找」；`super::foo` 往上一级找，子模块因此能访问父模块里连 `pub` 都没有的私有项。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_06` **迷你库：`pub mod` 暴露 API、私有辅助函数隐藏实现、`pub use` 汇总成一条门面**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:386`
  - 待实现函数：`exercise_09_06_mini_library`
  - 实现要求：组织 pub(crate) mod math（私有 clamp + pub sum）与 pub mod facade（pub use 重导出 + VERSION），返回 (facade::total(&[1, 4, 5]) = 10, "mini-calc 1.0")
  - 作者复习指引：复习 lesson_09 示例 8（mini_library_scenario）：一个「库」对外只需要稳定、简短的公开 API；内部模块划分、私有字段、私有辅助函数都可以随时重构，只要门面 `pub use` 的路径不变。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_07` **常见错误诊断：E0603（私有项）/ E0425（路径写错）/ E0255（use 与本地定义同名）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_09_packages_modules.rs:468`
  - 待实现函数：`exercise_09_07_diagnose_errors`
  - 实现要求：返回 ["E0603", "E0425", "E0255"]（私有项 / 路径写错 / use 与本地定义同名）
  - 作者复习指引：复习 lesson_09 示例 7（common_mistakes）：错误 1 是「调用私有函数或读取私有常量」（E0603），错误 2 是「`pub(super)` 项被更外层的模块调用」——编译器同样报 E0603，错误 3 是「路径写错，少了层级」的 `lessons::add`（E0425）。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`

### lesson_10 · 错误处理（通过 0/9，得分 0 · 待加强）

- `kp_10_01` **可恢复错误：用 Result 表达「除数为 0」，把决定权交给调用方**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:137`
  - 待实现函数：`exercise_10_01_divide`
  - 实现要求：b == 0 时返回 Err（信息里含 "0"），否则返回 Ok(a / b)
  - 作者复习指引：复习 lesson_10 示例 3（result_and_match）与示例 10（when_to_panic_or_result）：外部输入与可预期的运行结果用 Result 表达，只有「调用方违反了契约」才 panic。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_02` **unwrap / expect：成功时取值，失败时 panic（风险就在这里）**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:218`
  - 待实现函数：`exercise_10_02_unwrap_and_expect`
  - 实现要求：Some(7).unwrap() 与 Ok::<i32, String>(42).expect("不会失败")，返回 (7, 42)
  - 作者复习指引：复习 lesson_10 示例 2（unwrap_and_expect）：unwrap 出错时用默认信息 panic，expect("...") 可以自带上下文；两者都只适合「已经论证过不会失败」的场合。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_03` **用 match 处理 Result：两个分支都必须处理，编译器强制你面对错误**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:282`
  - 待实现函数：`exercise_10_03_match_result`
  - 实现要求：match text.trim().parse::<i32>()：Ok(n) → "数字：{n}"，Err(_) → "不是数字"
  - 作者复习指引：复习 lesson_10 示例 3（result_and_match）：`match` 的 Ok / Err 两个分支必须都写，失败时返回固定文案；`if let` 只适合只关心一侧的场景。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_04` **`?` 运算符：Ok 时取值继续执行，Err 时立刻 return Err(...)**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:353`
  - 待实现函数：`exercise_10_04_question_mark`
  - 实现要求：两个参数各自 trim 后 parse::<i32>()，用 `?` 传播错误（先 map_err 成 String），返回 Ok(a + b)
  - 作者复习指引：复习 lesson_10 示例 4（question_mark_operator）：`?` 只能用在返回 Result / Option 的函数里；两个 `?` 顺序传播，任何一个失败都会提前返回。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_05` **自定义错误类型：用枚举区分失败原因，用 Display 说人话**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:441`
  - 待实现函数：`exercise_10_05_parse`
  - 实现要求：空/纯空白 → Err(Empty)；解析失败 → Err(NotNumber(原文))；成功 → Ok(n)
  - 作者复习指引：复习 lesson_10 示例 5（custom_error_type）：错误类型要同时实现 Display 与std::error::Error；Display 给出「人能读懂的一句话」，Debug 给出结构化信息。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_06` **From 上转：`?` 自动把 ParseError 转换成 AppError**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:542`
  - 待实现函数：`exercise_10_06_from_conversion`
  - 实现要求：用 exercise_10_05_parse(text)? 让 `?` 自动上转成 AppError，成功返回 Ok(n)
  - 作者复习指引：复习 lesson_10 示例 6（from_conversion_via_question_mark）：`expr?` 在出错时会做`From::from(err)`，把底层错误转成函数返回类型的错误类型。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_07` **Box<dyn Error> 统一出口：`?` 把 ParseIntError 自动装箱**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:620`
  - 待实现函数：`exercise_10_07_box_dyn_error`
  - 实现要求：text.trim().parse::<i32>()? 之后返回 Ok(text.trim().len())
  - 作者复习指引：复习 lesson_10 示例 7（box_dyn_error）：标准库已提供 `impl From<E> for Box<dyn Error>`（E: Error + 'static），所以 `?` 能直接把 ParseIntError 装箱后返回。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_08` **典型场景：解析 key=value 配置，跳过空行与 # 注释行，坏行错误信息保留原文**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:700`
  - 待实现函数：`exercise_10_08_parse_config`
  - 实现要求：跳过空行与 # 注释行，按 = 拆成 (key, value) 并 trim；没有 = 的行返回含该行原文的 Err
  - 作者复习指引：复习 lesson_10 示例 8（config_parsing_scenario）：全有或全无地解析配置，空行跳过；出错时把「哪一行坏了」写进错误信息，方便使用者直接定位。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_09` **常见错误诊断：`?` 用错位置、对 Option 用 `?`、缺 From 分别报什么错**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_10_error_handling.rs:794`
  - 待实现函数：`exercise_10_09_diagnose_errors`
  - 实现要求：返回课程示例 9 前三个错误的编号（顺序一致）：["E0277", "E0277", "E0277"]
  - 作者复习指引：复习 lesson_10 示例 9（common_mistakes）：`?` 相关的错误都落在 E0277（trait 约束不满足）这一族里，读错误信息时要顺着 note: 找到缺失的 trait。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`

### lesson_11 · 泛型（通过 0/10，得分 0 · 待加强）

- `kp_11_01` **泛型函数：`fn largest<T: PartialOrd>(list: &[T]) -> Option<&T>` 一份代码适配多种类型**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:53`
  - 待实现函数：`exercise_11_01_largest`
  - 实现要求：返回最大元素的引用：空切片 → None；相等元素保留最先出现的那个
  - 作者复习指引：复习 lesson_11 示例 1（generic_function）：类型参数 T 代表「待定的类型」，`<T: PartialOrd>` 是 trait bound，告诉编译器 T 支持比较；空切片必须返回 None。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_02` **泛型结构体：`struct Point<T>` 能生成 Point<i32>、Point<f64> 等互相独立的具体类型**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:147`
  - 待实现函数：`exercise_11_02_generic_struct`
  - 实现要求：函数内定义 struct Point<T>，用 i32 与 f64 各实例化一次，返回 (1, 2.5)
  - 作者复习指引：复习 lesson_11 示例 2（generic_struct）：字段类型可以是类型参数；`Point<i32>` 与 `Point<f64>` 是两个完全不同的类型，彼此不能互相赋值。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_03` **泛型枚举：标准库的 Option<T> / Result<T, E> 就是泛型枚举，自己定义的也能互相转换**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:203`
  - 待实现函数：`exercise_11_03_generic_enum`
  - 实现要求：函数内定义 MyOption<T> / MyResult<T, E>，把 Some(7) 与 Err(404) 转成标准库类型后返回
  - 作者复习指引：复习 lesson_11 示例 3（generic_enum）：`enum MyOption<T> { Some(T), None }` 与`enum MyResult<T, E> { Ok(T), Err(E) }` 的定义方式与标准库完全一致；每个变体都能无损地转成标准库对应变体。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_04` **泛型方法：impl<T: Copy + Add<Output = T>> Pair<T> 让 sum() 对两种数值类型都可用**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:265`
  - 待实现函数：`exercise_11_04_generic_method`
  - 实现要求：函数内定义 Pair<T> 与 impl<T: Copy + Add<Output = T>> Pair<T> 的 sum()，返回 (7, 4.0)
  - 作者复习指引：复习 lesson_11 示例 4（generic_method）：`impl<T> Point<T>` 表示「对任意 T 都提供这些方法」；约束写在 impl 块上时对块内所有方法生效。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_05` **trait bound 用起来：`<T: Display, U: Display>` 让两种不同类型的值拼成一句话**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:322`
  - 待实现函数：`exercise_11_05_display_pair`
  - 实现要求：返回 format!("{a}-{b}")：两个类型参数各自独立，只需都满足 Display
  - 作者复习指引：复习 lesson_11 示例 6（trait_bounds_syntax）：bound 表达「使用方对类型的要求」，编译器会拿每一个具体类型去检查；`Display` 是 `{}` 格式化所需要的约束。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_06` **where 子句：约束放在函数签名下方，比挤在尖括号里更好读**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:387`
  - 待实现函数：`exercise_11_06_where_clause`
  - 实现要求：用 where 子句写约束，返回最小元素的引用：空切片 → None；相等元素保留最先出现的
  - 作者复习指引：复习 lesson_11 示例 7（where_clause）：`fn largest_where<T>(list: &[T]) -> T whereT: PartialOrd + Copy` 与尖括号写法语义等价，只是排版不同（rustfmt 也偏好它）。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_07` **多个泛型参数：`<K: Display, V: Display>` 让键与值各自是不同类型**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:466`
  - 待实现函数：`exercise_11_07_multiple_params`
  - 实现要求：返回 format!("{key}={value}")：两个泛型参数各自独立
  - 作者复习指引：复习 lesson_11 示例 8（multiple_type_params）：类型参数可以有两个及以上、各自独立；泛型结构体的字段可以分别使用它们（MixedPoint<T, U> 就是例子）。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_08` **const 泛型：`<const N: usize>` 把编译期常量也作为参数，数组长度是类型的一部分**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:532`
  - 待实现函数：`exercise_11_08_const_generics`
  - 实现要求：返回 (N, 元素之和)；N = 0 时和必须为 0
  - 作者复习指引：复习 lesson_11 示例 9（const_generics）：`Buffer<4>` 与 `Buffer<8>` 是不同类型；函数上的 `fn filled<T: Copy, const N: usize>(value: T) -> [T; N]` 也是同一原理。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_09` **单态化（monomorphization）：泛型在编译期展开，运行期没有类型判断开销**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:603`
  - 待实现函数：`exercise_11_09_monomorphization`
  - 实现要求：函数内定义 fn size_of_val<T>(_: &T) -> usize，用 i32 与 f64 各调用一次，返回 (4, 8)
  - 作者复习指引：复习 lesson_11 示例 10（monomorphization）：编译器为每个用到的具体类型各生成一份专用代码（相当于自动写出 size_of_val_i32、size_of_val_f64），代价是二进制体积与编译时间增加。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_10` **常见错误诊断：泛型里直接相加、用 {} 打印未约束 Display、类型参数没被使用**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_11_generics.rs:670`
  - 待实现函数：`exercise_11_10_diagnose_errors`
  - 实现要求：返回课程示例 12 前三个错误的编号（顺序一致）：["E0369", "E0277", "E0392"]
  - 作者复习指引：复习 lesson_11 示例 12（common_mistakes）：泛型代码的错误几乎都指向「约束不足」或「类型不匹配」；读错误信息时先看是哪一类，再决定补约束还是改类型参数。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`

### lesson_12 · Trait（通过 0/11，得分 0 · 待加强）

- `kp_12_01` **为自定义类型实现 trait：impl Area for Circle 与 impl Area for Square**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:101`
  - 待实现函数：`impl Area for Circle 的 area()`
  - 实现要求：返回 std::f64::consts::PI * self.radius * self.radius
  - 作者复习指引：复习 lesson_12 示例 1（定义 trait 并为自定义类型实现）与示例 10（Shape 的 area 实现）：trait 只声明「能做什么」，`impl Trait for 类型 { ... }` 才给出「怎么做」；圆的面积是 πr²（π 用 `std::f64::consts::PI`），正方形是边长²。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_02` **默认方法：只实现必需方法，就能得到 shout() 的默认行为**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:200`
  - 待实现函数：`exercise_12_02_default_method`
  - 实现要求：在函数内定义带默认方法 shout() 的 trait Summary 与本地类型，只实现必需方法 summarize()，返回 note.shout()，即 "Rust 学习笔记!"
  - 作者复习指引：复习 lesson_12 示例 2（默认方法与覆盖）：trait 里带方法体的方法就是默认方法，实现者不写也能用；默认方法内部可以调用必需方法，例如 `fn shout(&self) -> String { format!("{}!", self.summarize()) }`。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_03` **trait 作为参数（写法一）：item: &impl Area 接收任意实现者**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:266`
  - 待实现函数：`exercise_12_03_impl_trait_arg`
  - 实现要求：返回 item.area() * 2.0（参数写成 &impl Area）
  - 作者复习指引：复习 lesson_12 示例 4（trait 作为参数：impl Trait）：`&impl Area` 是「一个匿名类型参数 + Area 约束」的语法糖，同样是静态分发（单态化）。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_04` **trait 作为参数（写法二）：泛型参数 + trait bound，可 turbofish**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:330`
  - 待实现函数：`exercise_12_04_generic_bound`
  - 实现要求：返回 item.area() * 2.0（泛型参数 T: Area）
  - 作者复习指引：复习 lesson_12 示例 3（trait 作为参数：泛型 + trait bound）：`fn f<T: Area>(item: &T)` 与 `fn f(item: &impl Area)` 都是静态分发；区别是泛型参数**有名字**，调用处可以写 `f::<Circle>(&c)` 显式指定类型，函数体里也能用 `T` 做别的事。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_05` **返回 impl Trait：调用方只依赖 Area，看不到具体类型**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:396`
  - 待实现函数：`exercise_12_05_return_impl_trait`
  - 实现要求：返回 Square { side }，用 impl Area 隐藏具体类型
  - 作者复习指引：复习 lesson_12 示例 6（返回 impl Trait）：`-> impl Area` 表示「返回某个实现了 Area 的具体类型，但调用方不需要知道它是谁」；代价是所有 return 分支必须是同一个具体类型。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_06` **返回 Box<dyn Area>：分支返回不同类型时必须用 trait 对象**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:464`
  - 待实现函数：`exercise_12_06_box_dyn`
  - 实现要求：kind == "circle" 时返回 Box::new(Circle { radius: size })，其它一律返回 Box::new(Square { side: size })
  - 作者复习指引：复习 lesson_12 示例 7（返回 Box<dyn Trait>）：`dyn Trait` 的大小在编译期未知，必须装在指针后面（`Box<dyn Trait>` / `&dyn Trait`）；分支各自返回不同类型的唯一办法就是先统一装箱成同一种 `dyn` 类型。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_07` **多个 trait bound：T: Area + Debug，既能算面积又能 {:?} 打印**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:534`
  - 待实现函数：`exercise_12_07_multiple_bounds`
  - 实现要求：返回 format!("{item:?} 的面积是 {:.2}", item.area())
  - 作者复习指引：复习 lesson_12 示例 5（trait bound 组合与 +）：约束用 `+` 叠加，`T: Area + std::fmt::Debug` 表示「既要能算面积，又要有派生的 Debug 实现」；约束叠得越多，能进来的类型越少，所以要按需叠加。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_08` **关联类型：type Item = u32，调用处不需要写类型标注**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:596`
  - 待实现函数：`exercise_12_08_associated_type`
  - 实现要求：构造 CounterProducer { limit: 5 }，调用 3 次 produce()，返回 vec![5, 5, 5]
  - 作者复习指引：复习 lesson_12 示例 8（关联类型 vs 泛型参数）：`type Item = u32;` 表达「这个生产者天生只产出一种类型」，所以调用处不用标注；泛型参数的版本（`ConvertTo<T>`）允许同一类型有多份实现，但调用处必须标注，否则报 `error[E0283]: type annotations needed`。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_09` **dyn trait 对象集合：Vec<Box<dyn Area>> 排序后返回面积列表**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:699`
  - 待实现函数：`exercise_12_09_dyn_collection`
  - 实现要求：用 Vec<Box<dyn Area>> 装 Rectangle{3.0,4.0} / Circle{1.0} / Square{2.5}，按面积升序排序后返回面积列表 vec![π, 6.25, 12.0]
  - 作者复习指引：复习 lesson_12 示例 9 与示例 10（dyn trait 对象、按面积排序）：`Vec<Box<dyn Area>>` 能同时装 Circle / Square / 你自定义的 Rectangle，排序用 `sort_by(\|a, b\| a.area().total_cmp(&b.area()))`（f64 用 total_cmp 得到全序，避免 partial_cmp 返回 None）。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_10` **派生标准库 trait：Default 置零、PartialEq 逐字段比较、Debug 的固定格式**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:783`
  - 待实现函数：`exercise_12_10_derived_traits`
  - 实现要求：返回 (Version::default() == Version { major: 0, minor: 0 }, format!("{:?}", Version { major: 1, minor: 2 }))
  - 作者复习指引：复习 lesson_12 示例 11（标准库 trait 的派生）：`#[derive(Debug, Clone, PartialEq, Default)]` 生成的实现都是「逐字段」的；Debug 的输出格式固定为 `Version { major: 1, minor: 2 }`，Default 把 u32 字段全部置 0。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_11` **常见错误诊断：trait bound 未满足 / impl 漏写必需方法 / 重复实现**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_12_traits.rs:853`
  - 待实现函数：`exercise_12_11_diagnose_errors`
  - 实现要求：返回 ["E0277", "E0046", "E0119"]（trait bound 未满足 / 漏写必需方法 / 重复实现）
  - 作者复习指引：复习 lesson_12 示例 13（common_mistakes）：本课列的 7 个坑里，前三个分别是「类型没有实现 trait」「impl 块漏写必需方法」「同一个类型重复实现同一个 trait」。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`

### lesson_13 · 生命周期（通过 0/9，得分 0 · 待加强）

- `kp_13_01` **函数签名里的生命周期注解：返回较长的那个字符串切片**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:82`
  - 待实现函数：`exercise_13_01_longest`
  - 实现要求：按字节长度返回较长的切片，长度相等（含都为空串）时返回 a
  - 作者复习指引：复习 lesson_13 示例 2（函数签名中的生命周期注解）：`fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` 的意思是「返回值活得和这两个参数一样久」——`'a` 会被推断为两个实参中**较短**的那个存活区间；注解只描述关系，不延长任何数据的寿命。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_02` **生命周期省略规则 1 + 2：只有一个输入引用时，输出引用就取它**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:169`
  - 待实现函数：`exercise_13_02_first_word`
  - 实现要求：返回第一个空格之前的切片；没有空格则返回整个 text（空串返回空串）
  - 作者复习指引：复习 lesson_13 示例 3（生命周期省略的三条规则）：规则 1 给每个引用参数各一个生命周期参数，规则 2 在「只有一个输入引用」时把输出引用的生命周期取成它；所以 `fn first_word(text: &str) -> &str` 等价于 `fn first_word<'a>(text: &'a str) -> &'a str`。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_03` **两个独立生命周期参数：'a 与 'b 互不影响，返回值只借用 'a**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:249`
  - 待实现函数：`exercise_13_03_pick_first`
  - 实现要求：只返回 first（第二个参数只用于对比，不要返回它）
  - 作者复习指引：复习 lesson_13 示例 4（多个生命周期参数）：`fn pick_first<'a, 'b>(first: &'a str, second: &'b str) -> &'a str` 明确写出「只与 first 有关」；如果两个参数共用同一个 `'a`，`'a` 会被压缩成较短的那个，返回值就带不出内层作用域了。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_04` **结构体持有引用：Excerpt<'a> 的 new / part / announce**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:324`
  - 待实现函数：`Excerpt::new`
  - 实现要求：返回 Self { part }（把借来的 &'a str 存进字段）
  - 作者复习指引：复习 lesson_13 示例 5 与示例 6（结构体持有引用、impl 块与方法中的生命周期）：字段是引用就必须写 `struct Excerpt<'a> { part: &'a str }`；`fn part(&self) -> &str` 用省略规则 3（输出引用取 `&self` 的生命周期）；`announce` 返回 `self.part`，绝不能返回 `msg` —— 因为返回类型借的是 self，不是 msg。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_05` **生命周期省略：签名里一个 'a 都不写，也能安全返回切片**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:456`
  - 待实现函数：`exercise_13_05_elision`
  - 实现要求：返回 text.trim()（签名里不写生命周期注解）
  - 作者复习指引：复习 lesson_13 示例 3（省略规则）与示例 9（注解不会延长存活时间）：只有一个输入引用时，输出引用的生命周期自动取它；返回的切片仍然借用 `text`，所以调用方不能让 `text` 先失效。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_06` **'static：字符串字面量活在只读区，带出任何作用域都有效**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:533`
  - 待实现函数：`exercise_13_06_static_lifetime`
  - 实现要求：返回 ("hello-rust-lessons", "hello-rust-lessons".len())
  - 作者复习指引：复习 lesson_13 示例 7（'static 生命周期）：字符串字面量的类型就是 `&'static str`，它的数据嵌在可执行文件里，程序全程有效；要分清 `&'static T`（引用活得够久）与 `T: 'static`（类型内部不含短生命周期借用，拥有所有权的 String、i32 都满足）这两种完全不同的含义。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_07` **生命周期参数 + trait bound：T: Display + 'a，返回拥有所有权的 String**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:603`
  - 待实现函数：`exercise_13_07_lifetime_bound`
  - 实现要求：返回 format!("{value}")（签名含 <'a, T: Display + 'a>）
  - 作者复习指引：复习 lesson_13 示例 8（生命周期与 trait bound）：`<'a, T: Display + 'a>` 里`'a` 是生命周期参数、`T` 是类型参数，`T: 'a` 约束表示「T 里不含比 'a 更短的借用」；返回 `String` 与 `'a` 无关，所以调用方拿到的东西不受借用区间限制。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_08` **常见错误诊断：E0106 缺生命周期注解（课程前三个错误都是它）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:683`
  - 待实现函数：`exercise_13_08_diagnose_errors`
  - 实现要求：返回 ["E0106", "E0106", "E0106"]（课程示例 11 的前三个错误都是缺生命周期注解）
  - 作者复习指引：复习 lesson_13 示例 11（common_mistakes）：前三个坑分别是「两个引用参数省略了返回值注解」「结构体字段是引用却没写生命周期参数」「返回局部变量的引用」——这三处编译器的诊断都是 missing lifetime specifier（E0106）；再往后才是 E0515（返回局部变量）、E0621（漏标一个参数）、E0597（被借用数据活得太短）。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_09` **结构体持有引用：Config<'a> 的 new / endpoint / host**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_13_lifetimes.rs:747`
  - 待实现函数：`Config::new`
  - 实现要求：返回 Self { host, port }（host 是借来的 &'a str）
  - 作者复习指引：复习 lesson_13 示例 10（典型场景：借用式配置）：配置项常常来自命令行参数或环境变量，本来就是长命的 String，用 `&'a str` 借用可以省掉一次分配；`endpoint()` 返回拥有所有权的 `String`，`host()` 用省略规则返回借用 `self` 的 `&str`。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`

### lesson_14 · 闭包（通过 0/9，得分 0 · 待加强）

- `kp_14_01` **闭包基础：竖线参数、可省花括号、闭包能捕获定义处的变量**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:53`
  - 待实现函数：`exercise_14_01_closure_basics`
  - 实现要求：定义闭包 \|x: i32\| base + x（捕获 base），返回 (闭包(1), 闭包(10))
  - 作者复习指引：复习 lesson_14 示例 1（syntax_vs_function）：`let add = \|x: i32\| base + x;` 与普通函数的第一个区别就是——闭包体里的 `base` 来自定义处环境（这里按不可变借用捕获）。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_02` **Fn trait：只读捕获的闭包可被调用任意多次（泛型参数 + Fn bound）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:114`
  - 待实现函数：`exercise_14_02_fn_trait`
  - 实现要求：循环 times 次调用 f() 并把返回值累加，返回总和（times = 0 时返回 0）
  - 作者复习指引：复习 lesson_14 示例 5（fn_fnmut_fnonce_hierarchy）与示例 6（closure_as_parameter）：参数写成 `F: Fn() -> i32` 就是要求「可以被调用多次且不会改动捕获值」；在函数里循环 `times` 次调用 `f()` 并累加即可。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_03` **FnMut trait：可变借用捕获 + 函数参数用 mut 绑定，两次调用共享同一份状态**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:178`
  - 待实现函数：`exercise_14_03_fnmut`
  - 实现要求：依次调用 f(a) 与 f(b)（同一个闭包，状态连续累加），返回 (f(a), f(b))
  - 作者复习指引：复习 lesson_14 示例 3（capture_by_mutable_borrow）与示例 5 的 `call_three_times`：闭包体里**写入**了外部变量 → 按 `&mut` 捕获 → 该闭包只实现 FnMut/FnOnce；接收入参时要写 `mut f: F` 才能调用（调用 FnMut 需要 `&mut self`）。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_04` **move 捕获与 FnOnce：所有权搬进闭包，调用后由返回值交还**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:256`
  - 待实现函数：`exercise_14_04_move_closure`
  - 实现要求：用 move 把 String::from("move 闭包带走了所有权") 移进闭包，调用后返回该文本
  - 作者复习指引：复习 lesson_14 示例 4（capture_by_move）：`move` 把捕获变量的所有权搬进闭包；如果闭包体里用掉了那个值（例如把它当返回值返回），闭包就只能是 `FnOnce`，只允许调用一次。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_05` **闭包作为参数（impl Trait）：静态分发，写法最简**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:319`
  - 待实现函数：`exercise_14_05_apply_impl`
  - 实现要求：参数写成 impl Fn(i32) -> i32，返回 f(value)
  - 作者复习指引：复习 lesson_14 示例 6（closure_as_parameter）的 `apply_twice`：`fn apply(value: i32, f: impl Fn(i32) -> i32) -> i32 { f(value) }`，`impl Trait` 等价于一个匿名的泛型参数，编译期单态化、零额外开销。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_06` **返回闭包（impl Fn）：必须 move 把 n 搬进闭包，否则借用随函数结束失效**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:382`
  - 待实现函数：`exercise_14_06_make_adder`
  - 实现要求：返回 move 闭包 \|x\| x + n（必须 move，否则借用随函数结束失效）
  - 作者复习指引：复习 lesson_14 示例 7（returning_closure）的 `make_multiplier`：`fn make_multiplier(factor: i64) -> impl Fn(i64) -> i64 { move \|x\| x * factor }`；去掉 `move` 会报 `error[E0373]: closure may outlive the current function`。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_07` **Box<dyn Fn>：异构闭包必须装箱才能放进同一个 Vec（动态分发）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:447`
  - 待实现函数：`exercise_14_07_boxed_closures`
  - 实现要求：用 Vec<Box<dyn Fn(i32) -> i32>> 装 (\|x\| x + 1) 与 (move \|x\| x + base)，两者都以 7 调用，返回两次结果之和
  - 作者复习指引：复习 lesson_14 示例 8（box_dyn_closure）：`let ops: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(\|x\| x + 1), Box::new(move \|x\| x + offset)];`——两个闭包是**不同的匿名类型**，只有 `dyn Fn` 能把它们统一成同一种元素类型。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_08` **任务队列：Vec<Box<dyn FnOnce() -> String>> 按顺序执行消费型任务**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:511`
  - 待实现函数：`exercise_14_08_task_queue`
  - 实现要求：用 Vec<Box<dyn FnOnce() -> String>> 收集两个任务（分别返回「任务 1 完成」「任务 2 完成」），按顺序调用后返回结果列表
  - 作者复习指引：复习 lesson_14 示例 9（scenario_task_queue）：把「行为」作为值存进集合、由调用方按序执行；与示例 8 的 `Box<dyn Fn>` 相比，这里的任务返回 `String` 并耗尽自身，所以 trait 对象要写`Box<dyn FnOnce() -> String>`，也只能调用一次。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_09` **常见错误诊断：闭包参数类型锁定 E0308 / 可变借用冲突 E0502 / move 后使用 E0382**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_14_closures.rs:572`
  - 待实现函数：`exercise_14_09_diagnose_errors`
  - 实现要求：返回 ["E0308", "E0502", "E0382"]（类型不匹配 / 可变借用冲突 / move 后使用）
  - 作者复习指引：复习 lesson_14 示例 10（common_mistakes）：本课列的 5 个坑按注释顺序是——错误 1 闭包参数类型被推断锁定（E0308）、错误 2 闭包持有可变借用时又读同一变量（E0502）、错误 3 move 之后又使用原变量（E0382）；再往后还有 E0373 与 E0499。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`

### lesson_15 · 迭代器（通过 0/9，得分 0 · 待加强）

- `kp_15_01` **for 循环与 Iterator 的关系：for 只是「反复调用 next()」的语法糖**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:122`
  - 待实现函数：`exercise_15_01_sum_with_for`
  - 实现要求：用 for 循环遍历 items 累加求和并返回（空切片返回 0）
  - 作者复习指引：复习 lesson_15 示例 1（iterator_and_for）：`for n in &nums` 等价于 `(&nums).into_iter()` 加一个循环；本题先用最朴素的 `for` 写出求和，作为后面 `iter().sum()` 的对照。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_02` **iter / iter_mut / into_iter 的所有权差别：iter_mut() 产出 &mut T，可原地改写**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:189`
  - 待实现函数：`exercise_15_02_three_modes`
  - 实现要求：用 iter_mut() 把 values 里每个元素原地翻倍，返回 (翻倍后元素之和, 元素个数)
  - 作者复习指引：复习 lesson_15 示例 2（three_iteration_modes）：`iter()` 借用产出 `&T`（集合保留）、`iter_mut()` 可变借用产出 `&mut T`（原地改写）、`into_iter()` 消费集合产出 `T`（集合被移走）；本题用 `for n in values.iter_mut() { *n *= 2; }` 改写后，再统计和与个数。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_03` **适配器流水线：enumerate + filter + map（惰性组合，不打乱原始下标）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:260`
  - 待实现函数：`exercise_15_03_adapters`
  - 实现要求：enumerate + filter（保留偶数）+ map（值 × 2）组成流水线，collect 成 Vec<(usize, i32)>
  - 作者复习指引：复习 lesson_15 示例 3（adapters）：`nums.iter().enumerate().filter(..).map(..).collect()`；注意本题 `enumerate` 放在 `filter` **之前**，所以保留下来的下标是原始下标（会跳号），不是过滤后的序号。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_04` **消费器：sum() / max() / count() / any()（只有消费器才驱动流水线）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:323`
  - 待实现函数：`exercise_15_04_consumers`
  - 实现要求：返回 (sum, max, count, any(\|x\| *x < 0))：sum::<i32>() / max().copied() / count() / any()
  - 作者复习指引：复习 lesson_15 示例 4（consumers）：`sum::<i32>()` 求和、`max()` 返回 `Option<&T>`、`count()` 返回元素个数、`any(pred)` 返回是否**存在**满足条件的元素；本题返回 `(和, 最大值, 个数, 是否有负数)`。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_05` **fold 与 collect：fold 是自定义聚合，collect 是「换容器」**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:386`
  - 待实现函数：`exercise_15_05_fold_collect`
  - 实现要求：用 fold(0, \|acc, x\| acc + *x) 求和，用 map + collect 生成 ["n=<值>", ...]，返回 (和, 列表)
  - 作者复习指引：复习 lesson_15 示例 5（fold_and_collect）：`fold(初始值, \|acc, x\| 新累积值)` 可以表达任意聚合（求和、连乘、拼 CSV……）；`collect()` 的目标类型由接收方决定，这里用 `map` 生成`n=<值>` 的字符串再收成 `Vec<String>`。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_06` **惰性与短路：适配器只搭流水线，消费器才按需驱动（take(3) 只加工 3 个元素）**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:456`
  - 待实现函数：`exercise_15_06_laziness`
  - 实现要求：用 std::cell::Cell<usize> 统计 map 闭包调用次数：(0..100).map(计数).take(3).collect::<Vec<_>>()，返回 (调用次数, 结果长度)
  - 作者复习指引：复习 lesson_15 示例 6（laziness_proof）：`map` 的闭包在 `collect()` 之前一次都不会执行；本题用 `std::cell::Cell<usize>` 计数，`(0..100).map(计数).take(3).collect()` 只会加工 3 个元素——这就是「惰性 + 按需驱动」的事实证据（不是 100 次，也不是 0 次）。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_07` **自定义迭代器：实现 Counter::new 与 Iterator::next，免费获得全部适配器与消费器**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:519`
  - 待实现函数：`Counter::new`
  - 实现要求：返回 Counter { count: 0, max }（count 从 0 开始，next() 里先自增再判断）
  - 作者复习指引：复习 lesson_15 示例 8（custom_iterator）：自定义类型只要实现 `Iterator` 的 `next()`（外加关联类型 `Item`），就能直接用 `sum()` / `collect()` / `filter()` 等全套工具；本题的 `Counter` 产出 `1..=max`，耗尽后 `next()` 必须返回 `None`。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_08` **词频统计：split_whitespace + 统一小写 + 计数 + 确定性排序（次数降序、字典序升序）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:557`
  - 待实现函数：`exercise_15_08_word_freq`
  - 实现要求：按空白切分、统一小写、统计词频，按次数降序（同次数按字典序升序）返回 Vec<(String, usize)>
  - 作者复习指引：复习 lesson_15 示例 9（scenario_word_freq）：`text.split_whitespace()` 切词、`HashMap` 计数（entry API 见 lesson_08）、收集成 `Vec` 后`sort_by(\|a, b\| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))` 让输出**确定**。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_09` **常见错误诊断：迭代期间修改 E0502 / into_iter 后使用 E0382 / collect 类型不明 E0282**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_15_iterators.rs:635`
  - 待实现函数：`exercise_15_09_diagnose_errors`
  - 实现要求：返回 ["E0502", "E0382", "E0282"]（迭代期间修改 / move 后使用 / collect 类型不明）
  - 作者复习指引：复习 lesson_15 示例 10（common_mistakes）：本课列的 5 个坑按注释顺序是——错误 1 遍历时修改集合（E0502）、错误 2 `into_iter()` 消费后又使用原集合（E0382）、错误 3 `collect()` 目标类型不明（E0282）；再往后还有「只有适配器没有消费器」与`find` 返回引用的运算错误（E0369）。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`

### lesson_16 · 智能指针（通过 0/9，得分 0 · 待加强）

- `kp_16_01` **Box 与递归类型：用 Box 把「无限大小」变成固定大小，再递归处理它**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:106`
  - 待实现函数：`List::len`
  - 实现要求：递归求长度：Cons(_, tail) => 1 + tail.len()，Nil => 0
  - 作者复习指引：复习 lesson_16 示例 1（box_and_recursive_type）：`enum List { Cons(i32, Box<List>), Nil }` 靠 `Box`（固定 8 字节指针）打断「大小递归」；`len` / `sum` 两个方法都要写递归调用（`1 + tail.len()` / `*value + tail.sum()`），`Nil` 分支返回 0。不加 Box 会报 `error[E0072]: recursive type `List` has infinite size`。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_02` **Box<T>：唯一所有权 + 堆分配，`*b` 解引用取出里面的值**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:214`
  - 待实现函数：`exercise_16_02_box_deref`
  - 实现要求：用 Box::new(5) 造一个堆上的值，返回 (*b, *b + 1) = (5, 6)
  - 作者复习指引：复习 lesson_16 示例 2（box_basics）与示例 3（deref_coercion）：`Box::new(5)` 把 5 分配到堆上，变量本身只是一个固定大小的指针（在栈上）；`*b` 通过 `Deref` 取出堆上的值，所以 `&Box<String>` 还能自动强转成 `&str`——这就是 Deref 强制转换。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_03` **Rc 共享所有权：`Rc::clone` 只让计数 +1，不复制堆上的数据**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:286`
  - 待实现函数：`exercise_16_03_rc_counts`
  - 实现要求：Rc::new(String::from("shared")) + Rc::clone + drop(second)，返回 (2, 1)
  - 作者复习指引：复习 lesson_16 示例 4（rc_shared_ownership）：`Rc::clone(&a)` 复制的只是一个「指向同一份堆数据的指针」，`Rc::strong_count` 因此从 1 变成 2；`drop` 掉一个所有者后计数回到 1；只有计数归零（最后一个所有者离开）时，堆上的数据才真正释放。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_04` **RefCell 内部可变性：把借用检查从编译期挪到运行期**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:352`
  - 待实现函数：`exercise_16_04_refcell`
  - 实现要求：RefCell::new(0) 之后两次 *cell.borrow_mut() += 5，返回 10
  - 作者复习指引：复习 lesson_16 示例 5（refcell_interior_mutability）：`let cell = RefCell::new(0);` 没写 `mut`，却可以写 `*cell.borrow_mut() += 5;`——`borrow_mut()` 在运行期动态登记「当前有一个可变借用」，守卫（`RefMut`）一 drop 就归还；编译期只看到「共享引用」，所以借用检查被推迟到了运行期。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_05` **RefCell 的运行期边界：可变借用没释放就再借用 → panic（不是编译错误）**｜类型：边界｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:411`
  - 失败原因：实际 panic：【未实现·边界】智能指针 · kp_16_05「RefCell 的运行期边界：可变借用没释放就再借用 → panic（不是编译错误）」
  - 提示：边界：`borrow_mut()` 的守卫还活着时再 `borrow()`，RefCell 必然 panic（already mutably borrowed）——这就是「内部可变性把借用检查挪到运行期」的代价
  - 待实现函数：`exercise_16_05_double_borrow`
  - 实现要求：先 let a = cell.borrow_mut()，在 a 还活着时再 let b = cell.borrow()，返回 *b（必然 panic）
  - 作者复习指引：复习 lesson_16 示例 5 的注释与示例 9 的错误 2：`let first = cell.borrow(); let second = cell.borrow_mut();` 能编译通过，运行期却 panic：`already borrowed: BorrowMutError`；修正办法是缩小借用作用域、用完立刻 drop。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_06` **Rc<RefCell<T>>：Rc 管「几个所有者」，RefCell 管「能不能改」**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:480`
  - 待实现函数：`exercise_16_06_rc_refcell`
  - 实现要求：Rc<RefCell<i32>> 的两个克隆各 borrow_mut +1，返回 (最终值 2, strong_count 2)
  - 作者复习指引：复习 lesson_16 示例 6（rc_refcell_combo）：两个 `Rc::clone` 指向同一份 `RefCell`，各自 `borrow_mut()` 改的都是**同一份**数据（不是各自的副本），所以两次 +1 得到 2；同时 `Rc::strong_count` 说明确实只有 2 个所有者。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_07` **Weak 打破循环引用：`Rc::downgrade` 不加强计数，`upgrade()` 拿临时强引用**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:545`
  - 待实现函数：`Node::link`
  - 实现要求：把 parent.next（RefCell<Weak<Node>>）设为 Rc::downgrade(child)
  - 作者复习指引：复习 lesson_16 示例 7（weak_breaks_cycle）：正向持有用 `Rc`，回指 / 环上一律用 `Weak`——`Rc::downgrade(&child)` 存进 `RefCell<Weak<Node>>`，强计数不增加；要读的时候 `upgrade()` 返回 `Option<Rc<Node>>`，对方已被释放时得到 `None`（不会悬垂）。如果两边都用 `Rc` 互指，计数永不归零 → 内存泄漏（示例 9 的错误 3）。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_08` **典型场景：`Rc<RefCell<HashMap>>` 做共享可变缓存，并统计命中次数**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:655`
  - 待实现函数：`exercise_16_08_shared_cache`
  - 实现要求：Rc<RefCell<HashMap<String, i32>>> 缓存：第一次未命中算 6 * 7 = 42 写入（命中 0），第二次命中（命中 1），返回 (42, 1)
  - 作者复习指引：复习 lesson_16 示例 8（scenario_shared_cache）：缓存的句柄用 `Rc::clone` 共享，内部用 `RefCell<HashMap<..>>` 允许修改；「命中」= 查到了已有键，「未命中」= 没查到、需要计算并写入。注意借用的作用域：先取值（语句结束即归还借用），再考虑要不要 `borrow_mut()` 插入，否则会撞上运行期借用冲突。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_09` **常见错误诊断：改 Rc 内部值 / Rc 跨线程 / 递归类型没加间接层，分别报什么错**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_16_smart_pointers.rs:726`
  - 待实现函数：`exercise_16_09_diagnose_errors`
  - 实现要求：返回 ["E0596", "E0277", "E0072"]（改 Rc 内部值 / Rc 跨线程 / 递归类型无限大小）
  - 作者复习指引：复习 lesson_16 示例 9（common_mistakes）与示例 1：示例 9 里**带编译错误编号**的三个坑依次是「想直接改 `Rc` 里的值」（E0596）、「把 `Rc` 送进线程」（E0277）、「递归类型忘记加 `Box`」（E0072）——示例 9 末尾那行 `println!` 直接把这三个编号连在一起打印；另外两个坑（RefCell 借用重叠、循环引用泄漏）都是运行期问题，没有编译错误编号。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`

### lesson_17 · 线程与通道（通过 0/9，得分 0 · 待加强）

- `kp_17_01` **thread::spawn + join：等子线程结束，并把它的返回值收回主线程**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:55`
  - 待实现函数：`exercise_17_01_spawn_join`
  - 实现要求：thread::spawn(\|\| 42u32) 之后 handle.join().expect(...) 取回 42
  - 作者复习指引：复习 lesson_17 示例 1（spawn_and_join）：`thread::spawn(\|\| 42)` 返回 `JoinHandle<u32>`；`handle.join()` 阻塞到子线程结束，返回 `Result<u32, Box<dyn Any + Send>>`——子线程正常结束是 `Ok(值)`，子线程 panic 才是 `Err`（所以实现里要先 `expect(...)` 取出来）。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_02` **move 闭包与所有权转移：编译器用「独占」保证没有数据竞争**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:119`
  - 待实现函数：`exercise_17_02_move_ownership`
  - 实现要求：thread::spawn(move \|\| text.len()) 之后 join，返回 text 的字节长度
  - 作者复习指引：复习 lesson_17 示例 2（move_ownership）：`thread::spawn(move \|\| data.len())` 把 `data` 的所有权整体搬进子线程——主线程再也碰不到它，所以不可能出现两个线程同时读写同一份数据；不写 `move` 而去借用栈上变量会报 `error[E0373]: closure may outlive the current function`。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_03` **mpsc 通道：一个生产者线程发送，主线程收集求和**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:183`
  - 待实现函数：`exercise_17_03_channel_sum`
  - 实现要求：生产者线程用 mpsc 通道发送 values 里的全部值，主线程 for 收集求和并返回
  - 作者复习指引：复习 lesson_17 示例 3（mpsc_single_producer）：`mpsc::channel()` 拿到 `(tx, rx)`；生产者把值逐个 `send`，闭包结束时 `tx` 被 drop、通道关闭，接收端的 `for value in rx` 才会结束；主线程把收到的值累加求和。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_04` **多生产者单消费者：用 `tx.clone()` 把发送端分给多个线程**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:249`
  - 待实现函数：`exercise_17_04_multi_producer`
  - 实现要求：两个生产者线程各 tx.clone() 后发送 1..=3，主线程 drop(tx) 并收集求和 = 12
  - 作者复习指引：复习 lesson_17 示例 4（mpsc_multi_producer）：每个生产者线程拿一个 `tx.clone()`，各发自己那批消息；主线程必须 `drop(tx)`（丢掉自己手里那份发送端），否则接收端等不到「所有发送端关闭」，`for` 会一直阻塞；各消息到达顺序不确定，所以先收集再统计（求和与顺序无关）。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_05` **Arc<Mutex<T>>：Arc 管「多线程共享」，Mutex 管「同一时刻只有一个能改」**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:311`
  - 待实现函数：`exercise_17_05_arc_mutex`
  - 实现要求：10 个线程共享 Arc<Mutex<i32>>，各自 lock 后 +1，全部 join 后返回 10
  - 作者复习指引：复习 lesson_17 示例 5（arc_mutex_shared_state）：每个线程 `Arc::clone` 一份句柄（原子计数），`counter.lock().unwrap()` 拿到互斥守卫后才能改；守卫离开作用域自动解锁（RAII），临界区越小越好。**所有线程 `join()` 之后**主线程再读最终值，结果才是确定的。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_06` **Send / Sync：`Arc<Mutex<i32>>` 两者都满足（换成 `Rc` 根本编译不过）**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:374`
  - 待实现函数：`exercise_17_06_send_sync_check`
  - 实现要求：函数内定义 assert_send / assert_sync，对 Arc<Mutex<i32>> 调用两者后返回 true
  - 作者复习指引：复习 lesson_17 示例 7（send_and_sync）与示例 9 的错误 3：`Send` = 值可以**转移**到别的线程，`Sync` = `&T` 可以被多个线程同时引用；`Rc` 的计数只是普通加减，所以既不是 `Send` 也不是 `Sync`，把 `Rc` 送进线程会报 `error[E0277]: `Rc<i32>` cannot be sent between threads safely`。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_07` **thread::scope：子线程借用局部数据，不需要 move 所有权**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:444`
  - 待实现函数：`exercise_17_07_scoped_sum`
  - 实现要求：thread::scope 借用 values 分块 spawn 求和再合并；空切片返回 0
  - 作者复习指引：复习 lesson_17 示例 6（scoped_threads）：`thread::scope(\|scope\| { ... })` 保证块结束时所有子线程都已被 join，所以子线程可以安全借用**没有 move 的**局部变量；每个线程算自己那一块的 `chunk.iter().sum()`，最后把各部分和相加。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_08` **并行求和：按 chunks 分块、并发执行、收集合并（切分 → 并发 → 合并）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:523`
  - 待实现函数：`exercise_17_08_parallel_sum`
  - 实现要求：空数据返回 0；chunks = 0 按 1 处理；块大小 = len.div_ceil(chunks)，用 thread::scope 分块 spawn 求和再汇总
  - 作者复习指引：复习 lesson_17 示例 8（scenario_parallel_sum）：`chunk_size = (len + workers - 1) / workers` （或 `len.div_ceil(workers)`）向上取整，保证切出来的块数不超过 worker 数；每块用 `thread::scope` 借用计算部分和，最后把部分和相加。边界：空数据直接返回 0；`chunks = 0` 按 1 处理（否则除零 / `chunks(0)` panic）。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_09` **常见错误诊断：忘记 join / 闭包缺 move / Rc 跨线程，各是什么性质的问题**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_17_threads_channels.rs:610`
  - 待实现函数：`exercise_17_09_diagnose_errors`
  - 实现要求：返回 ["忘 join（运行期，无编译编号）", "E0373", "E0277"]（忘 join / 缺 move / Rc 跨线程）
  - 作者复习指引：复习 lesson_17 示例 9（common_mistakes）与示例 7：按注释顺序的前三条错误里，第 1 条「忘记 join」是**运行期**现象（编译能过，主线程提前退出，没有编译错误编号），第 2 条「闭包缺 move」报 `error[E0373]`，第 3 条「Rc 跨线程」报 `error[E0277]`；再往后的锁中毒、死锁同样是运行期问题。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`

### lesson_18 · 声明宏（通过 0/8，得分 0 · 待加强）

- `kp_18_01` **可变参数宏：macro_rules! 用重复模式 $(...)* 接受任意个参数（含 0 个）**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:53`
  - 待实现函数：`exercise_18_01_macro_sum`
  - 实现要求：在函数内定义 macro_rules! sum_all（$(...),* 重复模式），返回 sum_all!(1, 2, 3, 4)
  - 作者复习指引：复习 lesson_18 示例 1（demo_1_macro_vs_function）：宏按「调用形状」选择匹配臂展开成代码，因此参数个数可以变；函数签名固定，做不到这一点。零参数时 `$(...)*` 一次都不展开，展开结果就只剩那个初值 `0`。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_02` **片段说明符：$i:ident 要名字、$e:expr 要表达式、$t:ty 要类型**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:137`
  - 待实现函数：`exercise_18_02_fragment_specifiers`
  - 实现要求：定义 describe!（$i:ident / $t:ty / $e:expr + stringify!），返回 describe!(count: u32 = 7)
  - 作者复习指引：复习 lesson_18 示例 2（demo_2_fragment_specifiers）：常用说明符有 expr / ident / literal / ty / pat / block / tt；说明符决定了「这个位置允许填什么」，填错形状就会报 `expected identifier` 之类的错误。stringify! 能把实参的源码文本转成字符串。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_03` **重复模式：$(...),+ 起「逗号分隔」的展开，$(,)? 吃掉可选尾逗号**｜类型：核心｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:219`
  - 待实现函数：`exercise_18_03_repetition`
  - 实现要求：定义 make_vec!（$(...),* $(,)? 重复模式），返回 make_vec!(1, 2, 3)
  - 作者复习指引：复习 lesson_18 示例 3（demo_3_repetition）：`$( $x:expr ),+` 表示逗号分隔、至少一个；`$(,)?` 表示「可以有、也可以没有的尾随逗号」，与函数调用风格保持一致。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_04` **stringify! 把源码文本变成字符串；concat! 在编译期拼接字面量**｜类型：基础｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:331`
  - 待实现函数：`exercise_18_04_stringify_concat`
  - 实现要求：返回 (stringify!(1 + 2).to_string(), concat!("a", "-", "b"))
  - 作者复习指引：复习 lesson_18 示例 4（demo_4_stringify_and_concat）：`stringify!(1 + 2)` 得到的是源码文本 `"1 + 2"`（不是 3）；`concat!` 只接受字面量，在编译期就拼成一个 `&'static str`。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_05` **提前返回宏 ensure!：展开成 if !cond { return Err(...) }，校验代码收敛成一行**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:422`
  - 待实现函数：`exercise_18_05_ensure`
  - 实现要求：定义 ensure! 宏校验 value > 0（否则 Err 且含「正数」），通过后返回 Ok(value * 2)
  - 作者复习指引：复习 lesson_18 示例 5（demo_5_ensure_early_return）：ensure! 展开后就是一行`if !($cond) { return Err(String::from($msg)); }`，所以它**只能**用在能返回 Result 的函数里；宏不改变错误链路，Result 依然是 lesson_10 的那一套。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_06` **宏写小型 DSL：`"备份" => 10` 这种箭头语法只有宏能表达，展开成 Vec 后顺序与书写一致**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:496`
  - 待实现函数：`exercise_18_06_schedule_dsl`
  - 实现要求：定义 schedule! 宏（$name:expr => $hours:expr 重复），返回含三项、顺序一致的 Vec
  - 作者复习指引：复习 lesson_18 示例 6（demo_6_scenario_schedule_dsl）：课程里的 schedule! 展开成 BTreeMap（按键排序）；本考核要求展开成 `Vec<(String, u32)>`，因此**顺序就是书写顺序**。宏 DSL 的边界：只在「消除重复、提升表达力」时使用。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_07` **宏卫生性（hygiene）：宏内部定义的名字不会污染调用方的命名空间**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:612`
  - 待实现函数：`exercise_18_07_hygiene`
  - 实现要求：宏内部 let x = 10、外部 let x = 1，返回 (add_inner_x!(1), x) = (11, 1)
  - 作者复习指引：复习 lesson_18 示例 7 的错误 5（卫生性）：宏展开里的 `hidden` 与调用处的同名变量是「不同的 hidden」；在宏外访问它报 `error[E0425]: cannot find value`。这是刻意设计——宏不应该悄悄污染调用方的命名空间。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_08` **宏的典型报错：先用后定义 / 形状不匹配 / 分隔符不符（取自课程示例 7 的注释原文）**｜类型：难点｜状态：**未实现**
  - 位置：`assessments\lesson_18_macros.rs:694`
  - 待实现函数：`exercise_18_08_diagnose_errors`
  - 实现要求：返回课程示例 7 注释原文：["cannot find macro `late` in this scope", "no rules expected the token `)`", "expected `,` or `)`"]
  - 作者复习指引：复习 lesson_18 示例 7（demo_7_common_mistakes）：本课列的 5 个坑里，前三个分别是「宏先用后定义」「实参形状与匹配臂对不上」「重复里的分隔符用错」；这三个错误在课程注释里给的是 `error:` 原文（没有 E 编号），只有第 5 个卫生性例子带 E0425。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`

## 五、需要加强的领域（Top 5）

### 1. lesson_12 · Trait（未通过 0，未实现 11，运行期错误/中断 0）

- 待实现函数 11 个：`impl Area for Circle 的 area()`、`exercise_12_02_default_method`、`exercise_12_03_impl_trait_arg`、`exercise_12_04_generic_bound`、`exercise_12_05_return_impl_trait`、`exercise_12_06_box_dyn` 等；这些练习函数的函数体里现在只有一行 `todo_exercise(...)` 占位，删掉那一行、按函数上方「实现要求」写实现即可（考核文件里的注释已写清输入输出）。
- `kp_12_01`（为自定义类型实现 trait：impl Area for Circle 与 impl Area for Square）未实现：返回 std::f64::consts::PI * self.radius * self.radius
- `kp_12_02`（默认方法：只实现必需方法，就能得到 shout() 的默认行为）未实现：在函数内定义带默认方法 shout() 的 trait Summary 与本地类型，只实现必需方法 summarize()，返回 note.shout()，即 "Rust 学习笔记!"
- `kp_12_03`（trait 作为参数（写法一）：item: &impl Area 接收任意实现者）未实现：返回 item.area() * 2.0（参数写成 &impl Area）
- `kp_12_04`（trait 作为参数（写法二）：泛型参数 + trait bound，可 turbofish）未实现：返回 item.area() * 2.0（泛型参数 T: Area）
- 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`

### 2. lesson_11 · 泛型（未通过 0，未实现 10，运行期错误/中断 0）

- 待实现函数 10 个：`exercise_11_01_largest`、`exercise_11_02_generic_struct`、`exercise_11_03_generic_enum`、`exercise_11_04_generic_method`、`exercise_11_05_display_pair`、`exercise_11_06_where_clause` 等；这些练习函数的函数体里现在只有一行 `todo_exercise(...)` 占位，删掉那一行、按函数上方「实现要求」写实现即可（考核文件里的注释已写清输入输出）。
- `kp_11_01`（泛型函数：`fn largest<T: PartialOrd>(list: &[T]) -> Option<&T>` 一份代码适配多种类型）未实现：返回最大元素的引用：空切片 → None；相等元素保留最先出现的那个
- `kp_11_02`（泛型结构体：`struct Point<T>` 能生成 Point<i32>、Point<f64> 等互相独立的具体类型）未实现：函数内定义 struct Point<T>，用 i32 与 f64 各实例化一次，返回 (1, 2.5)
- `kp_11_03`（泛型枚举：标准库的 Option<T> / Result<T, E> 就是泛型枚举，自己定义的也能互相转换）未实现：函数内定义 MyOption<T> / MyResult<T, E>，把 Some(7) 与 Err(404) 转成标准库类型后返回
- `kp_11_04`（泛型方法：impl<T: Copy + Add<Output = T>> Pair<T> 让 sum() 对两种数值类型都可用）未实现：函数内定义 Pair<T> 与 impl<T: Copy + Add<Output = T>> Pair<T> 的 sum()，返回 (7, 4.0)
- 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`

### 3. lesson_05 · 所有权系统（未通过 0，未实现 9，运行期错误/中断 0）

- 待实现函数 9 个：`exercise_05_01_move_semantics`、`exercise_05_02_copy_vs_clone`、`exercise_05_03_shared_borrows`、`exercise_05_04_mutable_borrow`、`exercise_05_05_longest_word`、`exercise_05_06_vec_len_capacity` 等；这些练习函数的函数体里现在只有一行 `todo_exercise(...)` 占位，删掉那一行、按函数上方「实现要求」写实现即可（考核文件里的注释已写清输入输出）。
- `kp_05_01`（move 语义：String 传参即移交所有权，函数通过返回值把所有权交还）未实现：按值接收 String，返回 (同一个 String, 它的字节长度)
- `kp_05_02`（Copy 与 Clone：i32 赋值是复制、String 必须显式 .clone()）未实现：返回 (2 * n, text.clone() 得到的字符串, 原 text 的字节长度)
- `kp_05_03`（共享借用：同一时刻可以存在任意多个 &T（只读共享是安全的））未实现：在同一作用域持有两个不可变借用，返回 (len1, len2, len1 + len2)
- `kp_05_04`（可变借用：两次 &mut 只要不重叠就合法（每次借用用完即结束））未实现：往 v 里 push 1 和 2，返回全部元素之和
- 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`

### 4. lesson_07 · 枚举与模式匹配（未通过 0，未实现 9，运行期错误/中断 0）

- 待实现函数 9 个：`exercise_07_01_area`、`exercise_07_02_option_sum`、`exercise_07_03_parse_port`、`exercise_07_04_classify`、`exercise_07_05_guarded`、`exercise_07_06_if_let_while_let` 等；这些练习函数的函数体里现在只有一行 `todo_exercise(...)` 占位，删掉那一行、按函数上方「实现要求」写实现即可（考核文件里的注释已写清输入输出）。
- `kp_07_01`（match 的穷尽性：每个变体都要有分支）未实现：用 match 覆盖 Shape 的三个变体：Circle(r) 返回 PI * r * r，Rectangle { width, height } 返回 width * height，Point 返回 0.0
- `kp_07_02`（Option<T>：把「可能没有值」写进类型里）未实现：两个都是 Some 时返回 Some(x + y)，任意一边是 None 时返回 None
- `kp_07_03`（Result<T, E>：成功取 Ok，失败给出可读原因）未实现：trim 之后 parse::<u16>()：成功返回 Ok，失败返回 Err(String)，且错误信息里必须包含「端口」二字
- `kp_07_04`（或模式 `\|` 与通配 `_`：合并同结果分支、兜住其余取值）未实现：match n：1 => "一"，2 \| 3 => "二或三"，_ => "其它"
- 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`

### 5. lesson_08 · 常见集合（未通过 0，未实现 9，运行期错误/中断 0）

- 待实现函数 9 个：`exercise_08_01_vec_create_and_push`、`exercise_08_02_vec_access`、`exercise_08_03_vec_mutate`、`exercise_08_04_string_utf8`、`exercise_08_05_concat_three_ways`、`exercise_08_06_hashmap_basics` 等；这些练习函数的函数体里现在只有一行 `todo_exercise(...)` 占位，删掉那一行、按函数上方「实现要求」写实现即可（考核文件里的注释已写清输入输出）。
- `kp_08_01`（Vec 的创建与 push：`to_vec()` 复制出拥有所有权的向量，`push` 在尾部追加）未实现：用 input.to_vec() 构造 Vec<i32>，push(extra) 后返回
- `kp_08_02`（Vec 的索引与安全访问：`get` / `first` 返回 `Option`，`[]` 越界会 panic）未实现：返回 (input.get(index).copied(), input.first().copied().unwrap_or(0))
- `kp_08_03`（Vec 的原地修改：`retain` 按条件删除、`iter_mut` 解引用修改元素）未实现：用 retain 删掉负数、iter_mut 翻倍，返回 (len, 元素之和)
- `kp_08_04`（String 与 UTF-8：`len()` 是字节数，`chars().count()` 是字符数）未实现：返回 (text.len(), text.chars().count())
- 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`

## 六、学习进度对比

这是本工具记录的**首次记录**：`history.jsonl` 里还没有更早的运行，暂时无可对比的历史。继续完成练习后再次运行本命令，就能看到分数变化与各模块进步。

本次基线：0 分，通过 0 个知识点。


> 对比口径：`history.jsonl` 记录的是每次运行的**模块级聚合**（模块通过数），因此这里给出总分变化与各模块通过数增减；单条知识点的增减见上表明细。历史文件：`target\ledger-sample-new\history.jsonl`（本次已追加）。

## 七、下一步行动清单

1. 实现 157 个还留着 `todo_exercise(...)` 的知识点（`lesson_12` 里最多，共 11 个）：删掉占位那一行，按函数上方「实现要求」写代码，再运行 `cargo test --test <目标名>` 验证。
2. 把 `lesson_12`（Trait）作为本次重点：从 `kp_12_01` 开始。
3. 每次练习后重新运行 `cargo run --bin assessment_report`，用「学习进度对比」确认没有退步。

