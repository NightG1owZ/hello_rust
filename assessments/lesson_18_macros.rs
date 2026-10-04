//! assessments/lesson_18_macros.rs —— 考核：声明宏（对应 lesson_18）
//!
//! - 对应课程：`src/tutorial/lesson_18_macros.rs`
//! - 知识点出处：`src/tutorial/README.md` 第五阶段「18 声明宏」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_18_macros     # 只考这一课
//!   cargo test                             # 考全部 18 课
//!   cargo run --bin assessment_report      # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_18_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_18_macros`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - `macro_rules!` 是**文本作用域**：必须先定义、后使用。本文件要求把宏定义写在
//!   `exercise_*` 函数体内（不能提到文件顶层——那样骨架态会报「宏从未被使用」）；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 7）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | 先 `late!()` 后 `macro_rules! late` | `error: cannot find macro \`late\` in this scope` | 把宏定义移到首次使用之前（宏是文本作用域） |
//! | `max_of!(1)`（只填一个表达式） | `error: no rules expected the token \`)\`` | 补一条匹配臂，或按匹配臂的形状传参 |
//! | `sum_all!(1; 2)`（分隔符写成分号） | `error: expected \`,\` or \`)\`` | 按匹配臂声明的分隔符传参：`sum_all!(1, 2)` |
//! | `bad!(1 + 2)` 而匹配臂写的是 `$name:ident` | `error: expected identifier, found \`1\`` | 要表达式用 `$x:expr`，要名字才用 `$x:ident` |
//! | 在宏外访问宏内部 `let hidden = 42;` | `error[E0425]: cannot find value \`hidden\` in this scope` | 宏是卫生的：让宏把值作为表达式结果返回 |

use assessment_harness::{Kind, assess, eq, eq_slice, err, ok};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_18";

// ===========================================================================
// kp_18_01 可变参数宏：一个匹配臂吃下任意个参数
// ===========================================================================

/// 知识点考核：用重复模式写可变参数宏，并覆盖「零参数」这一条边界。
#[test]
fn kp_18_01_macro_sum() {
    assess(
        M,
        "kp_18_01",
        "可变参数宏：macro_rules! 用重复模式 $(...)* 接受任意个参数（含 0 个）",
        Kind::Basic,
        "复习 lesson_18 示例 1（demo_1_macro_vs_function）：宏按「调用形状」选择匹配臂展开成代码，\
         因此参数个数可以变；函数签名固定，做不到这一点。零参数时 `$(...)*` 一次都不展开，\
         展开结果就只剩那个初值 `0`。",
        || {
            // 正常用例：四个参数逐个展开成 `0 + 1 + 2 + 3 + 4`
            eq(
                exercise_18_01_macro_sum(),
                10,
                "sum_all!(1, 2, 3, 4) 展开成 0 + 1 + 2 + 3 + 4，结果为 10",
            );
            // 边界用例：零参数必须也得 0（`*` 允许零次重复；若写成 `+` 就编译不过）
            eq(
                exercise_18_01_macro_sum_empty(),
                0,
                "sum_all!() 零参数时重复体一次都不展开，应得到初值 0（用 `*` 而不是 `+`）",
            );
        },
    );
}

/// 【待实现】在函数体内定义可变参数宏 `sum_all!`，并用它求和。
///
/// 实现要求：
///   - 在**函数体内**写：
///     `macro_rules! sum_all { ($($x:expr),*) => { 0 $(+ $x)* } }`
///     （`$(...),*` 表示「逗号分隔、零次或多次」，所以零参数也合法）；
///   - 返回 `sum_all!(1, 2, 3, 4)`，即 `10`；
///   - 宏定义必须写在函数体内、调用之前（`macro_rules!` 是文本作用域：先定义后使用）；
///   - 零参数分支不需要额外写匹配臂：`$(...)*` 一次都不展开时宏体里只剩初值 `0`，
///     天然覆盖 `sum_all!()`；若把 `*` 写成 `+`（至少一次），
///     `sum_all!()` 会报 `error: no rules expected the token \`)\``。
///
/// 示例输入：
/// ```text
/// sum_all!(1, 2, 3, 4)
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_18_01_macro_sum() -> i32 {
    assessment_harness::todo_exercise(
        "exercise_18_01_macro_sum",
        "在函数内定义 macro_rules! sum_all（$(...),* 重复模式），返回 sum_all!(1, 2, 3, 4)",
        (),
    )
}

/// 【待实现】同一个 `sum_all!` 的零参数调用（边界用例的独立练习函数）。
///
/// 实现要求：
///   - 再定义一次 `macro_rules! sum_all`（宏是文本作用域，每个函数各定义一份即可）；
///   - 返回 `sum_all!()`；因为重复体零次展开，结果就是初值 `0`；
///   - 重复模式必须用 `*`（零次或多次）而不是 `+`，才能覆盖「零个参数」这种调用形状；
///     宏调用里的空括号 `sum_all!()` 是完全合法的调用。
///
/// 示例输入：
/// ```text
/// sum_all!()
/// ```
/// 示例输出：
/// ```text
/// 0
/// ```
fn exercise_18_01_macro_sum_empty() -> i32 {
    assessment_harness::todo_exercise(
        "exercise_18_01_macro_sum_empty",
        "定义同样的 sum_all 宏，返回 sum_all!()（零参数应为 0）",
        (),
    )
}

// ===========================================================================
// kp_18_02 片段说明符：$i:ident / $e:expr / $t:ty
// ===========================================================================

/// 知识点考核：一条匹配臂同时使用 ident、expr、ty 三种片段说明符。
#[test]
fn kp_18_02_fragment_specifiers() {
    assess(
        M,
        "kp_18_02",
        "片段说明符：$i:ident 要名字、$e:expr 要表达式、$t:ty 要类型",
        Kind::Core,
        "复习 lesson_18 示例 2（demo_2_fragment_specifiers）：常用说明符有 expr / ident / \
         literal / ty / pat / block / tt；说明符决定了「这个位置允许填什么」，\
         填错形状就会报 `expected identifier` 之类的错误。stringify! 能把实参的源码文本转成字符串。",
        || {
            // 正常用例：一条匹配臂同时吃到 名字 / 类型 / 表达式
            eq(
                exercise_18_02_fragment_specifiers(),
                String::from("count: u32 = 7"),
                "describe!(count: u32 = 7) 用 stringify! 还原源码文本，应得到 \"count: u32 = 7\"",
            );
            // 边界用例：`$i:ident` 允许任何合法标识符（哪怕带下划线或很长）
            eq(
                exercise_18_02_fragment_specifiers_edge(),
                String::from("_tmp: f64 = 1.5"),
                "ident 只要「是名字」即可：_tmp 也合法；expr / ty 原样 stringify 出来",
            );
        },
    );
}

/// 【待实现】定义同时使用三种片段说明符的宏 `describe!`。
///
/// 实现要求：
///   - 在函数体内写：
///     `macro_rules! describe { ($i:ident : $t:ty = $e:expr) => { format!("{}: {} = {}", stringify!($i), stringify!($t), $e) } }`
///   - 返回 `describe!(count: u32 = 7)`，即 `"count: u32 = 7"`；
///   - 三个占位符分别用 `stringify!` 转成源码文本，`$e:expr` 则直接求值参与 `format!`；
///   - `stringify!($i)` 得到的是 `"count"`（去掉空白后的源码文本），`stringify!($t)` 得到的是
///     `"u32"`；说明符必须与实参形状对应，写成 `$e:ty` 或 `$i:expr` 就会类型不匹配。
///
/// 示例输入：
/// ```text
/// describe!(count: u32 = 7)
/// ```
/// 示例输出：
/// ```text
/// "count: u32 = 7"
/// ```
fn exercise_18_02_fragment_specifiers() -> String {
    assessment_harness::todo_exercise(
        "exercise_18_02_fragment_specifiers",
        "定义 describe!（$i:ident / $t:ty / $e:expr + stringify!），返回 describe!(count: u32 = 7)",
        (),
    )
}

/// 【待实现】用同一个 `describe!` 覆盖标识符与类型的边界形状。
///
/// 实现要求：
///   - 同样在当前函数体内定义 `macro_rules! describe`；
///   - 返回 `describe!(_tmp: f64 = 1.5)`，即 `"_tmp: f64 = 1.5"`；
///   - `_tmp` 也是合法的 `ident`（下划线开头不会被 `stringify!` 改写）；
///     `1.5` 是浮点字面量，属于合法的 `expr`，格式化后就是 `1.5`。
///
/// 示例输入：
/// ```text
/// describe!(_tmp: f64 = 1.5)
/// ```
/// 示例输出：
/// ```text
/// "_tmp: f64 = 1.5"
/// ```
fn exercise_18_02_fragment_specifiers_edge() -> String {
    assessment_harness::todo_exercise(
        "exercise_18_02_fragment_specifiers_edge",
        "定义 describe! 宏，返回 describe!(_tmp: f64 = 1.5)",
        (),
    )
}

// ===========================================================================
// kp_18_03 重复模式：$(...),+ 与可选尾逗号 $(,)?
// ===========================================================================

/// 知识点考核：用重复模式生成 `Vec`，并覆盖空调用与尾随逗号两个边界。
#[test]
fn kp_18_03_repetition() {
    assess(
        M,
        "kp_18_03",
        "重复模式：$(...),+ 起「逗号分隔」的展开，$(,)? 吃掉可选尾逗号",
        Kind::Core,
        "复习 lesson_18 示例 3（demo_3_repetition）：`$( $x:expr ),+` 表示逗号分隔、至少一个；\
         `$(,)?` 表示「可以有、也可以没有的尾随逗号」，与函数调用风格保持一致。",
        || {
            // 正常用例：三个元素按书写顺序进入 Vec
            eq_slice(
                &exercise_18_03_repetition(),
                &vec![1i32, 2, 3],
                "make_vec!(1, 2, 3) 应按书写顺序展开成 [1, 2, 3]",
            );
            // 边界用例：尾随逗号必须被 `$(,)?` 吃掉，不能多出一个元素
            eq_slice(
                &exercise_18_03_repetition_trailing_comma(),
                &vec![10i32],
                "make_vec!(10,) 的尾逗号由 $(,)? 匹配，结果只有 1 个元素 [10]",
            );
            // 边界用例：`*` 允许零个元素，空调用得到空 Vec
            eq_slice(
                &exercise_18_03_repetition_empty(),
                &Vec::<i32>::new(),
                "make_vec![] 是零次重复，应得到空 Vec（类型需显式标注）",
            );
        },
    );
}

/// 【待实现】定义带重复模式的宏 `make_vec!` 并生成 `Vec`。
///
/// 实现要求：
///   - 在函数体内写：
///     `macro_rules! make_vec { ($($x:expr),* $(,)?) => { vec![$($x),*] } }`
///     （`$(,)?` 专门匹配「有或没有」的尾随逗号）；
///   - 返回 `make_vec!(1, 2, 3)`，即 `vec![1, 2, 3]`；
///   - 宏体里可以直接调用标准库的 `vec![]` 宏（宏之间可以嵌套展开）；若把 `$(,)?` 删掉，
///     `make_vec!(1, 2, 3,)` 会报 `error: no rules expected the token \`,\``。
///
/// 示例输入：
/// ```text
/// make_vec!(1, 2, 3)
/// ```
/// 示例输出：
/// ```text
/// [1, 2, 3]
/// ```
fn exercise_18_03_repetition() -> Vec<i32> {
    assessment_harness::todo_exercise(
        "exercise_18_03_repetition",
        "定义 make_vec!（$(...),* $(,)? 重复模式），返回 make_vec!(1, 2, 3)",
        (),
    )
}

/// 【待实现】同一个宏的尾随逗号调用（边界用例）。
///
/// 实现要求：
///   - 定义 `make_vec!`，匹配臂写成 `($($x:expr),* $(,)?)`；
///   - 返回 `make_vec!(10,)`，即 `vec![10]`（只有 1 个元素）；
///   - `$(,)?` 跟在重复之后，表示「再多一个可选的逗号」；没有它，
///     尾随逗号会让宏找不到能匹配的规则。
///
/// 示例输入：
/// ```text
/// make_vec!(10,)
/// ```
/// 示例输出：
/// ```text
/// [10]
/// ```
fn exercise_18_03_repetition_trailing_comma() -> Vec<i32> {
    assessment_harness::todo_exercise(
        "exercise_18_03_repetition_trailing_comma",
        "定义带 $(,)? 的 make_vec!，返回 make_vec!(10,)（应等于 [10]）",
        (),
    )
}

/// 【待实现】同一个宏的空调用（边界用例）。
///
/// 实现要求：
///   - 定义 `make_vec!`，用 `*`（零次或多次）而不是 `+`；
///   - 返回 `make_vec![]`；因为一个元素都没有，必须显式标注返回类型
///     （`Vec<i32>`，由函数签名决定）；
///   - 方括号 `[...]` 与圆括号 `(...)` 都是合法的宏调用定界符，匹配臂里对应写成
///     `[...]` 即可；空调用时 `vec![]` 需要上下文提供类型信息。
///
/// 示例输入：
/// ```text
/// make_vec![]
/// ```
/// 示例输出：
/// ```text
/// []
/// ```
fn exercise_18_03_repetition_empty() -> Vec<i32> {
    assessment_harness::todo_exercise(
        "exercise_18_03_repetition_empty",
        "定义 make_vec!（用 `*` 允许零个元素），返回 make_vec![]（空 Vec）",
        (),
    )
}

// ===========================================================================
// kp_18_04 内建宏 stringify! 与 concat!：把代码文本带进字符串
// ===========================================================================

/// 知识点考核：把表达式源码变字符串、把字面量在编译期拼起来。
#[test]
fn kp_18_04_stringify_concat() {
    assess(
        M,
        "kp_18_04",
        "stringify! 把源码文本变成字符串；concat! 在编译期拼接字面量",
        Kind::Basic,
        "复习 lesson_18 示例 4（demo_4_stringify_and_concat）：`stringify!(1 + 2)` 得到的是\
         源码文本 `\"1 + 2\"`（不是 3）；`concat!` 只接受字面量，在编译期就拼成一个 `&'static str`。",
        || {
            // 正常用例：表达式源码文本 + 字面量拼接
            let (source, joined) = exercise_18_04_stringify_concat();
            eq(
                source,
                String::from("1 + 2"),
                "stringify!(1 + 2) 保留源码文本（含空格）：\"1 + 2\"，不是计算结果 3",
            );
            eq(
                joined,
                "a-b",
                "concat!(\"a\", \"-\", \"b\") 在编译期拼成 \"a-b\"",
            );
            // 边界用例：单个字面量也能 concat（拼接零次=原样），以及带括号的表达式源码
            let (source_edge, joined_edge) = exercise_18_04_stringify_concat_edge();
            eq(
                source_edge,
                String::from("(1 + 2) * 3"),
                "stringify! 原样保留括号与运算符：\"(1 + 2) * 3\"",
            );
            eq(
                joined_edge,
                "rust",
                "concat!(\"rust\") 只有一个字面量时结果就是它本身",
            );
        },
    );
}

/// 【待实现】用 `stringify!` 与 `concat!` 生成两段文本。
///
/// 实现要求（全部在函数体内完成）：
///   - 第一个分量：`stringify!(1 + 2).to_string()`，即 `"1 + 2"`；
///   - 第二个分量：`concat!("a", "-", "b")`（类型就是 `&'static str`），即 `"a-b"`；
///   - 返回 `(String, &'static str)`；
///   - `stringify!` 的结果是 `&'static str`，要变成 `String` 就加 `.to_string()`；
///     `concat!` 的实参**必须**是字面量，写 `concat!(x, y)`（变量）会编译报错。
///
/// 示例输入：
/// ```text
/// stringify!(1 + 2) 与 concat!("a", "-", "b")
/// ```
/// 示例输出：
/// ```text
/// ("1 + 2", "a-b")
/// ```
fn exercise_18_04_stringify_concat() -> (String, &'static str) {
    assessment_harness::todo_exercise(
        "exercise_18_04_stringify_concat",
        "返回 (stringify!(1 + 2).to_string(), concat!(\"a\", \"-\", \"b\"))",
        (),
    )
}

/// 【待实现】`stringify!` / `concat!` 的边界形状。
///
/// 实现要求：
///   - 第一个分量：`stringify!((1 + 2) * 3).to_string()`，即 `"(1 + 2) * 3"`；
///   - 第二个分量：`concat!("rust")`，即 `"rust"`（只有一个字面量）；
///   - `stringify!` 不会替你加括号、也不会化简表达式，源码怎么写就怎么转。
///
/// 示例输入：
/// ```text
/// stringify!((1 + 2) * 3) 与 concat!("rust")
/// ```
/// 示例输出：
/// ```text
/// ("(1 + 2) * 3", "rust")
/// ```
fn exercise_18_04_stringify_concat_edge() -> (String, &'static str) {
    assessment_harness::todo_exercise(
        "exercise_18_04_stringify_concat_edge",
        "返回 (stringify!((1 + 2) * 3).to_string(), concat!(\"rust\"))",
        (),
    )
}

// ===========================================================================
// kp_18_05 ensure! 提前返回宏：宏里也能 return
// ===========================================================================

/// 知识点考核：用自定义 `ensure!` 做参数校验并提前返回 `Err`。
#[test]
fn kp_18_05_ensure() {
    assess(
        M,
        "kp_18_05",
        "提前返回宏 ensure!：展开成 if !cond { return Err(...) }，校验代码收敛成一行",
        Kind::Hard,
        "复习 lesson_18 示例 5（demo_5_ensure_early_return）：ensure! 展开后就是一行\
         `if !($cond) { return Err(String::from($msg)); }`，所以它**只能**用在能返回 Result 的函数里；\
         宏不改变错误链路，Result 依然是 lesson_10 的那一套。",
        || {
            // 正常用例：正数校验通过，返回 Ok(value * 2)
            eq(
                ok(exercise_18_05_ensure(1), "1 > 0 应当通过校验并返回 Ok"),
                2,
                "value = 1 通过校验后返回 Ok(1 * 2) = Ok(2)",
            );
            // 边界用例：0 不算正数，必须提前返回 Err
            let at_zero = err(
                exercise_18_05_ensure(0),
                "0 不满足 value > 0，应当提前返回 Err",
            );
            eq(
                at_zero.contains("正数"),
                true,
                "0 的错误信息里应含「正数」这个关键字（ensure! 传入的那句提示语）",
            );
            // 边界用例：负数同样被拦下
            let negative = err(
                exercise_18_05_ensure(-7),
                "-7 不满足 value > 0，应当提前返回 Err",
            );
            eq(
                negative.contains("正数"),
                true,
                "-7 的错误信息同样应含「正数」，说明走的是同一条 ensure! 分支",
            );
        },
    );
}

/// 【待实现】用 `ensure!` 校验参数，通过则返回 `Ok(value * 2)`。
///
/// 实现要求：
///   - 在函数体内定义：
///     `macro_rules! ensure { ($cond:expr, $msg:expr) => { if !$cond { return Err($msg.to_string()); } } }`
///     （注意匹配臂里的 `!$cond`，直接写 `!$cond` 就能兼容调用处传来的任意表达式）；
///   - 写 `ensure!(value > 0, "value 必须是正数");`
///   - 校验通过后返回 `Ok(value * 2)`；
///   - 返回值类型必须是 `Result<i32, String>`，否则 `return Err(...)` 无法通过编译；
///   - `return` 的**类型**由所在函数的签名决定，宏只是把这段 `return` 展开到调用处：
///     用在返回 `()` 的函数里会报 `error[E0308]: mismatched types`。
///
/// 示例输入：
/// ```text
/// value = 1
/// ```
/// 示例输出：
/// ```text
/// Ok(2)
/// ```
fn exercise_18_05_ensure(value: i32) -> Result<i32, String> {
    assessment_harness::todo_exercise(
        "exercise_18_05_ensure",
        "定义 ensure! 宏校验 value > 0（否则 Err 且含「正数」），通过后返回 Ok(value * 2)",
        (value,),
    )
}

// ===========================================================================
// kp_18_06 小型 DSL：key => value 展开成有序的 Vec
// ===========================================================================

/// 知识点考核：用宏定义自己的语法形状 `"名字" => 数值`，展开成有序数据。
#[test]
fn kp_18_06_schedule_dsl() {
    assess(
        M,
        "kp_18_06",
        "宏写小型 DSL：`\"备份\" => 10` 这种箭头语法只有宏能表达，展开成 Vec 后顺序与书写一致",
        Kind::Hard,
        "复习 lesson_18 示例 6（demo_6_scenario_schedule_dsl）：课程里的 schedule! 展开成 BTreeMap\
         （按键排序）；本考核要求展开成 `Vec<(String, u32)>`，因此**顺序就是书写顺序**。\
         宏 DSL 的边界：只在「消除重复、提升表达力」时使用。",
        || {
            // 正常用例：三项按书写顺序进入 Vec
            eq_slice(
                &exercise_18_06_schedule_dsl(),
                &vec![
                    (String::from("备份"), 10u32),
                    (String::from("巡检"), 20),
                    (String::from("周报"), 30),
                ],
                "schedule!{...} 展开成 Vec，顺序必须与书写顺序完全一致（不是排序后的）",
            );
            // 边界用例：只有一个条目，且带尾随逗号
            eq_slice(
                &exercise_18_06_schedule_dsl_single(),
                &vec![(String::from("单条"), 1u32)],
                "单条 + 尾随逗号也应展开成只含 1 个元素的 Vec",
            );
            // 边界用例：零条目 → 空 Vec（`*` 允许零次重复）
            eq_slice(
                &exercise_18_06_schedule_dsl_empty(),
                &Vec::<(String, u32)>::new(),
                "空调用应得到空 Vec，不要 panic",
            );
        },
    );
}

/// 【待实现】定义小型 DSL 宏 `schedule!`，展开成 `Vec<(String, u32)>`。
///
/// 实现要求：
///   - 在函数体内写：
///     `macro_rules! schedule { ($($name:expr => $hours:expr),* $(,)?) => { vec![$(($name.to_string(), $hours)),*] } }`
///   - 返回 `schedule! { "备份" => 10, "巡检" => 20, "周报" => 30 }`；
///   - 顺序必须与书写顺序一致（`vec!` 保序；不要用 `BTreeMap` 之类会重排的容器）；
///   - 宏可以自定义成任何「记号序列」，`=>` 只是普通的两个 token，所以 `"备份" => 10`
///     这种函数参数写不出来的形状，宏完全可以匹配；元素类型 `(String, u32)` 已在函数签名里
///     定死，`vec![]` 的类型能自动推断。
///
/// 示例输入：
/// ```text
/// schedule! { "备份" => 10, "巡检" => 20, "周报" => 30 }
/// ```
/// 示例输出：
/// ```text
/// [("备份", 10), ("巡检", 20), ("周报", 30)]
/// ```
fn exercise_18_06_schedule_dsl() -> Vec<(String, u32)> {
    assessment_harness::todo_exercise(
        "exercise_18_06_schedule_dsl",
        "定义 schedule! 宏（$name:expr => $hours:expr 重复），返回含三项、顺序一致的 Vec",
        (),
    )
}

/// 【待实现】`schedule!` 的单条 + 尾随逗号调用（边界用例）。
///
/// 实现要求：
///   - 同样定义 `schedule!`，匹配臂里带 `$(,)?`；
///   - 返回 `schedule! { "单条" => 1, }`，即 `vec![(String::from("单条"), 1u32)]`；
///   - 只写一个条目时也保留尾随逗号，前提是匹配臂里有 `$(,)?`。
///
/// 示例输入：
/// ```text
/// schedule! { "单条" => 1, }
/// ```
/// 示例输出：
/// ```text
/// [("单条", 1)]
/// ```
fn exercise_18_06_schedule_dsl_single() -> Vec<(String, u32)> {
    assessment_harness::todo_exercise(
        "exercise_18_06_schedule_dsl_single",
        "用同一个 schedule! 宏返回 schedule!{ \"单条\" => 1, }（1 个元素）",
        (),
    )
}

/// 【待实现】`schedule!` 的空调用（边界用例）。
///
/// 实现要求：
///   - 定义 `schedule!`，重复用 `*` 而不是 `+`；
///   - 返回 `schedule! {}`（一个条目都不写）；结果必须是空的 `Vec<(String, u32)>`；
///   - 空调用时 `vec![]` 没有任何元素可以推断类型，必须靠函数返回类型
///     `Vec<(String, u32)>` 兜住。
///
/// 示例输入：
/// ```text
/// schedule! {}
/// ```
/// 示例输出：
/// ```text
/// []
/// ```
fn exercise_18_06_schedule_dsl_empty() -> Vec<(String, u32)> {
    assessment_harness::todo_exercise(
        "exercise_18_06_schedule_dsl_empty",
        "同一个 schedule! 宏返回 schedule! {}（空 Vec，不 panic）",
        (),
    )
}

// ===========================================================================
// kp_18_07 卫生性：宏内部的名字不会泄漏到调用处
// ===========================================================================

/// 知识点考核：宏内部的 `let x` 与调用处的 `x` 是两个不同的绑定。
#[test]
fn kp_18_07_hygiene() {
    assess(
        M,
        "kp_18_07",
        "宏卫生性（hygiene）：宏内部定义的名字不会污染调用方的命名空间",
        Kind::Hard,
        "复习 lesson_18 示例 7 的错误 5（卫生性）：宏展开里的 `hidden` 与调用处的同名变量是\
         「不同的 hidden」；在宏外访问它报 `error[E0425]: cannot find value`。\
         这是刻意设计——宏不应该悄悄污染调用方的命名空间。",
        || {
            // 正常用例：宏内部 x=10 参与运算，得到 1 + 10 = 11
            eq(
                exercise_18_07_hygiene(),
                (11, 1),
                "add_inner_x!(1) 用宏内部的 x=10 算出 11；宏外的 x 仍然是 1",
            );
            // 边界用例：入参为 0 时，结果恰好等于宏内部那个 10，说明两者互不干扰
            eq(
                exercise_18_07_hygiene_edge(),
                (10, 1),
                "入参 0 时结果为 10（宏内部 x），外部 x 依然独立地是 1",
            );
        },
    );
}

/// 【待实现】验证宏的卫生性：宏内部与外部可以同名而不互相干扰。
///
/// 实现要求：
///   - 在函数体内定义：
///     `macro_rules! add_inner_x { ($value:expr) => {{ let x = 10; $value + x }} }`
///     （宏体里的 `let x = 10;` 属于**宏自己的**文本作用域）；
///   - 在函数里写 `let x = 1;`（外部绑定，与宏内部那个 `x` 同名但无关）；
///   - 返回 `(add_inner_x!(1), x)`，即 `(11, 1)`；
///   - `macro_rules!` 的卫生性让宏内部的局部变量对调用处不可见，因此内外同名也不会互相干扰。
///
/// 示例输入：
/// ```text
/// (add_inner_x!(1), x)
/// ```
/// 示例输出：
/// ```text
/// (11, 1)
/// ```
fn exercise_18_07_hygiene() -> (i32, i32) {
    assessment_harness::todo_exercise(
        "exercise_18_07_hygiene",
        "宏内部 let x = 10、外部 let x = 1，返回 (add_inner_x!(1), x) = (11, 1)",
        (),
    )
}

/// 【待实现】卫生性的边界：入参为 0 时结果等于宏内部的值。
///
/// 实现要求：
///   - 定义同样的 `add_inner_x!` 宏（内部 `let x = 10;`）；
///   - 外部写 `let x = 1;`；
///   - 返回 `(add_inner_x!(0), x)`，即 `(10, 1)`；
///   - 关键是外部的 `x` 没有被宏改成 10：若宏污染了命名空间，第二个分量就会变成 10。
///
/// 示例输入：
/// ```text
/// (add_inner_x!(0), x)
/// ```
/// 示例输出：
/// ```text
/// (10, 1)
/// ```
fn exercise_18_07_hygiene_edge() -> (i32, i32) {
    assessment_harness::todo_exercise(
        "exercise_18_07_hygiene_edge",
        "同样的宏与外部 let x = 1，返回 (add_inner_x!(0), x) = (10, 1)",
        (),
    )
}

// ===========================================================================
// kp_18_08 常见错误诊断：宏报错信息与场景的对应
// ===========================================================================

/// 知识点考核：把宏的典型报错场景与课程注释里的原文一一对上。
#[test]
fn kp_18_08_diagnose_errors() {
    assess(
        M,
        "kp_18_08",
        "宏的典型报错：先用后定义 / 形状不匹配 / 分隔符不符（取自课程示例 7 的注释原文）",
        Kind::Hard,
        "复习 lesson_18 示例 7（demo_7_common_mistakes）：本课列的 5 个坑里，前三个分别是\
         「宏先用后定义」「实参形状与匹配臂对不上」「重复里的分隔符用错」；\
         这三个错误在课程注释里给的是 `error:` 原文（没有 E 编号），只有第 5 个卫生性例子带 E0425。",
        || {
            eq_slice(
                &exercise_18_08_diagnose_errors(),
                &[
                    "cannot find macro `late` in this scope",
                    "no rules expected the token `)`",
                    "expected `,` or `)`",
                ],
                "顺序必须是：先用后定义、实参形状不匹配、重复分隔符不对；\
                 三条都必须逐字取自课程示例 7 的注释原文",
            );
        },
    );
}

/// 【待实现】写出三个宏报错场景对应的**课程注释原文**。
///
/// 场景（与课程示例 7 一致）：
///   1. 先写 `fn early() { late!(); }`、后写 `macro_rules! late { ... }`
///      —— 宏是文本作用域，从定义处向下才可见；
///   2. `max_of!(1);`（匹配臂要两个表达式，只给了一个）
///      —— 宏没有能匹配这次调用的规则；
///   3. `sum_all!(1; 2);`（匹配臂约定逗号分隔，却写了分号）
///      —— 重复模式里找不到约定的分隔符。
///
/// 实现要求：
///   - 返回 3 个字符串，顺序与上面 1、2、3 一致；
///   - 每个字符串必须与课程示例 7 注释里 `error:` 后面的**关键短语逐字一致**
///     （课程注释没给 E 编号，所以不要写编号）；
///   - 这三条短语都在课程 `demo_7_common_mistakes` 的注释里，可以直接抄；
///     只有第 5 个例子（卫生性）带 `error[E0425]`，不在本题范围内。
///
/// 示例输入：
/// ```text
/// late!();
/// max_of!(1);
/// sum_all!(1; 2);
/// ```
/// 示例输出：
/// ```text
/// ["cannot find macro `late` in this scope",
///  "no rules expected the token `)`",
///  "expected `,` or `)`"]
/// ```
fn exercise_18_08_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_18_08_diagnose_errors",
        "返回课程示例 7 注释原文：[\"cannot find macro `late` in this scope\", \
         \"no rules expected the token `)`\", \"expected `,` or `)`\"]",
        (),
    )
}
