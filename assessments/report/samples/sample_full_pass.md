# 学习评估报告（assessment_report）

- 生成时间：2026-10-04 18:23:49（本地时间）
- 运行 id：`solution-verify`
- 本次结束时间：2026-10-04 18:23:42（本地时间）
- 数据来源（账本）：`target\solution-verify\ledger.tsv`
- 报告文件：`target\sample-full\assessment_report.md`
- 知识点矩阵：`assessments\docs\04_knowledge_map.md`

**计分口径**
- 预期知识点集合来源：**知识点矩阵**；知识点矩阵与账本并集一致。
- 预期知识点总数：**157**（账本中所有运行出现过的知识点并集；矩阵存在时以矩阵为准）
- **分母口径**：总分 = 通过知识点数 ÷ 预期知识点总数 × 100（四舍五入）= 157 ÷ 157 → **100 分**；某个模块本轮**完全没跑**时，它的知识点**仍然计入分母**（本轮记作「未运行或编译失败」），这样跨次运行比较分数才有意义。
- 等级线：≥90 优秀 / ≥75 良好 / ≥60 合格 / <60 待加强
- 判定口径：同一次运行内同一知识点只取**最后一组**（最后一个 `start` 及其之后的事件），这样重复运行同一测试目标不会重复计数；
  有 `pass` → 通过，否则依次看 `missing`（未实现）/ `fail`（未通过）/ `panic`（运行期错误），有 `start` 但没有终态事件的算「未完成或中断」；本轮一条记录都没有的算「未运行或编译失败」。

## 一、总览

| 指标 | 数值 |
| --- | --- |
| 总分 | 100 分 |
| 等级 | 优秀 |
| 通过 | 157 |
| 未实现 | 0 |
| 未通过 | 0 |
| 运行期错误 | 0 |
| 未完成或中断（本次有记录但没有终态事件） | 0 |
| 本轮未运行或编译失败（计入分母） | 0 |
| 预期知识点总数 | 157 |

**一句话诊断**：本次运行 157 个知识点全部通过（100 分 · 优秀）：可以进入下一课，并重新运行 `--emit-map` 更新知识点矩阵。

## 二、模块得分表（按课号顺序）

| 模块 | 模块名 | 知识点数 | 通过 | 未实现 | 未通过 | 得分 | 等级 | 本次是否运行 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| lesson_01 | 变量与可变性 | 8 | 8 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_02 | 数据类型 | 8 | 8 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_03 | 函数 | 8 | 8 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_04 | 流程控制 | 8 | 8 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_05 | 所有权系统 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_06 | 结构体 | 8 | 8 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_07 | 枚举与模式匹配 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_08 | 常见集合 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_09 | 包和模块 | 7 | 7 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_10 | 错误处理 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_11 | 泛型 | 10 | 10 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_12 | Trait | 11 | 11 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_13 | 生命周期 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_14 | 闭包 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_15 | 迭代器 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_16 | 智能指针 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_17 | 线程与通道 | 9 | 9 | 0 | 0 | 100 | 优秀 | 已运行 |
| lesson_18 | 声明宏 | 8 | 8 | 0 | 0 | 100 | 优秀 | 已运行 |
## 三、分类掌握率

| 类型 | 通过 | 总数 | 掌握率 |
| --- | --- | --- | --- |
| 基础 | 25 | 25 | 25/25（100%） |
| 核心 | 75 | 75 | 75/75（100%） |
| 难点 | 45 | 45 | 45/45（100%） |
| 边界 | 12 | 12 | 12/12（100%） |

各类型掌握率都比较均衡，继续保持「先跑通、再优化」的节奏。

## 四、逐条明细（按模块分组）

### lesson_01 · 变量与可变性（通过 8/8，得分 100 · 优秀）

- `kp_01_01` **let 绑定与 mut：默认不可变，需要改值就写 mut**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:50`
  - 作者复习指引：复习 lesson_01 示例 1（immutable_by_default）与示例 2（mut）：`let x = 1; x = 2;` 会报 error[E0384]，必须写成 `let mut x = 1;`。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_02` **变量遮蔽（shadowing）：同名 let 覆盖旧绑定**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:113`
  - 作者复习指引：复习 lesson_01 示例 3（shadowing）：遮蔽是「新建一个同名绑定」，不是修改旧值，所以不需要 mut，也可以换类型。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_03` **遮蔽可以改变类型，mut 不能（这一条是两者的本质区别）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:180`
  - 作者复习指引：复习 lesson_01 示例 3 与示例 7 的错误 2：`let mut v = 5; v = "hi";` 报 E0308，而 `let v = 5; let v = "hi";` 合法——遮蔽新建绑定，允许类型变化。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_04` **const 与 static：编译期常量 vs 静态变量（命名用 SCREAMING_SNAKE_CASE）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:243`
  - 作者复习指引：复习 lesson_01 示例 5（const_and_static）：const 在编译期求值、可写在任何作用域；static 有固定内存地址、全程只有一个实例；两者都必须显式写类型。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_05` **类型推断与显式标注：能推断就推断，推断不了必须标注**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:294`
  - 作者复习指引：复习 lesson_01 示例 4（type_inference_and_annotation）与示例 7 的错误 5：`.parse()` 与 `Vec::new()` 这类无法从上下文推断目标类型的地方，必须显式标注。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_06` **作用域与遮蔽：内层 let 遮蔽不影响外层绑定的值**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:348`
  - 作者复习指引：复习 lesson_01 示例 3 的作用域部分：遮蔽是「新绑定」，内层块结束时旧绑定重新可见。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_07` **const 表达式：单位换算在编译期完成，零运行期开销**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:395`
  - 作者复习指引：复习 lesson_01 示例 5：`const KB: usize = 1024; const MB: usize = KB * 1024;` 这类写法在编译期就求值，运行期只是一个立即数。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`
- `kp_01_08` **常见错误诊断：E0384 / E0308 / E0381 分别对应哪类错误**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_01_variables_mutability.rs:445`
  - 作者复习指引：复习 lesson_01 示例 7（common_mistakes）：本课列的 5 个坑里，前三个分别是「二次赋值给不可变变量」「mut 变量赋了不同类型」「未初始化就使用」。
  - 复习入口：`复习入口：cargo run --bin lesson_01_variables_mutability（src/tutorial/lesson_01_variables_mutability.rs）；说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」`

### lesson_02 · 数据类型（通过 8/8，得分 100 · 优秀）

- `kp_02_01` **整数类型与字面量：位宽决定取值范围，字面量用后缀指定类型**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:53`
  - 作者复习指引：复习 lesson_02 示例 1（integer_types）：整数默认是 i32；u8 的范围是 0..=255，i16 的范围是 -32768..=32767；下划线只是可读性分隔符，后缀（u8/i16/u32/u64）才决定类型。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_02` **整数溢出：wrapping_add / saturating_add / overflowing_add / checked_add 的区别**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:126`
  - 作者复习指引：复习 lesson_02 示例 8 的错误 1（common_mistakes）：debug 构建下 `250u8 + 10` 会直接 panic（attempt to add with overflow），必须改用四个显式方法之一；示例 1 里的 `u8::MAX` 是理解「溢出边界」的起点。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_03` **浮点基础：默认 f64、0.1 + 0.2 != 0.3、NaN 不等于任何值（包括自身）**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:200`
  - 作者复习指引：复习 lesson_02 示例 2（float_types）与示例 8 的错误 5：浮点数不能用 `==` 判断「相等」，要比差值 `(a - b).abs() < 1e-9`；f64 的 NaN 与任何值比较都是 false。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_04` **布尔与字符：bool 只有 true/false，char 是 4 字节的 Unicode 标量值**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:269`
  - 作者复习指引：复习 lesson_02 示例 3（bool_and_char_literals）：char 用单引号书写、占 4 字节；`as u32` 得到 Unicode 码点，`len_utf8()` 得到这个字符在 UTF-8 里占几个字节。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_05` **元组解构：一次把元组拆成多个变量，再参与运算**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:337`
  - 作者复习指引：复习 lesson_02 示例 4（tuples）：解构是 `let (a, b, c) = 元组;`；也可以用 `.0/.1/.2` 按位置访问，两种写法等价，但解构更适合「一次拿到全部字段」。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_06` **数组安全访问：get 返回 Option，比 [] 索引更安全（越界不 panic）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:401`
  - 作者复习指引：复习 lesson_02 示例 5（arrays）与示例 8 的错误 2：`数组[index]` 越界会 panic（index out of bounds），而 `数组.get(index)` 返回 `Option`，越界只是 `None`。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_07` **类型转换：`as` 静默截断，`u8::try_from` 做范围检查并返回 Result**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:474`
  - 作者复习指引：复习 lesson_02 示例 6（type_conversion）：`300i32 as u8` 得到 44（二进制截断）；`u8::try_from(300)` 得到 `Err`；`From` 只用于「一定成功」的加宽转换，如 `u32::from(1000u16)`。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`
- `kp_02_08` **元组比较与数组长度：元组按字典序逐元素比较，数组长度是类型的一部分**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_02_data_types.rs:571`
  - 作者复习指引：复习 lesson_02 示例 4（tuples）与示例 5（arrays）：元组实现了 `PartialOrd`，从第 0 个元素开始逐个比较、第一个不同的元素决定结果；`[u8; 4]` 里的 4 是类型的一部分。
  - 复习入口：`复习入口：cargo run --bin lesson_02_data_types（src/tutorial/lesson_02_data_types.rs）；说明见 src/tutorial/README.md 第一阶段「02 数据类型」`

### lesson_03 · 函数（通过 8/8，得分 100 · 优秀）

- `kp_03_01` **函数定义：参数逐个标注类型，尾表达式就是返回值**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:60`
  - 作者复习指引：复习 lesson_03 示例 1（define_and_call）与示例 2（multiple_params_and_return）：签名写成 `fn add(a: i32, b: i32) -> i32`，函数体最后一行 `a + b` 不带分号；示例 1 还演示了「调用时机与定义顺序无关」。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_02` **多返回值：用一个元组同时带回面积与周长**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:128`
  - 作者复习指引：复习 lesson_03 示例 2（multiple_params_and_return）：`divide_with_remainder` 用 `-> (i32, i32)` 一次返回商和余数，调用方用 `let (q, r) = ...` 解构接收；参数类型也必须逐个写全，不能省略。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_03` **语句与表达式：`if` 是表达式，放在函数体末尾就是返回值**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:198`
  - 作者复习指引：复习 lesson_03 示例 3（statement_vs_expression）：`let sign_label = if x > 0 { ... } else { ... };` 说明 if 会产生值；示例 7 的错误 3 演示了尾表达式多写分号会得到 `()`（E0308）。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_04` **提前 return + 尾表达式：先挡掉非法输入，再走正常路径**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:267`
  - 作者复习指引：复习 lesson_03 示例 3 的 `positive_only`：`if value <= 0 { return 0; }` 提前返回，末尾直接写 `value` 作为尾表达式；示例 6 的 `safe_divide` 也用了同样的「守卫」写法。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_05` **函数指针：`fn(i32) -> i32` 是类型，函数名与不捕获环境的闭包都能当值传**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:336`
  - 作者复习指引：复习 lesson_03 示例 4（function_pointer）：`let operation: fn(i32, i32) -> i32 = add;` 说明函数名可以直接赋给函数指针；`let increment: fn(i32) -> i32 = \|value\| value + 1;` 说明不捕获外部变量的闭包能强制转换成 fn 指针（示例 1 把折扣率写成常量，正是为此）。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_06` **发散函数（`!`）：永不返回的函数可被强制转换成任意类型**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:403`
  - 作者复习指引：复习 lesson_03 示例 5（diverging_function）：`fn exit_with_error(message: &str) -> !` 能放在需要 i32 的 match 分支里，因为 `!` 可以强转成任何类型；示例 7 的错误 6 提醒：发散函数之后的语句不可达，编译器会给出 unreachable_code 警告。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_07` **函数指针数组：按顺序把每个函数作用到 value 上（空数组原样返回）**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:496`
  - 作者复习指引：复习 lesson_03 示例 4（function_pointer）的「策略表」：`let strategies: [fn(i32, i32) -> i32; 3] = [add, subtract, max_of];`；示例 6 的 `pipeline` 演示了把函数指针放进数组依次执行、组合小函数完成完整计算。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`
- `kp_03_08` **常见错误诊断：函数课前三个坑分别报什么（两个无编号解析错误 + E0308）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_03_functions.rs:579`
  - 作者复习指引：复习 lesson_03 示例 7（common_mistakes）：错误 1「参数漏写类型」报的是解析阶段的 `expected one of `:`, `@`, or `\|``，课程注释明确写了**没有 E 编号**；错误 2「漏写返回类型却用表达式返回值」报 E0308（expected `()`, found `i32`）；错误 3「尾表达式多写分号」同样报 E0308（expected `i32`, found `()`）。
  - 复习入口：`复习入口：cargo run --bin lesson_03_functions（src/tutorial/lesson_03_functions.rs）；说明见 src/tutorial/README.md 第一阶段「03 函数」`

### lesson_04 · 流程控制（通过 8/8，得分 100 · 优秀）

- `kp_04_01` **if 是表达式：分支的值可以直接做函数尾表达式**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:53`
  - 作者复习指引：复习 lesson_04 示例 1（if_expression）：`if` 有值、两个分支必须同类型，所以 `if n > 0 { "正数" } else { "负数" }` 可以直接当返回值，末尾**不要**写分号，也**不要**写 `return`。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_02` **if / else if 链：条件自上而下判断，命中第一个为真的分支**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:128`
  - 作者复习指引：复习 lesson_04 示例 2（if_else_chain）：多分支按**从高到低**的顺序比较，顺序写反会让高分先落进低档；最后的 `else` 是兜底分支，本题约定分数 > 100 这类越界输入也落到 `'D'`。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_03` **loop 与 break 带值：把循环当成有返回值的表达式**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:220`
  - 作者复习指引：复习 lesson_04 示例 4（break_value_and_continue）：`let x = loop { … break 值; };` 是 Rust 特有的写法——`loop` 是表达式，`break 值` 就是它的值；只有 `loop` 支持 break 带值，`while` / `for` 不行（会报 E0571）。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_04` **while 循环与 continue：跳过奇数，只累加偶数**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:290`
  - 作者复习指引：复习 lesson_04 示例 3（loop_while_for）与示例 4 的 continue 部分：`while` 先判断条件再执行循环体，因此必须保证循环变量趋向结束；`continue` 立即进入下一轮，它后面的累加语句本轮不会执行。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_05` **for 遍历切片：元素个数由迭代器决定，不需要手写下标**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:366`
  - 作者复习指引：复习 lesson_04 示例 3 的 for 部分：`for value in items { … }` 直接遍历 `&[i32]`，因为切片实现了 `IntoIterator`；`for index in 0..items.len()` 也能写，但一旦下标写错就越界 panic，所以优先用 for-each。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_06` **循环标签（'outer:）：break 'outer 一次跳出两层嵌套循环**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:442`
  - 作者复习指引：复习 lesson_04 示例 5（loop_labels）：给外层循环写 `'outer: for row in …`，内层用 `break 'outer;` 跳出外层——这比用布尔标志位「绕圈」退出清晰得多；找不到目标时两个循环都自然跑完，函数尾部返回 `None`。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_07` **循环实现 Collatz 步数：偶数减半、奇数 3n+1，数到 1 为止**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:519`
  - 作者复习指引：复习 lesson_04 示例 3 与示例 4：这一题考的是「循环 + 计数器 + 终止条件」三件套；关键是**先判断是否已经等于 1**——n 本来就是 1 时一步都不该走，返回 0。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`
- `kp_04_08` **常见错误诊断：E0308（loop 的值）/ E0571（while 里 break 带值）/ E0425（循环外用了循环变量）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_04_control_flow.rs:590`
  - 作者复习指引：复习 lesson_04 示例 7（common_mistakes）：本课列了 7 个坑，其中**有编译器错误编号**的第一个是「loop 体最后是表达式却没写 break 带值」（E0308），第二个是「while 循环里 break 带值」（E0571），第三个是「循环变量在循环外被使用」（E0425）；注意「while 条件永远不变」属于运行期死循环，没有错误编号。
  - 复习入口：`复习入口：cargo run --bin lesson_04_control_flow（src/tutorial/lesson_04_control_flow.rs）；说明见 src/tutorial/README.md 第一阶段「04 流程控制」`

### lesson_05 · 所有权系统（通过 9/9，得分 100 · 优秀）

- `kp_05_01` **move 语义：String 传参即移交所有权，函数通过返回值把所有权交还**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:81`
  - 作者复习指引：复习 lesson_05 示例 1（three_rules）与示例 2（move_vs_copy）：`String` 没有实现 Copy，所以 `let t = s;` 或把 `s` 传进函数都是 move；move 之后原变量**不可再使用**，否则报 `error[E0382]: borrow of moved value`。本题的练习函数按值接收 String、再把同一个 String 装进元组返回，所有权于是回到调用方。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_02` **Copy 与 Clone：i32 赋值是复制、String 必须显式 .clone()**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:167`
  - 作者复习指引：复习 lesson_05 示例 2（move_vs_copy）与示例 3（clone_explicit）：`i32` 实现了 `Copy`，赋值/传参只是复制 4 个字节，原变量照样能用，所以 `n` 可以在同一个表达式里出现两次；`String` 没有 Copy，想要「一份留着、一份交出去」只能写 `.clone()`，它在堆上真分配、真拷贝。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_03` **共享借用：同一时刻可以存在任意多个 &T（只读共享是安全的）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:277`
  - 作者复习指引：复习 lesson_05 示例 5（borrow_rules）：`let a: &String = &text; let b: &String = &text;` 两个不可变借用同时存在完全合法，因为只读、不会有人看到「写了一半」的数据；反过来说，只要出现一个 `&mut`，就要求「没有其它任何借用同时活着」。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_04` **可变借用：两次 &mut 只要不重叠就合法（每次借用用完即结束）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:349`
  - 作者复习指引：复习 lesson_05 示例 6（mutable_borrow）与示例 7（nll）：`v.push(1); v.push(2);` 看起来是两次可变借用，但第一次借用在 `push` 返回时就结束了，所以第二次借用不冲突——这就是 NLL（非词法生命周期）；反过来，`let r1 = &mut v; let r2 = &mut v;` 两个引用同时活着就报 `error[E0499]`。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_05` **返回切片：最长单词是原句的一部分，返回 &str 而不是 String**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:431`
  - 作者复习指引：复习 lesson_05 示例 9（typical_scenario_longest_word）：返回值写成 `&str` 并在省略生命周期的情况下由编译器绑定到唯一的引用参数 `text` 上；实现上用 `longest.len()` 做「严格大于」比较，长度相同时保留**先出现**的那个单词，而 `longest` 的初值必须是 `""`，这样空串才能安全返回空切片。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_06` **len 与 capacity：len 是已有元素个数，capacity 是已分配的坑位数**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:528`
  - 作者复习指引：复习 lesson_05 示例 4（stack_and_heap）：`Vec::with_capacity(10)` 只预留坑位、不放元素，所以 `len() == 0` 而 `capacity() == 10`；`push(1)` 放了一个元素，`len()` 变成 1，而预留的容量足够，`capacity()` 仍是 10（不会被无故放大）。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_07` **NLL 边界：先把值取出来（借用结束），再可变借用修改**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:580`
  - 作者复习指引：复习 lesson_05 示例 7（nll_non_lexical_lifetime）：借用的有效范围由**最后一次使用**决定；本题中 `last` 是一个 `i32`（Copy 类型），`*v.last().unwrap()` 一取值，不可变借用就结束了，所以后面 `v.push(99)` 完全不冲突；如果写成 `let last = v.last().unwrap();`（保留 `&i32`）再 push，就会报 `error[E0502]`。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_08` **常见错误诊断：E0382（move 后使用）/ E0499（两个可变借用）/ E0502（不可变与可变重叠）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:675`
  - 作者复习指引：复习 lesson_05 示例 10（common_mistakes）：错误 1 是「move 之后继续用原变量」（E0382），错误 2 是「同一时刻两个 `&mut`」（E0499），错误 3 是「不可变借用与可变借用重叠」（E0502）；这三个编号覆盖了初学者 90% 的所有权编译失败。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`
- `kp_05_09` **Drop 顺序：作用域结束时后声明的先 drop，内层作用域先于外层**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_05_ownership_borrowing.rs:729`
  - 作者复习指引：复习 lesson_05 示例 1（three_rules）的规则三与示例 4（stack_and_heap）：值离开作用域时自动 drop；同一个作用域内**后声明的先 drop**（逆序），内层块整体先于外层块结束。本题期望的顺序是 ["b", "c", "a"]。
  - 复习入口：`复习入口：cargo run --bin lesson_05_ownership_borrowing（src/tutorial/lesson_05_ownership_borrowing.rs）；说明见 src/tutorial/README.md 第二阶段「05 所有权系统」`

### lesson_06 · 结构体（通过 8/8，得分 100 · 优秀）

- `kp_06_01` **结构体定义与实例化：字段写全、用点号访问**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:112`
  - 作者复习指引：复习 lesson_06 示例 1（define_and_instantiate）：先 `struct Rectangle { width: u32, height: u32 }` 定义形状，再 `Rectangle { width: 30, height: 50 }` 实例化，然后用 `rect.width` / `rect.height` 点号读字段——实例化时字段必须写全（漏了会报 E0063）。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_02` **字段初始化简写与结构体更新语法（..other）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:166`
  - 作者复习指引：复习 lesson_06 示例 2（field_init_shorthand）与示例 3（struct_update_syntax）：字段名与变量名相同时可以只写一次名字；`Config { port: 9090, ..first }` 只覆盖写出来的字段，其余字段从 `first` 补齐，非 Copy 字段（String）会被 move。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_03` **方法接收者：&self 读、&mut self 改、self 消费**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:265`
  - 作者复习指引：复习 lesson_06 示例 6（methods_and_receivers）：`&self` 只读借用、`&mut self` 可写借用、`self` 按值接收会把调用者的所有权 move 进方法（调用后原变量失效）。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_04` **关联函数与 Self：用 `类型名::函数名` 调用，不依赖实例**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:383`
  - 作者复习指引：复习 lesson_06 示例 7（associated_functions）：`impl` 块里不带 self 的函数是关联函数，用 `Point::origin()` 调用；返回类型写 `Self` 就是当前类型的别名，等价于写 `Point`。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_05` **元组结构体（.0 位置访问）与单元结构体（零大小标记）**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:492`
  - 作者复习指引：复习 lesson_06 示例 4（tuple_struct）与示例 5（unit_struct）：元组结构体字段没有名字，只能用 `.0` / `.1` 按位置访问；单元结构体没有任何字段，值就是类型名本身，占 0 字节。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_06` **手动实现 Display（{}）与派生 Debug（{:?}）的分工**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:555`
  - 作者复习指引：复习 lesson_06 示例 8（debug_and_display）：`#[derive(Debug)]` 给的是面向开发者的 `{:?}`，而 `{}` 必须手动 `impl std::fmt::Display`；Display 负责把内部数据翻译成用户语言。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_07` **链式调用：构造器方法消费 self 并返回 Self**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:625`
  - 作者复习指引：复习 lesson_06 示例 6 的消耗型方法（`self` 接收者）与示例 7 的构造函数惯例：链式调用的每一步都要把 `self` 交出去（返回 `Self`），最后一步 `build` 才产出结果；中间任何一步返回 `&mut Self` 或 `()` 都会让链断掉。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`
- `kp_06_08` **部分移动：move 出 String 字段的同时仍可读取 Copy 字段**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_06_structs.rs:747`
  - 作者复习指引：复习 lesson_06 示例 3 与示例 10 的错误 7：从结构体里把非 Copy 字段（String）move 出来之后，该实例不能再用作整体，但**没被移走**的字段（如 u32 的 id）仍然可以读。
  - 复习入口：`复习入口：cargo run --bin lesson_06_structs（src/tutorial/lesson_06_structs.rs）；说明见 src/tutorial/README.md 第二阶段「06 结构体」`

### lesson_07 · 枚举与模式匹配（通过 9/9，得分 100 · 优秀）

- `kp_07_01` **match 的穷尽性：每个变体都要有分支**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:85`
  - 作者复习指引：复习 lesson_07 示例 4（match_basics_and_wildcard）：`match` 必须覆盖所有可能，少写一个变体会报 `error[E0004]: non-exhaustive patterns`；元组变体用 `Shape::Circle(r)` 取值，结构体变体用 `Shape::Rectangle { width, height }` 按字段名绑定。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_02` **Option<T>：把「可能没有值」写进类型里**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:171`
  - 作者复习指引：复习 lesson_07 示例 2（option）：`Option` 只有 `Some(值)` 与 `None` 两个变体，处理时必须显式应对 `None`；`match (a, b)` 一次匹配两个 Option 是最直观的写法。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_03` **Result<T, E>：成功取 Ok，失败给出可读原因**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:245`
  - 作者复习指引：复习 lesson_07 示例 3（result）：`Result` 的 `Err` 分支携带「为什么失败」，比 `Option` 更适合解析类操作；`.parse::<u16>()` 的 `Err` 类型是 `ParseIntError`，要转成自己的 `String` 说明。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_04` **或模式 `\|` 与通配 `_`：合并同结果分支、兜住其余取值**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:334`
  - 作者复习指引：复习 lesson_07 示例 4 的 `\|` 与 `_` 部分：`2 \| 3` 把两个模式合并到同一个分支；`_` 兜住所有剩余取值，但它必须写在**最后一个**分支。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_05` **匹配守卫（if）与 @ 绑定：范围判断 + 取到原值**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:415`
  - 作者复习指引：复习 lesson_07 示例 5（guards_and_bindings）：守卫写在模式之后、`=>` 之前，例如 `n if n % 2 == 0`；`@` 绑定既做范围判断又把值绑给变量，例如 `s @ 0..=59`。分支顺序很重要：带守卫的分支必须写在同范围的裸模式之前。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_06` **while let：不断取出直到 None，None 用 continue 跳过**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:501`
  - 作者复习指引：复习 lesson_07 示例 6（if_let_and_while_let）：`while let Some(v) = iter.next()` 每次循环都尝试取出一个值，取出 `None` 时循环自然结束，因此特别适合「消耗式遍历」。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_07` **let else：失败分支发散，成功路径无需再嵌套**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:573`
  - 作者复习指引：复习 lesson_07 示例 7（let_else）：`let ... else { ... }` 的 else 块必须发散（`return` / `continue` / `break` / `panic!`），因此后面的代码可以直接使用解包后的值。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_08` **枚举 + match 的典型用法：文本 → 结构化命令**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:645`
  - 作者复习指引：复习 lesson_07 示例 8（typical_scenario_command_parser）：`split_whitespace()` 天然处理多余空格，`parts.next()` 逐个取词，取不到参数就返回 `None`；解析成功后用枚举变体（元组变体 / 结构体变体 / 无数据变体）把结果结构化。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`
- `kp_07_09` **常见错误诊断：E0004 / E0308 / E0005 分别对应哪类错误**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_07_enums_pattern_matching.rs:741`
  - 作者复习指引：复习 lesson_07 示例 9（common_mistakes）的错误 1~3：错误 1 是 match 分支没写全（non-exhaustive patterns），错误 2 是模式与值的类型不一致（mismatched types: expected integer, found `&str`），错误 3 是把可反驳模式用在 `let` 位置（refutable pattern in local binding）。
  - 复习入口：`复习入口：cargo run --bin lesson_07_enums_pattern_matching（src/tutorial/lesson_07_enums_pattern_matching.rs）；说明见 src/tutorial/README.md 第二阶段「07 枚举与模式匹配」`

### lesson_08 · 常见集合（通过 9/9，得分 100 · 优秀）

- `kp_08_01` **Vec 的创建与 push：`to_vec()` 复制出拥有所有权的向量，`push` 在尾部追加**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:64`
  - 作者复习指引：复习 lesson_08 示例 1（vec_create_and_push）：`Vec::new` / `vec![]` / `Vec::with_capacity` 三种创建方式，以及 `push` 把元素追加到末尾（必要时扩容）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_02` **Vec 的索引与安全访问：`get` / `first` 返回 `Option`，`[]` 越界会 panic**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:134`
  - 作者复习指引：复习 lesson_08 示例 2（vec_access_and_modify）：`v[i]` 越界在运行期 panic （index out of bounds），`v.get(i)` 把「可能越界」表达成 `Option`，`v.first()` 同理。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_03` **Vec 的原地修改：`retain` 按条件删除、`iter_mut` 解引用修改元素**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:205`
  - 作者复习指引：复习 lesson_08 示例 1 与示例 3（vec_iterate_and_capacity）：`retain` 就地保留满足条件的元素（比倒序 `remove` 直观），`for value in v.iter_mut() { *value *= 2; }` 才能修改元素。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_04` **String 与 UTF-8：`len()` 是字节数，`chars().count()` 是字符数**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:275`
  - 作者复习指引：复习 lesson_08 示例 4（string_basics_and_utf8）：中文一个字占 3 字节，emoji 占 4 字节，所以 `len()` 常大于 `chars().count()`；按字节下标切分不在字符边界上会 panic。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_05` **字符串拼接三种方式：`push_str` 原地追加、`+` 消耗左侧、`format!` 不消耗任何操作数**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:351`
  - 作者复习指引：复习 lesson_08 示例 5（string_concat_and_format）：`push_str` 不转移所有权；`a + &b` 里 `a` 的所有权被消耗（之后不能再使用 `a`）；`format!` 返回新 `String`，参数都还可用。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_06` **HashMap 基础：`insert` 覆盖旧值并返回旧值；遍历顺序不固定，输出前必须排序**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:448`
  - 作者复习指引：复习 lesson_08 示例 6（hashmap_basics）：`HashMap` 不保证遍历顺序，所以「需要确定输出」时必须先排序；`insert` 对已存在的键会覆盖旧值（并返回被覆盖的旧值）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_07` **entry API 统计词频：`entry(word).or_insert(0)` 一次查找完成「查 + 改」，输出前排序**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:522`
  - 作者复习指引：复习 lesson_08 示例 7 与示例 8（hashmap_entry_api / word_frequency_scenario）：用 `*counts.entry(word).or_insert(0) += 1;` 统计；排序规则是`b.1.cmp(&a.1).then_with(\|\| a.0.cmp(&b.0))`（次数多的在前，次数相同按字典序）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_08` **借用陷阱：`get` 拿到引用期间不能 `insert`（E0502），先把值拷贝出来即可**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:609`
  - 作者复习指引：复习 lesson_08 示例 9（ownership_and_borrow_traps）陷阱三：`let value = counter.get("hits").unwrap(); counter.insert("misses", 1);` 会报`error[E0502]: cannot borrow as mutable because it is also borrowed as immutable`；修正就是先 `copied()` 把值取出来，让不可变借用立刻结束。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`
- `kp_08_09` **常见错误诊断：E0596 / E0282 / E0277 分别对应哪类集合错误**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_08_collections.rs:691`
  - 作者复习指引：复习 lesson_08 示例 10（common_mistakes）：错误 1 是「`Vec::new()` 忘了 mut 就 push」（E0596），错误 2 是「元素类型完全没有来源」的 `Vec::new()`（E0282），错误 3 是「用整数下标访问 `String`」（E0277：`str` 不能用 `{integer}` 索引）。
  - 复习入口：`复习入口：cargo run --bin lesson_08_collections（src/tutorial/lesson_08_collections.rs）；说明见 src/tutorial/README.md 第三阶段「08 常见集合」`

### lesson_09 · 包和模块（通过 7/7，得分 100 · 优秀）

- `kp_09_01` **模块基础：`mod math { pub fn ... }` + 路径 `math::add` 调用**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:64`
  - 作者复习指引：复习 lesson_09 示例 1（module_basics）：模块用 `mod 名字 { ... }` 声明，里面的项默认私有，必须写 `pub fn` 才能从模块外用 `模块名::函数名` 调用。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_02` **可见性层级：`pub(crate)`（本 crate 可见）/ `pub(super)`（父模块可见）/ 私有（仅本模块可见）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:121`
  - 作者复习指引：复习 lesson_09 示例 2（visibility_levels）：四种可见性分别是 `pub`、`pub(crate)`、`pub(super)`、不加修饰（私有，只有本模块及其子模块可见）；访问范围只能缩小不能放大。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_03` **`use` 引入与 `as` 重命名：路径别名不改变可见性，只改调用时写的名字**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:193`
  - 作者复习指引：复习 lesson_09 示例 3（use_and_rename）：`use 路径::项 as 别名;` 之后就能用别名调用；别名可以解决同名冲突，也可以让调用点更短。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_04` **`pub use` 重导出：门面模块让调用方只记住一条短路径**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:253`
  - 作者复习指引：复习 lesson_09 示例 4（pub_use_reexport）：`pub use` 把内部的项「搬」到当前模块名下，内部结构怎么改都不影响调用方；重导出的可见性不能放大（`pub(crate)` 项不能被 `pub use` 到 crate 外，否则报 `error[E0364]` / `error[E0365]`）。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_05` **模块路径：`super::` 父模块、`self::` 当前模块、`crate::` 绝对路径（crate 根）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:324`
  - 作者复习指引：复习 lesson_09 示例 5（module_paths）：`crate::` 从 crate 根出发，任何模块里都能用；`self::foo` 与 `foo` 等价，强调「在本模块里找」；`super::foo` 往上一级找，子模块因此能访问父模块里连 `pub` 都没有的私有项。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_06` **迷你库：`pub mod` 暴露 API、私有辅助函数隐藏实现、`pub use` 汇总成一条门面**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:386`
  - 作者复习指引：复习 lesson_09 示例 8（mini_library_scenario）：一个「库」对外只需要稳定、简短的公开 API；内部模块划分、私有字段、私有辅助函数都可以随时重构，只要门面 `pub use` 的路径不变。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`
- `kp_09_07` **常见错误诊断：E0603（私有项）/ E0425（路径写错）/ E0255（use 与本地定义同名）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_09_packages_modules.rs:468`
  - 作者复习指引：复习 lesson_09 示例 7（common_mistakes）：错误 1 是「调用私有函数或读取私有常量」（E0603），错误 2 是「`pub(super)` 项被更外层的模块调用」——编译器同样报 E0603，错误 3 是「路径写错，少了层级」的 `lessons::add`（E0425）。
  - 复习入口：`复习入口：cargo run --bin lesson_09_packages_modules（src/tutorial/lesson_09_packages_modules.rs）；说明见 src/tutorial/README.md 第三阶段「09 包和模块」`

### lesson_10 · 错误处理（通过 9/9，得分 100 · 优秀）

- `kp_10_01` **可恢复错误：用 Result 表达「除数为 0」，把决定权交给调用方**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:137`
  - 作者复习指引：复习 lesson_10 示例 3（result_and_match）与示例 10（when_to_panic_or_result）：外部输入与可预期的运行结果用 Result 表达，只有「调用方违反了契约」才 panic。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_02` **unwrap / expect：成功时取值，失败时 panic（风险就在这里）**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:218`
  - 作者复习指引：复习 lesson_10 示例 2（unwrap_and_expect）：unwrap 出错时用默认信息 panic，expect("...") 可以自带上下文；两者都只适合「已经论证过不会失败」的场合。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_03` **用 match 处理 Result：两个分支都必须处理，编译器强制你面对错误**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:282`
  - 作者复习指引：复习 lesson_10 示例 3（result_and_match）：`match` 的 Ok / Err 两个分支必须都写，失败时返回固定文案；`if let` 只适合只关心一侧的场景。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_04` **`?` 运算符：Ok 时取值继续执行，Err 时立刻 return Err(...)**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:353`
  - 作者复习指引：复习 lesson_10 示例 4（question_mark_operator）：`?` 只能用在返回 Result / Option 的函数里；两个 `?` 顺序传播，任何一个失败都会提前返回。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_05` **自定义错误类型：用枚举区分失败原因，用 Display 说人话**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:441`
  - 作者复习指引：复习 lesson_10 示例 5（custom_error_type）：错误类型要同时实现 Display 与std::error::Error；Display 给出「人能读懂的一句话」，Debug 给出结构化信息。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_06` **From 上转：`?` 自动把 ParseError 转换成 AppError**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:542`
  - 作者复习指引：复习 lesson_10 示例 6（from_conversion_via_question_mark）：`expr?` 在出错时会做`From::from(err)`，把底层错误转成函数返回类型的错误类型。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_07` **Box<dyn Error> 统一出口：`?` 把 ParseIntError 自动装箱**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:620`
  - 作者复习指引：复习 lesson_10 示例 7（box_dyn_error）：标准库已提供 `impl From<E> for Box<dyn Error>`（E: Error + 'static），所以 `?` 能直接把 ParseIntError 装箱后返回。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_08` **典型场景：解析 key=value 配置，跳过空行与 # 注释行，坏行错误信息保留原文**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:700`
  - 作者复习指引：复习 lesson_10 示例 8（config_parsing_scenario）：全有或全无地解析配置，空行跳过；出错时把「哪一行坏了」写进错误信息，方便使用者直接定位。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`
- `kp_10_09` **常见错误诊断：`?` 用错位置、对 Option 用 `?`、缺 From 分别报什么错**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_10_error_handling.rs:794`
  - 作者复习指引：复习 lesson_10 示例 9（common_mistakes）：`?` 相关的错误都落在 E0277（trait 约束不满足）这一族里，读错误信息时要顺着 note: 找到缺失的 trait。
  - 复习入口：`复习入口：cargo run --bin lesson_10_error_handling（src/tutorial/lesson_10_error_handling.rs）；说明见 src/tutorial/README.md 第三阶段「10 错误处理」`

### lesson_11 · 泛型（通过 10/10，得分 100 · 优秀）

- `kp_11_01` **泛型函数：`fn largest<T: PartialOrd>(list: &[T]) -> Option<&T>` 一份代码适配多种类型**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:53`
  - 作者复习指引：复习 lesson_11 示例 1（generic_function）：类型参数 T 代表「待定的类型」，`<T: PartialOrd>` 是 trait bound，告诉编译器 T 支持比较；空切片必须返回 None。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_02` **泛型结构体：`struct Point<T>` 能生成 Point<i32>、Point<f64> 等互相独立的具体类型**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:147`
  - 作者复习指引：复习 lesson_11 示例 2（generic_struct）：字段类型可以是类型参数；`Point<i32>` 与 `Point<f64>` 是两个完全不同的类型，彼此不能互相赋值。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_03` **泛型枚举：标准库的 Option<T> / Result<T, E> 就是泛型枚举，自己定义的也能互相转换**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:203`
  - 作者复习指引：复习 lesson_11 示例 3（generic_enum）：`enum MyOption<T> { Some(T), None }` 与`enum MyResult<T, E> { Ok(T), Err(E) }` 的定义方式与标准库完全一致；每个变体都能无损地转成标准库对应变体。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_04` **泛型方法：impl<T: Copy + Add<Output = T>> Pair<T> 让 sum() 对两种数值类型都可用**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:265`
  - 作者复习指引：复习 lesson_11 示例 4（generic_method）：`impl<T> Point<T>` 表示「对任意 T 都提供这些方法」；约束写在 impl 块上时对块内所有方法生效。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_05` **trait bound 用起来：`<T: Display, U: Display>` 让两种不同类型的值拼成一句话**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:322`
  - 作者复习指引：复习 lesson_11 示例 6（trait_bounds_syntax）：bound 表达「使用方对类型的要求」，编译器会拿每一个具体类型去检查；`Display` 是 `{}` 格式化所需要的约束。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_06` **where 子句：约束放在函数签名下方，比挤在尖括号里更好读**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:387`
  - 作者复习指引：复习 lesson_11 示例 7（where_clause）：`fn largest_where<T>(list: &[T]) -> T whereT: PartialOrd + Copy` 与尖括号写法语义等价，只是排版不同（rustfmt 也偏好它）。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_07` **多个泛型参数：`<K: Display, V: Display>` 让键与值各自是不同类型**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:466`
  - 作者复习指引：复习 lesson_11 示例 8（multiple_type_params）：类型参数可以有两个及以上、各自独立；泛型结构体的字段可以分别使用它们（MixedPoint<T, U> 就是例子）。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_08` **const 泛型：`<const N: usize>` 把编译期常量也作为参数，数组长度是类型的一部分**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:532`
  - 作者复习指引：复习 lesson_11 示例 9（const_generics）：`Buffer<4>` 与 `Buffer<8>` 是不同类型；函数上的 `fn filled<T: Copy, const N: usize>(value: T) -> [T; N]` 也是同一原理。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_09` **单态化（monomorphization）：泛型在编译期展开，运行期没有类型判断开销**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:603`
  - 作者复习指引：复习 lesson_11 示例 10（monomorphization）：编译器为每个用到的具体类型各生成一份专用代码（相当于自动写出 size_of_val_i32、size_of_val_f64），代价是二进制体积与编译时间增加。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`
- `kp_11_10` **常见错误诊断：泛型里直接相加、用 {} 打印未约束 Display、类型参数没被使用**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_11_generics.rs:670`
  - 作者复习指引：复习 lesson_11 示例 12（common_mistakes）：泛型代码的错误几乎都指向「约束不足」或「类型不匹配」；读错误信息时先看是哪一类，再决定补约束还是改类型参数。
  - 复习入口：`复习入口：cargo run --bin lesson_11_generics（src/tutorial/lesson_11_generics.rs）；说明见 src/tutorial/README.md 第四阶段「11 泛型」`

### lesson_12 · Trait（通过 11/11，得分 100 · 优秀）

- `kp_12_01` **为自定义类型实现 trait：impl Area for Circle 与 impl Area for Square**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:101`
  - 作者复习指引：复习 lesson_12 示例 1（定义 trait 并为自定义类型实现）与示例 10（Shape 的 area 实现）：trait 只声明「能做什么」，`impl Trait for 类型 { ... }` 才给出「怎么做」；圆的面积是 πr²（π 用 `std::f64::consts::PI`），正方形是边长²。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_02` **默认方法：只实现必需方法，就能得到 shout() 的默认行为**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:200`
  - 作者复习指引：复习 lesson_12 示例 2（默认方法与覆盖）：trait 里带方法体的方法就是默认方法，实现者不写也能用；默认方法内部可以调用必需方法，例如 `fn shout(&self) -> String { format!("{}!", self.summarize()) }`。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_03` **trait 作为参数（写法一）：item: &impl Area 接收任意实现者**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:266`
  - 作者复习指引：复习 lesson_12 示例 4（trait 作为参数：impl Trait）：`&impl Area` 是「一个匿名类型参数 + Area 约束」的语法糖，同样是静态分发（单态化）。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_04` **trait 作为参数（写法二）：泛型参数 + trait bound，可 turbofish**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:330`
  - 作者复习指引：复习 lesson_12 示例 3（trait 作为参数：泛型 + trait bound）：`fn f<T: Area>(item: &T)` 与 `fn f(item: &impl Area)` 都是静态分发；区别是泛型参数**有名字**，调用处可以写 `f::<Circle>(&c)` 显式指定类型，函数体里也能用 `T` 做别的事。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_05` **返回 impl Trait：调用方只依赖 Area，看不到具体类型**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:396`
  - 作者复习指引：复习 lesson_12 示例 6（返回 impl Trait）：`-> impl Area` 表示「返回某个实现了 Area 的具体类型，但调用方不需要知道它是谁」；代价是所有 return 分支必须是同一个具体类型。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_06` **返回 Box<dyn Area>：分支返回不同类型时必须用 trait 对象**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:464`
  - 作者复习指引：复习 lesson_12 示例 7（返回 Box<dyn Trait>）：`dyn Trait` 的大小在编译期未知，必须装在指针后面（`Box<dyn Trait>` / `&dyn Trait`）；分支各自返回不同类型的唯一办法就是先统一装箱成同一种 `dyn` 类型。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_07` **多个 trait bound：T: Area + Debug，既能算面积又能 {:?} 打印**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:534`
  - 作者复习指引：复习 lesson_12 示例 5（trait bound 组合与 +）：约束用 `+` 叠加，`T: Area + std::fmt::Debug` 表示「既要能算面积，又要有派生的 Debug 实现」；约束叠得越多，能进来的类型越少，所以要按需叠加。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_08` **关联类型：type Item = u32，调用处不需要写类型标注**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:596`
  - 作者复习指引：复习 lesson_12 示例 8（关联类型 vs 泛型参数）：`type Item = u32;` 表达「这个生产者天生只产出一种类型」，所以调用处不用标注；泛型参数的版本（`ConvertTo<T>`）允许同一类型有多份实现，但调用处必须标注，否则报 `error[E0283]: type annotations needed`。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_09` **dyn trait 对象集合：Vec<Box<dyn Area>> 排序后返回面积列表**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:699`
  - 作者复习指引：复习 lesson_12 示例 9 与示例 10（dyn trait 对象、按面积排序）：`Vec<Box<dyn Area>>` 能同时装 Circle / Square / 你自定义的 Rectangle，排序用 `sort_by(\|a, b\| a.area().total_cmp(&b.area()))`（f64 用 total_cmp 得到全序，避免 partial_cmp 返回 None）。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_10` **派生标准库 trait：Default 置零、PartialEq 逐字段比较、Debug 的固定格式**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:783`
  - 作者复习指引：复习 lesson_12 示例 11（标准库 trait 的派生）：`#[derive(Debug, Clone, PartialEq, Default)]` 生成的实现都是「逐字段」的；Debug 的输出格式固定为 `Version { major: 1, minor: 2 }`，Default 把 u32 字段全部置 0。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`
- `kp_12_11` **常见错误诊断：trait bound 未满足 / impl 漏写必需方法 / 重复实现**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_12_traits.rs:853`
  - 作者复习指引：复习 lesson_12 示例 13（common_mistakes）：本课列的 7 个坑里，前三个分别是「类型没有实现 trait」「impl 块漏写必需方法」「同一个类型重复实现同一个 trait」。
  - 复习入口：`复习入口：cargo run --bin lesson_12_traits（src/tutorial/lesson_12_traits.rs）；说明见 src/tutorial/README.md 第四阶段「12 Trait」`

### lesson_13 · 生命周期（通过 9/9，得分 100 · 优秀）

- `kp_13_01` **函数签名里的生命周期注解：返回较长的那个字符串切片**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:82`
  - 作者复习指引：复习 lesson_13 示例 2（函数签名中的生命周期注解）：`fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` 的意思是「返回值活得和这两个参数一样久」——`'a` 会被推断为两个实参中**较短**的那个存活区间；注解只描述关系，不延长任何数据的寿命。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_02` **生命周期省略规则 1 + 2：只有一个输入引用时，输出引用就取它**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:169`
  - 作者复习指引：复习 lesson_13 示例 3（生命周期省略的三条规则）：规则 1 给每个引用参数各一个生命周期参数，规则 2 在「只有一个输入引用」时把输出引用的生命周期取成它；所以 `fn first_word(text: &str) -> &str` 等价于 `fn first_word<'a>(text: &'a str) -> &'a str`。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_03` **两个独立生命周期参数：'a 与 'b 互不影响，返回值只借用 'a**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:249`
  - 作者复习指引：复习 lesson_13 示例 4（多个生命周期参数）：`fn pick_first<'a, 'b>(first: &'a str, second: &'b str) -> &'a str` 明确写出「只与 first 有关」；如果两个参数共用同一个 `'a`，`'a` 会被压缩成较短的那个，返回值就带不出内层作用域了。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_04` **结构体持有引用：Excerpt<'a> 的 new / part / announce**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:324`
  - 作者复习指引：复习 lesson_13 示例 5 与示例 6（结构体持有引用、impl 块与方法中的生命周期）：字段是引用就必须写 `struct Excerpt<'a> { part: &'a str }`；`fn part(&self) -> &str` 用省略规则 3（输出引用取 `&self` 的生命周期）；`announce` 返回 `self.part`，绝不能返回 `msg` —— 因为返回类型借的是 self，不是 msg。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_05` **生命周期省略：签名里一个 'a 都不写，也能安全返回切片**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:456`
  - 作者复习指引：复习 lesson_13 示例 3（省略规则）与示例 9（注解不会延长存活时间）：只有一个输入引用时，输出引用的生命周期自动取它；返回的切片仍然借用 `text`，所以调用方不能让 `text` 先失效。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_06` **'static：字符串字面量活在只读区，带出任何作用域都有效**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:533`
  - 作者复习指引：复习 lesson_13 示例 7（'static 生命周期）：字符串字面量的类型就是 `&'static str`，它的数据嵌在可执行文件里，程序全程有效；要分清 `&'static T`（引用活得够久）与 `T: 'static`（类型内部不含短生命周期借用，拥有所有权的 String、i32 都满足）这两种完全不同的含义。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_07` **生命周期参数 + trait bound：T: Display + 'a，返回拥有所有权的 String**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:603`
  - 作者复习指引：复习 lesson_13 示例 8（生命周期与 trait bound）：`<'a, T: Display + 'a>` 里`'a` 是生命周期参数、`T` 是类型参数，`T: 'a` 约束表示「T 里不含比 'a 更短的借用」；返回 `String` 与 `'a` 无关，所以调用方拿到的东西不受借用区间限制。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_08` **常见错误诊断：E0106 缺生命周期注解（课程前三个错误都是它）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:683`
  - 作者复习指引：复习 lesson_13 示例 11（common_mistakes）：前三个坑分别是「两个引用参数省略了返回值注解」「结构体字段是引用却没写生命周期参数」「返回局部变量的引用」——这三处编译器的诊断都是 missing lifetime specifier（E0106）；再往后才是 E0515（返回局部变量）、E0621（漏标一个参数）、E0597（被借用数据活得太短）。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`
- `kp_13_09` **结构体持有引用：Config<'a> 的 new / endpoint / host**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_13_lifetimes.rs:747`
  - 作者复习指引：复习 lesson_13 示例 10（典型场景：借用式配置）：配置项常常来自命令行参数或环境变量，本来就是长命的 String，用 `&'a str` 借用可以省掉一次分配；`endpoint()` 返回拥有所有权的 `String`，`host()` 用省略规则返回借用 `self` 的 `&str`。
  - 复习入口：`复习入口：cargo run --bin lesson_13_lifetimes（src/tutorial/lesson_13_lifetimes.rs）；说明见 src/tutorial/README.md 第四阶段「13 生命周期」`

### lesson_14 · 闭包（通过 9/9，得分 100 · 优秀）

- `kp_14_01` **闭包基础：竖线参数、可省花括号、闭包能捕获定义处的变量**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:53`
  - 作者复习指引：复习 lesson_14 示例 1（syntax_vs_function）：`let add = \|x: i32\| base + x;` 与普通函数的第一个区别就是——闭包体里的 `base` 来自定义处环境（这里按不可变借用捕获）。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_02` **Fn trait：只读捕获的闭包可被调用任意多次（泛型参数 + Fn bound）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:114`
  - 作者复习指引：复习 lesson_14 示例 5（fn_fnmut_fnonce_hierarchy）与示例 6（closure_as_parameter）：参数写成 `F: Fn() -> i32` 就是要求「可以被调用多次且不会改动捕获值」；在函数里循环 `times` 次调用 `f()` 并累加即可。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_03` **FnMut trait：可变借用捕获 + 函数参数用 mut 绑定，两次调用共享同一份状态**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:178`
  - 作者复习指引：复习 lesson_14 示例 3（capture_by_mutable_borrow）与示例 5 的 `call_three_times`：闭包体里**写入**了外部变量 → 按 `&mut` 捕获 → 该闭包只实现 FnMut/FnOnce；接收入参时要写 `mut f: F` 才能调用（调用 FnMut 需要 `&mut self`）。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_04` **move 捕获与 FnOnce：所有权搬进闭包，调用后由返回值交还**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:256`
  - 作者复习指引：复习 lesson_14 示例 4（capture_by_move）：`move` 把捕获变量的所有权搬进闭包；如果闭包体里用掉了那个值（例如把它当返回值返回），闭包就只能是 `FnOnce`，只允许调用一次。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_05` **闭包作为参数（impl Trait）：静态分发，写法最简**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:319`
  - 作者复习指引：复习 lesson_14 示例 6（closure_as_parameter）的 `apply_twice`：`fn apply(value: i32, f: impl Fn(i32) -> i32) -> i32 { f(value) }`，`impl Trait` 等价于一个匿名的泛型参数，编译期单态化、零额外开销。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_06` **返回闭包（impl Fn）：必须 move 把 n 搬进闭包，否则借用随函数结束失效**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:382`
  - 作者复习指引：复习 lesson_14 示例 7（returning_closure）的 `make_multiplier`：`fn make_multiplier(factor: i64) -> impl Fn(i64) -> i64 { move \|x\| x * factor }`；去掉 `move` 会报 `error[E0373]: closure may outlive the current function`。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_07` **Box<dyn Fn>：异构闭包必须装箱才能放进同一个 Vec（动态分发）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:447`
  - 作者复习指引：复习 lesson_14 示例 8（box_dyn_closure）：`let ops: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(\|x\| x + 1), Box::new(move \|x\| x + offset)];`——两个闭包是**不同的匿名类型**，只有 `dyn Fn` 能把它们统一成同一种元素类型。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_08` **任务队列：Vec<Box<dyn FnOnce() -> String>> 按顺序执行消费型任务**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:511`
  - 作者复习指引：复习 lesson_14 示例 9（scenario_task_queue）：把「行为」作为值存进集合、由调用方按序执行；与示例 8 的 `Box<dyn Fn>` 相比，这里的任务返回 `String` 并耗尽自身，所以 trait 对象要写`Box<dyn FnOnce() -> String>`，也只能调用一次。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`
- `kp_14_09` **常见错误诊断：闭包参数类型锁定 E0308 / 可变借用冲突 E0502 / move 后使用 E0382**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_14_closures.rs:572`
  - 作者复习指引：复习 lesson_14 示例 10（common_mistakes）：本课列的 5 个坑按注释顺序是——错误 1 闭包参数类型被推断锁定（E0308）、错误 2 闭包持有可变借用时又读同一变量（E0502）、错误 3 move 之后又使用原变量（E0382）；再往后还有 E0373 与 E0499。
  - 复习入口：`复习入口：cargo run --bin lesson_14_closures（src/tutorial/lesson_14_closures.rs）；说明见 src/tutorial/README.md 第五阶段「14 闭包」`

### lesson_15 · 迭代器（通过 9/9，得分 100 · 优秀）

- `kp_15_01` **for 循环与 Iterator 的关系：for 只是「反复调用 next()」的语法糖**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:122`
  - 作者复习指引：复习 lesson_15 示例 1（iterator_and_for）：`for n in &nums` 等价于 `(&nums).into_iter()` 加一个循环；本题先用最朴素的 `for` 写出求和，作为后面 `iter().sum()` 的对照。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_02` **iter / iter_mut / into_iter 的所有权差别：iter_mut() 产出 &mut T，可原地改写**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:189`
  - 作者复习指引：复习 lesson_15 示例 2（three_iteration_modes）：`iter()` 借用产出 `&T`（集合保留）、`iter_mut()` 可变借用产出 `&mut T`（原地改写）、`into_iter()` 消费集合产出 `T`（集合被移走）；本题用 `for n in values.iter_mut() { *n *= 2; }` 改写后，再统计和与个数。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_03` **适配器流水线：enumerate + filter + map（惰性组合，不打乱原始下标）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:260`
  - 作者复习指引：复习 lesson_15 示例 3（adapters）：`nums.iter().enumerate().filter(..).map(..).collect()`；注意本题 `enumerate` 放在 `filter` **之前**，所以保留下来的下标是原始下标（会跳号），不是过滤后的序号。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_04` **消费器：sum() / max() / count() / any()（只有消费器才驱动流水线）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:323`
  - 作者复习指引：复习 lesson_15 示例 4（consumers）：`sum::<i32>()` 求和、`max()` 返回 `Option<&T>`、`count()` 返回元素个数、`any(pred)` 返回是否**存在**满足条件的元素；本题返回 `(和, 最大值, 个数, 是否有负数)`。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_05` **fold 与 collect：fold 是自定义聚合，collect 是「换容器」**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:386`
  - 作者复习指引：复习 lesson_15 示例 5（fold_and_collect）：`fold(初始值, \|acc, x\| 新累积值)` 可以表达任意聚合（求和、连乘、拼 CSV……）；`collect()` 的目标类型由接收方决定，这里用 `map` 生成`n=<值>` 的字符串再收成 `Vec<String>`。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_06` **惰性与短路：适配器只搭流水线，消费器才按需驱动（take(3) 只加工 3 个元素）**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:456`
  - 作者复习指引：复习 lesson_15 示例 6（laziness_proof）：`map` 的闭包在 `collect()` 之前一次都不会执行；本题用 `std::cell::Cell<usize>` 计数，`(0..100).map(计数).take(3).collect()` 只会加工 3 个元素——这就是「惰性 + 按需驱动」的事实证据（不是 100 次，也不是 0 次）。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_07` **自定义迭代器：实现 Counter::new 与 Iterator::next，免费获得全部适配器与消费器**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:519`
  - 作者复习指引：复习 lesson_15 示例 8（custom_iterator）：自定义类型只要实现 `Iterator` 的 `next()`（外加关联类型 `Item`），就能直接用 `sum()` / `collect()` / `filter()` 等全套工具；本题的 `Counter` 产出 `1..=max`，耗尽后 `next()` 必须返回 `None`。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_08` **词频统计：split_whitespace + 统一小写 + 计数 + 确定性排序（次数降序、字典序升序）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:557`
  - 作者复习指引：复习 lesson_15 示例 9（scenario_word_freq）：`text.split_whitespace()` 切词、`HashMap` 计数（entry API 见 lesson_08）、收集成 `Vec` 后`sort_by(\|a, b\| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))` 让输出**确定**。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`
- `kp_15_09` **常见错误诊断：迭代期间修改 E0502 / into_iter 后使用 E0382 / collect 类型不明 E0282**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_15_iterators.rs:635`
  - 作者复习指引：复习 lesson_15 示例 10（common_mistakes）：本课列的 5 个坑按注释顺序是——错误 1 遍历时修改集合（E0502）、错误 2 `into_iter()` 消费后又使用原集合（E0382）、错误 3 `collect()` 目标类型不明（E0282）；再往后还有「只有适配器没有消费器」与`find` 返回引用的运算错误（E0369）。
  - 复习入口：`复习入口：cargo run --bin lesson_15_iterators（src/tutorial/lesson_15_iterators.rs）；说明见 src/tutorial/README.md 第五阶段「15 迭代器」`

### lesson_16 · 智能指针（通过 9/9，得分 100 · 优秀）

- `kp_16_01` **Box 与递归类型：用 Box 把「无限大小」变成固定大小，再递归处理它**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:106`
  - 作者复习指引：复习 lesson_16 示例 1（box_and_recursive_type）：`enum List { Cons(i32, Box<List>), Nil }` 靠 `Box`（固定 8 字节指针）打断「大小递归」；`len` / `sum` 两个方法都要写递归调用（`1 + tail.len()` / `*value + tail.sum()`），`Nil` 分支返回 0。不加 Box 会报 `error[E0072]: recursive type `List` has infinite size`。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_02` **Box<T>：唯一所有权 + 堆分配，`*b` 解引用取出里面的值**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:214`
  - 作者复习指引：复习 lesson_16 示例 2（box_basics）与示例 3（deref_coercion）：`Box::new(5)` 把 5 分配到堆上，变量本身只是一个固定大小的指针（在栈上）；`*b` 通过 `Deref` 取出堆上的值，所以 `&Box<String>` 还能自动强转成 `&str`——这就是 Deref 强制转换。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_03` **Rc 共享所有权：`Rc::clone` 只让计数 +1，不复制堆上的数据**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:286`
  - 作者复习指引：复习 lesson_16 示例 4（rc_shared_ownership）：`Rc::clone(&a)` 复制的只是一个「指向同一份堆数据的指针」，`Rc::strong_count` 因此从 1 变成 2；`drop` 掉一个所有者后计数回到 1；只有计数归零（最后一个所有者离开）时，堆上的数据才真正释放。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_04` **RefCell 内部可变性：把借用检查从编译期挪到运行期**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:352`
  - 作者复习指引：复习 lesson_16 示例 5（refcell_interior_mutability）：`let cell = RefCell::new(0);` 没写 `mut`，却可以写 `*cell.borrow_mut() += 5;`——`borrow_mut()` 在运行期动态登记「当前有一个可变借用」，守卫（`RefMut`）一 drop 就归还；编译期只看到「共享引用」，所以借用检查被推迟到了运行期。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_05` **RefCell 的运行期边界：可变借用没释放就再借用 → panic（不是编译错误）**｜类型：边界｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:411`
  - 作者复习指引：复习 lesson_16 示例 5 的注释与示例 9 的错误 2：`let first = cell.borrow(); let second = cell.borrow_mut();` 能编译通过，运行期却 panic：`already borrowed: BorrowMutError`；修正办法是缩小借用作用域、用完立刻 drop。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_06` **Rc<RefCell<T>>：Rc 管「几个所有者」，RefCell 管「能不能改」**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:480`
  - 作者复习指引：复习 lesson_16 示例 6（rc_refcell_combo）：两个 `Rc::clone` 指向同一份 `RefCell`，各自 `borrow_mut()` 改的都是**同一份**数据（不是各自的副本），所以两次 +1 得到 2；同时 `Rc::strong_count` 说明确实只有 2 个所有者。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_07` **Weak 打破循环引用：`Rc::downgrade` 不加强计数，`upgrade()` 拿临时强引用**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:545`
  - 作者复习指引：复习 lesson_16 示例 7（weak_breaks_cycle）：正向持有用 `Rc`，回指 / 环上一律用 `Weak`——`Rc::downgrade(&child)` 存进 `RefCell<Weak<Node>>`，强计数不增加；要读的时候 `upgrade()` 返回 `Option<Rc<Node>>`，对方已被释放时得到 `None`（不会悬垂）。如果两边都用 `Rc` 互指，计数永不归零 → 内存泄漏（示例 9 的错误 3）。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_08` **典型场景：`Rc<RefCell<HashMap>>` 做共享可变缓存，并统计命中次数**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:655`
  - 作者复习指引：复习 lesson_16 示例 8（scenario_shared_cache）：缓存的句柄用 `Rc::clone` 共享，内部用 `RefCell<HashMap<..>>` 允许修改；「命中」= 查到了已有键，「未命中」= 没查到、需要计算并写入。注意借用的作用域：先取值（语句结束即归还借用），再考虑要不要 `borrow_mut()` 插入，否则会撞上运行期借用冲突。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`
- `kp_16_09` **常见错误诊断：改 Rc 内部值 / Rc 跨线程 / 递归类型没加间接层，分别报什么错**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_16_smart_pointers.rs:726`
  - 作者复习指引：复习 lesson_16 示例 9（common_mistakes）与示例 1：示例 9 里**带编译错误编号**的三个坑依次是「想直接改 `Rc` 里的值」（E0596）、「把 `Rc` 送进线程」（E0277）、「递归类型忘记加 `Box`」（E0072）——示例 9 末尾那行 `println!` 直接把这三个编号连在一起打印；另外两个坑（RefCell 借用重叠、循环引用泄漏）都是运行期问题，没有编译错误编号。
  - 复习入口：`复习入口：cargo run --bin lesson_16_smart_pointers（src/tutorial/lesson_16_smart_pointers.rs）；说明见 src/tutorial/README.md 第五阶段「16 智能指针」`

### lesson_17 · 线程与通道（通过 9/9，得分 100 · 优秀）

- `kp_17_01` **thread::spawn + join：等子线程结束，并把它的返回值收回主线程**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:55`
  - 作者复习指引：复习 lesson_17 示例 1（spawn_and_join）：`thread::spawn(\|\| 42)` 返回 `JoinHandle<u32>`；`handle.join()` 阻塞到子线程结束，返回 `Result<u32, Box<dyn Any + Send>>`——子线程正常结束是 `Ok(值)`，子线程 panic 才是 `Err`（所以实现里要先 `expect(...)` 取出来）。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_02` **move 闭包与所有权转移：编译器用「独占」保证没有数据竞争**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:119`
  - 作者复习指引：复习 lesson_17 示例 2（move_ownership）：`thread::spawn(move \|\| data.len())` 把 `data` 的所有权整体搬进子线程——主线程再也碰不到它，所以不可能出现两个线程同时读写同一份数据；不写 `move` 而去借用栈上变量会报 `error[E0373]: closure may outlive the current function`。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_03` **mpsc 通道：一个生产者线程发送，主线程收集求和**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:183`
  - 作者复习指引：复习 lesson_17 示例 3（mpsc_single_producer）：`mpsc::channel()` 拿到 `(tx, rx)`；生产者把值逐个 `send`，闭包结束时 `tx` 被 drop、通道关闭，接收端的 `for value in rx` 才会结束；主线程把收到的值累加求和。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_04` **多生产者单消费者：用 `tx.clone()` 把发送端分给多个线程**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:249`
  - 作者复习指引：复习 lesson_17 示例 4（mpsc_multi_producer）：每个生产者线程拿一个 `tx.clone()`，各发自己那批消息；主线程必须 `drop(tx)`（丢掉自己手里那份发送端），否则接收端等不到「所有发送端关闭」，`for` 会一直阻塞；各消息到达顺序不确定，所以先收集再统计（求和与顺序无关）。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_05` **Arc<Mutex<T>>：Arc 管「多线程共享」，Mutex 管「同一时刻只有一个能改」**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:311`
  - 作者复习指引：复习 lesson_17 示例 5（arc_mutex_shared_state）：每个线程 `Arc::clone` 一份句柄（原子计数），`counter.lock().unwrap()` 拿到互斥守卫后才能改；守卫离开作用域自动解锁（RAII），临界区越小越好。**所有线程 `join()` 之后**主线程再读最终值，结果才是确定的。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_06` **Send / Sync：`Arc<Mutex<i32>>` 两者都满足（换成 `Rc` 根本编译不过）**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:374`
  - 作者复习指引：复习 lesson_17 示例 7（send_and_sync）与示例 9 的错误 3：`Send` = 值可以**转移**到别的线程，`Sync` = `&T` 可以被多个线程同时引用；`Rc` 的计数只是普通加减，所以既不是 `Send` 也不是 `Sync`，把 `Rc` 送进线程会报 `error[E0277]: `Rc<i32>` cannot be sent between threads safely`。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_07` **thread::scope：子线程借用局部数据，不需要 move 所有权**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:444`
  - 作者复习指引：复习 lesson_17 示例 6（scoped_threads）：`thread::scope(\|scope\| { ... })` 保证块结束时所有子线程都已被 join，所以子线程可以安全借用**没有 move 的**局部变量；每个线程算自己那一块的 `chunk.iter().sum()`，最后把各部分和相加。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_08` **并行求和：按 chunks 分块、并发执行、收集合并（切分 → 并发 → 合并）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:523`
  - 作者复习指引：复习 lesson_17 示例 8（scenario_parallel_sum）：`chunk_size = (len + workers - 1) / workers` （或 `len.div_ceil(workers)`）向上取整，保证切出来的块数不超过 worker 数；每块用 `thread::scope` 借用计算部分和，最后把部分和相加。边界：空数据直接返回 0；`chunks = 0` 按 1 处理（否则除零 / `chunks(0)` panic）。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`
- `kp_17_09` **常见错误诊断：忘记 join / 闭包缺 move / Rc 跨线程，各是什么性质的问题**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_17_threads_channels.rs:610`
  - 作者复习指引：复习 lesson_17 示例 9（common_mistakes）与示例 7：按注释顺序的前三条错误里，第 1 条「忘记 join」是**运行期**现象（编译能过，主线程提前退出，没有编译错误编号），第 2 条「闭包缺 move」报 `error[E0373]`，第 3 条「Rc 跨线程」报 `error[E0277]`；再往后的锁中毒、死锁同样是运行期问题。
  - 复习入口：`复习入口：cargo run --bin lesson_17_threads_channels（src/tutorial/lesson_17_threads_channels.rs）；说明见 src/tutorial/README.md 第五阶段「17 线程与通道」`

### lesson_18 · 声明宏（通过 8/8，得分 100 · 优秀）

- `kp_18_01` **可变参数宏：macro_rules! 用重复模式 $(...)* 接受任意个参数（含 0 个）**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:53`
  - 作者复习指引：复习 lesson_18 示例 1（demo_1_macro_vs_function）：宏按「调用形状」选择匹配臂展开成代码，因此参数个数可以变；函数签名固定，做不到这一点。零参数时 `$(...)*` 一次都不展开，展开结果就只剩那个初值 `0`。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_02` **片段说明符：$i:ident 要名字、$e:expr 要表达式、$t:ty 要类型**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:137`
  - 作者复习指引：复习 lesson_18 示例 2（demo_2_fragment_specifiers）：常用说明符有 expr / ident / literal / ty / pat / block / tt；说明符决定了「这个位置允许填什么」，填错形状就会报 `expected identifier` 之类的错误。stringify! 能把实参的源码文本转成字符串。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_03` **重复模式：$(...),+ 起「逗号分隔」的展开，$(,)? 吃掉可选尾逗号**｜类型：核心｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:219`
  - 作者复习指引：复习 lesson_18 示例 3（demo_3_repetition）：`$( $x:expr ),+` 表示逗号分隔、至少一个；`$(,)?` 表示「可以有、也可以没有的尾随逗号」，与函数调用风格保持一致。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_04` **stringify! 把源码文本变成字符串；concat! 在编译期拼接字面量**｜类型：基础｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:331`
  - 作者复习指引：复习 lesson_18 示例 4（demo_4_stringify_and_concat）：`stringify!(1 + 2)` 得到的是源码文本 `"1 + 2"`（不是 3）；`concat!` 只接受字面量，在编译期就拼成一个 `&'static str`。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_05` **提前返回宏 ensure!：展开成 if !cond { return Err(...) }，校验代码收敛成一行**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:422`
  - 作者复习指引：复习 lesson_18 示例 5（demo_5_ensure_early_return）：ensure! 展开后就是一行`if !($cond) { return Err(String::from($msg)); }`，所以它**只能**用在能返回 Result 的函数里；宏不改变错误链路，Result 依然是 lesson_10 的那一套。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_06` **宏写小型 DSL：`"备份" => 10` 这种箭头语法只有宏能表达，展开成 Vec 后顺序与书写一致**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:496`
  - 作者复习指引：复习 lesson_18 示例 6（demo_6_scenario_schedule_dsl）：课程里的 schedule! 展开成 BTreeMap（按键排序）；本考核要求展开成 `Vec<(String, u32)>`，因此**顺序就是书写顺序**。宏 DSL 的边界：只在「消除重复、提升表达力」时使用。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_07` **宏卫生性（hygiene）：宏内部定义的名字不会污染调用方的命名空间**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:612`
  - 作者复习指引：复习 lesson_18 示例 7 的错误 5（卫生性）：宏展开里的 `hidden` 与调用处的同名变量是「不同的 hidden」；在宏外访问它报 `error[E0425]: cannot find value`。这是刻意设计——宏不应该悄悄污染调用方的命名空间。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`
- `kp_18_08` **宏的典型报错：先用后定义 / 形状不匹配 / 分隔符不符（取自课程示例 7 的注释原文）**｜类型：难点｜状态：**通过**
  - 位置：`assessments\lesson_18_macros.rs:694`
  - 作者复习指引：复习 lesson_18 示例 7（demo_7_common_mistakes）：本课列的 5 个坑里，前三个分别是「宏先用后定义」「实参形状与匹配臂对不上」「重复里的分隔符用错」；这三个错误在课程注释里给的是 `error:` 原文（没有 E 编号），只有第 5 个卫生性例子带 E0425。
  - 复习入口：`复习入口：cargo run --bin lesson_18_macros（src/tutorial/lesson_18_macros.rs）；说明见 src/tutorial/README.md 第五阶段「18 声明宏」`

## 五、需要加强的领域（Top 5）

本次运行没有「未通过 / 未实现 / 运行期错误」的知识点，保持这个节奏即可。

## 六、学习进度对比

这是本工具记录的**首次记录**：`history.jsonl` 里还没有更早的运行，暂时无可对比的历史。继续完成练习后再次运行本命令，就能看到分数变化与各模块进步。

本次基线：100 分，通过 157 个知识点。


> 对比口径：`history.jsonl` 记录的是每次运行的**模块级聚合**（模块通过数），因此这里给出总分变化与各模块通过数增减；单条知识点的增减见上表明细。历史文件：`target\solution-verify\history.jsonl`（本次已追加）。

## 七、下一步行动清单

1. 本课已全部通过：进入下一课，并在学完后运行 `cargo run --bin assessment_report -- --emit-map` 更新知识点矩阵。
2. 每次练习后重新运行 `cargo run --bin assessment_report`，用「学习进度对比」确认没有退步。

