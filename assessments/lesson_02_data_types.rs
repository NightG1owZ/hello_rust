//! assessments/lesson_02_data_types.rs —— 考核：数据类型（对应 lesson_02）
//!
//! - 对应课程：`src/tutorial/lesson_02_data_types.rs`
//! - 知识点出处：`src/tutorial/README.md` 第一阶段「02 数据类型」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_02_data_types     # 只考这一课
//!   cargo test                                 # 考全部 18 课
//!   cargo run --bin assessment_report          # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_02_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_02_data_types`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 8）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `let x: u8 = 255; let y = x + 1;` | 编译期 `this arithmetic operation will overflow`；运行期 panic（无 E 编号） | 换更大的类型，或改用 `checked_add` / `wrapping_add` / `saturating_add` |
//! | `let arr = [1, 2, 3]; arr[3]` | 常量下标：`this operation will panic at runtime`；变量下标：运行期 `index out of bounds`（无 E 编号） | 用 `arr.get(3)` 拿 `Option`，或先判断 `index < arr.len()` |
//! | `let port: u32 = 8080i32;` | `error[E0308]` mismatched types | 显式转换：`as`（已确认不溢出）或 `u32::try_from(...)` |
//! | `let c: char = "A";` | `error[E0308]` mismatched types | 单字符用单引号 `'A'`，字符串要取字符用 `.chars()` |
//! | `if 0.1 + 0.2 == 0.3 { ... }` | 能编译，但条件恒为 false（逻辑错误） | 比较差值：`(a - b).abs() < 1e-9` |
//! | `let n: i32 = 1.0;` | `error[E0308]` mismatched types | 整数写 `1`；确实要截断写 `1.0 as i32` |
//! | `let t = (1, 2); t.2` | `error[E0609]` no field `2` on type | 元组下标从 `.0` 开始且元素个数固定；字段多时改用结构体（lesson 06） |

use assessment_harness::{Kind, approx, assess, eq, is_false, is_true, none, some};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_02";

// ===========================================================================
// kp_02_01 整数类型与字面量后缀：位宽决定取值范围，后缀决定类型
// ===========================================================================

/// 知识点考核：用字面量与类型后缀写出四种整型的极值/大数。
#[test]
fn kp_02_01_integer_literals() {
    assess(
        M,
        "kp_02_01",
        "整数类型与字面量：位宽决定取值范围，字面量用后缀指定类型",
        Kind::Basic,
        "复习 lesson_02 示例 1（integer_types）：整数默认是 i32；u8 的范围是 0..=255，\
         i16 的范围是 -32768..=32767；下划线只是可读性分隔符，后缀（u8/i16/u32/u64）才决定类型。",
        || {
            let (max_u8, min_i16, million, big) = exercise_02_01_integer_literals();

            // 正常用例：u8 的最大值由位宽决定，是 255（不是 256）
            eq(
                max_u8,
                255u8,
                "u8 是无符号 8 位整数，取值范围 0..=255，最大值是 255",
            );
            // 正常用例：i16 的最小值是 -32768（不是 -32767）
            eq(
                min_i16,
                -32768i16,
                "i16 是有符号 16 位整数，最小值是 -32768（补码表示）",
            );
            // 正常用例：下划线分隔符不影响数值大小
            eq(
                million,
                1_000_000u32,
                "1_000_000 的下划线只是可读性分隔符，数值就是 1000000",
            );
            // 边界用例：这个值远超 i32::MAX，必须用 u64 才装得下
            eq(
                big,
                9_000_000_000u64,
                "90 亿超出了 i32::MAX（2147483647），必须写成 u64 后缀才能装下",
            );
        },
    );
}

/// 【待实现】用字面量加类型后缀写出四个整数。
///
/// 实现要求：
///   - 第一个：`u8::MAX`，写成 `255u8`；
///   - 第二个：`i16::MIN`，写成 `-32768i16`；
///   - 第三个：`1_000_000u32`（下划线只是可读性分隔符）；
///   - 第四个：`9_000_000_000u64`（超出 i32 范围，必须用 u64）；
///   - 按上面的顺序返回 `(u8, i16, u32, u64)`，即 `(255, -32768, 1000000, 9000000000)`；
///   - 整数类型分有符号 `i8/i16/i32/i64/i128/isize` 与无符号
///     `u8/u16/u32/u64/u128/usize`，位宽即取值范围；不写后缀时默认推断为 `i32`，
///     超范围的字面量会报 `error: literal out of range for i32`，所以四个字面量都要带后缀；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (255, -32768, 1000000, 9000000000)
/// ```
fn exercise_02_01_integer_literals() -> (u8, i16, u32, u64) {
    assessment_harness::todo_exercise(
        "exercise_02_01_integer_literals",
        "返回 (255u8, -32768i16, 1_000_000u32, 9_000_000_000u64)",
        (),
    )
}

// ===========================================================================
// kp_02_02 整数溢出：wrapping / saturating / overflowing / checked 四选一
// ===========================================================================

/// 知识点考核：`250u8 + 10` 的四种溢出处理方式。
#[test]
fn kp_02_02_overflow_handling() {
    assess(
        M,
        "kp_02_02",
        "整数溢出：wrapping_add / saturating_add / overflowing_add / checked_add 的区别",
        Kind::Edge,
        "复习 lesson_02 示例 8 的错误 1（common_mistakes）：debug 构建下 `250u8 + 10` \
         会直接 panic（attempt to add with overflow），必须改用四个显式方法之一；\
         示例 1 里的 `u8::MAX` 是理解「溢出边界」的起点。",
        || {
            let (wrapped, saturated, overflowed, overflow_checked) =
                exercise_02_02_overflow_handling();

            // 正常用例：wrapping 按 2^8 取模，只有低 8 位留下来 → 260 - 256 = 4
            eq(
                wrapped,
                4u8,
                "wrapping_add 按位宽环绕：250 + 10 = 260，丢掉进位后是 260 - 256 = 4",
            );
            // 边界用例：saturating 停在类型最大值，不再往上走 → 255
            eq(
                saturated,
                255u8,
                "saturating_add 溢出时停在 u8::MAX，所以是 255 而不是 260",
            );
            // 正常用例：overflowing_add().0 与 wrapping 结果一致
            eq(
                overflowed,
                4u8,
                "overflowing_add 返回 (环绕后的值, 是否溢出)，取 .0 就是 4",
            );
            // 边界用例：checked_add 溢出时返回 None
            is_true(
                overflow_checked,
                "checked_add 在 250 + 10 溢出时返回 None，所以「检测到溢出吗」这一项应为 true",
            );
        },
    );
}

/// 【待实现】用四种方法处理 `250u8 + 10` 的溢出。
///
/// 实现要求：
///   - 先绑定 `let base: u8 = 250;`；
///   - `wrapping_add(10)` 的结果作为第 1 个返回值（环绕）；
///   - `saturating_add(10)` 的结果作为第 2 个返回值（饱和到最大值）；
///   - `overflowing_add(10).0` 的结果作为第 3 个返回值（环绕值 + 溢出标志）；
///   - `checked_add(10).is_none()` 的结果作为第 4 个返回值（是否溢出）；
///   - 返回 `(u8, u8, u8, bool)`，即 `(4, 255, 4, true)`；
///   - `250u8 + 10` 在 debug 构建下会 panic（`attempt to add with overflow`），
///     所以必须用这四个方法显式表达「溢出时怎么办」；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (4, 255, 4, true)
/// ```
fn exercise_02_02_overflow_handling() -> (u8, u8, u8, bool) {
    assessment_harness::todo_exercise(
        "exercise_02_02_overflow_handling",
        "以 250u8 加 10，返回 (wrapping_add, saturating_add, overflowing_add().0, checked_add().is_none())",
        (),
    )
}

// ===========================================================================
// kp_02_03 浮点类型：精度误差与 NaN 的「不等于自身」
// ===========================================================================

/// 知识点考核：浮点除法、二进制浮点误差与 NaN 的比较语义。
#[test]
fn kp_02_03_float_basics() {
    assess(
        M,
        "kp_02_03",
        "浮点基础：默认 f64、0.1 + 0.2 != 0.3、NaN 不等于任何值（包括自身）",
        Kind::Basic,
        "复习 lesson_02 示例 2（float_types）与示例 8 的错误 5：浮点数不能用 `==` \
         判断「相等」，要比差值 `(a - b).abs() < 1e-9`；f64 的 NaN 与任何值比较都是 false。",
        || {
            let (quotient, sum, sum_equals, nan_equals) = exercise_02_03_float_basics();

            // 正常用例：7.0 / 2.0 是浮点除法，不截断，结果正好 3.5
            approx(quotient, 3.5, 1e-12, "7.0 / 2.0 是浮点除法，结果是 3.5");
            // 边界用例：二进制浮点无法精确表示 0.1 与 0.2，用近似比较而不是 ==
            approx(
                sum,
                0.3,
                1e-12,
                "0.1 + 0.2 的实际值是 0.30000000000000004，与 0.3 的差在 1e-12 之内",
            );
            // 正常用例：正因为有误差，直接用 == 比较必须得到 false
            is_false(
                sum_equals,
                "(0.1 + 0.2) == 0.3 是浮点误差的经典陷阱，结果必须是 false",
            );
            // 边界用例：NaN 连自己都不等于，所以这一项也必须是 false
            is_false(
                nan_equals,
                "IEEE 754 规定 NaN 与任何值（包括 NaN 自身）比较都是 false",
            );
        },
    );
}

/// 【待实现】返回浮点运算的结果与比较结论。
///
/// 实现要求：
///   - 第 1 项：`7.0 / 2.0`（浮点除法，结果是 3.5）；
///   - 第 2 项：`0.1 + 0.2`（结果是 0.30000000000000004）；
///   - 第 3 项：`(0.1f64 + 0.2f64) == 0.3`（浮点误差导致它必须是 false）；
///   - 第 4 项：`f64::NAN == f64::NAN`（必须是 false）；
///   - 返回 `(f64, f64, bool, bool)`；
///   - 浮点字面量默认推断为 `f64`，但**先绑定到变量再调用方法**时可能无法推断，
///     报 `error[E0689]: can't call method ... on ambiguous numeric type`，
///     这时要显式写 `let sum: f64 = 0.1 + 0.2;`；
///   - NaN 永远不会等于自身，这是 IEEE 754 的规定，不是编译器 bug；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (3.5, 0.30000000000000004, false, false)
/// ```
fn exercise_02_03_float_basics() -> (f64, f64, bool, bool) {
    assessment_harness::todo_exercise(
        "exercise_02_03_float_basics",
        "返回 (7.0 / 2.0, 0.1 + 0.2, (0.1 + 0.2) == 0.3, f64::NAN == f64::NAN)",
        (),
    )
}

// ===========================================================================
// kp_02_04 布尔与字符：char 是 4 字节的 Unicode 标量值
// ===========================================================================

/// 知识点考核：布尔逻辑运算与字符的码点、UTF-8 字节长度。
#[test]
fn kp_02_04_bool_and_char() {
    assess(
        M,
        "kp_02_04",
        "布尔与字符：bool 只有 true/false，char 是 4 字节的 Unicode 标量值",
        Kind::Basic,
        "复习 lesson_02 示例 3（bool_and_char_literals）：char 用单引号书写、占 4 字节；\
         `as u32` 得到 Unicode 码点，`len_utf8()` 得到这个字符在 UTF-8 里占几个字节。",
        || {
            let (logic, ch, code_point, utf8_len) = exercise_02_04_bool_and_char();

            // 正常用例：true && !false 就是 true && true
            eq(
                logic,
                true,
                "true && !false 等价于 true && true，结果是 true",
            );
            // 正常用例：字符字面量用单引号
            eq(ch, '中', "字符字面量必须用单引号：'中'（双引号是字符串）");
            // 边界用例：多字节字符的码点是一个固定的十进制数
            eq(
                code_point,
                20013u32,
                "'中' 的 Unicode 码点是 U+4E2D，即十进制的 20013",
            );
            // 边界用例：码点与 UTF-8 字节数是两回事，别混淆
            eq(
                utf8_len,
                3usize,
                "len_utf8() 返回该字符在 UTF-8 里占的字节数：'中' 占 3 字节（char 本身仍是 4 字节）",
            );
        },
    );
}

/// 【待实现】返回布尔运算与字符的码点、UTF-8 长度。
///
/// 实现要求：
///   - 第 1 项：`true && !false`；
///   - 第 2 项：字符 `'中'`；
///   - 第 3 项：`'中' as u32`（Unicode 码点，十进制 20013）；
///   - 第 4 项：`'中'.len_utf8()`（这个字符在 UTF-8 里的字节数，3）；
///   - 返回 `(bool, char, u32, usize)`；
///   - `char` 是 Unicode 标量值，固定占 4 字节（`size_of::<char>()` 恒为 4）；
///     但同一个字符在 UTF-8 字符串里可能占 1~4 字节，`len_utf8()` 问的是后者；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (true, '中', 20013, 3)
/// ```
fn exercise_02_04_bool_and_char() -> (bool, char, u32, usize) {
    assessment_harness::todo_exercise(
        "exercise_02_04_bool_and_char",
        "返回 (true && !false, '中', '中' as u32, '中'.len_utf8())",
        (),
    )
}

// ===========================================================================
// kp_02_05 元组解构：一次拆成多个绑定
// ===========================================================================

/// 知识点考核：用 `let (a, b, c) = ...` 解构元组并做运算。
#[test]
fn kp_02_05_tuple_destructure() {
    assess(
        M,
        "kp_02_05",
        "元组解构：一次把元组拆成多个变量，再参与运算",
        Kind::Core,
        "复习 lesson_02 示例 4（tuples）：解构是 `let (a, b, c) = 元组;`；\
         也可以用 `.0/.1/.2` 按位置访问，两种写法等价，但解构更适合「一次拿到全部字段」。",
        || {
            let (sum, product, difference) = exercise_02_05_tuple_destructure();

            // 正常用例：3 + (-5) + 8 = 6，注意负号的参与
            eq(
                sum,
                6i32,
                "解构出 (a, b, c) = (3, -5, 8) 后求和：3 + (-5) + 8 = 6",
            );
            // 正常用例：3 * (-5) = -15
            eq(
                product,
                -15i32,
                "a * b = 3 * (-5) = -15，负数的乘法不要漏掉负号",
            );
            // 边界用例：c - a = 8 - 3 = 5，注意顺序不能反（反了是 -5）
            eq(
                difference,
                5i32,
                "c - a = 8 - 3 = 5；减法有方向，写反了会得到 -5",
            );
        },
    );
}

/// 【待实现】解构元组并返回三种运算结果。
///
/// 实现要求：
///   - 写 `let (a, b, c) = (3, -5, 8);` 一次解构出三个绑定；
///   - 返回 `(a + b + c, a * b, c - a)`，即 `(6, -15, 5)`；
///   - 本题要求用**解构**的写法，不能改成 `.0/.1/.2` 下标访问；
///   - 解构时左右两侧的元素个数必须一致，否则报
///     `error[E0308]: mismatched types`（expected a tuple with 3 elements）；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (6, -15, 5)
/// ```
fn exercise_02_05_tuple_destructure() -> (i32, i32, i32) {
    assessment_harness::todo_exercise(
        "exercise_02_05_tuple_destructure",
        "用 let (a, b, c) = (3, -5, 8); 解构，返回 (a + b + c, a * b, c - a)",
        (),
    )
}

// ===========================================================================
// kp_02_06 数组安全访问：get 返回 Option，索引越界会 panic
// ===========================================================================

/// 知识点考核：用 `.get(index).copied()` 安全访问数组元素。
#[test]
fn kp_02_06_array_get() {
    assess(
        M,
        "kp_02_06",
        "数组安全访问：get 返回 Option，比 [] 索引更安全（越界不 panic）",
        Kind::Core,
        "复习 lesson_02 示例 5（arrays）与示例 8 的错误 2：`数组[index]` 越界会 panic\
         （index out of bounds），而 `数组.get(index)` 返回 `Option`，越界只是 `None`。",
        || {
            // 正常用例：首元素
            eq(
                some(
                    exercise_02_06_array_get(0),
                    "下标 0 一定在 [10, 20, 30] 范围内，应返回 Some(10)",
                ),
                10i32,
                "数组 [10, 20, 30] 的 get(0) 是 Some(10)",
            );
            // 正常用例：末元素（下标 2 是最后一个合法下标）
            eq(
                some(
                    exercise_02_06_array_get(2),
                    "下标 2 是最后一个合法下标，应返回 Some(30)",
                ),
                30i32,
                "数组长度 3，最后一个合法下标是 2，get(2) 是 Some(30)",
            );
            // 边界用例：刚好越界的第一个下标
            none(
                exercise_02_06_array_get(3),
                "下标 3 越界（合法下标只有 0/1/2），get 应返回 None 而不是 panic",
            );
            // 边界用例：usize 的最大值，验证实现不能有 «下标 + 1» 这类越界写法
            none(
                exercise_02_06_array_get(usize::MAX),
                "usize::MAX 是越界的极值，get 仍应返回 None；若用 [] 索引这里会 panic",
            );
        },
    );
}

/// 【待实现】安全访问固定数组的某个下标。
///
/// 实现要求：
///   - 定义一个数组 `let data = [10, 20, 30];`；
///   - 用 `data.get(index).copied()` 返回 `Option<i32>`；
///   - 下标 0/1/2 返回 `Some(元素)`，下标 >= 3 返回 `None`，**任何情况下都不能 panic**；
///   - `.get()` 返回的是 `Option<&i32>`，加上 `.copied()` 才得到 `Option<i32>`；
///   - **不要**写 `data[index]`——越界索引会直接 panic；
///   - 本练习只有一个参数 `index`（`usize` 下标）。
///
/// 示例输入：
/// ```text
/// index = 0
/// ```
/// 示例输出：
/// ```text
/// Some(10)
/// ```
fn exercise_02_06_array_get(index: usize) -> Option<i32> {
    assessment_harness::todo_exercise(
        "exercise_02_06_array_get",
        "对 [10, 20, 30] 用 .get(index).copied() 安全取值，越界返回 None",
        (index,),
    )
}

// ===========================================================================
// kp_02_07 类型转换：as 截断 vs TryFrom 范围检查
// ===========================================================================

/// 知识点考核：`as` 静默截断与 `u8::try_from` 的范围检查。
#[test]
fn kp_02_07_conversions() {
    assess(
        M,
        "kp_02_07",
        "类型转换：`as` 静默截断，`u8::try_from` 做范围检查并返回 Result",
        Kind::Core,
        "复习 lesson_02 示例 6（type_conversion）：`300i32 as u8` 得到 44（二进制截断）；\
         `u8::try_from(300)` 得到 `Err`；`From` 只用于「一定成功」的加宽转换，如 `u32::from(1000u16)`。",
        || {
            // 正常用例：值在 u8 范围内，截断与检查的结果一致
            let (truncated, narrowed) = exercise_02_07_conversions(200);
            eq(
                truncated,
                200u32,
                "200 转 u32 没有任何信息丢失，结果还是 200",
            );
            eq(
                some(narrowed, "200 在 u8 范围内，try_from 必须成功"),
                200u8,
                "200 在 0..=255 内，u8::try_from(200) 是 Ok(200)",
            );

            // 边界用例：u8::MAX 是最后一个能转换成功的值
            let (truncated, narrowed) = exercise_02_07_conversions(255);
            eq(
                truncated,
                255u32,
                "255 恰好是 u8::MAX，转 u32 仍是 255（边界值本身合法）",
            );
            eq(
                some(narrowed, "255 恰好是 u8::MAX，仍在合法范围内"),
                255u8,
                "u8::MAX 本身是合法值，u8::try_from(255) 是 Ok(255)",
            );

            // 边界用例：刚好越界的第一个值，as 截断而 try_from 失败
            let (truncated, narrowed) = exercise_02_07_conversions(256);
            eq(
                truncated,
                256u32,
                "256 转 u32 不丢信息；截断只发生在「转成更窄的类型」时（这里是转 u8）",
            );
            none(
                narrowed,
                "256 超出 u8 上限 255，try_from 必须失败（返回 None），而 as 会静默截断成 0",
            );

            // 边界用例：u64 的极值，低 32 位全是 1
            let (truncated, narrowed) = exercise_02_07_conversions(u64::MAX);
            eq(
                truncated,
                4_294_967_295u32,
                "u64::MAX 的低 32 位全是 1，`as u32` 截断后得到 u32::MAX = 4294967295",
            );
            none(
                narrowed,
                "u64::MAX 远大于 255，try_from 必然失败；这就是 as 与 TryFrom 的关键差别",
            );
        },
    );
}

/// 【待实现】用两种方式把一个 u64 转成更窄的类型。
///
/// 实现要求：
///   - 第 1 项：用 `as` 把 `value` 截断成 `u32`（不做任何检查，只保留低 32 位）；
///   - 第 2 项：用 `u8::try_from(value).ok()` 做范围检查，成功是 `Some`、失败是 `None`；
///   - 返回 `(u32, Option<u8>)`；
///   - `as` 是**无检查**的强制转换，超范围时静默截断（`u64::MAX as u32` = 4294967295），
///     所以只在你能证明不会溢出时才用它；
///   - `TryFrom` 用于「可能失败」的收窄转换，失败信息是 `TryFromIntError`，用 `.ok()`
///     转成 `Option`；而 `From` 只覆盖一定成功的加宽转换（如 `u32::from(1000u16)`），
///     本函数用不到；
///   - 本练习只有一个参数 `value`（`u64`）。
///
/// 示例输入：
/// ```text
/// value = 200
/// ```
/// 示例输出：
/// ```text
/// (200, Some(200))
/// ```
fn exercise_02_07_conversions(value: u64) -> (u32, Option<u8>) {
    assessment_harness::todo_exercise(
        "exercise_02_07_conversions",
        "返回 (value as u32, u8::try_from(value).ok())",
        (value,),
    )
}

// ===========================================================================
// kp_02_08 元组比较与数组长度：字典序 vs 类型的一部分
// ===========================================================================

/// 知识点考核：元组的字典序比较与数组长度的编译期特性。
#[test]
fn kp_02_08_tuple_compare() {
    assess(
        M,
        "kp_02_08",
        "元组比较与数组长度：元组按字典序逐元素比较，数组长度是类型的一部分",
        Kind::Edge,
        "复习 lesson_02 示例 4（tuples）与示例 5（arrays）：元组实现了 `PartialOrd`，\
         从第 0 个元素开始逐个比较、第一个不同的元素决定结果；`[u8; 4]` 里的 4 是类型的一部分。",
        || {
            let (first_less, second_less, zeros_len) = exercise_02_08_tuple_compare();

            // 正常用例：第 0 个元素相等，比较第 1 个元素 2 < 3
            eq(
                first_less,
                true,
                "(1, 2) 与 (1, 3)：第 0 项相等，所以看第 1 项，2 < 3 → true",
            );
            // 边界用例：第 0 项就已经分出胜负，后面的 2 < 9 不再参与比较
            eq(
                second_less,
                false,
                "(1, 2) 与 (0, 9)：第 0 项 1 > 0 已定胜负，所以结果是 false（不是 true）",
            );
            // 正常用例：数组长度由类型写死，len() 直接给出 4
            eq(
                zeros_len,
                4usize,
                "[0u8; 4] 的长度写在类型里，len() 恒为 4（数组长度是类型的一部分）",
            );
        },
    );
}

/// 【待实现】比较元组并读取数组长度。
///
/// 实现要求：
///   - 第 1 项：`(1, 2) < (1, 3)` 的结果；
///   - 第 2 项：`(1, 2) < (0, 9)` 的结果；
///   - 第 3 项：`[0u8; 4].len()` 的结果；
///   - 返回 `(bool, bool, usize)`，即 `(true, false, 4)`；
///   - 元组的比较是**字典序**：先比 `.0`，相等才比 `.1`，以此类推；
///   - 比较时左右两侧的元素类型必须一致，否则编译器无法推断，需要显式标注
///     （例如 `let left: (i32, i32) = (1, 2);`）；
///   - 数组的 `[元素类型; 长度]` 中长度是类型的一部分，所以长度不同的数组不能互相赋值；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (true, false, 4)
/// ```
fn exercise_02_08_tuple_compare() -> (bool, bool, usize) {
    assessment_harness::todo_exercise(
        "exercise_02_08_tuple_compare",
        "返回 ((1, 2) < (1, 3), (1, 2) < (0, 9), [0u8; 4].len())",
        (),
    )
}
