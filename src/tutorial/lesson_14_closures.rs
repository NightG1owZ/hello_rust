//! lesson_14_closures.rs —— 主题：闭包（Closure）
//!
//! 学习目标：
//!   1. 会写闭包、分清闭包与函数的差别（可捕获环境、类型可推断）；
//!   2. 理解三种捕获结果对应的三个 trait：Fn（不可变借用）、FnMut（可变借用）、FnOnce（拿走所有权）；
//!   3. 会把闭包作为参数（`impl Fn` / 泛型 bound / `Box<dyn Fn>`）与返回值（`move` + `impl Fn`）；
//!   4. 掌握 `move` 关键字的两个经典用途：闭包活得更久、跨线程转移所有权；
//!   5. 会阅读闭包相关的典型编译错误（E0382 / E0499 / E0502 / E0308）。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_14_closures.rs -o lesson_14_closures && ./lesson_14_closures
//!   或在本项目根目录执行：cargo run --bin lesson_14_closures
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       闭包是迭代器（lesson_15）、线程（lesson_17）的语法基础；
//!       demoweb 的 `tokio::task::spawn_blocking(move || hash(plain, cost))` 就是本课
//!       「move + FnOnce」的实战用法，学完本课再去看它会一目了然。

fn main() {
    println!("========== lesson_14_closures：闭包 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_syntax_vs_function();
    demo_2_capture_by_shared_borrow();
    demo_3_capture_by_mutable_borrow();
    demo_4_capture_by_move();
    demo_5_fn_fnmut_fnonce_hierarchy();
    demo_6_closure_as_parameter();
    demo_7_returning_closure();
    demo_8_box_dyn_closure();
    demo_9_scenario_task_queue();
    demo_10_common_mistakes();
}

// ---------------------------------------------------------------------------
// 示例 1：闭包的基本语法，以及它与函数的三个差别
// ---------------------------------------------------------------------------

/// 普通函数做对照：参数类型必须写全，且无法记住定义处之外的环境
fn add_one_fn(x: i32) -> i32 {
    x + 1
}

fn demo_1_syntax_vs_function() {
    println!("--- 示例 1：闭包语法与函数对照 ---");

    // 闭包写法：竖线包参数，函数体可以省略花括号（表达式直接返回）
    let add_one_closure = |x: i32| x + 1;
    println!("函数版  add_one_fn(41) = {}", add_one_fn(41));
    println!("// 预期输出：函数版  add_one_fn(41) = 42");
    println!("闭包版  add_one_closure(41) = {}", add_one_closure(41));
    println!("// 预期输出：闭包版  add_one_closure(41) = 42");

    // 类型也可以省略：编译器根据第一次（唯一一次）使用推断出 x 是 i32
    let inferred = |x| x + 1;
    println!("推断版  inferred(1) = {}", inferred(1));
    println!("// 预期输出：推断版  inferred(1) = 2");

    // 闭包与函数的三个核心差别（详见本课各示例）：
    //   1. 闭包可以捕获定义处的变量（示例 2、3、4）；
    //   2. 闭包参数类型通常可推断（但一旦推断锁定就不再改变，见示例 10）；
    //   3. 闭包本身是「实现了 Fn 系列匿名类型」的值，可以作为参数/返回值传递（示例 6~8）。
    println!("闭包 = 可捕获环境的匿名函数值");
}

// ---------------------------------------------------------------------------
// 示例 2：捕获方式一 —— 不可变借用（Fn）
// ---------------------------------------------------------------------------

fn demo_2_capture_by_shared_borrow() {
    println!("--- 示例 2：捕获 = 不可变借用（Fn） ---");

    let name = String::from("Rust");
    // 闭包体里只「读」了 name，所以按不可变借用捕获：name 并没有移动进闭包
    let greet = || println!("你好，{name}");

    greet(); // 调用闭包：内部以 &name 的方式访问
    println!("// 预期输出：你好，Rust");

    // 关键证据：闭包还「活着」的时候，name 依然可以被直接使用 ——
    // 如果 name 被 move 进了闭包，下一行就会编译失败（error[E0382]: borrow of moved value）
    println!("name 仍可直接使用：{name}");
    println!("// 预期输出：name 仍可直接使用：Rust");

    // 小结：闭包只读捕获变量 => 按 &T 借用 => 对应 Fn trait，可调用任意多次。
    println!("只读捕获 => Fn：可调用任意多次");
}

// ---------------------------------------------------------------------------
// 示例 3：捕获方式二 —— 可变借用（FnMut）
// ---------------------------------------------------------------------------

fn demo_3_capture_by_mutable_borrow() {
    println!("--- 示例 3：捕获 = 可变借用（FnMut） ---");

    let mut count = 0;
    // 闭包体里「写」了 count，所以按可变借用捕获：
    // 注意闭包变量本身必须声明为 mut —— 借用 &mut count 存在闭包里
    let mut increment = || {
        count += 1;
        count
    };

    let first = increment();
    let second = increment();
    let third = increment();
    println!("三次调用返回：{first} {second} {third}");
    println!("// 预期输出：三次调用返回：1 2 3");

    // increment 借用着 &mut count，所以在它最后一次使用之后（NLL）
    // 才能再直接读 count；这里能编译，是因为闭包后面不再被调用
    println!("count 最终 = {count}");
    println!("// 预期输出：count 最终 = 3");

    // 小结：闭包写捕获变量 => 按 &mut T 借用 => 对应 FnMut，闭包变量要加 mut。
    println!("写入捕获 => FnMut：同一时刻只允许一个可变借用");
}

// ---------------------------------------------------------------------------
// 示例 4：捕获方式三 —— 拿走所有权（move / FnOnce）
// ---------------------------------------------------------------------------

fn demo_4_capture_by_move() {
    println!("--- 示例 4：捕获 = 所有权转移（move） ---");

    let payload = String::from("任务数据");

    // move 强制把 payload 的所有权搬进闭包（本例即使不写 move 也会 move，
    // 因为闭包体里调用了 drop(owner) 消耗了它；move 把意图写明确）
    let consume = move || {
        println!("消费：{payload}");
        // String 没有 Copy，drop 拿走所有权
        drop(payload);
    };

    consume();
    println!("// 预期输出：消费：任务数据");

    // consume 是 FnOnce（消费型）：只能调用一次。
    // consume(); // error[E0382]: use of moved value: `consume`
    //             //           值被上一次调用移动走了（FnOnce 的调用需要 self）

    // 关键结论：move 的两大经典用途 ——
    //   1. 闭包活得比捕获变量久（返回闭包、存进结构体），必须把数据搬进去；
    //   2. 跨线程转移所有权（std::thread::spawn / tokio::spawn 都要求 move），
    //      这是编译器防止数据竞争的手段，见 lesson_17_threads_channels 示例 2。
    println!("move = 把捕获变量的所有权搬进闭包");
}

// ---------------------------------------------------------------------------
// 示例 5：Fn / FnMut / FnOnce 的层级关系
// ---------------------------------------------------------------------------

/// 接受 FnOnce（能力最弱的约束，凡可调用的闭包都满足）
fn call_once_with<F: FnOnce() -> i32>(f: F) -> i32 {
    f() // FnOnce 的调用消耗 self，因此只能调一次
}

/// 接受 FnMut（约束更强：要求多次调用，且内部可以修改捕获）
fn call_three_times<F: FnMut() -> i32>(mut f: F) -> i32 {
    let mut total = 0;
    for _ in 0..3 {
        total += f();
    }
    total
}

/// 接受 Fn（约束最强：要求多次调用且只读捕获；Fn 自动也实现 FnMut 与 FnOnce）
fn call_with_value<F: Fn() -> i32>(f: F) -> i32 {
    f() + f()
}

fn demo_5_fn_fnmut_fnonce_hierarchy() {
    println!("--- 示例 5：Fn / FnMut / FnOnce 层级 ---");

    // 只读捕获的闭包是 Fn；Fn「是」FnMut，也是 FnOnce —— 三者是叠加关系：
    //   Fn     : FnMut + FnOnce 的超集（只读捕获，可调任意次）
    //   FnMut  : FnOnce 的超集（可变捕获，可调多次）
    //   FnOnce : 最底层（可能消耗捕获值，至少可调一次）
    let multiplier = 10;
    let read_only = || multiplier * 2; // Fn
    println!("call_once_with(Fn) = {}", call_once_with(read_only));
    println!("// 预期输出：call_once_with(Fn) = 20");
    println!("call_three_times(Fn) = {}", call_three_times(read_only));
    println!("// 预期输出：call_three_times(Fn) = 60");
    println!("call_with_value(Fn) = {}", call_with_value(read_only));
    println!("// 预期输出：call_with_value(Fn) = 40");

    // 可变捕获的闭包是 FnMut：能传给 FnMut / FnOnce 约束，不能传给 Fn 约束
    let mut counter = 100;
    let mut count_up = || {
        counter += 1;
        counter
    };
    println!("call_three_times(FnMut) = {}", call_three_times(&mut count_up));
    println!("// 预期输出：call_three_times(FnMut) = 306");
    println!("call_once_with(FnMut) = {}", call_once_with(&mut count_up));
    println!("// 预期输出：call_once_with(FnMut) = 104");

    // 写参数 bound 的选择原则：写「满足需求的最弱约束」——
    //   只调一次            => FnOnce（对调用方最宽容）
    //   多次调用、可能改状态 => FnMut
    //   多次调用、只读      => Fn
    println!("约束选择：按需取最弱的（FnOnce ⊂ FnMut ⊂ Fn）");
}

// ---------------------------------------------------------------------------
// 示例 6：闭包作为参数（impl Trait / 泛型 / 函数指针）
// ---------------------------------------------------------------------------

/// 写法一：impl Trait —— 最简洁，静态分发
fn apply_twice(value: i32, f: impl Fn(i32) -> i32) -> i32 {
    f(f(value))
}

/// 写法二：泛型 + trait bound —— 可以在签名里指代、复用、加多重约束
fn apply_n_times<F: Fn(i32) -> i32>(value: i32, n: u32, f: F) -> i32 {
    let mut acc = value;
    for _ in 0..n {
        acc = f(acc);
    }
    acc
}

/// 写法三：函数指针 fn —— 没有「环境」可捕获的纯函数也能传（闭包若不捕获也能转 fn 指针）
fn apply_fn_pointer(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn demo_6_closure_as_parameter() {
    println!("--- 示例 6：闭包作为参数 ---");

    println!("apply_twice(3, |x| x + 1) = {}", apply_twice(3, |x| x + 1));
    println!("// 预期输出：apply_twice(3, |x| x + 1) = 5");

    println!(
        "apply_n_times(1, 4, |x| x * 3) = {}",
        apply_n_times(1, 4, |x| x * 3)
    );
    println!("// 预期输出：apply_n_times(1, 4, |x| x * 3) = 81");

    // 不捕获任何变量的闭包可以当函数指针用；普通函数本身就fn类型
    println!("apply_fn_pointer(5, double) = {}", apply_fn_pointer(5, double));
    println!("// 预期输出：apply_fn_pointer(5, double) = 10");
    println!(
        "apply_fn_pointer(5, |x| x - 1) = {}",
        apply_fn_pointer(5, |x| x - 1)
    );
    println!("// 预期输出：apply_fn_pointer(5, |x| x - 1) = 4");

    // 三种写法的取舍（同 lesson_12 的 impl Trait vs 泛型）：
    //   * 只是「收一个闭包用一下」 => impl Trait；
    //   * 需要在签名里复用/指代类型、叠加约束 => 泛型 bound；
    //   * 需要「无环境纯函数」且追求最简表示（或存进无堆分配结构）=> fn 指针。
    println!("参数写法：impl Trait 最简，泛型最灵活，fn 指针最轻");
}

// ---------------------------------------------------------------------------
// 示例 7：返回闭包 —— 必须 move，且只能返回一种具体闭包类型
// ---------------------------------------------------------------------------

/// 闭包里捕获了 factor，而 factor 是局部变量，函数返回后就没了，
/// 所以必须 move 把它搬进闭包；返回类型写成 impl Fn（编译器只知道「某个实现 Fn 的匿名类型」）
fn make_multiplier(factor: i64) -> impl Fn(i64) -> i64 {
    move |x| x * factor
}

/// 返回闭包时若不小心借用了局部变量，会直接编译失败（见示例 10 的错误对照）
fn make_greeter(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{prefix}，{name}")
}

fn demo_7_returning_closure() {
    println!("--- 示例 7：返回闭包 ---");

    let triple = make_multiplier(3);
    println!("triple(14) = {}", triple(14));
    println!("// 预期输出：triple(14) = 42");

    let greet_zh = make_greeter(String::from("你好"));
    println!("{}", greet_zh("小明"));
    println!("// 预期输出：你好，小明");

    // 两个返回闭包的函数各返回「不同的匿名类型」：
    // triple 与 greet_zh 类型不同，不能放进同一个普通 Vec —— 想混装见示例 8 的 dyn。
    println!("返回 impl Fn = 把具体闭包类型藏起来（每个函数一种）");
}

// ---------------------------------------------------------------------------
// 示例 8：Box<dyn Fn> —— 把不同闭包装进同一个集合
// ---------------------------------------------------------------------------

/// 把「不同类型的闭包」统一为 Box<dyn Fn(i32) -> i32>：动态分发，运行期查 vtable
fn demo_8_box_dyn_closure() {
    println!("--- 示例 8：Box<dyn Fn> 混装闭包 ---");

    let offset = 100;
    // 三个闭包类型各不相同（捕获不同、代码不同），但都能装进 Box<dyn Fn(i32) -> i32>
    let operations: Vec<(String, Box<dyn Fn(i32) -> i32>)> = vec![
        (String::from("加 1"), Box::new(|x| x + 1)),
        (String::from("翻倍"), Box::new(|x| x * 2)),
        (String::from("加偏移"), Box::new(move |x| x + offset)),
    ];

    let input = 7;
    for (name, op) in &operations {
        println!("{name}({input}) = {}", op(input));
    }
    println!("// 预期输出：加 1(7) = 8");
    println!("// 预期输出：翻倍(7) = 14");
    println!("// 预期输出：加偏移(7) = 107");

    // 代价与取舍（与 lesson_12 的 dyn trait 对象一致）：
    //   * Box 堆分配 + 每次调用一次 vtable 间接跳转；
    //   * 换来「同一集合装多种闭包」的灵活性；
    //   * 不混装就优先用泛型/impl Trait（零开销）。
    println!("混装不同闭包 => Box<dyn Fn>：灵活但有一次间接跳转");
}

// ---------------------------------------------------------------------------
// 示例 9：典型场景 —— 用 FnMut 驱动一个可复用的「重试调度器」
// ---------------------------------------------------------------------------

/// 一个极简的任务执行器：不断调用任务，直到任务报告成功或重试次数用尽。
/// 任务参数用 FnMut（每次调用都可能改变内部状态，例如退避计数）。
fn run_with_retry<F>(mut task: F, max_attempts: u32) -> u32
where
    F: FnMut(u32) -> bool,
{
    let mut attempt = 0;
    while attempt < max_attempts {
        attempt += 1;
        if task(attempt) {
            return attempt; // 返回第几次成功
        }
    }
    0 // 0 表示重试耗尽仍未成功
}

fn demo_9_scenario_task_queue() {
    println!("--- 示例 9：场景：用 FnMut 写重试调度器 ---");

    // 任务内部状态：第 1、2 次「失败」，第 3 次起成功 —— 用 FnMut 捕获可变状态实现
    let mut remaining_failures = 2;
    let succeeded_at = run_with_retry(
        |attempt| {
            if remaining_failures > 0 {
                remaining_failures -= 1;
                println!("第 {attempt} 次尝试：失败（还剩 {remaining_failures} 次宽限）");
                false
            } else {
                println!("第 {attempt} 次尝试：成功");
                true
            }
        },
        5,
    );
    println!("第 {succeeded_at} 次尝试成功");
    // 注意：下面 4 行断言对应「重试器内部的 3 行输出 + 本函数的 1 行输出」。
    println!("// 预期输出：第 1 次尝试：失败（还剩 1 次宽限）");
    println!("// 预期输出：第 2 次尝试：失败（还剩 0 次宽限）");
    println!("// 预期输出：第 3 次尝试：成功");
    println!("// 预期输出：第 3 次尝试成功");

    // 对照：永远失败的任务 —— max_attempts 用尽，返回 0
    let exhausted = run_with_retry(|_| false, 3);
    println!("重试耗尽返回 = {exhausted}");
    println!("// 预期输出：重试耗尽返回 = 0");

    // 场景要点：调用方只依赖 FnMut 这个「行为契约」，任务逻辑完全由闭包注入 ——
    // 这正是 demoweb 里「服务层依赖 trait 抽象」思想在函数级别的缩影。
    println!("FnMut 把「行为」变成可注入的参数");
}

// ---------------------------------------------------------------------------
// 示例 10：常见错误对照（全部注释掉，取消注释即可看到真实编译错误）
// ---------------------------------------------------------------------------

fn demo_10_common_mistakes() {
    println!("--- 示例 10：常见错误 ---");

    // 错误 1：闭包推断会「锁定」参数类型 —— 第一次使用后就不能换类型
    // let locked = |x| x + 1;
    // locked(1);            // 由此推断出 x: i32
    // locked("字符串");      // error[E0308]: mismatched types
    //                        //   expected integer, found `&str`
    // 修正：需要多态就写成泛型函数，或在定义处显式标注 |x: i32|

    // 错误 2：闭包持有可变借用的同时，又直接使用同一变量
    // let mut total = 0;
    // let mut add = || total += 1;
    // add();
    // println!("{total}");   // error[E0502]: cannot borrow `total` as immutable
    //                        //   because it is also borrowed as mutable
    //                        //   （add 还要被继续调用，它的 &mut total 仍然有效）
    // 修正：先完成闭包的所有调用，再读变量；或用 scope 包住闭包的生存期

    // 错误 3：move 之后又使用原变量
    // let data = String::from("数据");
    // let take = move || println!("{data}");
    // take();
    // println!("{data}");    // error[E0382]: borrow of moved value: `data`
    //                        //   move 已经把 data 的所有权搬进闭包
    // 修正：需要两边都用 => 先 clone 再 move；或改用不可变借用捕获（去掉 move）

    // 错误 4：返回闭包时借用局部变量
    // fn broken() -> impl Fn() -> i32 {
    //     let local = 42;
    //     || local            // error[E0373]: closure may outlive the current function
    //                         //   （闭包按引用捕获 local，而 local 马上要被销毁）
    // }
    // 修正：move || local —— 把值搬进闭包（本课示例 7 的写法）

    // 错误 5：同一个 FnMut 闭包被两个调用同时持有
    // fn broken_two(f: &mut impl FnMut()) -> impl FnMut() + '_ {
    //     // 想把「同一个可调用对象」同时借给两处使用，会触发
    //     // error[E0499]: cannot borrow `*f` as mutable more than once at a time
    // }
    // 修正：需要多处共享调用 => 用 Rc<RefCell<F>>（见 lesson_16_smart_pointers）
    //       或跨线程 Arc<Mutex<F>>（见 lesson_17_threads_channels）

    println!("闭包高频错误：E0308（类型锁定）/ E0502（借用冲突）");
    println!("E0382（move 后使用）/ E0373（闭包比捕获活得久）/ E0499（重复可变借用）");
}

// 本文件示例按 lesson-conventions.md 约定编写：
// 「// 预期输出：」为字面量断言，可用 .dsh/check_expected_output.ps1 一键核对。
