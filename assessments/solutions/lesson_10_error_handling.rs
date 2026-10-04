//! assessments/lesson_10_error_handling.rs —— 考核：错误处理（对应 lesson_10）
//!
//! - 对应课程：`src/tutorial/lesson_10_error_handling.rs`
//! - 知识点出处：`src/tutorial/README.md` 第三阶段「10 错误处理」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_10_error_handling     # 只考这一课
//!   cargo test                                     # 考全部 18 课
//!   cargo run --bin assessment_report              # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_10_xx_xxx` 练习函数——有的知识点要实现的不是普通函数，
//!    而是类型上的 `impl`（例如 `ParseError` 的 `Display`、`AppError` 的 `From`），
//!    它们都在上面的「提供给你的类型」小节里，函数体同样只有一行 `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_10_error_handling`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数与标着「【待实现】」的 `impl` 方法体，可以按需增加局部变量与
//!   辅助函数；「提供给你的类型」里的枚举定义本身**不要改**；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 9）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | 在返回 `()` 的函数里用 `?` | `error[E0277]` the `?` operator can only be used in a function that returns `Result` or `Option` | 让函数返回 `Result<(), E>`，或改用 `match` 显式处理 |
//! | 在返回 `Result` 的函数里对 `Option` 用 `?` | `error[E0277]` the `?` operator can only be used on `Result`s, not `Option`s | 先 `ok_or_else(...)` 转成 `Result`，或让函数返回 `Option` |
//! | 源错误类型没有对应的 `From` | `error[E0277]` `?` couldn't convert the error to `MyErr` | 补 `impl From<源错误> for MyErr` |
//! | 自定义错误没实现 `Display` / `Error` 就装箱 | `error[E0277]` the trait bound `MyErr: std::error::Error` is not satisfied（紧接着还有 `error[E0599]` 缺 `Display`） | 同时实现 `Display` 与 `Error`，本文件的 `ParseError` 就是最小写法 |
//! | 丢弃未处理的 `Result` | `warning: unused Result that must be used` | 用 `let _ = ...;` 显式忽略，或真正处理它 |
//! | 常量除零 `let x = 1 / 0;` | `error: this operation will panic at runtime` | 用 `checked_div`，或像本课一样返回 `Result`（浮点除零不 panic，得到 inf / NaN，更隐蔽） |

use assessment_harness::{
    Kind, assess, contains, eq, eq_slice, err, is_false, is_true, ok, panics,
};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_10";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// 两个错误类型的最小骨架：Display 与 From 由你实现（见下面两个【待实现】的 impl）。
// ===========================================================================

/// 解析错误：面向使用者的消息（`Display`）由你实现，对应 kp_10_05。
///
/// - `Empty`：输入是空串或纯空白；
/// - `NotNumber(String)`：输入不是合法整数，`String` 里保留**原始输入**，出错时便于定位。
#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    NotNumber(String),
}

/// 应用层错误：`From<ParseError>` 由你实现，对应 kp_10_06。
#[derive(Debug, PartialEq)]
enum AppError {
    /// 底层解析错误（由 `?` 通过 `From` 自动上转得到）
    Parse(ParseError),
    /// 数值超出可表示范围（扩展变体：本课的练习路径不会产生它，但 `PartialEq` 要能区分它）
    Overflow,
}

// 标准库给 Error 的必需方法（source 等）都有默认实现，所以空 impl 就成立：
// 真正的前提是 ParseError 已经实现了 Debug + Display——Display 正是你要写的东西。
impl std::error::Error for ParseError {}

/// 【待实现】`ParseError` 的面向使用者的错误消息（对应 kp_10_05）。
///
/// 实现要求：
///   - `ParseError::Empty` 的消息里**必须包含「空」**，例如 `"输入为空，无法解析"`；
///   - `ParseError::NotNumber(text)` 的消息里**必须包含「不是数字」**，并且带上原文，
///     例如 `format!("不是数字：{text}")`；
///   - `Display` 是 `std::error::Error` 的父 trait：不实现它，上面的
///     `impl Error for ParseError` 根本无法通过编译；`Display`（`{}`）面向使用者，要写成
///     人能读懂的一句话，`Debug`（`{:?}`）则面向开发者。
///
/// 示例输入：
/// ```text
/// self = ParseError::NotNumber("abc")
/// ```
/// 示例输出：
/// ```text
/// "不是数字：abc"
/// ```
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display 面向使用者（`{}`）：Empty 说「空」，NotNumber 带上原文
        match self {
            ParseError::Empty => write!(f, "输入为空，无法解析"),
            ParseError::NotNumber(text) => write!(f, "不是数字：{text}"),
        }
    }
}

/// 【待实现】把 `ParseError` 上转成 `AppError`（对应 kp_10_06）。
///
/// 实现要求：
///   - 返回 `AppError::Parse(err)`，即把传入的 `ParseError` 原样包进 `AppError::Parse`；
///   - 有了这个 `From`，返回 `Result<_, AppError>` 的函数就能对 `Result<_, ParseError>`
///     直接用 `?`：编译器会自动插入一次 `From::from`，上层因此不必认识底层错误类型。
///
/// 示例输入：
/// ```text
/// err = ParseError::NotNumber("abc")
/// ```
/// 示例输出：
/// ```text
/// AppError::Parse(ParseError::NotNumber("abc"))
/// ```
impl From<ParseError> for AppError {
    fn from(err: ParseError) -> Self {
        // `?` 出错时会调用这里，把底层的 ParseError 原样包进 AppError::Parse
        Self::Parse(err)
    }
}

// ===========================================================================
// kp_10_01 可恢复错误：用 Result 表达「除数为 0」而不是 panic
// ===========================================================================

/// 知识点考核：用 `Result` 处理可预期的失败。
#[test]
fn kp_10_01_result_divide() {
    assess(
        M,
        "kp_10_01",
        "可恢复错误：用 Result 表达「除数为 0」，把决定权交给调用方",
        Kind::Basic,
        "复习 lesson_10 示例 3（result_and_match）与示例 10（when_to_panic_or_result）：\
         外部输入与可预期的运行结果用 Result 表达，只有「调用方违反了契约」才 panic。",
        || {
            // 正常用例：除数为非 0 时返回 Ok
            eq(
                ok(
                    exercise_10_01_divide(10, 2),
                    "10 / 2 是合法除法，必须返回 Ok(5)",
                ),
                5i32,
                "10 / 2 = 5",
            );

            // 边界：被除数为 0（0 / 5 是合法的，结果就是 0）
            eq(
                ok(
                    exercise_10_01_divide(0, 5),
                    "被除数为 0 不是错误，应返回 Ok(0)",
                ),
                0i32,
                "0 / 5 = 0：只有**除数**为 0 才是错误",
            );

            // 边界：负数的整数除法向零截断（-7 / 2 == -3，不是向下取整的 -4）
            eq(
                ok(exercise_10_01_divide(-7, 2), "负数除法也要返回 Ok"),
                -3i32,
                "Rust 的整数除法向零截断：-7 / 2 == -3（不是 -4）",
            );

            // 边界：除数为 0 → Err，且信息里必须含 "0"
            let message = err(
                exercise_10_01_divide(5, 0),
                "除数为 0 必须返回 Err，绝不能让程序 panic",
            );
            contains(
                &message,
                "0",
                "错误信息里必须提到 0，使用者才能立刻明白是「除数为 0」",
            );
        },
    );
}

/// 【待实现】可恢复的整数除法。
///
/// 实现要求：
///   - `b == 0` 时返回 `Err(String)`，错误信息里**必须包含 "0"**（例如 `"除数不能为 0"`）；
///   - 否则返回 `Ok(a / b)`；
///   - 整数除法**向零截断**：`-7 / 2 == -3`，`7 / -2 == -3`；重点是「可预期的失败用
///     `Result` 表达」，而不是 `panic!` 或 `unwrap()`。
///
/// 示例输入：
/// ```text
/// a = 10
/// b = 2
/// ```
/// 示例输出：
/// ```text
/// Ok(5)
/// ```
fn exercise_10_01_divide(a: i32, b: i32) -> Result<i32, String> {
    // 可预期的失败用 Result 表达：只有除数为 0 才是错误
    if b == 0 {
        Err(String::from("除数不能为 0"))
    } else {
        Ok(a / b)
    }
}

// ===========================================================================
// kp_10_02 unwrap 与 expect：成功路径直取，失败路径直接 panic
// ===========================================================================

/// 知识点考核：`unwrap` / `expect` 的语义与它们隐藏的 panic 风险。
#[test]
fn kp_10_02_unwrap_and_expect() {
    assess(
        M,
        "kp_10_02",
        "unwrap / expect：成功时取值，失败时 panic（风险就在这里）",
        Kind::Basic,
        "复习 lesson_10 示例 2（unwrap_and_expect）：unwrap 出错时用默认信息 panic，\
         expect(\"...\") 可以自带上下文；两者都只适合「已经论证过不会失败」的场合。",
        || {
            let (from_option, from_result) = exercise_10_02_unwrap_and_expect();

            // 正常用例 1：Option 上的 unwrap
            eq(from_option, 7i32, "Some(7).unwrap() 取出 7");

            // 正常用例 2：Result 上的 expect（成功路径与 unwrap 等价）
            eq(
                from_result,
                42i32,
                "Ok::<i32, String>(42).expect(\"不会失败\") 取出 42",
            );

            // 边界/风险用例：unwrap 遇到 Err 会立刻 panic，把进程/线程带走
            panics(
                || {
                    let _: i32 = "abc".parse().unwrap();
                },
                "\"abc\" 无法解析成 i32：unwrap 在 Err 上必然 panic，\
                 这正是它在库代码里被禁用的原因（改成 match / `?` / unwrap_or）",
            );
        },
    );
}

/// 【待实现】分别用 `unwrap` 与 `expect` 取出成功路径的值。
///
/// 实现要求：
///   - 用 `Some(7).unwrap()` 取出 `7`；
///   - 用 `Ok::<i32, String>(42).expect("不会失败")` 取出 `42`；
///   - 返回 `(7, 42)`；
///   - `Ok::<i32, String>(42)` 里的 turbofish 是必须的：不写编译器就不知道错误类型是谁；
///     真实项目里 `expect` 后面那句话要写清「为什么这里不会失败」。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (7, 42)
/// ```
fn exercise_10_02_unwrap_and_expect() -> (i32, i32) {
    // Option::unwrap：成功路径直取；Result::expect 可以带上「为什么不会失败」的上下文
    let from_option = Some(7).unwrap();
    let from_result = Ok::<i32, String>(42).expect("不会失败");
    (from_option, from_result)
}

// ===========================================================================
// kp_10_03 用 match 处理 Result：Ok 取值、Err 走另一条路
// ===========================================================================

/// 知识点考核：`match` 是处理 `Result` 最完整的写法。
#[test]
fn kp_10_03_match_result() {
    assess(
        M,
        "kp_10_03",
        "用 match 处理 Result：两个分支都必须处理，编译器强制你面对错误",
        Kind::Core,
        "复习 lesson_10 示例 3（result_and_match）：`match` 的 Ok / Err 两个分支必须都写，\
         失败时返回固定文案；`if let` 只适合只关心一侧的场景。",
        || {
            // 正常用例：前后空白要先 trim 再解析
            eq(
                exercise_10_03_match_result(" 42 "),
                String::from("数字：42"),
                "先 trim 再 parse：\" 42 \" 是合法数字，输出固定格式「数字：<n>」",
            );

            // 边界：负数也是合法数字
            eq(
                exercise_10_03_match_result("-7"),
                String::from("数字：-7"),
                "负数同样能解析成功：\"-7\" → 「数字：-7」（负号也要保留）",
            );

            // 边界：解析失败走 Err 分支
            eq(
                exercise_10_03_match_result("abc"),
                String::from("不是数字"),
                "解析失败时返回固定文案「不是数字」",
            );

            // 边界：空串同样走 Err 分支（不能 panic、也不能返回「数字：」）
            eq(
                exercise_10_03_match_result(""),
                String::from("不是数字"),
                "空串解析必然失败：应走 Err 分支返回「不是数字」，而不是 panic",
            );
        },
    );
}

/// 【待实现】用 `match` 把解析结果转成一句话。
///
/// 实现要求：
///   - 对 `text.trim().parse::<i32>()` 做 `match`；
///   - `Ok(n)` → `format!("数字：{n}")`（注意是全角冒号「：」）；
///   - `Err(_)` → `String::from("不是数字")`；
///   - `.parse::<i32>()` 的 turbofish 不能省，否则报 E0282 无法推断类型；`Err(_)` 表示
///     丢掉错误对象（真实项目里往往要记录它）。
///
/// 示例输入：
/// ```text
/// text = " 42 "
/// ```
/// 示例输出：
/// ```text
/// "数字：42"
/// ```
fn exercise_10_03_match_result(text: &str) -> String {
    // match 必须写全两个分支：Ok 取值组句，Err 走固定文案
    match text.trim().parse::<i32>() {
        Ok(number) => format!("数字：{number}"),
        Err(_) => String::from("不是数字"),
    }
}

// ===========================================================================
// kp_10_04 `?` 运算符：出错提前返回，成功继续往下走
// ===========================================================================

/// 知识点考核：在返回 `Result` 的函数里用 `?` 传播两次解析错误。
#[test]
fn kp_10_04_question_mark() {
    assess(
        M,
        "kp_10_04",
        "`?` 运算符：Ok 时取值继续执行，Err 时立刻 return Err(...)",
        Kind::Core,
        "复习 lesson_10 示例 4（question_mark_operator）：`?` 只能用在返回 Result / Option 的\
         函数里；两个 `?` 顺序传播，任何一个失败都会提前返回。",
        || {
            // 正常用例：两个数都合法
            eq(
                ok(
                    exercise_10_04_question_mark("3", "4"),
                    "两个合法数字应返回 Ok(和)",
                ),
                7i32,
                "\"3\" 与 \"4\" 用 `?` 解析成功后相加 = 7",
            );

            // 边界：负数与前后空白
            eq(
                ok(
                    exercise_10_04_question_mark(" -5 ", " 2 "),
                    "带空白的负数也要能解析",
                ),
                -3i32,
                "-5 + 2 = -3：两个入参都要先 trim 再 parse",
            );

            // 边界：第一个参数失败
            let first_bad = err(
                exercise_10_04_question_mark("", "1"),
                "空串解析失败：第一个 `?` 必须立刻返回 Err",
            );
            is_false(
                first_bad.is_empty(),
                "错误信息不能是空串：调用者需要知道是哪一个输入坏了",
            );

            // 边界：第二个参数失败（说明两个 `?` 都要写）
            let second_bad = err(
                exercise_10_04_question_mark("1", "x"),
                "第二个参数非法时同样要返回 Err，而不是返回 Ok(1)",
            );
            is_false(
                second_bad.is_empty(),
                "第二个 `?` 也不能漏：少了它就会把错误当成 0 继续算下去",
            );
        },
    );
}

/// 【待实现】用 `?` 解析两个数字并求和。
///
/// 实现要求：
///   - 两个入参都 `.trim()` 后 `.parse::<i32>()`，用 `?` 在失败时提前返回 `Err`；
///   - 由于 `ParseIntError` 不能自动变成 `String`，需要先 `.map_err(|err| err.to_string())`
///     （这正好说明：`?` 的自动转换要靠 `From`，而 `String` 并没有 `From<ParseIntError>`）；
///   - 成功时返回 `Ok(和)`；
///   - `?` 在 `Ok` 分支上会取出里面的值（`let a: i32 = ...?;` 之后 `a` 就是普通整数）；
///     函数返回类型若是 `()`，用 `?` 会得到 `error[E0277]`；
///   - `.parse()` 一定要写 turbofish（`.parse::<i32>()`）：中间夹了 `map_err` 之后，闭包
///     参数的类型推断不出来，会报 `error[E0282]: type annotations needed`。
///
/// 示例输入：
/// ```text
/// a = "3"
/// b = "4"
/// ```
/// 示例输出：
/// ```text
/// Ok(7)
/// ```
fn exercise_10_04_question_mark(a: &str, b: &str) -> Result<i32, String> {
    // `?` 在 Ok 上取值、在 Err 上提前 return；ParseIntError 不能自动变 String，先 map_err
    let a: i32 = a.trim().parse::<i32>().map_err(|err| err.to_string())?;
    let b: i32 = b.trim().parse::<i32>().map_err(|err| err.to_string())?;
    Ok(a + b)
}

// ===========================================================================
// kp_10_05 自定义错误类型：Display 给出面向使用者的消息
// ===========================================================================

/// 知识点考核：自定义错误的 `PartialEq` 语义与 `Display` 文案。
/// （`Display` 的实现在上面的「提供给你的类型」小节里。）
#[test]
fn kp_10_05_custom_error_display() {
    assess(
        M,
        "kp_10_05",
        "自定义错误类型：用枚举区分失败原因，用 Display 说人话",
        Kind::Core,
        "复习 lesson_10 示例 5（custom_error_type）：错误类型要同时实现 Display 与\
         std::error::Error；Display 给出「人能读懂的一句话」，Debug 给出结构化信息。",
        || {
            // 正常用例：合法输入（前后空白要 trim）
            eq(
                ok(exercise_10_05_parse(" 42 "), "合法整数应返回 Ok(42)"),
                42i32,
                "先 trim 再 parse：\" 42 \" → Ok(42)",
            );

            // 边界：空串 → Empty
            eq(
                err(
                    exercise_10_05_parse(""),
                    "空串必须返回 Err(ParseError::Empty)",
                ),
                ParseError::Empty,
                "空输入要落到 Empty 变体，而不是 NotNumber(\"\")",
            );

            // 边界：纯空白 → 同样属于 Empty（trim 之后什么都没有）
            eq(
                err(
                    exercise_10_05_parse("   \t"),
                    "纯空白输入也必须返回 Err(ParseError::Empty)",
                ),
                ParseError::Empty,
                "只有空白字符时 trim 之后为空：仍然是 Empty",
            );

            // 边界：负数也是合法输入
            eq(
                ok(exercise_10_05_parse("-7"), "负数是合法整数，应返回 Ok(-7)"),
                -7i32,
                "\"-7\" → Ok(-7)",
            );

            // 非法输入：必须保留**原始输入**，且 Display 文案含「不是数字」
            let error = err(
                exercise_10_05_parse("abc"),
                "非数字输入必须返回 Err(ParseError::NotNumber(..))",
            );
            contains(
                &error.to_string(),
                "不是数字",
                "NotNumber 的 Display 消息里必须包含「不是数字」，使用者才知道哪里不对",
            );
            eq(
                error,
                ParseError::NotNumber(String::from("abc")),
                "NotNumber 里必须保留原始输入原文（\"abc\"），出错时才能定位",
            );

            // Empty 的 Display 文案含「空」
            contains(
                &ParseError::Empty.to_string(),
                "空",
                "Empty 的 Display 消息里必须包含「空」（例如「输入为空」）",
            );
        },
    );
}

/// 【待实现】解析整数，失败时给出**带变体信息**的自定义错误。
///
/// 实现要求：
///   - `text.trim()` 为空（空串或纯空白）→ `Err(ParseError::Empty)`；
///   - `text.trim().parse::<i32>()` 失败 → `Err(ParseError::NotNumber(原文.to_string()))`，
///     注意括号里存的是**原始输入**（可以直接用 `text.to_string()`，不 trim 也可以）；
///   - 成功 → `Ok(n)`；
///   - `ParseError` 的 `Display` 实现在上面的「提供给你的类型」小节里，这里不要再写一个。
///
/// 示例输入：
/// ```text
/// text = " 42 "
/// ```
/// 示例输出：
/// ```text
/// Ok(42)
/// ```
fn exercise_10_05_parse(text: &str) -> Result<i32, ParseError> {
    // 空 / 纯空白 → Empty；解析失败 → NotNumber（保留原始输入）；成功 → Ok(n)
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    match trimmed.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(ParseError::NotNumber(text.to_string())),
    }
}

// ===========================================================================
// kp_10_06 From 上转：`?` 如何把 ParseError 自动变成 AppError
// ===========================================================================

/// 知识点考核：`?` 借助 `From` 完成错误类型上转。
/// （`impl From<ParseError> for AppError` 在上面的「提供给你的类型」小节里。）
#[test]
fn kp_10_06_from_conversion() {
    assess(
        M,
        "kp_10_06",
        "From 上转：`?` 自动把 ParseError 转换成 AppError",
        Kind::Hard,
        "复习 lesson_10 示例 6（from_conversion_via_question_mark）：`expr?` 在出错时会做\
         `From::from(err)`，把底层错误转成函数返回类型的错误类型。",
        || {
            // 正常用例：合法输入
            eq(
                ok(
                    exercise_10_06_from_conversion("  7 "),
                    "合法输入应返回 Ok(7)",
                ),
                7i32,
                "先 trim 再 parse：\"  7 \" → Ok(7)",
            );

            // 边界：空输入 → 先得到 ParseError::Empty，再被 `?` 自动上转成 AppError::Parse(..)
            eq(
                err(
                    exercise_10_06_from_conversion("  "),
                    "空输入必须返回 Err(AppError::Parse(ParseError::Empty))",
                ),
                AppError::Parse(ParseError::Empty),
                "`?` 触发的 From 上转：ParseError::Empty 应被包进 AppError::Parse",
            );

            // 边界：非法输入，且必须保留原文
            eq(
                err(
                    exercise_10_06_from_conversion("abc"),
                    "非法输入必须返回 Err(AppError::Parse(ParseError::NotNumber(..)))",
                ),
                AppError::Parse(ParseError::NotNumber(String::from("abc"))),
                "上转不能丢信息：NotNumber 里的原文 \"abc\" 必须一路保留到 AppError",
            );

            // 显式构造 Overflow 变体，确认 PartialEq 能把两个变体区分开
            is_true(
                AppError::Overflow != AppError::Parse(ParseError::Empty),
                "AppError 的 Overflow 与 Parse 是两个不同的变体：PartialEq 必须能把它们区分开",
            );
        },
    );
}

/// 【待实现】用 `?` 把 `ParseError` 自动上转成 `AppError`。
///
/// 实现要求：
///   - 复用 kp_10_05 的 `exercise_10_05_parse(text)?`（`?` 会调用你写的 `From` 实现）；
///   - 成功时返回 `Ok(n)`；
///   - `From<ParseError> for AppError` 的实现在上面的「提供给你的类型」小节里；那个 impl
///     还空着时，这里的 `?` 会报 `error[E0277]`（`From<ParseError>` is not implemented）。
///
/// 示例输入：
/// ```text
/// text = "  7 "
/// ```
/// 示例输出：
/// ```text
/// Ok(7)
/// ```
fn exercise_10_06_from_conversion(text: &str) -> Result<i32, AppError> {
    // `?` 会用我们写的 From 把 ParseError 自动上转成 AppError::Parse
    let number = exercise_10_05_parse(text)?;
    Ok(number)
}

// ===========================================================================
// kp_10_07 Box<dyn Error>：统一错误出口与自动装箱
// ===========================================================================

/// 知识点考核：多种底层错误塞进同一个 `Box<dyn Error>` 出口。
#[test]
fn kp_10_07_box_dyn_error() {
    assess(
        M,
        "kp_10_07",
        "Box<dyn Error> 统一出口：`?` 把 ParseIntError 自动装箱",
        Kind::Core,
        "复习 lesson_10 示例 7（box_dyn_error）：标准库已提供 `impl From<E> for Box<dyn Error>`\
         （E: Error + 'static），所以 `?` 能直接把 ParseIntError 装箱后返回。",
        || {
            // 正常用例：返回的是**trim 之后**的长度（" 42 " → "42" → 2）
            eq(
                ok(
                    exercise_10_07_box_dyn_error(" 42 "),
                    "合法输入应返回 Ok(长度)",
                ),
                2usize,
                "返回 trim 之后的字节长度：\" 42 \" → 2（不是 4）",
            );

            // 正常用例：单个数字
            eq(
                ok(exercise_10_07_box_dyn_error("7"), "合法输入应返回 Ok(长度)"),
                1usize,
                "\"7\" 的长度是 1",
            );

            // 边界：纯空白 trim 之后解析失败 → Err（装箱后的 ParseIntError）
            let blank = err(
                exercise_10_07_box_dyn_error("   "),
                "纯空白 trim 之后无法解析成 i32，必须返回 Err 而不是 panic",
            );
            is_false(
                blank.to_string().is_empty(),
                "Box<dyn Error> 要能通过 Display 打印出底层错误消息：不能是空串",
            );

            // 边界：非数字输入 → Err
            let bad = err(
                exercise_10_07_box_dyn_error("abc"),
                "非数字输入必须返回 Err（ParseIntError 经 From 装箱）",
            );
            is_false(
                bad.to_string().is_empty(),
                "装箱之后底层错误的消息仍然可读，这就是 dyn Error 的用处",
            );
        },
    );
}

/// 【待实现】用 `Box<dyn Error>` 作为统一错误出口。
///
/// 实现要求：
///   - 先 `text.trim().parse::<i32>()?`——`ParseIntError` 会通过 `From` 自动装箱成
///     `Box<dyn Error>`（这一步不用你手写任何转换代码）；
///   - 再返回 `Ok(text.trim().len())`（注意是 **trim 之后**的长度）；
///   - `Box<dyn Error>` 等价于 `Box<dyn Error + 'static>`，代价是失去静态类型信息：要按
///     类型分支处理得用 `downcast_ref::<具体类型>()`（见课程示例 7）。
///
/// 示例输入：
/// ```text
/// text = " 42 "
/// ```
/// 示例输出：
/// ```text
/// Ok(2)
/// ```
fn exercise_10_07_box_dyn_error(text: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // `?` 直接用标准库的 From<E> for Box<dyn Error>，把 ParseIntError 自动装箱
    let _number: i32 = text.trim().parse::<i32>()?;
    Ok(text.trim().len())
}

// ===========================================================================
// kp_10_08 典型场景：解析 key=value 配置，跳过空行与注释
// ===========================================================================

/// 知识点考核：多行文本解析——跳过空行与注释、坏行报错且信息可定位。
#[test]
fn kp_10_08_parse_config() {
    assess(
        M,
        "kp_10_08",
        "典型场景：解析 key=value 配置，跳过空行与 # 注释行，坏行错误信息保留原文",
        Kind::Hard,
        "复习 lesson_10 示例 8（config_parsing_scenario）：全有或全无地解析配置，\
         空行跳过；出错时把「哪一行坏了」写进错误信息，方便使用者直接定位。",
        || {
            // 正常用例：两行配置，顺序与输入一致，key / value 都要 trim
            let pairs = ok(
                exercise_10_08_parse_config("port = 8080\ntimeout = 2.5"),
                "合法配置必须返回 Ok，而不是 panic",
            );
            eq_slice(
                &pairs,
                &[
                    (String::from("port"), String::from("8080")),
                    (String::from("timeout"), String::from("2.5")),
                ],
                "每行按 `=` 拆成 (key, value)，两侧空白都要 trim：\
                 \"port = 8080\" → (\"port\", \"8080\")",
            );

            // 边界：空输入 → Ok(vec![])，而不是 Err
            let empty = ok(
                exercise_10_08_parse_config(""),
                "空输入不是错误：必须返回 Ok(vec![])，不能 panic",
            );
            eq(
                empty.len(),
                0usize,
                "空输入应得到 0 个配置项（Result 的 Ok 分支里放空 Vec）",
            );

            // 边界：空行与以 # 开头的注释行都要跳过
            let cleaned = ok(
                exercise_10_08_parse_config("# 注释行\n\n   \n  a  =  1  \n"),
                "空行与 # 注释行应当被跳过，而不是被当成坏行报错",
            );
            eq_slice(
                &cleaned,
                &[(String::from("a"), String::from("1"))],
                "只有 `a = 1` 是有效配置：多余的首尾空白要 trim 掉，注释与空行跳过",
            );

            // 边界：缺少 `=` 的行必须报错，并且错误信息里保留该行原文
            let message = err(
                exercise_10_08_parse_config("port = 8080\nhost 127.0.0.1"),
                "缺少 `=` 的行必须返回 Err，不能悄悄忽略掉",
            );
            contains(
                &message,
                "host 127.0.0.1",
                "错误信息里必须保留出错的整行内容，使用者才能定位到哪一行",
            );
        },
    );
}

/// 【待实现】把多行 `key=value` 文本解析成有序的键值对列表。
///
/// 实现要求：
///   - 用 `text.lines()` 逐行处理；
///   - 每行先 `trim()`：为空的行**跳过**，以 `#` 开头的行（注释）也**跳过**；
///   - 用 `split_once('=')` 切分：返回 `None` 说明这一行没有 `=` → 返回 `Err(String)`，
///     错误信息里**必须包含该行原文**（例如 `format!("配置行缺少 = 分隔符：{line}")`）；
///   - key 与 value 都 `trim()` 之后作为 `(String, String)` 依次 push 进 `Vec`（顺序与输入一致）；
///   - 空输入返回 `Ok(Vec::new())`；
///   - 返回 `Vec` 而不是 `HashMap`，是为了让顺序可断言（课程示例 8 的 `HashMap` 必须排序
///     才能得到确定输出）。
///
/// 示例输入：
/// ```text
/// text = "port = 8080\ntimeout = 2.5"
/// ```
/// 示例输出：
/// ```text
/// Ok([("port", "8080"), ("timeout", "2.5")])
/// ```
fn exercise_10_08_parse_config(text: &str) -> Result<Vec<(String, String)>, String> {
    // 全有或全无：空行与 # 注释行跳过，坏行立刻返回带原文的错误
    let mut pairs: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.split_once('=') {
            Some((key, value)) => pairs.push((key.trim().to_string(), value.trim().to_string())),
            None => return Err(format!("配置行缺少 = 分隔符：{line}")),
        }
    }
    Ok(pairs)
}

// ===========================================================================
// kp_10_09 常见错误诊断：`?` 相关的错误编号
// ===========================================================================

/// 知识点考核：读懂课程示例 9 的注释，写出前三个错误的编号。
#[test]
fn kp_10_09_diagnose_errors() {
    assess(
        M,
        "kp_10_09",
        "常见错误诊断：`?` 用错位置、对 Option 用 `?`、缺 From 分别报什么错",
        Kind::Hard,
        "复习 lesson_10 示例 9（common_mistakes）：`?` 相关的错误都落在 E0277\
         （trait 约束不满足）这一族里，读错误信息时要顺着 note: 找到缺失的 trait。",
        || {
            eq_slice(
                &exercise_10_09_diagnose_errors(),
                &["E0277", "E0277", "E0277"],
                "按课程示例 9 的注释顺序，错误 1 / 2 / 3 的编号都是 E0277：\
                 「函数没返回 Result / Option」「对 Option 用 ?」「错误类型缺 From」，\
                 最终都表现为 trait 约束不满足",
            );
        },
    );
}

/// 【待实现】写出课程示例 9 里前三个错误的编译器错误编号。
///
/// 场景（与课程示例 9 的注释一一对应，顺序也一致）：
///   1. `fn helper() { let value: i32 = "1".parse::<i32>()?; }` —— 在返回 `()` 的函数里用 `?`；
///   2. `fn helper() -> Result<(), Box<dyn Error>> { let v = Some(1)?; Ok(()) }` ——
///      在返回 `Result` 的函数里对 `Option` 用 `?`；
///   3. `fn helper() -> Result<(), MyErr> { let n: i32 = "1".parse::<i32>()?; Ok(()) }` ——
///      `?` 的源错误类型无法转换成函数的错误类型（`MyErr` 缺 `From<ParseIntError>`）。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0277"`），顺序与上面一致；
///   - 三个场景看起来不同，但编译器给出的都是同一个编号：错误信息里的 `note:` 会告诉你
///     缺少哪个 trait 实现，想确认可以把这三段代码逐段编译一次。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0277", "E0277", "E0277"]
/// ```
fn exercise_10_09_diagnose_errors() -> [&'static str; 3] {
    // 三个场景都表现为「trait 约束不满足」，编译器给的编号都是 E0277
    ["E0277", "E0277", "E0277"]
}
