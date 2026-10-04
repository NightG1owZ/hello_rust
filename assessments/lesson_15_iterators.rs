//! assessments/lesson_15_iterators.rs —— 考核：迭代器（对应 lesson_15）
//!
//! - 对应课程：`src/tutorial/lesson_15_iterators.rs`
//! - 知识点出处：`src/tutorial/README.md` 第五阶段「15 迭代器」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_15_iterators     # 只考这一课
//!   cargo test                                 # 考全部 18 课
//!   cargo run --bin assessment_report          # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_15_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_15_iterators`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体与 `impl Counter` 里的两个方法，
//!   可以按需增加局部变量与辅助函数；
//! - 练习函数里需要的 `use`（例如 `std::cell::Cell`）请写在**函数体内**，
//!   这样骨架态不会出现 unused import 警告；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 10）
//!
//! | 场景 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | 迭代（`for x in &v`）期间又 `v.push(..)` | `error[E0502]` cannot borrow as mutable because it is also borrowed as immutable | 先把要追加的值收集到新 `Vec`，循环结束后再 `extend` |
//! | `into_iter()` 消费集合后又使用原集合 | `error[E0382]` borrow of moved value | 需要保留集合就用 `iter()`；确定消费就之后别再碰它 |
//! | `collect()` 的目标类型推断不出来 | `error[E0282]` type annotations needed | 写明接收类型 `let xs: Vec<i32> = ...`，或 `collect::<Vec<i32>>()` |
//! | 只有适配器没有消费器 | 编译通过但一行都不执行 | 接上 `collect` / `sum` / `for_each` 等消费器，适配器才被驱动 |
//! | `find` 返回 `&T`，却当值继续参与运算 | `error[E0369]` cannot multiply `&i32` by `{integer}` | 先用 `.copied()` 拿到 `i32` 再算，或用模式解构把 `&i32` 解掉 |

use assessment_harness::{Kind, assess, eq, eq_slice};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_15";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// ===========================================================================

/// 计数器迭代器：`next()` 依次产出 `1, 2, ..., max`（闭区间，`max` 本身也要产出）。
///
/// 字段含义（实现 `next()` 时按这个约定来）：
///   - `count`：已经产出到几，初始为 `0`，`next()` 里**先自增再判断**；
///   - `max`：产出上限。
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    /// 【待实现】构造一个上限为 `max` 的计数器。
    ///
    /// 实现要求：
    ///   - 返回 `Counter { count: 0, max }`：`count` 必须从 0 开始，这样 `next()`
    ///     第一次自增后恰好产出 1；
    ///   - 结构体字面量可以简写：字段名与变量名同名时写 `max` 就等于 `max: max`。
    ///
    /// 示例输入：
    /// ```text
    /// max = 3
    /// ```
    /// 示例输出：
    /// ```text
    /// 该迭代器产出 1、2、3（`sum()` = 6；`Counter` 没有实现 `Debug`，所以不做打印断言）
    /// ```
    fn new(max: u32) -> Self {
        assessment_harness::todo_exercise(
            "Counter::new",
            "返回 Counter { count: 0, max }（count 从 0 开始，next() 里先自增再判断）",
            (max,),
        )
    }
}

impl Iterator for Counter {
    type Item = u32;

    /// 【待实现】推进计数器：产出 1..=max，耗尽后返回 `None`。
    ///
    /// 实现要求：
    ///   - `self.count` 先 `+= 1`；
    ///   - 若 `self.count <= self.max` 则返回 `Some(self.count)`，否则返回 `None`；
    ///   - `max = 0` 时第一次调用就必须返回 `None`（空迭代器）；
    ///   - `next()` 返回 `None` 之后，适配器/消费器会立刻停止驱动，所以
    ///     `Counter::new(3).sum::<u32>()` 只会取到 1、2、3。
    ///
    /// 示例输入：
    /// ```text
    /// self = Counter { count: 0, max: 3 }
    /// ```
    /// 示例输出：
    /// ```text
    /// Some(1)
    /// ```
    fn next(&mut self) -> Option<u32> {
        assessment_harness::todo_exercise(
            "Counter::next",
            "count 先 +1；count <= max 时返回 Some(count)，否则返回 None",
            (&mut self.count, self.max),
        )
    }
}

// ===========================================================================
// kp_15_01 for 循环求和：后面所有迭代器写法的对照基准
// ===========================================================================

/// 知识点考核：用 `for` 循环遍历切片求和。
#[test]
fn kp_15_01_sum_with_for() {
    assess(
        M,
        "kp_15_01",
        "for 循环与 Iterator 的关系：for 只是「反复调用 next()」的语法糖",
        Kind::Basic,
        "复习 lesson_15 示例 1（iterator_and_for）：`for n in &nums` 等价于 `(&nums).into_iter()` \
         加一个循环；本题先用最朴素的 `for` 写出求和，作为后面 `iter().sum()` 的对照。",
        || {
            // 正常用例
            eq(
                exercise_15_01_sum_with_for(&[1, 2, 3, 4]),
                10,
                "1 + 2 + 3 + 4 = 10",
            );
            // 正常用例：只读借用的 for 循环不消费切片
            let items = vec![10, 20, 30];
            eq(
                exercise_15_01_sum_with_for(&items),
                60,
                "10 + 20 + 30 = 60；练习函数接收 &[i32]，不会拿走 items",
            );
            // 边界：空切片
            eq(
                exercise_15_01_sum_with_for(&[]),
                0,
                "边界：空切片应返回 0（循环体一次都没执行），不能 panic",
            );
            // 边界：负数
            eq(
                exercise_15_01_sum_with_for(&[-5, 5, -10]),
                -10,
                "边界负数：-5 + 5 + (-10) = -10，累加器要从 0 开始",
            );
        },
    );
}

/// 【待实现】用 `for` 循环求和。
///
/// 实现要求：
///   - 用 `let mut total = 0;` 作为累加器；
///   - 遍历求和：`for x in items { total += *x; }`（或写 `for &x in items { total += x; }`）；
///   - 返回 `total`；空切片时循环体一次都不执行，直接返回 0。
///
/// 示例输入：
/// ```text
/// items = [1, 2, 3, 4]
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_15_01_sum_with_for(items: &[i32]) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_15_01_sum_with_for",
        "用 for 循环遍历 items 累加求和并返回（空切片返回 0）",
        (items,),
    )
}

// ===========================================================================
// kp_15_02 三种迭代方式的所有权差别：用 iter_mut() 原地改写
// ===========================================================================

/// 知识点考核：用 `iter_mut()` 把每个元素原地翻倍，并返回求和与个数。
#[test]
fn kp_15_02_three_modes() {
    assess(
        M,
        "kp_15_02",
        "iter / iter_mut / into_iter 的所有权差别：iter_mut() 产出 &mut T，可原地改写",
        Kind::Core,
        "复习 lesson_15 示例 2（three_iteration_modes）：`iter()` 借用产出 `&T`（集合保留）、\
         `iter_mut()` 可变借用产出 `&mut T`（原地改写）、`into_iter()` 消费集合产出 `T`（集合被移走）；\
         本题用 `for n in values.iter_mut() { *n *= 2; }` 改写后，再统计和与个数。",
        || {
            // 正常用例：原地翻倍 + 统计
            let mut values = vec![1, 2, 3];
            let (sum, count) = exercise_15_02_three_modes(&mut values);
            eq(sum, 12, "原地翻倍后是 [2, 4, 6]，2 + 4 + 6 = 12");
            eq(count, 3, "元素个数是 3");
            eq_slice(
                &values,
                &[2, 4, 6],
                "iter_mut() 的副作用必须落到原 Vec 上：调用方拿到的 values 已被改写",
            );
            // 边界：空 Vec
            let mut empty: Vec<i32> = Vec::new();
            eq(
                exercise_15_02_three_modes(&mut empty),
                (0, 0),
                "边界空 Vec → (0, 0)：不 panic、不返回 count = 1",
            );
            // 边界：负数与 0
            let mut mixed = vec![-3, 0, 7];
            eq(
                exercise_15_02_three_modes(&mut mixed),
                (8, 3),
                "边界：翻倍后 [-6, 0, 14]，和 = -6 + 0 + 14 = 8，元素个数仍是 3",
            );
            eq_slice(&mixed, &[-6, 0, 14], "边界：负数翻倍后仍是负数");
        },
    );
}

/// 【待实现】用 `iter_mut()` 原地翻倍，并返回 (和, 个数)。
///
/// 实现要求：
///   - 用 `for n in values.iter_mut() { *n *= 2; }` 把每个元素**原地**翻倍
///     （也可以写成 `values.iter_mut().for_each(|n| *n *= 2);`）；
///   - 返回 `(values.iter().sum::<i32>(), values.len() as i32)`；
///   - 三种迭代方式的差别就在所有权：`iter()` 借出 `&T`、`iter_mut()` 借出 `&mut T`、
///     `into_iter()` 交出元素所有权让集合失效（`error[E0382]`）；
///   - 本题必须用 `iter_mut()`，因为调用方（测试）后面还要检查同一个 `Vec`。
///
/// 示例输入：
/// ```text
/// values = [1, 2, 3]
/// ```
/// 示例输出：
/// ```text
/// (12, 3)
/// ```
fn exercise_15_02_three_modes(values: &mut Vec<i32>) -> (i32, i32) {
    assessment_harness::todo_exercise(
        "exercise_15_02_three_modes",
        "用 iter_mut() 把 values 里每个元素原地翻倍，返回 (翻倍后元素之和, 元素个数)",
        (values,),
    )
}

// ===========================================================================
// kp_15_03 适配器流水线：enumerate + filter + map
// ===========================================================================

/// 知识点考核：用适配器组合出「带原始下标」的偶数翻倍列表。
#[test]
fn kp_15_03_adapters() {
    assess(
        M,
        "kp_15_03",
        "适配器流水线：enumerate + filter + map（惰性组合，不打乱原始下标）",
        Kind::Core,
        "复习 lesson_15 示例 3（adapters）：`nums.iter().enumerate().filter(..).map(..).collect()`；\
         注意本题 `enumerate` 放在 `filter` **之前**，所以保留下来的下标是原始下标（会跳号），\
         不是过滤后的序号。",
        || {
            // 正常用例：下标是过滤前的原始下标，奇数位置被跳过
            eq(
                exercise_15_03_adapters(&[1, 2, 3, 4]),
                vec![(1usize, 4i32), (3, 8)],
                "enumerate 在前：保留 (原始下标 1, 2*2) 与 (原始下标 3, 4*2)",
            );
            // 边界：空切片
            eq(
                exercise_15_03_adapters(&[]),
                Vec::new(),
                "边界：空切片 → 空 Vec（适配器链不 panic）",
            );
            // 边界：负数与 0 都是偶数
            eq(
                exercise_15_03_adapters(&[-2, 0, 5]),
                vec![(0usize, -4i32), (1, 0)],
                "边界：-2 与 0 都能被 2 整除，翻倍后是 -4 与 0",
            );
        },
    );
}

/// 【待实现】用适配器组合出偶数翻倍列表。
///
/// 实现要求：
///   - 依次接上 `enumerate()`、`filter(|(_, x)| **x % 2 == 0)`、`map(|(i, x)| (i, *x * 2))`；
///   - 用 `collect()` 收成 `Vec<(usize, i32)>` 并返回；
///   - `filter` 的闭包收到的是 `&(usize, &i32)`，所以判断写法是 `**x % 2 == 0`
///     （也可以用模式解构 `|&(_, x)| *x % 2 == 0`，代码更干净）；
///   - `enumerate()` 放在 `filter` 之前才能拿到原始下标。
///
/// 示例输入：
/// ```text
/// items = [1, 2, 3, 4]
/// ```
/// 示例输出：
/// ```text
/// [(1, 4), (3, 8)]
/// ```
fn exercise_15_03_adapters(items: &[i32]) -> Vec<(usize, i32)> {
    assessment_harness::todo_exercise(
        "exercise_15_03_adapters",
        "enumerate + filter（保留偶数）+ map（值 × 2）组成流水线，collect 成 Vec<(usize, i32)>",
        (items,),
    )
}

// ===========================================================================
// kp_15_04 消费器：sum / max / count / any 一次取四个结果
// ===========================================================================

/// 知识点考核：依次用四个消费器取出结果。
#[test]
fn kp_15_04_consumers() {
    assess(
        M,
        "kp_15_04",
        "消费器：sum() / max() / count() / any()（只有消费器才驱动流水线）",
        Kind::Core,
        "复习 lesson_15 示例 4（consumers）：`sum::<i32>()` 求和、`max()` 返回 `Option<&T>`、\
         `count()` 返回元素个数、`any(pred)` 返回是否**存在**满足条件的元素；\
         本题返回 `(和, 最大值, 个数, 是否有负数)`。",
        || {
            // 正常用例：全为正数，没有负数
            eq(
                exercise_15_04_consumers(&[4, 9, 1, 7]),
                (21, Some(9), 4usize, false),
                "4 + 9 + 1 + 7 = 21，max = Some(9)，count = 4，any(负数) = false",
            );
            // 正常用例：含负数
            eq(
                exercise_15_04_consumers(&[-3, 5]),
                (2, Some(5), 2usize, true),
                "-3 + 5 = 2，max = Some(5)，count = 2，any(|x| *x < 0) = true",
            );
            // 边界：空切片 —— max() 只能是 None
            eq(
                exercise_15_04_consumers(&[]),
                (0, None, 0usize, false),
                "边界空切片：总和 0、max 为 None、count 为 0、any 为 false",
            );
        },
    );
}

/// 【待实现】用四个消费器一次性取出结果。
///
/// 实现要求：返回一个四元组 `(sum, max, count, has_negative)`：
///   - `items.iter().sum::<i32>()`；
///   - `items.iter().max().copied()`（`max()` 返回 `Option<&i32>`，用 `copied()` 变成 `Option<i32>`）；
///   - `items.iter().count()`；
///   - `items.iter().any(|x| *x < 0)`；
///   - 空切片时 `any` 是 `false`、`max` 是 `None`——这就是消费器在「无元素」时的定义。
///
/// 示例输入：
/// ```text
/// items = [4, 9, 1, 7]
/// ```
/// 示例输出：
/// ```text
/// (21, Some(9), 4, false)
/// ```
fn exercise_15_04_consumers(items: &[i32]) -> (i32, Option<i32>, usize, bool) {
    assessment_harness::todo_exercise(
        "exercise_15_04_consumers",
        "返回 (sum, max, count, any(|x| *x < 0))：sum::<i32>() / max().copied() / count() / any()",
        (items,),
    )
}

// ===========================================================================
// kp_15_05 fold 折叠与 collect 收集：自定义聚合 + 换容器
// ===========================================================================

/// 知识点考核：用 `fold` 求和、用 `collect` 生成字符串列表。
#[test]
fn kp_15_05_fold_collect() {
    assess(
        M,
        "kp_15_05",
        "fold 与 collect：fold 是自定义聚合，collect 是「换容器」",
        Kind::Core,
        "复习 lesson_15 示例 5（fold_and_collect）：`fold(初始值, |acc, x| 新累积值)` 可以表达任意聚合\
         （求和、连乘、拼 CSV……）；`collect()` 的目标类型由接收方决定，这里用 `map` 生成\
         `n=<值>` 的字符串再收成 `Vec<String>`。",
        || {
            // 正常用例
            eq(
                exercise_15_05_fold_collect(&[1, 2, 3]),
                (
                    6,
                    vec![
                        String::from("n=1"),
                        String::from("n=2"),
                        String::from("n=3"),
                    ],
                ),
                "fold 求和 = 6；collect 得到 [\"n=1\", \"n=2\", \"n=3\"]（顺序与原切片一致）",
            );
            // 正常用例：负数要带上负号
            eq(
                exercise_15_05_fold_collect(&[-2, 5]),
                (3, vec![String::from("n=-2"), String::from("n=5")]),
                "-2 + 5 = 3；字符串里负数写成 n=-2，不要丢掉负号",
            );
            // 边界：空切片
            eq(
                exercise_15_05_fold_collect(&[]),
                (0, Vec::new()),
                "边界空切片：fold 返回初始值 0，collect 得到空 Vec",
            );
        },
    );
}

/// 【待实现】`fold` 求和 + `collect` 生成字符串列表。
///
/// 实现要求：
///   - 求和：`items.iter().fold(0, |acc, x| acc + *x)`（初始值是 0，空切片时必须返回 0）；
///   - 字符串列表：`items.iter().map(|x| format!("n={x}")).collect::<Vec<String>>()`；
///   - 返回 `(和, 字符串列表)`；
///   - `fold` 的第一个参数是**初始累积值**，它的类型决定了累积值的类型；
///   - `collect` 一定要让编译器知道收成什么（靠 `Vec<String>` 返回类型或 turbofish）。
///
/// 示例输入：
/// ```text
/// items = [1, 2, 3]
/// ```
/// 示例输出：
/// ```text
/// (6, ["n=1", "n=2", "n=3"])
/// ```
fn exercise_15_05_fold_collect(items: &[i32]) -> (i32, Vec<String>) {
    assessment_harness::todo_exercise(
        "exercise_15_05_fold_collect",
        "用 fold(0, |acc, x| acc + *x) 求和，用 map + collect 生成 [\"n=<值>\", ...]，返回 (和, 列表)",
        (items,),
    )
}

// ===========================================================================
// kp_15_06 惰性证明：用 Cell 数出 map 闭包真正被调用的次数
// ===========================================================================

/// 知识点考核：用 `Cell<usize>` 记录 `map` 闭包的真实调用次数。
#[test]
fn kp_15_06_laziness() {
    assess(
        M,
        "kp_15_06",
        "惰性与短路：适配器只搭流水线，消费器才按需驱动（take(3) 只加工 3 个元素）",
        Kind::Edge,
        "复习 lesson_15 示例 6（laziness_proof）：`map` 的闭包在 `collect()` 之前一次都不会执行；\
         本题用 `std::cell::Cell<usize>` 计数，`(0..100).map(计数).take(3).collect()` 只会加工 3 个元素——\
         这就是「惰性 + 按需驱动」的事实证据（不是 100 次，也不是 0 次）。",
        || {
            let (calls, len) = exercise_15_06_laziness();
            // 正常用例：只有被消费的 3 个元素经过了 map
            eq(
                calls,
                3,
                "take(3) 只让 3 个元素流过 map，闭包恰好被调用 3 次",
            );
            // 正常用例：结果长度
            eq(len, 3, "collect 得到 3 个元素");
            // 边界：两个数字必须相等 —— 一次调用对应一个结果元素，说明没有多余加工
            eq(
                calls,
                len,
                "边界：调用次数恰好等于结果长度，100 个元素里的另外 97 个一次都没被加工",
            );
        },
    );
}

/// 【待实现】用 `Cell` 统计 `map` 闭包的真实调用次数。
///
/// 实现要求（全部在函数体内完成）：
///   - 在函数体内 `use std::cell::Cell;`，然后 `let calls = Cell::new(0usize);`
///   - 搭流水线：`(0..100).map(|i| { calls.set(calls.get() + 1); i }).take(3).collect::<Vec<_>>()`
///   - 返回 `(calls.get(), 结果.len())`，即 `(3, 3)`；
///   - `Cell` 提供「内部可变性」，所以 `map` 的闭包按 `&calls` 捕获也能改计数
///     （用 `let mut calls` 会被闭包可变借用，写法更绕）；
///   - 适配器是惰性的：`map` 只在 `collect` 按需拉动时才执行，`take(3)` 只让 3 个元素
///     流过流水线，计数与结果长度都是 3，而不是 100。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (3, 3)
/// ```
fn exercise_15_06_laziness() -> (usize, usize) {
    assessment_harness::todo_exercise(
        "exercise_15_06_laziness",
        "用 std::cell::Cell<usize> 统计 map 闭包调用次数：\
         (0..100).map(计数).take(3).collect::<Vec<_>>()，返回 (调用次数, 结果长度)",
        (),
    )
}

// ===========================================================================
// kp_15_07 自定义迭代器：为 Counter 实现 Iterator
// ===========================================================================

/// 知识点考核：为提供的 `Counter` 实现 `Iterator`，接入整个迭代器生态。
#[test]
fn kp_15_07_custom_iterator() {
    assess(
        M,
        "kp_15_07",
        "自定义迭代器：实现 Counter::new 与 Iterator::next，免费获得全部适配器与消费器",
        Kind::Hard,
        "复习 lesson_15 示例 8（custom_iterator）：自定义类型只要实现 `Iterator` 的 `next()`\
         （外加关联类型 `Item`），就能直接用 `sum()` / `collect()` / `filter()` 等全套工具；\
         本题的 `Counter` 产出 `1..=max`，耗尽后 `next()` 必须返回 `None`。",
        || {
            // 正常用例：sum 消费器
            let sum: u32 = Counter::new(3).sum();
            eq(sum, 6, "Counter::new(3) 产出 1、2、3，sum = 6");
            // 正常用例：collect 消费器，顺序必须是从小到大
            let collected: Vec<u32> = Counter::new(3).collect();
            eq(collected, vec![1u32, 2, 3], "顺序必须是 [1, 2, 3]");
            // 边界：max = 0 → 空迭代器
            eq(
                Counter::new(0).collect::<Vec<u32>>(),
                Vec::new(),
                "边界 max = 0：第一次 next() 就返回 None，collect 得到空 Vec",
            );
            // 边界：max = 1 → 只产出一个元素
            eq(
                Counter::new(1).sum::<u32>(),
                1,
                "边界 max = 1：闭区间只包含 1，sum = 1（不要多产出 0）",
            );
        },
    );
}

// ===========================================================================
// kp_15_08 词频统计：split / 小写 / 计数 / 确定性排序
// ===========================================================================

/// 知识点考核：统计词频并按「次数降序、同次数按字典序」返回。
#[test]
fn kp_15_08_word_freq() {
    assess(
        M,
        "kp_15_08",
        "词频统计：split_whitespace + 统一小写 + 计数 + 确定性排序（次数降序、字典序升序）",
        Kind::Hard,
        "复习 lesson_15 示例 9（scenario_word_freq）：`text.split_whitespace()` 切词、\
         `HashMap` 计数（entry API 见 lesson_08）、收集成 `Vec` 后\
         `sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))` 让输出**确定**。",
        || {
            // 正常用例：与课程示例 9 同一段文本
            eq(
                exercise_15_08_word_freq("the quick brown fox the lazy dog the fox"),
                vec![
                    (String::from("the"), 3usize),
                    (String::from("fox"), 2),
                    (String::from("brown"), 1),
                    (String::from("dog"), 1),
                    (String::from("lazy"), 1),
                    (String::from("quick"), 1),
                ],
                "the 3 次、fox 2 次，其余各 1 次；次数相同时按字典序：brown < dog < lazy < quick",
            );
            // 边界：大小写与多余空白都要归并到同一个词
            eq(
                exercise_15_08_word_freq("Rust rust  RUST\nrust\t"),
                vec![(String::from("rust"), 4usize)],
                "边界：统一小写 + 按空白切分（空格、换行、制表符、连续空白）后只剩一个词 rust",
            );
            // 边界：空串与纯空白
            eq(
                exercise_15_08_word_freq(""),
                Vec::new(),
                "边界空串 → 空 Vec，不能 panic",
            );
            eq(
                exercise_15_08_word_freq("   \t\n "),
                Vec::new(),
                "边界：全是空白字符时同样没有任何词，返回空 Vec",
            );
        },
    );
}

/// 【待实现】按空白切分统计词频并排序返回。
///
/// 实现要求：
///   - 用 `text.split_whitespace()` 切词，每个词 `to_lowercase()` 统一小写；
///   - 用 `HashMap<String, usize>` 计数（可以用 `entry(w).or_insert(0)` 累加）；
///   - 用 `collect()` 收成 `Vec<(String, usize)>`，再 `sort_by` 排序：
///     **次数降序**，次数相同则**单词字典序升序**；
///   - 返回排序后的 `Vec`（空串或纯空白 → 空 `Vec`）；
///   - `HashMap` 的迭代顺序是随机的，不排序结果就不确定，
///     所以 `sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))` 这一步不能省；
///   - 用 `split_whitespace()` 而不是 `split(' ')`——后者会把连续空白切出空字符串。
///
/// 示例输入：
/// ```text
/// text = "the quick brown fox the lazy dog the fox"
/// ```
/// 示例输出：
/// ```text
/// [("the", 3), ("fox", 2), ("brown", 1), ("dog", 1), ("lazy", 1), ("quick", 1)]
/// ```
fn exercise_15_08_word_freq(text: &str) -> Vec<(String, usize)> {
    assessment_harness::todo_exercise(
        "exercise_15_08_word_freq",
        "按空白切分、统一小写、统计词频，按次数降序（同次数按字典序升序）返回 Vec<(String, usize)>",
        (text,),
    )
}

// ===========================================================================
// kp_15_09 常见错误诊断：迭代器的三个高频报错
// ===========================================================================

/// 知识点考核：写出课程示例 10 里前三个错误对应的编译器编号。
#[test]
fn kp_15_09_diagnose_errors() {
    assess(
        M,
        "kp_15_09",
        "常见错误诊断：迭代期间修改 E0502 / into_iter 后使用 E0382 / collect 类型不明 E0282",
        Kind::Hard,
        "复习 lesson_15 示例 10（common_mistakes）：本课列的 5 个坑按注释顺序是——\
         错误 1 遍历时修改集合（E0502）、错误 2 `into_iter()` 消费后又使用原集合（E0382）、\
         错误 3 `collect()` 目标类型不明（E0282）；再往后还有「只有适配器没有消费器」与\
         `find` 返回引用的运算错误（E0369）。",
        || {
            let codes = exercise_15_09_diagnose_errors();
            // 正常用例：前三个错误编号，顺序必须与课程注释一致
            eq_slice(
                &codes,
                &["E0502", "E0382", "E0282"],
                "顺序必须是：迭代期间修改集合（E0502）、into_iter 后使用原集合（E0382）、\
                 collect 类型不明（E0282）",
            );
            // 边界：编号格式统一是「E + 4 位数字」的 5 个字符
            eq(
                codes.iter().all(|c| c.starts_with('E') && c.len() == 5),
                true,
                "边界：错误编号必须是形如 E0xxx 的 5 个字符，别写成中文说明或大小写混排",
            );
        },
    );
}

/// 【待实现】写出三个迭代器场景对应的编译器错误编号。
///
/// 场景（与课程示例 10 的注释顺序一致）：
///   1. `for x in &v { v.push(*x); }` —— 不可变借用未结束时又要可变借用；
///   2. `let it = v.into_iter();` 之后又用 `v` —— 使用了被 move 的值；
///   3. `let xs = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();` —— 收什么类型说不清。
///
/// 实现要求：返回 3 个错误编号字符串（形如 `"E0502"`），顺序与上面一致；
/// 编号格式是「`E` + 4 位数字」共 5 个字符，不要写成中文说明或大小写混排。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0502", "E0382", "E0282"]
/// ```
fn exercise_15_09_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_15_09_diagnose_errors",
        "返回 [\"E0502\", \"E0382\", \"E0282\"]（迭代期间修改 / move 后使用 / collect 类型不明）",
        (),
    )
}
