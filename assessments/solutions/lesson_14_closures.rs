//! assessments/lesson_14_closures.rs —— 考核：闭包（对应 lesson_14）
//!
//! - 对应课程：`src/tutorial/lesson_14_closures.rs`
//! - 知识点出处：`src/tutorial/README.md` 第五阶段「14 闭包」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_14_closures     # 只考这一课
//!   cargo test                               # 考全部 18 课
//!   cargo run --bin assessment_report        # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_14_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_14_closures`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 练习函数里需要的 `use`（例如 `std::cell::Cell`）请写在**函数体内**，
//!   这样骨架态不会出现 unused import 警告；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 10）
//!
//! | 场景 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | 闭包第一次用 `locked(1)` 锁定参数为 `i32`，之后又传字符串 | `error[E0308]` mismatched types | 闭包参数类型一旦被推断就固定；需要多态就写泛型函数，或在定义处标注 `x: i32` |
//! | 闭包的 `&mut` 捕获还活着时就读取同一变量 | `error[E0502]` cannot borrow as immutable because it is also borrowed as mutable | 先完成闭包的全部调用，或用一个作用域把闭包限制起来，再读变量 |
//! | `move` 之后又使用原变量 | `error[E0382]` borrow of moved value | 两边都要用就先 `clone` 再 `move`；只想读就去掉 `move` |
//! | 返回闭包时按引用捕获局部变量 | `error[E0373]` closure may outlive the current function | 写 `move`，把值搬进闭包（课程示例 7 的写法） |
//! | 同一个 `FnMut` 闭包被两处同时可变借用 | `error[E0499]` cannot borrow as mutable more than once at a time | 需要共享调用就用 `Rc<RefCell<F>>`（见 lesson_16）或 `Arc<Mutex<F>>`（见 lesson_17） |

use assessment_harness::{Kind, assess, eq, eq_slice, is_true};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_14";

// ===========================================================================
// kp_14_01 闭包基础：写 |x: i32| base + x，捕获定义处的环境
// ===========================================================================

/// 知识点考核：定义捕获 `base` 的闭包，并调用两次。
#[test]
fn kp_14_01_closure_basics() {
    assess(
        M,
        "kp_14_01",
        "闭包基础：竖线参数、可省花括号、闭包能捕获定义处的变量",
        Kind::Basic,
        "复习 lesson_14 示例 1（syntax_vs_function）：`let add = |x: i32| base + x;` \
         与普通函数的第一个区别就是——闭包体里的 `base` 来自定义处环境（这里按不可变借用捕获）。",
        || {
            // 正常用例：base = 5 → 闭包(1) = 6、闭包(10) = 15
            let (one, ten) = exercise_14_01_closure_basics(5);
            eq(one, 6, "闭包以 x = 1 调用时应返回 base + 1 = 6");
            eq(ten, 15, "闭包以 x = 10 调用时应返回 base + 10 = 15");
            // 边界：base = 0，闭包退化成「原样返回 x」
            eq(
                exercise_14_01_closure_basics(0),
                (1, 10),
                "边界 base = 0：base + 1 = 1、base + 10 = 10，不能 panic",
            );
            // 边界：负数
            eq(
                exercise_14_01_closure_basics(-3),
                (-2, 7),
                "边界负数 base = -3：-3 + 1 = -2、-3 + 10 = 7",
            );
        },
    );
}

/// 【待实现】用闭包捕获环境变量。
///
/// 实现要求：
///   - 在函数体内定义 `let add = |x: i32| base + x;`（**必须**显式写 `x: i32`，
///     否则参数类型要靠第一次调用推断）；
///   - 闭包写在函数体里就自动「看得见」`base`，这是它与普通函数
///     （`fn add(x: i32)`）最本质的差别；闭包只读捕获 `base`，所以它是 `Fn`，
///     可以被调用任意多次；
///   - 返回 `(add(1), add(10))`，即 `(base + 1, base + 10)`。
///
/// 示例输入：
/// ```text
/// base = 5
/// ```
/// 示例输出：
/// ```text
/// (6, 15)
/// ```
fn exercise_14_01_closure_basics(base: i32) -> (i32, i32) {
    // 闭包体里的 `base` 来自定义处环境（只读捕获 → Fn，可反复调用）
    let add = |x: i32| base + x;
    (add(1), add(10))
}

// ===========================================================================
// kp_14_02 Fn：只读捕获的闭包可以调用任意多次
// ===========================================================================

/// 知识点考核：把一个 `Fn` 闭包调用 `times` 次并求和。
#[test]
fn kp_14_02_fn_trait() {
    assess(
        M,
        "kp_14_02",
        "Fn trait：只读捕获的闭包可被调用任意多次（泛型参数 + Fn bound）",
        Kind::Core,
        "复习 lesson_14 示例 5（fn_fnmut_fnonce_hierarchy）与示例 6（closure_as_parameter）：\
         参数写成 `F: Fn() -> i32` 就是要求「可以被调用多次且不会改动捕获值」；\
         在函数里循环 `times` 次调用 `f()` 并累加即可。",
        || {
            // 正常用例：闭包捕获了环境里的 base（只读捕获 → Fn）
            let base = 10;
            eq(
                exercise_14_02_fn_trait(|| base, 3),
                base * 3,
                "捕获 base = 10 的闭包调用 3 次：10 + 10 + 10 = 30",
            );
            // 正常用例：不捕获任何变量的闭包同样是 Fn
            eq(
                exercise_14_02_fn_trait(|| 3, 4),
                12,
                "常量闭包返回 3，调用 4 次：3 * 4 = 12（Fn 不消耗自身，可反复调用）",
            );
            // 边界：一次都不调用
            eq(
                exercise_14_02_fn_trait(|| 7, 0),
                0,
                "边界 times = 0：循环体一次都不执行，应返回 0（而不是 7）",
            );
        },
    );
}

/// 【待实现】按次数调用一个 `Fn` 闭包并累加。
///
/// 实现要求：
///   - 参数是泛型 `F: Fn() -> i32`，函数体里用一个 `let mut total = 0;` 累加；
///   - 循环 `times` 次，每次 `total += f();`
///   - `Fn` 表示「不拿走所有权、也不修改捕获」，所以可以反复调用；若把约束写成
///     `FnOnce`，一次调用就会消耗自身，编译器会报 `use of moved value: f`；
///   - 返回 `total`（`times` 为 0 时自然是 0）。
///
/// 示例输入：
/// ```text
/// f = || 10, times = 3
/// ```
/// 示例输出：
/// ```text
/// 30
/// ```
fn exercise_14_02_fn_trait<F: Fn() -> i32>(f: F, times: usize) -> i32 {
    // `Fn` 不消耗自身，所以循环里可以反复调用 f()
    let mut total = 0;
    for _ in 0..times {
        total += f();
    }
    total
}

// ===========================================================================
// kp_14_03 FnMut：同一个闭包被连续调用，内部状态会累加
// ===========================================================================

/// 知识点考核：依次用两个参数调用**同一个** `FnMut` 闭包。
#[test]
fn kp_14_03_fnmut() {
    assess(
        M,
        "kp_14_03",
        "FnMut trait：可变借用捕获 + 函数参数用 mut 绑定，两次调用共享同一份状态",
        Kind::Core,
        "复习 lesson_14 示例 3（capture_by_mutable_borrow）与示例 5 的 `call_three_times`：\
         闭包体里**写入**了外部变量 → 按 `&mut` 捕获 → 该闭包只实现 FnMut/FnOnce；\
         接收入参时要写 `mut f: F` 才能调用（调用 FnMut 需要 `&mut self`）。",
        || {
            // 正常用例：闭包每次调用先把捕获的计数器 +1，再返回 x + 计数器 ——
            // 所以第二次调用的结果里包含了第一次调用留下的状态
            let mut ticks = 0;
            let bump = |x: i32| {
                ticks += 1;
                x + ticks
            };
            let (first, second) = exercise_14_03_fnmut(bump, 10, 20);
            eq(first, 11, "第 1 次调用：计数器累加到 1，返回 10 + 1 = 11");
            eq(
                second,
                22,
                "第 2 次调用：同一个闭包的状态继续累加，返回 20 + 2 = 22（不是 21）",
            );
            eq(
                ticks,
                2,
                "两次调用改的是同一份捕获状态：闭包被移走后 ticks 最终为 2",
            );
            // 边界：两个参数都是 0，恒等闭包
            eq(
                exercise_14_03_fnmut(|x: i32| x, 0, 0),
                (0, 0),
                "边界 a = b = 0：恒等闭包应返回 (0, 0)",
            );
            // 边界：负数参数（且没有内部状态）
            eq(
                exercise_14_03_fnmut(|x: i32| -x, 0, -5),
                (0, 5),
                "边界 b = -5：取负闭包返回 (0, 5)，符号必须正确",
            );
        },
    );
}

/// 【待实现】依次用 `a`、`b` 调用同一个 `FnMut` 闭包。
///
/// 实现要求：
///   - 函数签名已经是 `mut f: F`（F: FnMut(i32) -> i32），**保留 `mut`**：
///     调用 FnMut 需要 `&mut self`，少了它报
///     `error[E0596]: cannot borrow f as mutable, as it is not declared as mutable`；
///   - 先 `let first = f(a);`，再 `let second = f(b);`；两次调用共用同一个闭包值，
///     所以闭包内部捕获的状态会**连续累加**；
///   - 返回 `(first, second)`。
///
/// 示例输入：
/// ```text
/// f = 闭包每次调用先做 ticks += 1，再返回 x + ticks（ticks 初值 0）
/// a = 10, b = 20
/// ```
/// 示例输出：
/// ```text
/// (11, 22)
/// ```
fn exercise_14_03_fnmut<F: FnMut(i32) -> i32>(mut f: F, a: i32, b: i32) -> (i32, i32) {
    // 两次调用共用同一个闭包值：调用 FnMut 需要 &mut self，所以绑定必须是 mut
    let first = f(a);
    let second = f(b);
    (first, second)
}

// ===========================================================================
// kp_14_04 move 捕获：把所有权搬进闭包，FnOnce 只能调用一次
// ===========================================================================

/// 知识点考核：用 `move` 把 `String` 搬进闭包，调用后把文本带回。
#[test]
fn kp_14_04_move_closure() {
    assess(
        M,
        "kp_14_04",
        "move 捕获与 FnOnce：所有权搬进闭包，调用后由返回值交还",
        Kind::Core,
        "复习 lesson_14 示例 4（capture_by_move）：`move` 把捕获变量的所有权搬进闭包；\
         如果闭包体里用掉了那个值（例如把它当返回值返回），闭包就只能是 `FnOnce`，只允许调用一次。",
        || {
            // 正常用例：move 进闭包的文本被完整带回
            eq(
                exercise_14_04_move_closure(),
                String::from("move 闭包带走了所有权"),
                "闭包应按 move 捕获同一个 String，调用后把文本原样返回",
            );
            // 边界：返回值必须是一份独立的所有权 —— 能再次被 move，且非空
            let owned = exercise_14_04_move_closure();
            let moved_again = owned; // 再次 move：证明所有权确实回到了调用方
            eq(
                moved_again.len(),
                "move 闭包带走了所有权".len(),
                "边界：返回的是独立 String，可被再次 move；len() 按 UTF-8 字节数计算",
            );
            is_true(
                !moved_again.is_empty(),
                "边界：返回文本不应为空（空串说明闭包里的 String 被提前 drop 掉了）",
            );
        },
    );
}

/// 【待实现】用 `move` 闭包搬运一个 `String`。
///
/// 实现要求（全部在函数体内完成）：
///   - `let text = String::from("move 闭包带走了所有权");`
///   - `let take = move || text;`（**必须**写 `move`：闭包不再借用调用方的变量，
///     而是**自己拥有** `text`）；
///   - 这样得到的 `take` 是 `FnOnce`，调用一次就把捕获的 `text` 交了出去，
///     第二次调用会报 `error[E0382]: use of moved value`；
///   - 返回 `take()` 的结果。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// "move 闭包带走了所有权"
/// ```
fn exercise_14_04_move_closure() -> String {
    // move 让闭包自己拥有 text；闭包体把 text 当返回值用掉 → 只能是 FnOnce
    let text = String::from("move 闭包带走了所有权");
    let take = move || text;
    take()
}

// ===========================================================================
// kp_14_05 impl Fn 参数：最简洁的闭包形参写法
// ===========================================================================

/// 知识点考核：把闭包形参写成 `impl Fn`。
#[test]
fn kp_14_05_apply_impl() {
    assess(
        M,
        "kp_14_05",
        "闭包作为参数（impl Trait）：静态分发，写法最简",
        Kind::Core,
        "复习 lesson_14 示例 6（closure_as_parameter）的 `apply_twice`：\
         `fn apply(value: i32, f: impl Fn(i32) -> i32) -> i32 { f(value) }`，\
         `impl Trait` 等价于一个匿名的泛型参数，编译期单态化、零额外开销。",
        || {
            // 正常用例：闭包把输入乘 3
            eq(
                exercise_14_05_apply_impl(|x| x * 3, 4),
                12,
                "f(x) = x * 3、value = 4 时应返回 12",
            );
            // 正常用例：闭包捕获环境（offset）
            let offset = 100;
            eq(
                exercise_14_05_apply_impl(|x| x + offset, 1),
                101,
                "捕获 offset = 100 的闭包：1 + 100 = 101",
            );
            // 边界：value = 0
            eq(
                exercise_14_05_apply_impl(|x| x * 3, 0),
                0,
                "边界 value = 0：0 * 3 = 0",
            );
        },
    );
}

/// 【待实现】`impl Fn` 参数的闭包调用。
///
/// 实现要求：
///   - 参数写成 `f: impl Fn(i32) -> i32`（签名已给出，不要改成 `fn` 指针）；
///   - `impl Fn(i32) -> i32` 与泛型 `F: Fn(i32) -> i32` 是同一件事的两种写法，
///     只是 `impl Trait` 不能在签名里再指代这个类型（要复用类型就写泛型）；
///   - 直接返回 `f(value)`。
///
/// 示例输入：
/// ```text
/// f = |x| x * 3, value = 4
/// ```
/// 示例输出：
/// ```text
/// 12
/// ```
fn exercise_14_05_apply_impl(f: impl Fn(i32) -> i32, value: i32) -> i32 {
    // impl Trait 形参就是匿名泛型：静态分发，直接调用即可
    f(value)
}

// ===========================================================================
// kp_14_06 返回闭包：move + impl Fn，把具体闭包类型藏起来
// ===========================================================================

/// 知识点考核：返回一个「加了 n」的闭包。
#[test]
fn kp_14_06_make_adder() {
    assess(
        M,
        "kp_14_06",
        "返回闭包（impl Fn）：必须 move 把 n 搬进闭包，否则借用随函数结束失效",
        Kind::Hard,
        "复习 lesson_14 示例 7（returning_closure）的 `make_multiplier`：\
         `fn make_multiplier(factor: i64) -> impl Fn(i64) -> i64 { move |x| x * factor }`；\
         去掉 `move` 会报 `error[E0373]: closure may outlive the current function`。",
        || {
            // 正常用例：n = 5 的加法器
            let add5 = exercise_14_06_make_adder(5);
            eq(add5(3), 8, "n = 5 的闭包：3 + 5 = 8");
            // 正常用例：同一个闭包再调用一次（返回的是 Fn，可多次调用）
            eq(add5(10), 15, "同一个加法器再调用一次：10 + 5 = 15");
            // 边界：n = 0
            let add0 = exercise_14_06_make_adder(0);
            eq(add0(0), 0, "边界 n = 0：0 + 0 = 0");
            // 边界：负数
            eq(
                exercise_14_06_make_adder(-4)(4),
                0,
                "边界负数 n = -4：-4 + 4 = 0，符号必须正确",
            );
        },
    );
}

/// 【待实现】返回一个捕获了 `n` 的加法闭包。
///
/// 实现要求：
///   - 返回类型保持 `impl Fn(i32) -> i32`（不要写成 `Box<dyn Fn>`）；
///   - 函数体只有一句：`move |x| x + n`（**必须** `move`：`n` 是局部变量，
///     函数一返回就没了，不加 `move` 的闭包按引用捕获它，编译器会拒绝，报
///     `error[E0373]: closure may outlive the current function`）；
///   - 返回 `impl Fn` 只承诺「某个实现了 `Fn` 的匿名类型」，调用方**不能**
///     写出它的具体类型名，也不能把它和别的闭包混装进同一个集合。
///
/// 示例输入：
/// ```text
/// n = 5
/// ```
/// 示例输出：
/// ```text
/// f(3) = 8
/// ```
fn exercise_14_06_make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| -> i32 {
        // move 把 n 搬进闭包，闭包返回后依然有效；只读捕获 n → impl Fn
        n + x
    }
}

// ===========================================================================
// kp_14_07 Box<dyn Fn>：把类型不同的闭包装进同一个集合
// ===========================================================================

/// 知识点考核：用 `Vec<Box<dyn Fn(i32) -> i32>>` 混装两个不同闭包。
#[test]
fn kp_14_07_boxed_closures() {
    assess(
        M,
        "kp_14_07",
        "Box<dyn Fn>：异构闭包必须装箱才能放进同一个 Vec（动态分发）",
        Kind::Hard,
        "复习 lesson_14 示例 8（box_dyn_closure）：`let ops: Vec<Box<dyn Fn(i32) -> i32>> = \
         vec![Box::new(|x| x + 1), Box::new(move |x| x + offset)];`——\
         两个闭包是**不同的匿名类型**，只有 `dyn Fn` 能把它们统一成同一种元素类型。",
        || {
            // 正常用例：两个闭包都用 7 调用，结果相加
            eq(
                exercise_14_07_boxed_closures(10),
                25,
                "base = 10：(7 + 1) + (7 + 10) = 8 + 17 = 25",
            );
            // 边界：base = 0
            eq(
                exercise_14_07_boxed_closures(0),
                15,
                "边界 base = 0：(7 + 1) + (7 + 0) = 8 + 7 = 15，第二个闭包退化为恒等",
            );
            // 边界：负数
            eq(
                exercise_14_07_boxed_closures(-3),
                12,
                "边界负数 base = -3：8 + (7 - 3) = 8 + 4 = 12",
            );
        },
    );
}

/// 【待实现】用 `Box<dyn Fn>` 混装两个闭包并求和。
///
/// 实现要求：
///   - 建一个 `Vec<Box<dyn Fn(i32) -> i32>>`，装两个**不同**的闭包：
///     第一个 `Box::new(|x| x + 1)`，第二个 `Box::new(move |x| x + base)`；
///   - 两个闭包类型各不相同，而 `Vec` 要求元素同类型，所以必须 `Box` 成 trait
///     对象；代价是堆分配加每次调用一次 vtable 间接跳转；
///   - 用输入值 `7` 依次调用两个闭包，把两次结果相加后返回。
///
/// 示例输入：
/// ```text
/// base = 10
/// ```
/// 示例输出：
/// ```text
/// 25
/// ```
fn exercise_14_07_boxed_closures(base: i32) -> i32 {
    // 两个闭包是不同匿名类型，只有 dyn Fn 能把它们统一成同一种元素类型
    let ops: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x + 1), Box::new(move |x| x + base)];
    let mut total = 0;
    for op in &ops {
        total += op(7);
    }
    total
}

// ===========================================================================
// kp_14_08 FnOnce 任务队列：消费型闭包只能调用一次
// ===========================================================================

/// 知识点考核：把两个 `FnOnce` 任务排队，逐个执行。
#[test]
fn kp_14_08_task_queue() {
    assess(
        M,
        "kp_14_08",
        "任务队列：Vec<Box<dyn FnOnce() -> String>> 按顺序执行消费型任务",
        Kind::Hard,
        "复习 lesson_14 示例 9（scenario_task_queue）：把「行为」作为值存进集合、由调用方按序执行；\
         与示例 8 的 `Box<dyn Fn>` 相比，这里的任务返回 `String` 并耗尽自身，所以 trait 对象要写\
         `Box<dyn FnOnce() -> String>`，也只能调用一次。",
        || {
            // 正常用例：顺序确定，期望值写死
            let queue = exercise_14_08_task_queue();
            eq_slice(
                &queue,
                &[String::from("任务 1 完成"), String::from("任务 2 完成")],
                "两个任务必须按入队顺序执行：先「任务 1 完成」，再「任务 2 完成」",
            );
            // 边界：任务数量恰好是两个，多一个或少一个都说明队列没跑对
            eq(
                queue.len(),
                2,
                "边界：两个 FnOnce 各调用一次，结果长度恰好为 2",
            );
        },
    );
}

/// 【待实现】用 `FnOnce` 任务队列顺序执行两个任务。
///
/// 实现要求：
///   - 建一个 `Vec<Box<dyn FnOnce() -> String>>`，装两个任务：
///     第 1 个返回 `String::from("任务 1 完成")`，第 2 个返回 `String::from("任务 2 完成")`；
///   - `FnOnce` 的调用会消耗 `self`，所以任务必须**按值**取出（`into_iter()` 而不是
///     `iter()`）；`dyn FnOnce()` 只能在 `Box` 后面使用，写 `Box<dyn FnOnce()>`
///     是标准库专门支持的特例；
///   - 按顺序逐个调用任务，把返回值 `push` 进 `Vec<String>` 后返回；
///   - 返回的 `Vec` 顺序必须与入队顺序一致，即 `["任务 1 完成", "任务 2 完成"]`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["任务 1 完成", "任务 2 完成"]
/// ```
fn exercise_14_08_task_queue() -> Vec<String> {
    // FnOnce 调用会消耗 self → 必须按值取出（into_iter 而不是 iter）
    let queue: Vec<Box<dyn FnOnce() -> String>> = vec![
        Box::new(|| String::from("任务 1 完成")),
        Box::new(|| String::from("任务 2 完成")),
    ];
    let mut results: Vec<String> = Vec::new();
    for task in queue.into_iter() {
        results.push(task());
    }
    results
}

// ===========================================================================
// kp_14_09 常见错误诊断：读得懂编译器报错，才改得动闭包
// ===========================================================================

/// 知识点考核：写出课程示例 10 里前三个错误对应的编译器编号。
#[test]
fn kp_14_09_diagnose_errors() {
    assess(
        M,
        "kp_14_09",
        "常见错误诊断：闭包参数类型锁定 E0308 / 可变借用冲突 E0502 / move 后使用 E0382",
        Kind::Hard,
        "复习 lesson_14 示例 10（common_mistakes）：本课列的 5 个坑按注释顺序是——\
         错误 1 闭包参数类型被推断锁定（E0308）、错误 2 闭包持有可变借用时又读同一变量（E0502）、\
         错误 3 move 之后又使用原变量（E0382）；再往后还有 E0373 与 E0499。",
        || {
            let codes = exercise_14_09_diagnose_errors();
            // 正常用例：前三个错误编号，顺序必须与课程注释一致
            eq_slice(
                &codes,
                &["E0308", "E0502", "E0382"],
                "顺序必须是：闭包参数类型锁定（E0308）、可变借用冲突（E0502）、move 后使用（E0382）",
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

/// 【待实现】写出三个闭包场景对应的编译器错误编号。
///
/// 场景（与课程示例 10 的注释顺序一致）：
///   1. 闭包第一次用 `locked(1)` 锁定了 `i32`，之后又传字符串 —— 类型不匹配；
///   2. 闭包还持有 `&mut total` 时又 `println!("{total}")` —— 可变借用与不可变借用冲突；
///   3. `let take = move || println!("{data}");` 之后再 `println!("{data}")`
///      —— 使用了被 move 的值。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0382"`），顺序与上面一致。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0308", "E0502", "E0382"]
/// ```
fn exercise_14_09_diagnose_errors() -> [&'static str; 3] {
    // 顺序：类型不匹配 / 可变借用与不可变借用冲突 / move 后使用
    ["E0308", "E0502", "E0382"]
}
