//! assessments/lesson_07_enums_pattern_matching.rs —— 考核：枚举与模式匹配（对应 lesson_07）
//!
//! - 对应课程：`src/tutorial/lesson_07_enums_pattern_matching.rs`
//! - 知识点出处：`src/tutorial/README.md` 第二阶段「07 枚举与模式匹配」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_07_enums_pattern_matching    # 只考这一课
//!   cargo test                                            # 考全部 18 课
//!   cargo run --bin assessment_report                     # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_07_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_07_enums_pattern_matching`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 顶部「提供给你的类型（不要修改）」一节里的 `enum` 定义与派生**不要修改**，
//!   测试要靠它们做 `eq` / `eq_slice` 比较；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 9）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `match light { TrafficLight::Green => "通行" }` | `error[E0004]` non-exhaustive patterns | 补全所有变体，或用 `_ => ...` 兜底 |
//! | `match n { "5" => ..., _ => {} }`（n 是整数） | `error[E0308]` mismatched types: expected integer, found `&str` | 模式里字面量的类型必须与匹配值一致 |
//! | `let Some(x) = Some(1);` | `error[E0005]` refutable pattern in local binding | `let` 只能接不可反驳模式，改用 `let ... else { ... }` |
//! | `let Some(y) = Some(2) else { println!("没有值"); };` | `error[E0308]` `else` clause of `let...else` does not diverge | else 块里必须 `return` / `continue` / `break` / `panic!` |
//! | `if let Some(value) = Some(3) { }` | `warning: unused variable: value` | 绑定就要用；确实不用就写成 `Some(_)` |
//! | `match &command { Command::Echo(ref text) => ... }` | `error: cannot explicitly borrow within an implicitly-borrowing pattern`（无 E 编号） | 已经在匹配引用时不要再写 `ref`，绑定会自动成为引用 |
//! | `parse_port("0").unwrap()` | 运行期 panic: called `Option::unwrap()` on a `None` value | 用 `match` / `if let` / `unwrap_or` 处理失败路径 |

use assessment_harness::{Kind, approx, assess, contains, eq, eq_slice, err, none, ok, some};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_07";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// ===========================================================================
//
// 下面这些枚举由考核文件提供，保证骨架态就能编译通过。
// 学员只需要实现标了「【待实现】」的练习函数；
// 枚举定义与 `#[derive(...)]` 请原样保留（`PartialEq` 是测试做比较的前提）。

/// 形状：一个元组变体、一个结构体变体、一个无数据变体（见 kp_07_01）。
#[derive(Debug, PartialEq)]
enum Shape {
    /// 元组变体：半径
    Circle(f64),
    /// 结构体变体：宽与高
    Rectangle { width: f64, height: f64 },
    /// 无数据变体
    Point,
}

/// 命令：命令解析的返回值（见 kp_07_08）。
#[derive(Debug, PartialEq)]
enum Command {
    /// 结构体变体：移动指令的坐标
    Move { x: i32, y: i32 },
    /// 元组变体：说一句话
    Say(String),
    /// 无数据变体：退出
    Quit,
}

// ===========================================================================
// kp_07_01 match 穷尽枚举所有变体
// ===========================================================================

/// 知识点考核：用 `match` 覆盖枚举的每一个变体（穷尽性）。
#[test]
fn kp_07_01_area() {
    assess(
        M,
        "kp_07_01",
        "match 的穷尽性：每个变体都要有分支",
        Kind::Basic,
        "复习 lesson_07 示例 4（match_basics_and_wildcard）：`match` 必须覆盖所有可能，\
         少写一个变体会报 `error[E0004]: non-exhaustive patterns`；\
         元组变体用 `Shape::Circle(r)` 取值，结构体变体用 `Shape::Rectangle { width, height }` 按字段名绑定。",
        || {
            // 正常用例：三种变体各来一个
            approx(
                exercise_07_01_area(&Shape::Circle(2.0)),
                std::f64::consts::PI * 4.0,
                1e-9,
                "半径 2.0 的圆面积 = π × 2² = 4π",
            );
            approx(
                exercise_07_01_area(&Shape::Rectangle {
                    width: 3.0,
                    height: 4.0,
                }),
                12.0,
                1e-9,
                "3.0 × 4.0 的矩形面积是 12.0（宽乘高）",
            );
            approx(
                exercise_07_01_area(&Shape::Point),
                0.0,
                1e-9,
                "Point 变体不携带任何数据，面积是 0.0",
            );
            // 边界：半径 0 与宽度 0
            approx(
                exercise_07_01_area(&Shape::Circle(0.0)),
                0.0,
                1e-9,
                "边界：半径 0 的圆面积必须正好是 0.0，不能是负数或 NaN",
            );
            approx(
                exercise_07_01_area(&Shape::Rectangle {
                    width: 0.0,
                    height: 7.5,
                }),
                0.0,
                1e-9,
                "边界：宽为 0 的矩形面积是 0.0（高再大也不影响）",
            );
        },
    );
}

/// 【待实现】用 `match` 穷尽枚举算面积。
///
/// 实现要求：
///   - 用 `match shape` 处理 `Shape` 的**三个**变体，一个都不能漏，少写一个变体会报
///     `error[E0004]: non-exhaustive patterns`；
///   - `Shape::Circle(r)` → `std::f64::consts::PI * r * r`；
///   - `Shape::Rectangle { width, height }` → `width * height`；
///   - `Shape::Point` → `0.0`；
///   - 入参是 `&Shape`，按匹配人体工程学（match ergonomics），`Circle(r)` 里的 `r` 是
///     `&f64`，直接写 `PI * r * r` 就能编译（`f64` 与 `&f64` 的乘法标准库已实现）。
///
/// 示例输入：
/// ```text
/// shape = Shape::Rectangle { width: 3.0, height: 4.0 }
/// ```
/// 示例输出：
/// ```text
/// 12.0
/// ```
fn exercise_07_01_area(shape: &Shape) -> f64 {
    // 匹配 &Shape：match 人体工程学让 r / width / height 自动绑定成 &f64
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { width, height } => width * height,
        Shape::Point => 0.0,
    }
}

// ===========================================================================
// kp_07_02 Option<T>：两个都有值才算得出来
// ===========================================================================

/// 知识点考核：`Option` 的四种组合。
#[test]
fn kp_07_02_option_sum() {
    assess(
        M,
        "kp_07_02",
        "Option<T>：把「可能没有值」写进类型里",
        Kind::Basic,
        "复习 lesson_07 示例 2（option）：`Option` 只有 `Some(值)` 与 `None` 两个变体，\
         处理时必须显式应对 `None`；`match (a, b)` 一次匹配两个 Option 是最直观的写法。",
        || {
            // 正常用例：两个都是 Some
            eq(
                exercise_07_02_option_sum(Some(2), Some(3)),
                Some(5),
                "Some(2) + Some(3) 应得到 Some(5)",
            );
            eq(
                exercise_07_02_option_sum(Some(-5), Some(3)),
                Some(-2),
                "Some(-5) + Some(3) 应得到 Some(-2)（负数照常相加）",
            );
            // 边界：只要有一个 None 就必须是 None
            none(
                exercise_07_02_option_sum(Some(2), None),
                "边界：b 是 None 时必须返回 None（不能把 None 当成 0 参与运算）",
            );
            none(
                exercise_07_02_option_sum(None, Some(3)),
                "边界：a 是 None 时同样返回 None",
            );
            none(
                exercise_07_02_option_sum(None, None),
                "边界：两个都是 None，结果还是 None",
            );
            eq(
                exercise_07_02_option_sum(Some(0), Some(0)),
                Some(0),
                "边界：Some(0) + Some(0) = Some(0)——注意 0 是有值，不要把它当成 None",
            );
        },
    );
}

/// 【待实现】两个 `Option<i32>` 都有效时求和。
///
/// 实现要求：
///   - 用 `match (a, b)` 同时匹配两个 `Option`；
///   - `(Some(x), Some(y))` → `Some(x + y)`；
///   - 其余情况（任意一边或两边是 `None`）→ `None`，**不能**当 0 处理；
///   - `(Some(x), Some(y))` 与 `_ => None` 两个分支就能覆盖全部四种组合；
///     `Some(0)` 是「有值且值为 0」，与 `None`（没有值）是两回事。
///
/// 示例输入：
/// ```text
/// a = Some(2)
/// b = Some(3)
/// ```
/// 示例输出：
/// ```text
/// Some(5)
/// ```
fn exercise_07_02_option_sum(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    // 一次匹配两个 Option：只有都是 Some 才算得出来，其余一律 None（0 是有值，不是 None）
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}

// ===========================================================================
// kp_07_03 Result<T, E>：失败时带上可读的原因
// ===========================================================================

/// 知识点考核：`trim` + `parse` 得到 `Result`，失败时给出含「端口」的说明。
#[test]
fn kp_07_03_parse_port() {
    assess(
        M,
        "kp_07_03",
        "Result<T, E>：成功取 Ok，失败给出可读原因",
        Kind::Core,
        "复习 lesson_07 示例 3（result）：`Result` 的 `Err` 分支携带「为什么失败」，\
         比 `Option` 更适合解析类操作；`.parse::<u16>()` 的 `Err` 类型是 `ParseIntError`，\
         要转成自己的 `String` 说明。",
        || {
            // 正常用例：带空白的合法端口
            let parsed = ok(
                exercise_07_03_parse_port(" 8080 "),
                "\" 8080 \" 首尾有空白，trim 之后是合法端口，应返回 Ok",
            );
            eq(parsed, 8080u16, "\" 8080 \" 要先 trim 再 parse，得到 8080");
            // 边界：0 与 u16::MAX
            eq(
                ok(
                    exercise_07_03_parse_port("0"),
                    "\"0\" 是合法的 u16，应返回 Ok(0)",
                ),
                0u16,
                "边界：\"0\" → Ok(0)，端口 0 也要能解析成功",
            );
            eq(
                ok(
                    exercise_07_03_parse_port("65535"),
                    "\"65535\" 是 u16::MAX，应解析成功",
                ),
                65535u16,
                "边界：u16::MAX（65535）是 u16 能表示的最大端口",
            );
            // 边界：越界、非数字、空串都必须返回 Err，且错误信息里要有「端口」
            let too_large = err(
                exercise_07_03_parse_port("65536"),
                "65536 超过 u16::MAX，必须返回 Err（不能截断、不能 panic）",
            );
            contains(
                &too_large,
                "端口",
                "错误信息里必须出现「端口」二字，方便使用者定位是哪项配置出错",
            );
            let not_number = err(exercise_07_03_parse_port("abc"), "非数字内容必须返回 Err");
            contains(
                &not_number,
                "端口",
                "边界：解析失败的错误信息同样要包含「端口」",
            );
            let empty = err(exercise_07_03_parse_port(""), "空字符串必须返回 Err");
            contains(&empty, "端口", "边界：空字符串的错误信息也要包含「端口」");
        },
    );
}

/// 【待实现】把字符串解析成端口号（`trim` + `parse` → `Result`）。
///
/// 实现要求：
///   - 先用 `text.trim()` 去掉首尾空白，再调用 `.parse::<u16>()`；
///   - 成功 → `Ok(端口)`；
///   - 失败（不是数字、为空、超过 65535）→ `Err(String)`，而且错误信息里
///     **必须包含「端口」二字**，例如 `format!("端口不是合法数字：{trimmed}")`；
///   - `.parse()` 的目标类型推断不出来时必须写成 `.parse::<u16>()`，否则报
///     `error[E0282]: type annotations needed`；用 `match` 处理 `parse` 的返回值，
///     不要用 `unwrap()`（那会让 `"abc"` 直接 panic）。
///
/// 示例输入：
/// ```text
/// text = " 8080 "
/// ```
/// 示例输出：
/// ```text
/// Ok(8080)
/// ```
fn exercise_07_03_parse_port(text: &str) -> Result<u16, String> {
    // trim 之后再 parse；失败路径必须带上可读原因，且含「端口」二字
    let trimmed = text.trim();
    match trimmed.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(error) => Err(format!("端口不是合法数字：\"{trimmed}\"（{error}）")),
    }
}

// ===========================================================================
// kp_07_04 match + 或模式 + 通配分支
// ===========================================================================

/// 知识点考核：用 `|` 合并分支、用 `_` 兜底。
#[test]
fn kp_07_04_classify() {
    assess(
        M,
        "kp_07_04",
        "或模式 `|` 与通配 `_`：合并同结果分支、兜住其余取值",
        Kind::Core,
        "复习 lesson_07 示例 4 的 `|` 与 `_` 部分：`2 | 3` 把两个模式合并到同一个分支；\
         `_` 兜住所有剩余取值，但它必须写在**最后一个**分支。",
        || {
            // 正常用例
            eq(
                exercise_07_04_classify(1),
                "一",
                "1 走第一个分支，返回「一」",
            );
            eq(
                exercise_07_04_classify(2),
                "二或三",
                "2 由或模式 `2 | 3` 命中，返回「二或三」",
            );
            eq(
                exercise_07_04_classify(3),
                "二或三",
                "3 与 2 共用同一个或模式分支，返回「二或三」",
            );
            eq(
                exercise_07_04_classify(4),
                "其它",
                "4 没有被显式列出，落到通配 `_` 分支",
            );
            // 边界：0、负数与极值都必须由 `_` 兜底（不能 panic）
            eq(
                exercise_07_04_classify(0),
                "其它",
                "边界：0 不在 1 / 2 / 3 之列，必须落到 `_` 分支",
            );
            eq(
                exercise_07_04_classify(-1),
                "其它",
                "边界：负数同样落到 `_` 分支（`_` 兜住一切剩余取值）",
            );
            eq(
                exercise_07_04_classify(i32::MIN),
                "其它",
                "边界：i32::MIN 这种极值也要被 `_` 接住，而不是 panic",
            );
        },
    );
}

/// 【待实现】用 `match` + 或模式 + 通配符分类。
///
/// 实现要求（一个 `match n` 里完成）：
///   - `1` → `"一"`；
///   - `2 | 3` → `"二或三"`（用或模式把结果相同的分支合并）；
///   - `_` → `"其它"`（兜住其余所有取值，包括 0 与负数）；
///   - `_` 必须放在最后一个分支，否则后面的分支永远不可达（编译报错或警告）；
///     返回类型是 `&'static str`，分支里直接写字符串字面量即可。
///
/// 示例输入：
/// ```text
/// n = 2
/// ```
/// 示例输出：
/// ```text
/// "二或三"
/// ```
fn exercise_07_04_classify(n: i32) -> &'static str {
    // `_` 必须放在最后一个分支，它兜住 0、负数、i32::MIN 等一切剩余取值
    match n {
        1 => "一",
        2 | 3 => "二或三",
        _ => "其它",
    }
}

// ===========================================================================
// kp_07_05 匹配守卫 + @ 绑定
// ===========================================================================

/// 知识点考核：同一个范围模式上，用守卫分流、用 `@` 把值绑下来。
#[test]
fn kp_07_05_guarded() {
    assess(
        M,
        "kp_07_05",
        "匹配守卫（if）与 @ 绑定：范围判断 + 取到原值",
        Kind::Hard,
        "复习 lesson_07 示例 5（guards_and_bindings）：守卫写在模式之后、`=>` 之前，\
         例如 `n if n % 2 == 0`；`@` 绑定既做范围判断又把值绑给变量，例如 `s @ 0..=59`。\
         分支顺序很重要：带守卫的分支必须写在同范围的裸模式之前。",
        || {
            // 正常用例：小偶数与小奇数
            eq(
                exercise_07_05_guarded(4),
                "小偶数",
                "4 在 1..=9 内且是偶数，被守卫 `x % 2 == 0` 命中",
            );
            eq(
                exercise_07_05_guarded(5),
                "小奇数",
                "5 在 1..=9 内但不是偶数，落到同范围的第二条 `@` 分支",
            );
            // 边界：范围的两个端点
            eq(
                exercise_07_05_guarded(1),
                "小奇数",
                "边界：范围下界 1 属于「小奇数」（1 不是偶数）",
            );
            eq(
                exercise_07_05_guarded(9),
                "小奇数",
                "边界：范围上界 9 属于「小奇数」（闭区间 1..=9 包含 9）",
            );
            // 边界：范围之外
            eq(
                exercise_07_05_guarded(0),
                "超范围",
                "边界：0 不在 1..=9 内，必须落到 `_` 分支",
            );
            eq(
                exercise_07_05_guarded(10),
                "超范围",
                "边界：10 刚好越过范围上界，属于超范围",
            );
            eq(
                exercise_07_05_guarded(-2),
                "超范围",
                "边界：-2 是偶数但不在 1..=9 内，仍必须返回「超范围」——守卫只在范围命中的前提下生效",
            );
        },
    );
}

/// 【待实现】同一个 `match` 里同时使用匹配守卫与 `@` 绑定。
///
/// 实现要求（分支顺序必须与下面一致）：
///   - `x @ 1..=9 if x % 2 == 0` → `"小偶数"`（`@` 先把范围内的值绑给 `x`，守卫再判奇偶）；
///   - `1..=9` → `"小奇数"`（同一范围、不绑定值，作为去掉守卫后的兜底）；
///   - `_` → `"超范围"`；
///   - 守卫是模式后面的 `if 条件`，可以读 `@` 绑定的变量；`1..=9` 是闭区间，带守卫的
///     分支必须写在裸范围模式之前，偶数才不会被第二条提前截胡；
///   - 第二条用不到具体数值，写 `1..=9` 即可——不绑定就不会有 `unused variable`
///     警告，只有真正要用原值的分支才写 `@` 绑定。
///
/// 示例输入：
/// ```text
/// n = 4
/// ```
/// 示例输出：
/// ```text
/// "小偶数"
/// ```
fn exercise_07_05_guarded(n: i32) -> &'static str {
    // 带守卫的分支必须写在同范围的裸模式之前，偶数才不会被第二条截胡
    match n {
        x @ 1..=9 if x % 2 == 0 => "小偶数",
        1..=9 => "小奇数",
        _ => "超范围",
    }
}

// ===========================================================================
// kp_07_06 while let：一直取到没有值为止
// ===========================================================================

/// 知识点考核：用 `while let` + `continue` 累加 `Some` 的值。
#[test]
fn kp_07_06_if_let_while_let() {
    assess(
        M,
        "kp_07_06",
        "while let：不断取出直到 None，None 用 continue 跳过",
        Kind::Core,
        "复习 lesson_07 示例 6（if_let_and_while_let）：`while let Some(v) = iter.next()` 每次循环\
         都尝试取出一个值，取出 `None` 时循环自然结束，因此特别适合「消耗式遍历」。",
        || {
            // 正常用例：Some / None 交错
            eq(
                exercise_07_06_if_let_while_let(&[Some(1), None, Some(2), Some(3)]),
                6,
                "只累加 Some 的值：1 + 2 + 3 = 6，中间的 None 用 continue 跳过",
            );
            // 边界：空切片、全是 None、正负相消、单个极大值
            eq(
                exercise_07_06_if_let_while_let(&[]),
                0,
                "边界：空切片时 while let 一次都不进入循环，返回 0",
            );
            eq(
                exercise_07_06_if_let_while_let(&[None, None]),
                0,
                "边界：全是 None 时累加值始终保持 0",
            );
            eq(
                exercise_07_06_if_let_while_let(&[Some(-4), Some(4)]),
                0,
                "边界：-4 + 4 = 0，说明负数也参与了累加（不是只加正数）",
            );
            eq(
                exercise_07_06_if_let_while_let(&[Some(i32::MAX)]),
                i32::MAX,
                "边界：单个 i32::MAX 要原样累加，不能溢出或截断",
            );
        },
    );
}

/// 【待实现】用 `while let` 累加切片里所有 `Some` 的值。
///
/// 实现要求：
///   - 用 `values.iter()` 拿到迭代器，循环写成 `while let Some(item) = iter.next()`；
///   - 每个元素是 `&Option<i32>`：是 `Some(v)` 就把 `*v` 累加到 `total`；
///   - `None` 用 `continue` 跳过（不要 `break`，后面的元素还要继续处理）；
///   - 返回累加结果 `total`（`i32`，初始为 0）；
///   - `item` 的类型是 `&Option<i32>`，用 `match item` 或 `if let Some(v) = item`
///     都能取出 `&i32`，需要 `*v` 解引用后再相加；空切片时循环体不执行，直接返回 0。
///
/// 示例输入：
/// ```text
/// values = &[Some(1), None, Some(2), Some(3)]
/// ```
/// 示例输出：
/// ```text
/// 6
/// ```
fn exercise_07_06_if_let_while_let(values: &[Option<i32>]) -> i32 {
    let mut iter = values.iter();
    let mut total = 0;
    // while let 每次尝试取一个值，取出 None 时循环自然结束
    while let Some(item) = iter.next() {
        match item {
            Some(v) => total += *v,
            // None 只跳过这一个元素，不能 break（后面的元素还要继续处理）
            None => continue,
        }
    }
    total
}

// ===========================================================================
// kp_07_07 let else：成功路径不缩进
// ===========================================================================

/// 知识点考核：用 `let ... else` 处理「可能失败的解构」。
#[test]
fn kp_07_07_let_else() {
    assess(
        M,
        "kp_07_07",
        "let else：失败分支发散，成功路径无需再嵌套",
        Kind::Hard,
        "复习 lesson_07 示例 7（let_else）：`let ... else { ... }` 的 else 块必须发散\
         （`return` / `continue` / `break` / `panic!`），因此后面的代码可以直接使用解包后的值。",
        || {
            // 正常用例
            eq(
                exercise_07_07_let_else("abc\ndef"),
                3,
                "只取第一行 \"abc\"，它的字节长度是 3",
            );
            eq(
                exercise_07_07_let_else("abc"),
                3,
                "只有一行时，这一行本身就是第一行，长度仍是 3",
            );
            // 边界：空串、空行、多字节
            eq(
                exercise_07_07_let_else(""),
                0,
                "边界：空字符串时 `lines().next()` 是 None，let else 的 else 分支必须 return 0",
            );
            eq(
                exercise_07_07_let_else("\nsecond"),
                0,
                "边界：第一行是空行，长度是 0（注意不是取第二行）",
            );
            eq(
                exercise_07_07_let_else("中文\nx"),
                6,
                "边界：UTF-8 多字节——\"中文\" 的字节长度是 6，而不是字符数 2",
            );
        },
    );
}

/// 【待实现】用 `let ... else` 取第一行并返回字节长度。
///
/// 实现要求：
///   - 必须写成 `let Some(first) = text.lines().next() else { return 0; };`
///     （else 分支用 `return 0` 发散，只写 `println!` 会报
///     `error[E0308]: else clause of let...else does not diverge`）；
///   - 返回第一行的字节长度 `first.len()`；
///   - `str::lines()` 按行切分且不保留换行符，空字符串没有行，`next()` 返回 `None`；
///     `len()` 返回的是**字节数**，"中文" 是 6 字节。
///
/// 示例输入：
/// ```text
/// text = "abc\ndef"
/// ```
/// 示例输出：
/// ```text
/// 3
/// ```
fn exercise_07_07_let_else(text: &str) -> usize {
    // else 块必须发散（这里用 return 0），之后就能直接用解包出来的 first
    let Some(first) = text.lines().next() else {
        return 0;
    };
    // len() 是**字节**长度："中文" 是 6 而不是 2
    first.len()
}

// ===========================================================================
// kp_07_08 典型场景：文本 → 枚举命令解析
// ===========================================================================

/// 知识点考核：把一行文本解析成 `Command` 枚举（对应课程示例 8）。
#[test]
fn kp_07_08_parse_command() {
    assess(
        M,
        "kp_07_08",
        "枚举 + match 的典型用法：文本 → 结构化命令",
        Kind::Hard,
        "复习 lesson_07 示例 8（typical_scenario_command_parser）：`split_whitespace()` 天然处理\
         多余空格，`parts.next()` 逐个取词，取不到参数就返回 `None`；\
         解析成功后用枚举变体（元组变体 / 结构体变体 / 无数据变体）把结果结构化。",
        || {
            // 正常用例：三种命令
            eq(
                exercise_07_08_parse_command("move 3 -4"),
                Some(Command::Move { x: 3, y: -4 }),
                "\"move 3 -4\" 要解析成 Move { x: 3, y: -4 }（y 允许是负数）",
            );
            eq(
                exercise_07_08_parse_command("say hello"),
                Some(Command::Say(String::from("hello"))),
                "\"say hello\" 要解析成 Say(\"hello\")",
            );
            eq(
                exercise_07_08_parse_command("quit"),
                Some(Command::Quit),
                "\"quit\" 要解析成无数据变体 Quit",
            );
            // 边界：负数坐标也要能解析
            let negative = some(
                exercise_07_08_parse_command("move -1 2"),
                "边界：x 为负数时同样要能解析成 Move",
            );
            eq(
                negative,
                Command::Move { x: -1, y: 2 },
                "边界：x = -1、y = 2 都要逐字对上，负号不能被吞掉",
            );
            // 边界：参数缺失、空行、未知命令都必须返回 None（不能 panic）
            eq(
                exercise_07_08_parse_command("move 3"),
                None,
                "边界：move 只有一个参数时必须返回 None，不能 unwrap 导致 panic",
            );
            eq(
                exercise_07_08_parse_command("say"),
                None,
                "边界：say 后面没有内容时返回 None",
            );
            eq(
                exercise_07_08_parse_command(""),
                None,
                "边界：空字符串连命令头都没有，必须返回 None",
            );
            eq(
                exercise_07_08_parse_command("unknown do something"),
                None,
                "边界：未知命令必须返回 None（不要在 `_` 分支里 panic）",
            );
        },
    );
}

/// 【待实现】解析一行文本为 `Command`。
///
/// 实现要求：
///   - 用 `line.split_whitespace()` 逐个取词，第一个词是命令名；
///   - `"move x y"`：两个参数都能解析成 `i32` → `Some(Command::Move { x, y })`；
///   - `"say 内容"`：有内容 → `Some(Command::Say(内容.to_string()))`；
///   - `"quit"` → `Some(Command::Quit)`；
///   - 其余情况（参数缺失、参数不是整数、空串、未知命令）→ `None`；
///   - `let mut parts = line.split_whitespace();` 之后用 `parts.next()` 取命令名，再按需取参数；
///     取参数可以写成 `match (parts.next(), parts.next()) { (Some(x), Some(y)) => ..., _ => None }`
///     这种「元组 + Option」的写法，能一次判掉「缺参数」的情况。
///
/// 示例输入：
/// ```text
/// input = "move 3 -4"
/// ```
/// 示例输出：
/// ```text
/// Some(Command::Move { x: 3, y: -4 })
/// ```
fn exercise_07_08_parse_command(input: &str) -> Option<Command> {
    let mut parts = input.split_whitespace();
    // 空串连命令头都没有：let else 的 else 块直接发散成 None
    let Some(name) = parts.next() else {
        return None;
    };
    match name {
        // 元组 + Option：一次判掉「缺参数」，再逐个 parse 掉非法参数
        "move" => match (parts.next(), parts.next()) {
            (Some(x), Some(y)) => match (x.parse::<i32>(), y.parse::<i32>()) {
                (Ok(x), Ok(y)) => Some(Command::Move { x, y }),
                _ => None,
            },
            _ => None,
        },
        "say" => parts.next().map(|text| Command::Say(text.to_string())),
        "quit" => Some(Command::Quit),
        // 未知命令返回 None，不要 panic
        _ => None,
    }
}

// ===========================================================================
// kp_07_09 常见错误诊断：读懂 E0004 / E0308 / E0005
// ===========================================================================

/// 知识点考核：按课程示例 9 的顺序写出前三个错误编号。
#[test]
fn kp_07_09_diagnose_errors() {
    assess(
        M,
        "kp_07_09",
        "常见错误诊断：E0004 / E0308 / E0005 分别对应哪类错误",
        Kind::Hard,
        "复习 lesson_07 示例 9（common_mistakes）的错误 1~3：\
         错误 1 是 match 分支没写全（non-exhaustive patterns），\
         错误 2 是模式与值的类型不一致（mismatched types: expected integer, found `&str`），\
         错误 3 是把可反驳模式用在 `let` 位置（refutable pattern in local binding）。",
        || {
            let codes = exercise_07_09_diagnose_errors();
            eq_slice(
                &codes,
                &["E0004", "E0308", "E0005"],
                "顺序必须是：match 分支没写全、模式与值类型不一致、let 位置用了可反驳模式",
            );
            eq(
                codes.len(),
                3usize,
                "边界：数组长度固定为 3，只报课程示例 9 里的前三个错误编号",
            );
        },
    );
}

/// 【待实现】写出三个场景对应的编译器错误编号。
///
/// 场景（与课程示例 9 的错误 1~3 一致）：
///   1. `match light { TrafficLight::Green => "通行" }` —— match 没覆盖全部变体；
///   2. `match n { "5" => ..., _ => {} }`（`n` 是整数）—— 模式与值的类型不一致；
///   3. `let Some(x) = Some(1);` —— 在 `let` 位置使用了可反驳模式。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0004"`），顺序与上面一致；
///   - 第 3 条不是 E0004 也不是 E0308，而是专门针对「let 只能接不可反驳模式」的那一个编号。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0004", "E0308", "E0005"]
/// ```
fn exercise_07_09_diagnose_errors() -> [&'static str; 3] {
    // 顺序：match 分支没写全 / 模式与值类型不一致 / let 位置用了可反驳模式
    ["E0004", "E0308", "E0005"]
}
