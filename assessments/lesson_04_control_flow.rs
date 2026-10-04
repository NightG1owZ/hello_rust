//! assessments/lesson_04_control_flow.rs —— 考核：流程控制（对应 lesson_04）
//!
//! - 对应课程：`src/tutorial/lesson_04_control_flow.rs`
//! - 知识点出处：`src/tutorial/README.md` 第一阶段「04 流程控制」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_04_control_flow          # 只考这一课
//!   cargo test                                        # 考全部 18 课
//!   cargo run --bin assessment_report                 # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_04_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_04_control_flow`，直到全部用例变绿；
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
//! | 场景 | 报错 / 现象 | 修正方法 |
//! | --- | --- | --- |
//! | `let value = loop { 1 };` | `error[E0308]` mismatched types（expected `()`, found integer） | 改成 `let value = loop { break 1; };`，`loop` 的值只能由 `break` 带出 |
//! | `while i < 3 { }` 里不改 `i` | 能编译，但**死循环**（没有错误编号，是运行期逻辑错误） | 在循环体里让条件趋向结束，例如 `i += 1;` |
//! | `while i < 5 { break 1; }` | `error[E0571]` `break` with value from a `while` loop | 只有 `loop` 能 `break` 带值；`while` 里先 `break`，值用循环外的变量接 |
//! | 嵌套循环里写无标签 `break` | 能编译，但只跳出**内层**（逻辑错误） | 给外层加标签：`'outer: for …` 配 `break 'outer;` |
//! | `if x { }`（x 是整数） | `error[E0308]` expected `bool`, found integer | Rust 不把整数当真值，写 `if x != 0 { }` |
//! | 在循环外用 `for index in 0..3 { }` 的 `index` | `error[E0425]` cannot find value `index` in this scope | 循环变量作用域只在循环内；把结果存到循环外声明的变量里 |
//! | `loop { break; println!("…"); }` | `warning: unreachable statement` | `break` 之后的同级语句不可达，删掉或移到循环外 |

use assessment_harness::{Kind, assess, eq, eq_slice, none};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_04";

// ===========================================================================
// kp_04_01 if 是表达式：分支的值就是整个表达式的值
// ===========================================================================

/// 知识点考核：用 `if` 表达式做函数尾表达式返回分类结论。
#[test]
fn kp_04_01_if_expression() {
    assess(
        M,
        "kp_04_01",
        "if 是表达式：分支的值可以直接做函数尾表达式",
        Kind::Basic,
        "复习 lesson_04 示例 1（if_expression）：`if` 有值、两个分支必须同类型，\
         所以 `if n > 0 { \"正数\" } else { \"负数\" }` 可以直接当返回值，\
         末尾**不要**写分号，也**不要**写 `return`。",
        || {
            // 正常用例：正数分支
            eq(
                exercise_04_01_if_expression(7),
                "正数",
                "7 大于 0，应走 if 分支返回 \"正数\"",
            );
            // 正常用例：负数分支
            eq(
                exercise_04_01_if_expression(-7),
                "负数",
                "-7 小于 0，应走 else 分支返回 \"负数\"",
            );
            // 边界：0 既不是正数也不是负数
            eq(
                exercise_04_01_if_expression(0),
                "零",
                "0 必须单独判定为 \"零\"，不能落进正数或负数分支",
            );
            // 边界：i32::MIN 是绝对值最大的负数
            eq(
                exercise_04_01_if_expression(i32::MIN),
                "负数",
                "i32::MIN = -2147483648 是最小值，仍然是负数",
            );
            // 边界：i32::MAX 是最大的正数
            eq(
                exercise_04_01_if_expression(i32::MAX),
                "正数",
                "i32::MAX = 2147483647 是最大值，仍然是正数",
            );
        },
    );
}

/// 【待实现】用 `if` 表达式给整数分类。
///
/// 实现要求：
///   - 用 `if n > 0 { … } else if n < 0 { … } else { … }` 写出三个分支；
///   - 三个分支分别求值为 `"正数"` / `"负数"` / `"零"`；
///   - 让这个 `if` 表达式**直接做函数体尾表达式**：不要 `let`、不要 `return`、
///     末尾不要写分号——多写一个分号会让表达式退化成语句，函数返回值变成 `()`，
///     编译器报 `error[E0308]: mismatched types`。
///
/// 示例输入：
/// ```text
/// n = 7
/// ```
/// 示例输出：
/// ```text
/// "正数"
/// ```
fn exercise_04_01_if_expression(n: i32) -> &'static str {
    assessment_harness::todo_exercise(
        "exercise_04_01_if_expression",
        "用 if / else if / else 表达式分别返回 \"正数\" / \"负数\" / \"零\"，并直接作为函数尾表达式",
        (n,),
    )
}

// ===========================================================================
// kp_04_02 if / else if 链：自上而下命中第一个为真的条件
// ===========================================================================

/// 知识点考核：用 if / else if 链把分数映射成等级字符。
#[test]
fn kp_04_02_grade() {
    assess(
        M,
        "kp_04_02",
        "if / else if 链：条件自上而下判断，命中第一个为真的分支",
        Kind::Core,
        "复习 lesson_04 示例 2（if_else_chain）：多分支按**从高到低**的顺序比较，\
         顺序写反会让高分先落进低档；最后的 `else` 是兜底分支，\
         本题约定分数 > 100 这类越界输入也落到 `'D'`。",
        || {
            // 正常用例：满分
            eq(exercise_04_02_grade(100), 'A', "满分 100 应判为 A");
            // 正常用例：中档
            eq(
                exercise_04_02_grade(85),
                'B',
                "85 分落在 [80, 90) 区间，应判为 B",
            );
            // 正常用例：及格线附近
            eq(
                exercise_04_02_grade(72),
                'C',
                "72 分落在 [60, 80) 区间，应判为 C",
            );
            // 正常用例：不及格
            eq(
                exercise_04_02_grade(30),
                'D',
                "30 分低于 60，应落在兜底的 else 分支，判为 D",
            );
            // 边界：最低分 0
            eq(exercise_04_02_grade(0), 'D', "0 分应判为 D，且不能 panic");
            // 边界：59 与 60 是 D / C 的分界
            eq(
                exercise_04_02_grade(59),
                'D',
                "59 分差 1 分不及格，必须是 D",
            );
            eq(exercise_04_02_grade(60), 'C', "60 分刚好及格，必须是 C");
            // 边界：89 与 90 是 B / A 的分界
            eq(
                exercise_04_02_grade(89),
                'B',
                "89 分差 1 分不到 90，必须是 B",
            );
            eq(
                exercise_04_02_grade(90),
                'A',
                "90 分刚好进入 A 档，边界写 `>= 90` 才对",
            );
            // 边界：越界输入（u32 没有负数，只可能过大）
            eq(
                exercise_04_02_grade(101),
                'D',
                "101 超出满分，按本课约定用兜底分支判为 D（不要 panic、不要给 A）",
            );
        },
    );
}

/// 【待实现】用 if / else if 链实现成绩等级判定。
///
/// 实现要求：
///   - `score >= 90` → `'A'`；
///   - 否则 `score >= 80` → `'B'`；
///   - 否则 `score >= 60` → `'C'`；
///   - 否则（含 `score > 100` 这类越界值）→ `'D'`；
///   - 比较顺序必须**从高到低**：写成 `score >= 60` 开头的升序链会让 95 分先命中 `'C'`；
///     末尾的 `else` 是兜底分支，保证任何 `u32` 都有返回值。
///
/// 示例输入：
/// ```text
/// score = 85
/// ```
/// 示例输出：
/// ```text
/// 'B'
/// ```
fn exercise_04_02_grade(score: u32) -> char {
    assessment_harness::todo_exercise(
        "exercise_04_02_grade",
        "用 if / else if 链实现：>=90 得 'A'，>=80 得 'B'，>=60 得 'C'，其余（含 >100）得 'D'",
        (score,),
    )
}

// ===========================================================================
// kp_04_03 loop + break 带值：循环本身就是一个表达式
// ===========================================================================

/// 知识点考核：用 `loop` 找到第一个平方大于 limit 的 n，并用 `break` 带值返回。
#[test]
fn kp_04_03_first_square_above() {
    assess(
        M,
        "kp_04_03",
        "loop 与 break 带值：把循环当成有返回值的表达式",
        Kind::Core,
        "复习 lesson_04 示例 4（break_value_and_continue）：`let x = loop { … break 值; };` \
         是 Rust 特有的写法——`loop` 是表达式，`break 值` 就是它的值；\
         只有 `loop` 支持 break 带值，`while` / `for` 不行（会报 E0571）。",
        || {
            // 正常用例：10 之后的第一个平方数 16 = 4²
            eq(
                exercise_04_03_first_square_above(10),
                4,
                "3² = 9 不大于 10，4² = 16 > 10，所以答案是 4",
            );
            // 边界：limit = 0 → 1² = 1 已经大于 0
            eq(
                exercise_04_03_first_square_above(0),
                1,
                "从 n = 1 开始找：1² = 1 > 0，应立刻返回 1",
            );
            // 边界：limit = 1 → 1² 只等于 1，不满足「大于」
            eq(
                exercise_04_03_first_square_above(1),
                2,
                "必须是严格大于：1² = 1 不满足，2² = 4 > 1，答案是 2",
            );
            // 边界：刚好等于平方数的 limit
            eq(
                exercise_04_03_first_square_above(100),
                11,
                "10² = 100 只等于 limit，不算「大于」，所以答案是 11",
            );
        },
    );
}

/// 【待实现】用 `loop` + `break` 带值找第一个平方大于 limit 的 n。
///
/// 实现要求：
///   - 用一个 `let mut n = 1;` 做循环变量；
///   - 用 `loop { … }` 不断检查 `n * n > limit`；
///   - 条件成立时用 **`break n;` 带值**结束循环，把这个值作为整个循环表达式的值返回
///     （写 `while n * n <= limit { n += 1; }` 再返回 `n` 结果也对，但本题考的是
///     `loop` 的 `break` 带值，请用 `let answer = loop { … break n; };` 的写法）；
///   - 判断条件是**严格大于** `limit`，不是「大于等于」。
///
/// 示例输入：
/// ```text
/// limit = 10
/// ```
/// 示例输出：
/// ```text
/// 4
/// ```
fn exercise_04_03_first_square_above(limit: u32) -> u32 {
    assessment_harness::todo_exercise(
        "exercise_04_03_first_square_above",
        "用 loop 递增 n 并用 break n 带值返回第一个满足 n * n > limit 的 n",
        (limit,),
    )
}

// ===========================================================================
// kp_04_04 while + continue：条件驱动循环，continue 跳过本轮剩余部分
// ===========================================================================

/// 知识点考核：用 `while` + `continue` 只累加偶数。
#[test]
fn kp_04_04_sum_even() {
    assess(
        M,
        "kp_04_04",
        "while 循环与 continue：跳过奇数，只累加偶数",
        Kind::Core,
        "复习 lesson_04 示例 3（loop_while_for）与示例 4 的 continue 部分：\
         `while` 先判断条件再执行循环体，因此必须保证循环变量趋向结束；\
         `continue` 立即进入下一轮，它后面的累加语句本轮不会执行。",
        || {
            // 正常用例：1..=10 的偶数 2+4+6+8+10 = 30
            eq(
                exercise_04_04_sum_even(10),
                30,
                "1..=10 中的偶数之和是 2 + 4 + 6 + 8 + 10 = 30",
            );
            // 正常用例：1..=5 的偶数 2 + 4 = 6
            eq(
                exercise_04_04_sum_even(5),
                6,
                "1..=5 中只有 2 和 4 是偶数，和为 6",
            );
            // 边界：limit = 0 → 范围为空
            eq(
                exercise_04_04_sum_even(0),
                0,
                "1..=0 是空范围，循环体一次都不执行，应返回 0",
            );
            // 边界：limit = 1 → 只有奇数 1
            eq(
                exercise_04_04_sum_even(1),
                0,
                "1 是奇数会被 continue 跳过，结果仍是 0",
            );
            // 边界：limit = 2 → 恰好只有一个偶数
            eq(
                exercise_04_04_sum_even(2),
                2,
                "上限刚好是第一个偶数 2，和为 2",
            );
        },
    );
}

/// 【待实现】用 `while` + `continue` 累加 1..=limit 中的偶数。
///
/// 实现要求：
///   - 用一个 `let mut i = 1;` 做循环变量，一个 `let mut sum = 0;` 做累加器；
///   - 用 `while i <= limit { … }` 遍历；
///   - 循环体内：`i` 是奇数时用 `continue` 跳过累加，是偶数时 `sum += i`；
///   - 因为 `continue` 会跳过本轮剩下的语句，`i += 1` 必须写在 `continue` **之前**，
///     否则 `i` 永远不变、条件恒真，循环不会结束；
///   - 返回 `sum`。
///
/// 示例输入：
/// ```text
/// limit = 10
/// ```
/// 示例输出：
/// ```text
/// 30
/// ```
fn exercise_04_04_sum_even(limit: u32) -> u32 {
    assessment_harness::todo_exercise(
        "exercise_04_04_sum_even",
        "用 while + continue 累加 1..=limit 中的所有偶数并返回",
        (limit,),
    )
}

// ===========================================================================
// kp_04_05 for 遍历切片：由迭代器管理边界，不用手写下标
// ===========================================================================

/// 知识点考核：用 `for` 遍历切片求和。
#[test]
fn kp_04_05_for_sum() {
    assess(
        M,
        "kp_04_05",
        "for 遍历切片：元素个数由迭代器决定，不需要手写下标",
        Kind::Basic,
        "复习 lesson_04 示例 3 的 for 部分：`for value in items { … }` 直接遍历 `&[i32]`，\
         因为切片实现了 `IntoIterator`；`for index in 0..items.len()` 也能写，\
         但一旦下标写错就越界 panic，所以优先用 for-each。",
        || {
            // 正常用例
            eq(
                exercise_04_05_for_sum(&[1, 2, 3, 4]),
                10,
                "1 + 2 + 3 + 4 = 10",
            );
            // 正常用例：单个元素
            eq(
                exercise_04_05_for_sum(&[42]),
                42,
                "只有一个元素时结果就是它本身",
            );
            // 边界：空切片
            eq(
                exercise_04_05_for_sum(&[]),
                0,
                "空切片没有任何元素，应是 0 而不是 panic",
            );
            // 边界：全负数
            eq(
                exercise_04_05_for_sum(&[-1, -2, -3]),
                -6,
                "负数要参与累加：-1 + -2 + -3 = -6；初始值必须是 0",
            );
            // 边界：正负相消
            eq(
                exercise_04_05_for_sum(&[-5, 5, -5, 5]),
                0,
                "正负相消后为 0，别把 0 当成「没算」",
            );
        },
    );
}

/// 【待实现】用 `for` 遍历切片求和。
///
/// 实现要求：
///   - 声明 `let mut sum = 0;`；
///   - 用 `for value in items { sum += value; }` 遍历（`items` 是 `&[i32]`，
///     直接 `for value in items` 得到的就是 `&i32`，`sum += *value` 或 `sum += value` 都可以）；
///   - 返回累加结果；空切片自然返回 0；
///   - 用 for-each 而不是 `for i in 0..items.len()`：后者要手写 `items[i]`，
///     下标写错就越界 panic，把边界交给迭代器更安全。
///
/// 示例输入：
/// ```text
/// items = [1, 2, 3, 4]
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_04_05_for_sum(items: &[i32]) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_04_05_for_sum",
        "用 for 遍历切片累加所有元素并返回（空切片返回 0）",
        (items,),
    )
}

// ===========================================================================
// kp_04_06 循环标签：一次跳出多层嵌套循环
// ===========================================================================

/// 知识点考核：用循环标签在二维数组里做查找。
#[test]
fn kp_04_06_find_in_matrix() {
    assess(
        M,
        "kp_04_06",
        "循环标签（'outer:）：break 'outer 一次跳出两层嵌套循环",
        Kind::Hard,
        "复习 lesson_04 示例 5（loop_labels）：给外层循环写 `'outer: for row in …`，\
         内层用 `break 'outer;` 跳出外层——这比用布尔标志位「绕圈」退出清晰得多；\
         找不到目标时两个循环都自然跑完，函数尾部返回 `None`。",
        || {
            let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            // 正常用例：中间的元素
            eq(
                exercise_04_06_find_in_matrix(&matrix, 5),
                Some((1, 1)),
                "5 出现在第 2 行第 2 列（下标从 0 开始），应返回 Some((1, 1))",
            );
            // 正常用例：每行的第一个元素
            eq(
                exercise_04_06_find_in_matrix(&matrix, 4),
                Some((1, 0)),
                "4 在第 2 行第 1 列，应返回 Some((1, 0))",
            );
            // 边界：目标在左上角
            eq(
                exercise_04_06_find_in_matrix(&matrix, 1),
                Some((0, 0)),
                "1 在最左上角，应返回 Some((0, 0))，别把 (0, 0) 当成「没找到」",
            );
            // 边界：目标在右下角（必须走完整个矩阵）
            eq(
                exercise_04_06_find_in_matrix(&matrix, 9),
                Some((2, 2)),
                "9 在最右下角，应返回 Some((2, 2))",
            );
            // 边界：找不到时必须返回 None
            none(
                exercise_04_06_find_in_matrix(&matrix, 11),
                "矩阵里没有 11，两个循环跑完后应返回 None（不要返回 Some((0, 0))）",
            );
        },
    );
}

/// 【待实现】用带标签的循环在 3×3 矩阵中查找目标值。
///
/// 实现要求：
///   - 用 `'outer: for (row_index, row) in matrix.iter().enumerate()` 遍历每一行；
///   - 内层用 `for (column_index, value) in row.iter().enumerate()` 遍历每个元素；
///   - 命中 `*value == target` 时返回 `Some((row_index, column_index))`，
///     并用 `break 'outer;` 立刻跳出**两层**循环（本文件约定：先保存结果，再 break 外层）；
///   - 遍历结束仍未找到时返回 `None`；
///   - 标签必须写在外层循环前（`'outer: for …`），只写 `break` 只能跳出内层，
///     外层会继续跑完，逻辑就错了；返回坐标的顺序是 `(行, 列)`，与测试期望一致。
///
/// 示例输入：
/// ```text
/// matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]], target = 5
/// ```
/// 示例输出：
/// ```text
/// Some((1, 1))
/// ```
fn exercise_04_06_find_in_matrix(matrix: &[[i32; 3]; 3], target: i32) -> Option<(usize, usize)> {
    assessment_harness::todo_exercise(
        "exercise_04_06_find_in_matrix",
        "用 'outer 标签 + break 'outer 做二维查找，返回 Some((行, 列))，找不到返回 None",
        (matrix, target),
    )
}

// ===========================================================================
// kp_04_07 Collatz 步数：循环 + 计数 + 终止条件
// ===========================================================================

/// 知识点考核：用循环统计 Collatz 步数，重点验证终止条件与 1 的处理。
#[test]
fn kp_04_07_collatz_steps() {
    assess(
        M,
        "kp_04_07",
        "循环实现 Collatz 步数：偶数减半、奇数 3n+1，数到 1 为止",
        Kind::Edge,
        "复习 lesson_04 示例 3 与示例 4：这一题考的是「循环 + 计数器 + 终止条件」三件套；\
         关键是**先判断是否已经等于 1**——n 本来就是 1 时一步都不该走，返回 0。",
        || {
            // 正常用例：6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1，共 8 步
            eq(
                exercise_04_07_collatz_steps(6),
                8,
                "6 要经过 8 次变换才到 1：6→3→10→5→16→8→4→2→1",
            );
            // 正常用例：3 → 10 → 5 → 16 → 8 → 4 → 2 → 1，共 7 步
            eq(
                exercise_04_07_collatz_steps(3),
                7,
                "3 要经过 7 次变换才到 1：3→10→5→16→8→4→2→1",
            );
            // 正常用例：4 → 2 → 1，共 2 步
            eq(exercise_04_07_collatz_steps(4), 2, "4 → 2 → 1 需要 2 步");
            // 边界：n = 1 时循环体一次都不执行
            eq(
                exercise_04_07_collatz_steps(1),
                0,
                "1 已经在终点，步数必须是 0（不要写成 1）",
            );
            // 边界：最小的非平凡输入
            eq(
                exercise_04_07_collatz_steps(2),
                1,
                "2 是偶数，2 / 2 = 1，只走 1 步",
            );
        },
    );
}

/// 【待实现】统计 n 变成 1 所需的 Collatz 步数。
///
/// 实现要求：
///   - 用 `let mut current = n;` 与 `let mut steps = 0u32;` 做循环状态；
///   - 用 `loop`（或 `while current != 1`）循环：`current` 是偶数就 `/= 2`，
///     是奇数就 `current = 3 * current + 1`，每变换一次 `steps += 1`；
///   - 当 `current == 1` 时用 `break` 结束，返回 `steps`；
///   - 终止条件必须先判断：`n = 1` 时不应进入循环体，用 `while current != 1` 或
///     在 `loop` 里先 `if current == 1 { break; }`；步数用 `u32` 足够，本题规模不会溢出。
///
/// 示例输入：
/// ```text
/// n = 6
/// ```
/// 示例输出：
/// ```text
/// 8
/// ```
fn exercise_04_07_collatz_steps(n: u64) -> u32 {
    assessment_harness::todo_exercise(
        "exercise_04_07_collatz_steps",
        "用循环统计 Collatz 步数：偶数 /2、奇数 3n+1，直到 1；n = 1 时返回 0",
        (n,),
    )
}

// ===========================================================================
// kp_04_08 常见错误诊断：读得懂编译器报错，才改得动循环
// ===========================================================================

/// 知识点考核：根据课程示例 7 的错误编号，写出三个场景对应的编译器错误码。
#[test]
fn kp_04_08_diagnose_errors() {
    assess(
        M,
        "kp_04_08",
        "常见错误诊断：E0308（loop 的值）/ E0571（while 里 break 带值）/ E0425（循环外用了循环变量）",
        Kind::Hard,
        "复习 lesson_04 示例 7（common_mistakes）：本课列了 7 个坑，其中**有编译器错误编号**的\
         第一个是「loop 体最后是表达式却没写 break 带值」（E0308），第二个是\
         「while 循环里 break 带值」（E0571），第三个是「循环变量在循环外被使用」（E0425）；\
         注意「while 条件永远不变」属于运行期死循环，没有错误编号。",
        || {
            eq_slice(
                &exercise_04_08_diagnose_errors(),
                &["E0308", "E0571", "E0425"],
                "顺序必须是：loop 的值类型不匹配（E0308）、while 里 break 带值（E0571）、\
                 循环变量在循环外不可见（E0425）",
            );
        },
    );
}

/// 【待实现】写出三个循环场景对应的编译器错误编号。
///
/// 场景（与课程示例 7 一致，按课程注释顺序）：
///   1. `let value = loop { 1 };` —— `loop` 体最后是表达式却没写 `break` 带值，
///      表达式被当成 `()`，与 `let value` 的推断类型冲突；
///   2. `let n = while i < 5 { break 1; };` —— 在 `while` 循环里用 `break` 带值；
///   3. `for index in 0..3 { } println!("{index}");` —— 循环变量在循环外被使用。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0308"`），顺序与上面一致；
///   - 这三个编号在课程示例 7 的注释里都能找到；示例 7 的「错误 2（while 条件永远不变）」
///     是运行期死循环、**没有**错误编号，所以本题取的是前三个带编译器错误编号的示例；
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0308", "E0571", "E0425"]
/// ```
fn exercise_04_08_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_04_08_diagnose_errors",
        "返回 [\"E0308\", \"E0571\", \"E0425\"]（loop 的值 / while 里 break 带值 / 循环外用了循环变量）",
        (),
    )
}
