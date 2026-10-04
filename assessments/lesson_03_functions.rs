//! assessments/lesson_03_functions.rs —— 考核：函数（对应 lesson_03）
//!
//! - 对应课程：`src/tutorial/lesson_03_functions.rs`
//! - 知识点出处：`src/tutorial/README.md` 第一阶段「03 函数」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_03_functions     # 只考这一课
//!   cargo test                                 # 考全部 18 课
//!   cargo run --bin assessment_report          # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_03_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_03_functions`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 7）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `fn double(value) -> i32 { value * 2 }` | 解析阶段报错且**无 E 编号**：expected one of `:`, `@`, or `|`, found `)` | 每个参数都要写「名字: 类型」（E0642 讲的是另一件事：无函数体的函数里不允许写模式） |
//! | `fn half(value: i32) { value / 2 }` | `error[E0308]` mismatched types：expected `()`, found `i32` | 补上返回类型 `-> i32`（之后把 `half(9)` 当整数用还会连带报 E0277） |
//! | `fn square_tail(value: i32) -> i32 { value * value; }` | `error[E0308]` mismatched types：expected `i32`, found `()` | 删掉尾表达式的分号，或显式写 `return value * value;` |
//! | `let a = (let b = 1);` | 解析阶段报错且**无 E 编号**：expected expression, found `let` statement | `let` 是语句，不能出现在表达式位置 |
//! | `add(1)` / `add(1, 2.0)` | `error[E0061]` 实参个数不对；`error[E0308]` 实参类型不匹配 | 按签名传够个数且类型一致，比如 `add(1, 2)` |
//! | 发散函数（`-> !`）之后还有语句 | 警告 `unreachable statement`（`unreachable_code`，属于 `unused`） | 删除 return 之后的代码，或把它移到 return 之前 |

use assessment_harness::{Kind, assess, eq, eq_slice, is_true, panics};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_03";

/// 把「不该发生」的情况变成一条带提示的断言失败（而不是直接用 `assert!` 少掉提示）。
///
/// 只在 `kp_03_06` 的前置防呆里用到：骨架态的练习函数会自己 panic，
/// 那种 panic 不能被当成「发散函数写对了」。
fn fail(reason: &str, hint: &str) {
    is_true(false, &format!("{reason} · {hint}"));
}

// ===========================================================================
// kp_03_01 函数定义：参数必须逐个标注类型，尾表达式即返回值
// ===========================================================================

/// 知识点考核：写一个最基础的加法函数。
#[test]
fn kp_03_01_add() {
    assess(
        M,
        "kp_03_01",
        "函数定义：参数逐个标注类型，尾表达式就是返回值",
        Kind::Basic,
        "复习 lesson_03 示例 1（define_and_call）与示例 2（multiple_params_and_return）：\
         签名写成 `fn add(a: i32, b: i32) -> i32`，函数体最后一行 `a + b` 不带分号；\
         示例 1 还演示了「调用时机与定义顺序无关」。",
        || {
            // 正常用例：最基本的加法
            eq(
                exercise_03_01_add(3, 4),
                7i32,
                "3 + 4 = 7，最基本的正常用例",
            );
            // 边界用例：0 是加法的单位元
            eq(
                exercise_03_01_add(0, 0),
                0i32,
                "0 + 0 = 0：加数为 0 时结果必须原样等于另一个加数",
            );
            // 边界用例：负数参与加法
            eq(
                exercise_03_01_add(-5, 2),
                -3i32,
                "-5 + 2 = -3，带负号的加法不要写错符号",
            );
            // 边界用例：i32 的极值相加，正好不溢出
            eq(
                exercise_03_01_add(i32::MAX, i32::MIN),
                -1i32,
                "i32::MAX + i32::MIN = 2147483647 + (-2147483648) = -1，恰好不会溢出",
            );
        },
    );
}

/// 【待实现】两数相加。
///
/// 实现要求：
///   - 签名已给出：`fn exercise_03_01_add(a: i32, b: i32) -> i32`；
///   - 用**尾表达式**返回两数之和：写 `a + b`，**不加分号**，也**不写 `return`**；
///   - 函数体最后一行不带分号的表达式就是返回值；一旦给它补上分号，它立刻变成语句，
///     函数返回类型就成了 `()`，编译器报 `error[E0308]: mismatched types`。
///
/// 示例输入：
/// ```text
/// a = 3, b = 4
/// ```
/// 示例输出：
/// ```text
/// 7
/// ```
fn exercise_03_01_add(a: i32, b: i32) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_03_01_add",
        "用尾表达式返回 a + b（不加分号、不写 return）",
        (a, b),
    )
}

// ===========================================================================
// kp_03_02 参数与返回值：多返回值用元组
// ===========================================================================

/// 知识点考核：返回矩形的面积与周长。
#[test]
fn kp_03_02_rect() {
    assess(
        M,
        "kp_03_02",
        "多返回值：用一个元组同时带回面积与周长",
        Kind::Basic,
        "复习 lesson_03 示例 2（multiple_params_and_return）：`divide_with_remainder` 用 \
         `-> (i32, i32)` 一次返回商和余数，调用方用 `let (q, r) = ...` 解构接收；\
         参数类型也必须逐个写全，不能省略。",
        || {
            // 正常用例：3 × 4 的矩形
            eq(
                exercise_03_02_rect(3, 4),
                (12u32, 14u32),
                "面积 = 3 * 4 = 12；周长 = 2 * (3 + 4) = 14，两个分量顺序不能反",
            );
            // 边界用例：边长含 0，退化成一条线段
            eq(
                exercise_03_02_rect(0, 5),
                (0u32, 10u32),
                "宽为 0 时面积是 0，但周长仍是 2 * (0 + 5) = 10（不要把周长也算成 0）",
            );
            // 边界用例：正方形，长宽相等
            eq(
                exercise_03_02_rect(5, 5),
                (25u32, 20u32),
                "正方形 5 × 5：面积 25，周长 2 * (5 + 5) = 20",
            );
            // 边界用例：最小非零边长
            eq(
                exercise_03_02_rect(1, 1),
                (1u32, 4u32),
                "1 × 1 是 u32 参数下最小的非零矩形：面积 1、周长 4",
            );
        },
    );
}

/// 【待实现】返回矩形的面积与周长。
///
/// 实现要求：
///   - 入参 `width`、`height` 都是 `u32`；
///   - 第 1 个返回值是面积 `width * height`；
///   - 第 2 个返回值是周长 `2 * (width + height)`；
///   - 返回类型是 `(u32, u32)`，**顺序不能颠倒**：两个分量类型相同，写反了编译器也不会报错；
///   - Rust 的「多返回值」就是**元组**，调用方用 `let (area, perimeter) = ...;` 解构接收；
///     `u32` 相乘会溢出，本考核只传入较小的边长。
///
/// 示例输入：
/// ```text
/// width = 3, height = 4
/// ```
/// 示例输出：
/// ```text
/// (12, 14)
/// ```
fn exercise_03_02_rect(width: u32, height: u32) -> (u32, u32) {
    assessment_harness::todo_exercise(
        "exercise_03_02_rect",
        "返回 (面积 width * height, 周长 2 * (width + height))",
        (width, height),
    )
}

// ===========================================================================
// kp_03_03 尾表达式：让 if 表达式成为函数体最后一项
// ===========================================================================

/// 知识点考核：用 `if` 表达式充当尾表达式。
#[test]
fn kp_03_03_tail_expression() {
    assess(
        M,
        "kp_03_03",
        "语句与表达式：`if` 是表达式，放在函数体末尾就是返回值",
        Kind::Core,
        "复习 lesson_03 示例 3（statement_vs_expression）：`let sign_label = if x > 0 { ... } else { ... };` \
         说明 if 会产生值；示例 7 的错误 3 演示了尾表达式多写分号会得到 `()`（E0308）。",
        || {
            // 正常用例：正数走 then 分支
            eq(
                exercise_03_03_tail_expression(5),
                10i32,
                "n = 5 > 0，走 then 分支：5 * 2 = 10",
            );
            // 边界用例：0 不满足 n > 0，必须落到 else
            eq(
                exercise_03_03_tail_expression(0),
                0i32,
                "n = 0 不满足 n > 0，进入 else 分支返回 0（判定写成 n >= 0 会让 0 也去乘 2）",
            );
            // 边界用例：负数，同样落到 else
            eq(
                exercise_03_03_tail_expression(-1),
                0i32,
                "n = -1 是负数，也进入 else 分支返回 0（返回值与 n 的大小无关）",
            );
            // 边界用例：最小的正整数，验证判断用的是 > 而不是 >=
            eq(
                exercise_03_03_tail_expression(1),
                2i32,
                "n = 1 > 0，1 * 2 = 2；如果错写成 n >= 0，那么 0 也会被算成 0 * 2（这里刚好也是 0，但逻辑已错）",
            );
        },
    );
}

/// 【待实现】用 `if` 尾表达式实现分段返回。
///
/// 实现要求：
///   - 整个函数体就是**一个 `if` 表达式**：`if n > 0 { n * 2 } else { 0 }`；
///   - `if` 后面**不要**加分号，也不要写 `return`，两个分支都是表达式（内部不带分号）；
///   - 即 `n > 0` 返回 `n * 2`，否则返回 `0`；
///   - Rust 里 `if` 是表达式，本身就有值，可以直接当返回值用；若在收尾的 `}` 后面误加分号，
///     函数实际返回 `()`，编译器报 `error[E0308]: mismatched types`；
///   - 两个分支的类型必须一致（这里都是 `i32`），否则同样报 E0308。
///
/// 示例输入：
/// ```text
/// n = 5
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_03_03_tail_expression(n: i32) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_03_03_tail_expression",
        "用 if 尾表达式：n > 0 时返回 n * 2，否则返回 0",
        (n,),
    )
}

// ===========================================================================
// kp_03_04 提前 return 与尾表达式共存
// ===========================================================================

/// 知识点考核：先挡掉非法输入，再写主流程。
#[test]
fn kp_03_04_early_return() {
    assess(
        M,
        "kp_03_04",
        "提前 return + 尾表达式：先挡掉非法输入，再走正常路径",
        Kind::Core,
        "复习 lesson_03 示例 3 的 `positive_only`：`if value <= 0 { return 0; }` 提前返回，\
         末尾直接写 `value` 作为尾表达式；示例 6 的 `safe_divide` 也用了同样的「守卫」写法。",
        || {
            // 正常用例：正数走主流程
            eq(
                exercise_03_04_early_return(4),
                16i32,
                "n = 4 不是负数，走主流程：4 * 4 = 16",
            );
            // 边界用例：0 不是负数，必须走到 n * n
            eq(
                exercise_03_04_early_return(0),
                0i32,
                "n = 0 满足 n >= 0，走主流程 0 * 0 = 0；若判定写成 n <= 0 就会返回 -1，结果明显不同",
            );
            // 边界用例：负数命中提前 return
            eq(
                exercise_03_04_early_return(-3),
                -1i32,
                "n = -3 < 0，命中提前 return，固定返回哨兵值 -1",
            );
            // 边界用例：i32::MIN 也是负数
            eq(
                exercise_03_04_early_return(i32::MIN),
                -1i32,
                "i32::MIN 是负数的极值，同样走提前 return 分支返回 -1",
            );
        },
    );
}

/// 【待实现】负数提前返回哨兵值，其余情况返回平方。
///
/// 实现要求：
///   - 若 `n < 0`，用 `return -1;` **提前返回**（`return` 是语句，必须带分号）；
///   - 其余情况（`n >= 0`）走主流程，用尾表达式返回 `n * n`；
///   - 即 `n < 0 → -1`、`n = 0 → 0`、`n = 4 → 16`；
///   - 判定条件必须是 `n < 0`：写成 `n <= 0` 会把 0 误判成非法输入；
///   - 「提前 `return` + 末尾尾表达式」是 Rust 的惯用组合，
///     编译器不会因为你写了 `return` 就要求末尾再写一个 `return`。
///
/// 示例输入：
/// ```text
/// n = 4
/// ```
/// 示例输出：
/// ```text
/// 16
/// ```
fn exercise_03_04_early_return(n: i32) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_03_04_early_return",
        "n < 0 时提前 return -1，否则用尾表达式返回 n * n",
        (n,),
    )
}

// ===========================================================================
// kp_03_05 函数指针：把「怎么算」当作参数传进来
// ===========================================================================

/// 知识点考核：接收 `fn(i32) -> i32` 并调用它。
#[test]
fn kp_03_05_apply() {
    assess(
        M,
        "kp_03_05",
        "函数指针：`fn(i32) -> i32` 是类型，函数名与不捕获环境的闭包都能当值传",
        Kind::Core,
        "复习 lesson_03 示例 4（function_pointer）：`let operation: fn(i32, i32) -> i32 = add;` \
         说明函数名可以直接赋给函数指针；`let increment: fn(i32) -> i32 = |value| value + 1;` \
         说明不捕获外部变量的闭包能强制转换成 fn 指针（示例 1 把折扣率写成常量，正是为此）。",
        || {
            // 正常用例：传具名函数
            fn double(n: i32) -> i32 {
                n * 2
            }
            eq(
                exercise_03_05_apply(double, 21),
                42i32,
                "把具名函数 double 当函数指针传入：double(21) = 42",
            );
            // 边界用例：传不捕获环境的闭包（能强制转成 fn 指针）
            eq(
                exercise_03_05_apply(|n| n + 1, 0),
                1i32,
                "不捕获外部变量的闭包可以强制转换成 fn(i32) -> i32：0 + 1 = 1",
            );
            // 边界用例：负数入参照样按传入的函数计算
            eq(
                exercise_03_05_apply(double, -4),
                -8i32,
                "函数指针的执行与入参正负无关：double(-4) = -8",
            );
        },
    );
}

/// 【待实现】调用传进来的函数指针。
///
/// 实现要求：
///   - 入参 `f` 的类型就是函数指针 `fn(i32) -> i32`（`fn(i32) -> i32` 是类型，`f` 是值）；
///   - 直接在函数体里调用它，并用尾表达式返回结果：`f(value)`；
///   - 不要对 `f` 做额外判断或变换；
///   - 具名函数（写函数名，不加括号）和**不捕获环境**的闭包都能传进来；
///     一旦闭包捕获了局部变量（如 `|n| n + offset`），它的类型就是匿名闭包类型，
///     无法再强制转换成 `fn` 指针，传参时报 E0308。
///
/// 示例输入：
/// ```text
/// f = double（double 是 n * 2 的具名函数）, value = 21
/// ```
/// 示例输出：
/// ```text
/// 42
/// ```
fn exercise_03_05_apply(f: fn(i32) -> i32, value: i32) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_03_05_apply",
        "用尾表达式返回 f(value)",
        (f, value),
    )
}

// ===========================================================================
// kp_03_06 发散函数：返回 `!` 的函数可以放在任何类型的位置
// ===========================================================================

/// 知识点考核：函数内定义返回 `!` 的局部函数并调用它。
#[test]
fn kp_03_06_diverge() {
    assess(
        M,
        "kp_03_06",
        "发散函数（`!`）：永不返回的函数可被强制转换成任意类型",
        Kind::Hard,
        "复习 lesson_03 示例 5（diverging_function）：`fn exit_with_error(message: &str) -> !` \
         能放在需要 i32 的 match 分支里，因为 `!` 可以强转成任何类型；\
         示例 7 的错误 6 提醒：发散函数之后的语句不可达，编译器会给出 unreachable_code 警告。",
        || {
            // 防呆：骨架态下 exercise_03_06_diverge 里只有一行 `todo_exercise`，
            // 它自己就会 panic——那种「panic」不能被当成「发散函数写对了」，
            // 所以先用 catch_unwind 把它单独接住并判失败（保留 panic 载荷，不再打印）。
            let skeleton = std::panic::catch_unwind(|| {
                exercise_03_06_diverge("boom");
            });
            match skeleton {
                Err(payload) => {
                    if let Some(text) = payload.downcast_ref::<&str>()
                        && text.contains("未实现")
                    {
                        fail(
                            "exercise_03_06_diverge 仍是未实现的占位（todo_exercise）",
                            "骨架态下它本来就会 panic：先删掉 `assessment_harness::todo_exercise(...)` 写上实现，再谈「必然 panic」",
                        );
                    }
                    if let Some(text) = payload.downcast_ref::<String>()
                        && text.contains("未实现")
                    {
                        fail(
                            "exercise_03_06_diverge 仍是未实现的占位（todo_exercise）",
                            "骨架态下它本来就会 panic：先删掉 `assessment_harness::todo_exercise(...)` 写上实现，再谈「必然 panic」",
                        );
                    }
                }
                Ok(_) => {
                    fail(
                        "发散函数竟然返回了一个 u32",
                        "`fail(msg)` 的返回类型是 `!`，调用它之后绝不应该有值返回",
                    );
                }
            }

            // 正常用例：调用发散函数必然 panic，且 panic 信息就是传入的 msg
            panics(
                || {
                    exercise_03_06_diverge("boom");
                },
                "返回 `!` 的函数必须真的永远不返回：调用它应当 panic，而不是返回一个 u32",
            );
            // 边界用例：空消息同样必须 panic
            panics(
                || {
                    exercise_03_06_diverge("");
                },
                "msg 为空串时同样必须 panic：「永不返回」与入参内容无关，不能对空串特事特办",
            );
        },
    );
}

/// 【待实现】在函数内定义发散函数并调用它。
///
/// 实现要求：
///   - 在本函数**内部**定义 `fn fail(msg: &str) -> ! { panic!("{msg}") }`；
///   - 然后调用它：`fail(msg)`（该调用的类型是 `!`，可直接充当 `u32` 位置的返回值）；
///   - 函数签名已给出 `-> u32`：**不要**为了「让编译器满意」而返回一个假值；
///   - `!` 是 never 类型，不属于任何具体类型，因此能强制转换成任何类型；
///   - `panic!("{msg}")` 用的是内联捕获变量写法（edition 2024 支持）；
///     发散函数之后的代码永远不可达，写了会得到 unreachable_code 警告。
///
/// 示例输入：
/// ```text
/// msg = "boom"
/// ```
/// 示例输出：
/// ```text
/// panic: boom
/// ```
fn exercise_03_06_diverge(msg: &str) -> u32 {
    assessment_harness::todo_exercise(
        "exercise_03_06_diverge",
        "函数内定义 fn fail(msg: &str) -> ! { panic!(\"{msg}\") }，调用它让本函数永不返回",
        (msg,),
    )
}

// ===========================================================================
// kp_03_07 函数指针数组：把多个步骤串成一条流水线
// ===========================================================================

/// 知识点考核：把函数指针数组依次作用到同一个值上。
#[test]
fn kp_03_07_pipeline() {
    assess(
        M,
        "kp_03_07",
        "函数指针数组：按顺序把每个函数作用到 value 上（空数组原样返回）",
        Kind::Edge,
        "复习 lesson_03 示例 4（function_pointer）的「策略表」：\
         `let strategies: [fn(i32, i32) -> i32; 3] = [add, subtract, max_of];`；\
         示例 6 的 `pipeline` 演示了把函数指针放进数组依次执行、组合小函数完成完整计算。",
        || {
            fn increment(n: i32) -> i32 {
                n + 1
            }
            fn triple(n: i32) -> i32 {
                n * 3
            }
            fn minus_five(n: i32) -> i32 {
                n - 5
            }

            // 正常用例：三个函数指针的组合（+1 → *3 → -5）
            let ops: [fn(i32) -> i32; 3] = [increment, triple, minus_five];
            eq(
                exercise_03_07_pipeline(&ops, 2),
                4i32,
                "依次执行：2 → +1 = 3 → *3 = 9 → -5 = 4；顺序写反结果就不同",
            );
            // 边界用例：空数组，原样返回入参
            eq(
                exercise_03_07_pipeline(&[], 7),
                7i32,
                "ops 为空时循环体一次都不执行，直接把 value 返回",
            );
            // 边界用例：只有一个函数指针
            let single: [fn(i32) -> i32; 1] = [triple];
            eq(
                exercise_03_07_pipeline(&single, 5),
                15i32,
                "只给一个函数时结果就是 triple(5) = 15",
            );
            // 边界用例：空数组 + 负数入参
            eq(
                exercise_03_07_pipeline(&[], -7),
                -7i32,
                "空数组遇到负数也要原样返回，不能有任何默认变换",
            );
        },
    );
}

/// 【待实现】把函数指针数组依次作用到 value 上。
///
/// 实现要求：
///   - 用一个可变变量保存当前值，初始为 `value`；
///   - 按**数组顺序**遍历 `ops`，每一步执行 `current = op(current)`；
///   - 返回最终结果；`ops` 为空时直接返回 `value`；
///   - `ops: &[fn(i32) -> i32]` 是**函数指针的切片**，`for op in ops` 拿到的元素类型
///     就是 `fn(i32) -> i32`（不是引用），可以直接调用 `op(current)`；
///   - 顺序很重要——流水线是有方向的，反过来算结果就变了。
///
/// 示例输入：
/// ```text
/// ops = [increment, triple, minus_five]（依次是 n + 1、n * 3、n - 5）
/// value = 2
/// ```
/// 示例输出：
/// ```text
/// 4
/// ```
fn exercise_03_07_pipeline(ops: &[fn(i32) -> i32], value: i32) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_03_07_pipeline",
        "按顺序把 ops 里的每个函数作用到 value 上并返回结果；空切片原样返回 value",
        (ops, value),
    )
}

// ===========================================================================
// kp_03_08 常见错误诊断：读得懂编译器报错，才改得动代码
// ===========================================================================

/// 知识点考核：写出课程示例 7 前三个错误示例的编译器错误编号。
#[test]
fn kp_03_08_diagnose_errors() {
    assess(
        M,
        "kp_03_08",
        "常见错误诊断：函数课前三个坑分别报什么（两个无编号解析错误 + E0308）",
        Kind::Hard,
        "复习 lesson_03 示例 7（common_mistakes）：错误 1「参数漏写类型」报的是解析阶段的 \
         `expected one of `:`, `@`, or `|``，课程注释明确写了**没有 E 编号**；\
         错误 2「漏写返回类型却用表达式返回值」报 E0308（expected `()`, found `i32`）；\
         错误 3「尾表达式多写分号」同样报 E0308（expected `i32`, found `()`）。",
        || {
            eq_slice(
                &exercise_03_08_diagnose_errors(),
                &["无编号", "E0308", "E0308"],
                "第 1 项是解析错误（课程注释写明「无编号」），第 2、3 项都是类型不匹配 E0308",
            );
        },
    );
}

/// 【待实现】写出课程示例 7 前三个错误示例对应的编译器错误编号。
///
/// 场景（与课程示例 7 的前三个错误一一对应，**顺序必须一致**）：
///   1. `fn double(value) -> i32 { value * 2 }` —— 参数漏写类型，
///      报 `error: expected one of `:`, `@`, or `|`, found `)`（**无 E 编号**）；
///   2. `fn half(value: i32) { value / 2 }` —— 漏写返回类型却用表达式返回值，
///      报 `error[E0308]: mismatched types（expected `()`, found `i32`）`；
///   3. `fn square_tail(value: i32) -> i32 { value * value; }` —— 尾表达式多写分号，
///      报 `error[E0308]: mismatched types（expected `i32`, found `()`）`。
///
/// 实现要求：
///   - 返回 `[&'static str; 3]`，顺序与上面一致；
///   - 有编号的写编号（形如 `"E0308"`）；课程注释里写明「无编号」的那一条，
///     请原样填 `"无编号"`——**不要**编造一个编号填进去；
///   - 这正是本课要培养的能力：先分清报错出现在「解析阶段」还是「类型检查阶段」；
///     解析错误通常没有 E 编号，类型不匹配几乎总是 E0308。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["无编号", "E0308", "E0308"]
/// ```
fn exercise_03_08_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_03_08_diagnose_errors",
        "返回 [\"无编号\", \"E0308\", \"E0308\"]（参数漏类型 / 漏返回类型 / 尾表达式多分号）",
        (),
    )
}
