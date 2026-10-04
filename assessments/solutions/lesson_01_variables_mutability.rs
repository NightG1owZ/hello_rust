//! assessments/lesson_01_variables_mutability.rs —— 考核：变量与可变性（对应 lesson_01）
//!
//! - 对应课程：`src/tutorial/lesson_01_variables_mutability.rs`
//! - 知识点出处：`src/tutorial/README.md` 第一阶段「01 变量与可变性」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_01_variables_mutability     # 只考这一课
//!   cargo test                                           # 考全部 18 课
//!   cargo run --bin assessment_report                    # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_01_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_01_variables_mutability`，直到全部用例变绿；
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
//! | `let lock = 10; lock = 20;` | `error[E0384]` cannot assign twice to immutable variable | 改成 `let mut lock`，或用遮蔽 `let lock = 20;` |
//! | `let mut v = 5; v = "hi";` | `error[E0308]` mismatched types | `mut` 只能改值不能改类型，换类型必须重新 `let`（遮蔽） |
//! | `let t: u64; println!("{t}");` | `error[E0381]` used binding isn't initialized | 声明时给初值，或保证使用前一定初始化 |
//! | `let v = Vec::new(); v.len();` | `error[E0282]` type annotations needed | 显式标注 `Vec<i32>` 或使用 turbofish |

use assessment_harness::{Kind, approx, assess, eq, eq_slice};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_01";

// ===========================================================================
// kp_01_01 let 绑定与 mut：默认不可变，需要改值必须显式声明 mut
// ===========================================================================

/// 知识点考核：用 `mut` 变量做累加。
#[test]
fn kp_01_01_mut_counter() {
    assess(
        M,
        "kp_01_01",
        "let 绑定与 mut：默认不可变，需要改值就写 mut",
        Kind::Basic,
        "复习 lesson_01 示例 1（immutable_by_default）与示例 2（mut）：\
         `let x = 1; x = 2;` 会报 error[E0384]，必须写成 `let mut x = 1;`。",
        || {
            // 从 0 开始累加 5 次、每次 +2 → 10
            eq(
                exercise_01_01_mut_counter(0, 5),
                10,
                "循环 5 次、每次加 2，最终值应为 10",
            );
            // 边界：一次都不加，结果应等于初始值
            eq(
                exercise_01_01_mut_counter(7, 0),
                7,
                "times 为 0 时不应改变初始值（循环体一次都不执行）",
            );
            // 边界：负数初始值同样要正确累加
            eq(
                exercise_01_01_mut_counter(-3, 2),
                1,
                "初始 -3 加上两次 2 应得到 1",
            );
        },
    );
}

/// 【待实现】用 `mut` 累加器累加。
///
/// 实现要求：
///   - 入参 `start` 是初始值，`times` 是要累加的**次数**；
///   - 用 `let mut` 声明一个累加器，循环 `times` 次，每次 `+= 2`；
///   - 循环写法用最直观的 `for _ in 0..times { ... }` 即可；
///   - 忘记写 `mut` 时编译器报
///     `error[E0384]: cannot assign twice to immutable variable`；
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
    let mut total = start;
    for _ in 0..times {
        total += 2;
    }
    total
}

// ===========================================================================
// kp_01_02 变量遮蔽（shadowing）的基本写法
// ===========================================================================

/// 知识点考核：用同名 `let` 遮蔽前一个绑定。
#[test]
fn kp_01_02_shadowing_chain() {
    assess(
        M,
        "kp_01_02",
        "变量遮蔽（shadowing）：同名 let 覆盖旧绑定",
        Kind::Basic,
        "复习 lesson_01 示例 3（shadowing）：遮蔽是「新建一个同名绑定」，\
         不是修改旧值，所以不需要 mut，也可以换类型。",
        || {
            // 前后有空白：先 trim 再取长度 → 4
            eq(
                exercise_01_02_shadowing_chain("  rust  "),
                4,
                "先遮蔽为 trim 后的 &str，再遮蔽为长度：\"  rust  \" → 4",
            );
            // 边界：空串
            eq(
                exercise_01_02_shadowing_chain(""),
                0,
                "空串的长度是 0，不要 panic",
            );
            // 边界：全是空白（trim 之后长度为 0）
            eq(
                exercise_01_02_shadowing_chain("   \t\n"),
                0,
                "只有空白字符时 trim 之后长度为 0",
            );
            // 中文是多字节：UTF-8 下「中文」占 6 个字节
            eq(
                exercise_01_02_shadowing_chain("中文"),
                6,
                "String::len() 返回的是字节数：\"中文\" 在 UTF-8 下是 6 字节",
            );
        },
    );
}

/// 【待实现】用遮蔽链清洗并测量字符串。
///
/// 实现要求：
///   - 先 `let text = text.trim();`（遮蔽为去掉首尾空白后的 `&str`）；
///   - 再 `let text = text.len();`（再次遮蔽，这次绑定的是 `usize`）；
///   - 这正是「遮蔽可以换类型」的最小例子：同一个名字先绑 `&str`、后绑 `usize`；
///   - 返回这个长度。
///
/// 示例输入：
/// ```text
/// text = "  rust  "
/// ```
/// 示例输出：
/// ```text
/// 4
/// ```
fn exercise_01_02_shadowing_chain(text: &str) -> usize {
    let text = text.trim();
    let text = text.len();
    text
}

// ===========================================================================
// kp_01_03 遮蔽 vs mut：遮蔽能换类型，mut 不能
// ===========================================================================

/// 知识点考核：解析字符串（换类型）然后计算。
#[test]
fn kp_01_03_shadowing_changes_type() {
    assess(
        M,
        "kp_01_03",
        "遮蔽可以改变类型，mut 不能（这一条是两者的本质区别）",
        Kind::Core,
        "复习 lesson_01 示例 3 与示例 7 的错误 2：`let mut v = 5; v = \"hi\";` 报 E0308，\
         而 `let v = 5; let v = \"hi\";` 合法——遮蔽新建绑定，允许类型变化。",
        || {
            // 返回 (原始字节长度, 解析成 i32 后乘以 2)
            eq(
                exercise_01_03_shadowing_changes_type("42"),
                (2, 84),
                "\"42\" 长度 2；解析为 42 后乘 2 得 84",
            );
            // 边界：解析出 0
            eq(
                exercise_01_03_shadowing_changes_type("0"),
                (1, 0),
                "\"0\" 长度 1；0 乘 2 仍是 0",
            );
            // 边界：负数与前后空白
            eq(
                exercise_01_03_shadowing_changes_type(" -7 "),
                (4, -14),
                "长度按**原始**字符串算（含空格，共 4 字节）；trim 后 parse 得 -7",
            );
        },
    );
}

/// 【待实现】遮蔽换类型：字符串 → 整数。
///
/// 实现要求：
///   - 先记住原始字符串的字节长度（`input.len()`，**不要**先 trim）；
///   - 再用遮蔽把 `input` 变成 `i32`：`let input: i32 = input.trim().parse().unwrap();`
///     （用 `unwrap()` 在这里可以接受：考核只会传入合法数字字符串）；
///   - `.parse()` 的返回类型无法推断时必须显式标注（或写 `.parse::<i32>()`），
///     否则报 `error[E0282]: type annotations needed`；
///   - 返回 `(原始字节长度, 整数 * 2)`。
///
/// 示例输入：
/// ```text
/// input = "42"
/// ```
/// 示例输出：
/// ```text
/// (2, 84)
/// ```
fn exercise_01_03_shadowing_changes_type(input: &str) -> (usize, i32) {
    // 先记下**原始**字符串的字节长度（含空格），再遮蔽成 i32
    let raw_len = input.len();
    let input: i32 = input.trim().parse().unwrap();
    (raw_len, input * 2)
}

// ===========================================================================
// kp_01_04 const 与 static 的声明与使用
// ===========================================================================

/// 知识点考核：在函数内声明 `const` 与 `static` 并使用。
#[test]
fn kp_01_04_const_and_static() {
    assess(
        M,
        "kp_01_04",
        "const 与 static：编译期常量 vs 静态变量（命名用 SCREAMING_SNAKE_CASE）",
        Kind::Core,
        "复习 lesson_01 示例 5（const_and_static）：const 在编译期求值、可写在任何作用域；\
         static 有固定内存地址、全程只有一个实例；两者都必须显式写类型。",
        || {
            // (MAX_RETRY * 2, APP_NAME, APP_NAME 的字节长度)
            eq(
                exercise_01_04_const_and_static(),
                (6u32, "assessment", 10usize),
                "MAX_RETRY=3 → 6；APP_NAME=\"assessment\" 长度 10",
            );
        },
    );
}

/// 【待实现】声明块级 `const` 与 `static`。
///
/// 实现要求（全部在函数体内完成）：
///   - 声明 `const MAX_RETRY: u32 = 3;`
///   - 声明 `static APP_NAME: &str = "assessment";`
///   - 两处都**必须**写类型标注（`const` / `static` 不允许省略类型），名字遵守
///     `SCREAMING_SNAKE_CASE`，否则 `cargo clippy` 会提示 non_upper_case_globals；
///   - 返回 `(MAX_RETRY * 2, APP_NAME, APP_NAME.len())`，即 `(6, "assessment", 10)`
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (6, "assessment", 10)
/// ```
fn exercise_01_04_const_and_static() -> (u32, &'static str, usize) {
    // const / static 都必须显式写类型，命名用 SCREAMING_SNAKE_CASE
    const MAX_RETRY: u32 = 3;
    static APP_NAME: &str = "assessment";
    (MAX_RETRY * 2, APP_NAME, APP_NAME.len())
}

// ===========================================================================
// kp_01_05 类型推断与显式类型标注
// ===========================================================================

/// 知识点考核：该让编译器推断的地方交给它，推断不了的地方显式标注。
#[test]
fn kp_01_05_inference_and_annotation() {
    assess(
        M,
        "kp_01_05",
        "类型推断与显式标注：能推断就推断，推断不了必须标注",
        Kind::Core,
        "复习 lesson_01 示例 4（type_inference_and_annotation）与示例 7 的错误 5：\
         `.parse()` 与 `Vec::new()` 这类无法从上下文推断目标类型的地方，必须显式标注。",
        || {
            let (sum, price, count) = exercise_01_05_inference_and_annotation();
            eq(
                sum,
                49i32,
                "40 + 2 + 7 = 49（40 与 2 靠推断，7 来自 parse::<i32>()）",
            );
            approx(price, 19.9, 1e-9, "显式标注的 f64 应原样返回");
            eq(count, 3usize, "显式标注的 usize 应原样返回");
        },
    );
}

/// 【待实现】混合使用类型推断与显式标注。
///
/// 实现要求：
///   - `let a = 40;` 与 `let b = 2;`（靠推断得到 `i32`），两数相加；
///   - `let n = "7".parse::<i32>().unwrap();`（用 turbofish 显式标注）再相加；
///   - `let price: f64 = 19.9;`（显式标注 f64）；
///   - `let count: usize = 3;`（显式标注 usize）；
///   - 整数字面量默认是 `i32`、浮点字面量默认是 `f64`，而返回值的第一个分量必须是
///     `i32`（与测试里的 `49i32` 对应）；
///   - 返回 `(a + b + n, price, count)`，即 `(49, 19.9, 3)`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (49, 19.9, 3)
/// ```
fn exercise_01_05_inference_and_annotation() -> (i32, f64, usize) {
    // 前两个靠推断（整数字面量默认 i32），第三个用 turbofish 标注
    let a = 40;
    let b = 2;
    let n = "7".parse::<i32>().unwrap();
    let price: f64 = 19.9;
    let count: usize = 3;
    (a + b + n, price, count)
}

// ===========================================================================
// kp_01_06 作用域：内层遮蔽不会影响外层绑定
// ===========================================================================

/// 知识点考核：块作用域里的遮蔽只在该块内生效。
#[test]
fn kp_01_06_scope_shadowing() {
    assess(
        M,
        "kp_01_06",
        "作用域与遮蔽：内层 let 遮蔽不影响外层绑定的值",
        Kind::Edge,
        "复习 lesson_01 示例 3 的作用域部分：遮蔽是「新绑定」，内层块结束时旧绑定重新可见。",
        || {
            eq(
                exercise_01_06_scope_shadowing(),
                (20i32, 10i32),
                "内层块里 x 被遮蔽成 20；离开块之后外层 x 仍然是 10",
            );
        },
    );
}

/// 【待实现】验证「内层遮蔽不影响外层」。
///
/// 实现要求：
///   - 在外层写 `let x = 10;`
///   - 用一个**块表达式**（`{ ... }`）在内层写 `let x = x * 2;`，并让这个块的值就是内层的 `x`；
///   - Rust 中 `{ ... }` 是表达式，把 `let inner = { ... };` 写出来即可；
///   - 返回 `(内层的 x, 外层的 x)`，即 `(20, 10)`；关键是遮蔽**不会**修改外层那个绑定。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (20, 10)
/// ```
fn exercise_01_06_scope_shadowing() -> (i32, i32) {
    let x = 10;
    // 块表达式：内层遮蔽只在这个块里生效
    let inner = {
        let x = x * 2;
        x
    };
    (inner, x)
}

// ===========================================================================
// kp_01_07 const 表达式：编译期就算好的值
// ===========================================================================

/// 知识点考核：用 `const` 表达式表达单位换算，体会「编译期求值」。
#[test]
fn kp_01_07_const_expression() {
    assess(
        M,
        "kp_01_07",
        "const 表达式：单位换算在编译期完成，零运行期开销",
        Kind::Edge,
        "复习 lesson_01 示例 5：`const KB: usize = 1024; const MB: usize = KB * 1024;` \
         这类写法在编译期就求值，运行期只是一个立即数。",
        || {
            eq(
                exercise_01_07_const_expression(),
                (1_048_576usize, 86_400u32),
                "1 MB = 1024 * 1024 = 1048576 字节；一天 = 24 * 60 * 60 = 86400 秒",
            );
        },
    );
}

/// 【待实现】用 `const` 表达式做单位换算。
///
/// 实现要求（全部在函数体内完成）：
///   - `const KB: usize = 1024;`
///   - `const MB: usize = KB * 1024;`（用一个 const 去算另一个 const）
///   - `const SECONDS_PER_DAY: u32 = 24 * 60 * 60;`
///   - 把结果直接写成 `1048576` 也能让测试通过，但那就失去了本知识点的意义，
///     务必用常量之间的**表达式**推导出来；
///   - 返回 `(MB, SECONDS_PER_DAY)`，即 `(1048576, 86400)`
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (1048576, 86400)
/// ```
fn exercise_01_07_const_expression() -> (usize, u32) {
    // 用一个 const 推导另一个 const：全部在编译期求值
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const SECONDS_PER_DAY: u32 = 24 * 60 * 60;
    (MB, SECONDS_PER_DAY)
}

// ===========================================================================
// kp_01_08 常见错误诊断：读得懂编译器报错，才改得动代码
// ===========================================================================

/// 知识点考核：根据错误描述写出正确的编译器错误编号。
#[test]
fn kp_01_08_diagnose_errors() {
    assess(
        M,
        "kp_01_08",
        "常见错误诊断：E0384 / E0308 / E0381 分别对应哪类错误",
        Kind::Hard,
        "复习 lesson_01 示例 7（common_mistakes）：本课列的 5 个坑里，\
         前三个分别是「二次赋值给不可变变量」「mut 变量赋了不同类型」「未初始化就使用」。",
        || {
            eq_slice(
                &exercise_01_08_diagnose_errors(),
                &["E0384", "E0308", "E0381"],
                "顺序必须是：不可变变量二次赋值、类型不匹配、未初始化就使用",
            );
        },
    );
}

/// 【待实现】写出三个场景对应的编译器错误编号。
///
/// 场景（与课程示例 7 一致）：
///   1. `let lock = 10; lock = 20;` —— 对不可变变量二次赋值；
///   2. `let mut value = 5; value = "hello";` —— mut 变量被赋了不同类型；
///   3. `let timeout: u64; println!("{timeout}");` —— 使用未初始化的绑定。
///
/// 实现要求：返回 3 个错误编号字符串（形如 `"E0384"`），顺序与上面一致；
/// 这些编号在课程示例 7 的注释里都能找到。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0384", "E0308", "E0381"]
/// ```
fn exercise_01_08_diagnose_errors() -> [&'static str; 3] {
    // 顺序：不可变变量二次赋值 / mut 变量赋不同类型 / 使用未初始化绑定
    ["E0384", "E0308", "E0381"]
}
