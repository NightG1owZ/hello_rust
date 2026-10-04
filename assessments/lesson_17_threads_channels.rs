//! assessments/lesson_17_threads_channels.rs —— 考核：线程与通道（对应 lesson_17）
//!
//! - 对应课程：`src/tutorial/lesson_17_threads_channels.rs`
//! - 知识点出处：`src/tutorial/README.md` 第五阶段「17 线程与通道」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_17_threads_channels     # 只考这一课
//!   cargo test                                        # 考全部 18 课
//!   cargo run --bin assessment_report                 # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_17_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_17_threads_channels`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 练习函数里需要的 `use`（`std::thread` / `std::sync::mpsc` / `std::sync::{Arc, Mutex}`）
//!   请写在**函数体内**：本文件顶部不导入任何 std 类型，这样骨架态不会出现 unused import 警告；
//! - **并发测试必须确定性**：本文件所有跨线程结果都先通过 `join()` 或通道收回主线程，
//!   再在主线程里比较；不依赖线程调度顺序、不使用 `sleep` 制造顺序假设；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 9）
//!
//! | 代码 | 报错 / 现象 | 修正方法 |
//! | --- | --- | --- |
//! | `thread::spawn(...)` 之后没有 `join` | 编译通过，运行期子线程可能一次都没跑完（进程随主线程退出） | 保存 `JoinHandle` 并 `join()`，或用通道把结果送回主线程 |
//! | `let data = vec![1, 2, 3]; thread::spawn(|| println!("{data:?}"));` | `error[E0373]` closure may outlive the current function, but it borrows `data` | 写 `move \|\|` 拿走所有权；只想借用就用 `thread::scope`（示例 6） |
//! | `let rc = Rc::new(1); thread::spawn(move \|\| println!("{rc}"));` | `error[E0277]` `Rc<i32>` cannot be sent between threads safely | 跨线程用 `Arc`；需要可变再配 `Mutex`（示例 7） |
//! | 线程带着锁 panic | 编译通过，运行期 `m.lock()` 返回 `Err(PoisonError)` | `lock().unwrap_or_else(\|p\| p.into_inner())`：确认数据一致后强行取出 |
//! | 两个线程以相反顺序获取两把锁 | 编译通过，运行期永久卡住（死锁） | 全程序约定统一的加锁顺序；缩小临界区、用完立刻释放 |

use assessment_harness::{Kind, assess, eq, eq_slice, is_false, is_true, ok};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_17";

// ===========================================================================
// kp_17_01 spawn + join：创建线程并取回它的返回值
// ===========================================================================

/// 知识点考核：`thread::spawn` 与 `join()`。
#[test]
fn kp_17_01_spawn_join() {
    assess(
        M,
        "kp_17_01",
        "thread::spawn + join：等子线程结束，并把它的返回值收回主线程",
        Kind::Basic,
        "复习 lesson_17 示例 1（spawn_and_join）：`thread::spawn(|| 42)` 返回 `JoinHandle<u32>`；\
         `handle.join()` 阻塞到子线程结束，返回 `Result<u32, Box<dyn Any + Send>>`——\
         子线程正常结束是 `Ok(值)`，子线程 panic 才是 `Err`（所以实现里要先 `expect(...)` 取出来）。",
        || {
            // 正常用例
            eq(
                exercise_17_01_spawn_join(),
                42u32,
                "正常用例：子线程的闭包返回 42，主线程 join() 之后取回的就是 42",
            );

            // 边界：join() 返回的是 Result —— 值必须从 Ok 里取出来，而不是「碰巧等于某个默认值」
            let handle = std::thread::spawn(|| 7u32);
            let joined = ok(
                handle.join(),
                "边界：子线程正常结束时 join() 是 Ok(值)——所以实现里要先 expect/unwrap 再取返回值",
            );
            eq(
                joined,
                7u32,
                "边界：Ok 里的值就是子线程闭包的返回值本身（这里是 7），不是 0 或别的兜底值",
            );
        },
    );
}

/// 【待实现】创建一个子线程并等它结束。
///
/// 实现要求：
///   - 在函数体内 `use std::thread;`（本文件顶部不导入任何 std 类型，需要什么就在函数体里 `use`）；
///   - `let handle = thread::spawn(|| 42u32);`（闭包返回 `u32`）；
///   - 用 `handle.join().expect("子线程不应 panic")` 把值取回来作为返回值（结果是 42）；
///   - `join()` 返回 `Result<闭包返回值, Box<dyn Any + Send>>`：子线程正常结束时是 `Ok(值)`，
///     子线程 panic 才是 `Err`，所以取值前必须 `unwrap()` / `expect(...)`；
///   - 忘记 `join` 是运行期问题：编译能过，但主线程会直接结束，子线程可能连一次都没跑完。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 42
/// ```
fn exercise_17_01_spawn_join() -> u32 {
    assessment_harness::todo_exercise(
        "exercise_17_01_spawn_join",
        "thread::spawn(|| 42u32) 之后 handle.join().expect(...) 取回 42",
        (),
    )
}

// ===========================================================================
// kp_17_02 move 闭包：所有权转移是编译器防数据竞争的手段
// ===========================================================================

/// 知识点考核：`move` 闭包把 `String` 的所有权交给子线程。
#[test]
fn kp_17_02_move_ownership() {
    assess(
        M,
        "kp_17_02",
        "move 闭包与所有权转移：编译器用「独占」保证没有数据竞争",
        Kind::Core,
        "复习 lesson_17 示例 2（move_ownership）：`thread::spawn(move || data.len())` 把 `data` 的\
         所有权整体搬进子线程——主线程再也碰不到它，所以不可能出现两个线程同时读写同一份数据；\
         不写 `move` 而去借用栈上变量会报 `error[E0373]: closure may outlive the current function`。",
        || {
            // 正常用例
            eq(
                exercise_17_02_move_ownership(String::from("rust")),
                4usize,
                "正常用例：\"rust\" 是 4 字节，长度在子线程里算出来后通过 join 交回主线程",
            );
            // 边界：多字节 UTF-8 —— len() 数的是字节，不是字符
            eq(
                exercise_17_02_move_ownership(String::from("中文")),
                6usize,
                "边界：String::len() 返回字节数，\"中文\" 在 UTF-8 下是 6 字节（不是 2 个字符）",
            );
            // 边界：空串
            eq(
                exercise_17_02_move_ownership(String::new()),
                0usize,
                "边界：空串长度是 0，子线程同样要正常结束并交回 0",
            );
        },
    );
}

/// 【待实现】用 `move` 闭包把字符串送进子线程并计算字节长度。
///
/// 实现要求：
///   - 在函数体内 `use std::thread;`；
///   - `let handle = thread::spawn(move || text.len());`（`move` 把 `text` 的所有权搬进闭包）；
///   - `handle.join().expect("子线程不应 panic")` 作为返回值（`usize`）；
///   - 这里**必须**写 `move`：子线程可能活得比本函数的栈帧更久，不写 `move` 而去借用局部变量
///     会被编译器判为悬垂借用（`error[E0373]`）；把数据「搬」进线程（而不是让两个线程同时指向它）
///     正是 Rust 防数据竞争的核心手段，搬走之后本函数里也不能再用 `text`（`error[E0382]`）。
///
/// 示例输入：
/// ```text
/// text = "rust"
/// ```
/// 示例输出：
/// ```text
/// 4
/// ```
fn exercise_17_02_move_ownership(text: String) -> usize {
    assessment_harness::todo_exercise(
        "exercise_17_02_move_ownership",
        "thread::spawn(move || text.len()) 之后 join，返回 text 的字节长度",
        (text,),
    )
}

// ===========================================================================
// kp_17_03 mpsc 通道：一个生产者发送，主线程收集求和
// ===========================================================================

/// 知识点考核：单生产者 + 通道求和。
#[test]
fn kp_17_03_channel_sum() {
    assess(
        M,
        "kp_17_03",
        "mpsc 通道：一个生产者线程发送，主线程收集求和",
        Kind::Core,
        "复习 lesson_17 示例 3（mpsc_single_producer）：`mpsc::channel()` 拿到 `(tx, rx)`；\
         生产者把值逐个 `send`，闭包结束时 `tx` 被 drop、通道关闭，接收端的 `for value in rx` \
         才会结束；主线程把收到的值累加求和。",
        || {
            // 正常用例
            eq(
                exercise_17_03_channel_sum(vec![1, 2, 3, 4]),
                10,
                "正常用例：生产者依次发送 1、2、3、4，主线程收集求和 = 10",
            );
            // 边界：空 Vec（一条消息都不发，通道也必须正常关闭）
            eq(
                exercise_17_03_channel_sum(Vec::new()),
                0,
                "边界：空 Vec → 0；若生产者结束时没有把 tx drop 掉，接收端的 for 会永远阻塞、测试挂住",
            );
            // 边界：负数与 0
            eq(
                exercise_17_03_channel_sum(vec![-5, 0, 5]),
                0,
                "边界：-5 + 0 + 5 = 0，通道既不丢消息、也不要把负数当无符号处理",
            );
        },
    );
}

/// 【待实现】用通道把数据从生产者线程送到主线程再求和。
///
/// 实现要求：
///   - 在函数体内 `use std::sync::mpsc;` 与 `use std::thread;`；
///   - `let (tx, rx) = mpsc::channel();`
///   - `thread::spawn(move || { for value in values { tx.send(value).expect(...); } });`
///     —— `tx` 被 `move` 进闭包，闭包结束时自动 drop，通道随之关闭；
///   - 主线程 `let mut total = 0; for value in rx { total += value; }`，返回 `total`；
///   - 接收端的 `for value in rx` 会一直等到**所有发送端都被 drop** 才结束：`values` 为空时
///     循环体一次都不执行，但 `tx` 仍会在闭包结束时被 drop，所以空 Vec 也能正常返回 0；
///   - `send` 返回 `Result`，接收端提前关闭时是 `Err`，这里用 `expect(...)` 表达「不应该发生」。
///
/// 示例输入：
/// ```text
/// values = [1, 2, 3, 4]
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_17_03_channel_sum(values: Vec<i32>) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_17_03_channel_sum",
        "生产者线程用 mpsc 通道发送 values 里的全部值，主线程 for 收集求和并返回",
        (values,),
    )
}

// ===========================================================================
// kp_17_04 多生产者：tx.clone() 把发送端分给多个线程
// ===========================================================================

/// 知识点考核：两个生产者线程（`tx.clone()`）。
#[test]
fn kp_17_04_multi_producer() {
    assess(
        M,
        "kp_17_04",
        "多生产者单消费者：用 `tx.clone()` 把发送端分给多个线程",
        Kind::Core,
        "复习 lesson_17 示例 4（mpsc_multi_producer）：每个生产者线程拿一个 `tx.clone()`，\
         各发自己那批消息；主线程必须 `drop(tx)`（丢掉自己手里那份发送端），\
         否则接收端等不到「所有发送端关闭」，`for` 会一直阻塞；\
         各消息到达顺序不确定，所以先收集再统计（求和与顺序无关）。",
        || {
            let total = exercise_17_04_multi_producer();

            // 正常用例
            eq(
                total,
                12i32,
                "正常用例：两个生产者各发 1、2、3 → (1 + 2 + 3) × 2 = 12",
            );
            // 边界/防呆：必须真的有两个生产者
            is_false(
                total == 6,
                "边界：6 只算了「一个生产者」——两个线程各自发送 1..=3，总和必须是 12",
            );
        },
    );
}

/// 【待实现】两个生产者线程向同一条通道发送数据。
///
/// 实现要求：
///   - 在函数体内 `use std::sync::mpsc;` 与 `use std::thread;`；
///   - `let (tx, rx) = mpsc::channel();`
///   - 起**两个**生产者线程：每个线程先 `let tx = tx.clone();`，
///     再 `for n in 1..=3 { tx.send(n).expect(...); }`；
///   - 循环结束后 `drop(tx);`（丢掉主线程手里那份发送端）；
///   - 主线程 `for value in rx { total += value; }` 收集求和，返回 `total`（应为 12）；
///   - `tx.clone()` 克隆的是**发送端句柄**（不是消息），这正是 mpsc 里 multi-producer 的实现方式；
///   - 忘记 `drop(tx)` 是本题最经典的坑：接收端会一直等一个永远不会关闭的发送端，`for` 就此阻塞。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 12
/// ```
fn exercise_17_04_multi_producer() -> i32 {
    assessment_harness::todo_exercise(
        "exercise_17_04_multi_producer",
        "两个生产者线程各 tx.clone() 后发送 1..=3，主线程 drop(tx) 并收集求和 = 12",
        (),
    )
}

// ===========================================================================
// kp_17_05 Arc<Mutex<T>>：多线程共享可变状态
// ===========================================================================

/// 知识点考核：10 个线程各自锁住共享计数器 +1。
#[test]
fn kp_17_05_arc_mutex() {
    assess(
        M,
        "kp_17_05",
        "Arc<Mutex<T>>：Arc 管「多线程共享」，Mutex 管「同一时刻只有一个能改」",
        Kind::Core,
        "复习 lesson_17 示例 5（arc_mutex_shared_state）：每个线程 `Arc::clone` 一份句柄（原子计数），\
         `counter.lock().unwrap()` 拿到互斥守卫后才能改；守卫离开作用域自动解锁（RAII），\
         临界区越小越好。**所有线程 `join()` 之后**主线程再读最终值，结果才是确定的。",
        || {
            let count = exercise_17_05_arc_mutex();

            // 正常用例
            eq(
                count,
                10i32,
                "正常用例：10 个线程各自锁住共享计数器并 +1，全部 join 之后结果是 10",
            );
            // 边界/防呆：每个线程都必须改到同一份共享数据
            is_false(
                count == 1,
                "边界：结果是 10 而不是 1——若各自改自己的副本（或只起了 1 个线程），就会停在 1",
            );
        },
    );
}

/// 【待实现】用 `Arc<Mutex<i32>>` 让 10 个线程各加 1。
///
/// 实现要求：
///   - 在函数体内 `use std::sync::{Arc, Mutex};` 与 `use std::thread;`；
///   - `let counter = Arc::new(Mutex::new(0i32));`
///   - 起 **10** 个线程：每个线程先 `let counter = Arc::clone(&counter);`，
///     再 `let mut guard = counter.lock().expect(...); *guard += 1;`
///     （守卫在作用域结束时自动 drop → 自动解锁）；
///   - 把 `JoinHandle` 收进 `Vec`，再循环 `handle.join().expect(...)` 等所有线程结束；
///   - 最后 `*counter.lock().expect(...)` 返回最终值（10）；
///   - **必须先 join 再读结果**：否则主线程可能在任何线程加完之前就读了值，结果不确定；
///   - `lock()` 返回 `Result`：持有锁的线程 panic 之后锁会「中毒」（示例 9 的错误 4），
///     这里用 `expect(...)` 表达「不应该发生」。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_17_05_arc_mutex() -> i32 {
    assessment_harness::todo_exercise(
        "exercise_17_05_arc_mutex",
        "10 个线程共享 Arc<Mutex<i32>>，各自 lock 后 +1，全部 join 后返回 10",
        (),
    )
}

// ===========================================================================
// kp_17_06 Send / Sync：编译期断言证明 Arc<Mutex<i32>> 两者都满足
// ===========================================================================

/// 知识点考核：用编译期断言确认 `Arc<Mutex<i32>>` 是 `Send + Sync`。
#[test]
fn kp_17_06_send_sync_check() {
    assess(
        M,
        "kp_17_06",
        "Send / Sync：`Arc<Mutex<i32>>` 两者都满足（换成 `Rc` 根本编译不过）",
        Kind::Core,
        "复习 lesson_17 示例 7（send_and_sync）与示例 9 的错误 3：`Send` = 值可以**转移**到别的线程，\
         `Sync` = `&T` 可以被多个线程同时引用；`Rc` 的计数只是普通加减，所以既不是 `Send` 也不是 `Sync`，\
         把 `Rc` 送进线程会报 `error[E0277]: `Rc<i32>` cannot be sent between threads safely`。",
        || {
            // 正常用例：编译期断言能过，本身就是证明
            is_true(
                exercise_17_06_send_sync_check(),
                "正常用例：函数里对 `Arc<Mutex<i32>>` 调用 assert_send / assert_sync 都能编译通过 → 返回 true",
            );

            // 边界：Send/Sync 不是「纸面标记」——真的把 Arc<Mutex<i32>> 移进另一个线程，
            // 主线程同时持有共享引用；结果先 join 收回，再在主线程里比较
            let shared = std::sync::Arc::new(std::sync::Mutex::new(1i32));
            let moved = std::sync::Arc::clone(&shared);
            let handle = std::thread::spawn(move || {
                let mut guard = moved.lock().expect("锁不应中毒");
                *guard += 1;
            });
            handle.join().expect("子线程不应 panic");
            eq(
                *shared.lock().expect("锁不应中毒"),
                2i32,
                "边界：跨线程共享可变状态真的生效（1 + 1 = 2）；若类型不满足 Send + Sync，\
                 这几行代码根本编译不过",
            );
        },
    );
}

/// 【待实现】在函数内定义编译期断言，证明 `Arc<Mutex<i32>>` 是 `Send + Sync`。
///
/// 实现要求：
///   - 在本函数**内部**定义两个空函数：
///     `fn assert_send<T: Send>(_: &T) {}` 与 `fn assert_sync<T: Sync>(_: &T) {}`；
///   - 造一个 `let shared = Arc::new(Mutex::new(0i32));`
///     （在函数体内 `use std::sync::{Arc, Mutex};` 之后写 `Arc::new(...)`）；
///   - 分别调用 `assert_send(&shared);` 与 `assert_sync(&shared);`；
///   - 返回 `true`；
///   - 这两个函数**没有任何运行时逻辑**，作用是「让编译器检查 trait bound」：如果传进去的是
///     `Rc<Mutex<i32>>`，`error[E0277]` 会立刻出现，这就是本知识点的价值所在；
///   - 嵌套函数不能捕获外部变量，而这里正好也不需要捕获。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// true
/// ```
fn exercise_17_06_send_sync_check() -> bool {
    assessment_harness::todo_exercise(
        "exercise_17_06_send_sync_check",
        "函数内定义 assert_send / assert_sync，对 Arc<Mutex<i32>> 调用两者后返回 true",
        (),
    )
}

// ===========================================================================
// kp_17_07 thread::scope：子线程借用切片，不需要 move 所有权
// ===========================================================================

/// 知识点考核：`thread::scope` 借用局部数据分块求和。
#[test]
fn kp_17_07_scoped_sum() {
    assess(
        M,
        "kp_17_07",
        "thread::scope：子线程借用局部数据，不需要 move 所有权",
        Kind::Core,
        "复习 lesson_17 示例 6（scoped_threads）：`thread::scope(|scope| { ... })` 保证块结束时\
         所有子线程都已被 join，所以子线程可以安全借用**没有 move 的**局部变量；\
         每个线程算自己那一块的 `chunk.iter().sum()`，最后把各部分和相加。",
        || {
            // 正常用例
            eq(
                exercise_17_07_scoped_sum(&[1, 2, 3, 4, 5, 6, 7, 8]),
                36,
                "正常用例：1 + 2 + … + 8 = 36（分块求和再合并，不能漏元素、不能重复）",
            );
            // 边界：空切片（一块都切不出来）
            eq(
                exercise_17_07_scoped_sum(&[]),
                0,
                "边界：空切片 → 0；注意 `chunks(0)` 会 panic，空输入必须单独挡住",
            );
            // 边界：负数
            eq(
                exercise_17_07_scoped_sum(&[-1, -2, 3]),
                0,
                "边界：-1 + (-2) + 3 = 0，分块边界不能丢元素、也不能把负数当无符号",
            );
            // 正常/边界：scope 只是借用，不转移所有权
            let owned = vec![2, 4, 6];
            eq(
                exercise_17_07_scoped_sum(&owned),
                12,
                "正常用例：scope 借用 `&[i32]`，不拿走调用方的所有权",
            );
            eq_slice(
                &owned,
                &[2, 4, 6],
                "边界：调用结束后 owned 仍归主线程所有（这正是 scope 比 spawn 多出的保证）",
            );
        },
    );
}

/// 【待实现】用 `thread::scope` 借用切片分块求和。
///
/// 实现要求：
///   - 在函数体内 `use std::thread;`；
///   - 空切片直接返回 `0`（`values.chunks(0)` 会 panic，先把空输入挡住）；
///   - 用 `thread::scope(|scope| { ... })`：把 `values` 切成若干块
///     （块大小可以用 `values.len().div_ceil(4)` 算），每块
///     `scope.spawn(move || chunk.iter().sum::<i32>())`；
///   - 在主线程 `join()` 每一块并求和，把总和作为返回值；
///   - `scope` 里 spawn 的闭包借的是 `values` 的切片（`&[i32]`）：因为 scope 会等所有子线程结束，
///     借用是安全的，所以**不需要**把整个 `values` move 进去；换成 `thread::spawn` 就必须 move；
///   - 给 `sum::<i32>()` 写 turbofish，否则元素类型推断不出来（`error[E0282]`）。
///
/// 示例输入：
/// ```text
/// values = [1, 2, 3, 4, 5, 6, 7, 8]
/// ```
/// 示例输出：
/// ```text
/// 36
/// ```
fn exercise_17_07_scoped_sum(values: &[i32]) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_17_07_scoped_sum",
        "thread::scope 借用 values 分块 spawn 求和再合并；空切片返回 0",
        (values,),
    )
}

// ===========================================================================
// kp_17_08 场景：按 chunks 分块的并行求和（对应课程示例 8）
// ===========================================================================

/// 知识点考核：分块 → 并发 → 收集合并。
#[test]
fn kp_17_08_parallel_sum() {
    assess(
        M,
        "kp_17_08",
        "并行求和：按 chunks 分块、并发执行、收集合并（切分 → 并发 → 合并）",
        Kind::Hard,
        "复习 lesson_17 示例 8（scenario_parallel_sum）：`chunk_size = (len + workers - 1) / workers` \
         （或 `len.div_ceil(workers)`）向上取整，保证切出来的块数不超过 worker 数；\
         每块用 `thread::scope` 借用计算部分和，最后把部分和相加。\
         边界：空数据直接返回 0；`chunks = 0` 按 1 处理（否则除零 / `chunks(0)` panic）。",
        || {
            // 正常用例：1..=100 分 4 块
            let values = (1..=100).collect::<Vec<i32>>();
            eq(
                exercise_17_08_parallel_sum(values, 4),
                5050,
                "正常用例：1..=100 分成 4 块并行求和 = 5050（分块既不能漏元素，也不能重复算）",
            );
            // 边界：空数据
            eq(
                exercise_17_08_parallel_sum(Vec::new(), 4),
                0,
                "边界：空数据 → 0（一块都切不出来时不要 spawn 空块、也不要除零）",
            );
            // 边界：chunks 比元素个数还多
            eq(
                exercise_17_08_parallel_sum(vec![1, 2, 3], 100),
                6,
                "边界：chunks 比元素还多 → 结果仍是 1 + 2 + 3 = 6（多余的分块根本不存在）",
            );
            // 边界：chunks = 0 按 1 处理（实现要求里写明了这条约定）
            eq(
                exercise_17_08_parallel_sum(vec![1, 2, 3, 4], 0),
                10,
                "边界：chunks = 0 时按 1 块处理，结果仍是 10（不能 panic、不能除零）",
            );
            // 边界：负数与单元素
            eq(
                exercise_17_08_parallel_sum(vec![-2, 5, -3], 2),
                0,
                "边界：-2 + 5 + (-3) = 0，负数分块同样要正确合并",
            );
            eq(
                exercise_17_08_parallel_sum(vec![9], 3),
                9,
                "边界：只有一个元素时结果就是 9（块大小不能算出 0）",
            );
        },
    );
}

/// 【待实现】把数据切成 `chunks` 块、并行求和再汇总。
///
/// 实现要求：
///   - 在函数体内 `use std::thread;`；
///   - 空数据 → 直接返回 0；
///   - `chunks == 0` 时**按 1 块处理**（避免除零与 `chunks(0)` panic）；
///   - 块大小 = `values.len().div_ceil(chunks)`（向上取整，保证块数不超过 chunks）；
///   - 再用 `thread::scope` 对每个 `values.chunks(块大小)` 起一个线程求和，
///     把所有部分和相加返回（对应课程示例 8 的「切分 → 并发 → 收集合并」）；
///   - `chunks` 比元素个数多时块大小会被算成 1，切出来的块数自然不超过元素个数，不需要额外特判；
///   - 别忘了 `sum::<i32>()` 的 turbofish。
///
/// 示例输入：
/// ```text
/// values = [1, 2, 3, 4, 5, 6, 7, 8]
/// chunks = 4
/// ```
/// 示例输出：
/// ```text
/// 36
/// ```
fn exercise_17_08_parallel_sum(values: Vec<i32>, chunks: usize) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_17_08_parallel_sum",
        "空数据返回 0；chunks = 0 按 1 处理；块大小 = len.div_ceil(chunks)，\
         用 thread::scope 分块 spawn 求和再汇总",
        (values, chunks),
    )
}

// ===========================================================================
// kp_17_09 常见错误诊断：运行期问题 / E0373 / E0277
// ===========================================================================

/// 知识点考核：写出课程示例 9 前三条「线程常见错误」的诊断标识。
#[test]
fn kp_17_09_diagnose_errors() {
    assess(
        M,
        "kp_17_09",
        "常见错误诊断：忘记 join / 闭包缺 move / Rc 跨线程，各是什么性质的问题",
        Kind::Hard,
        "复习 lesson_17 示例 9（common_mistakes）与示例 7：按注释顺序的前三条错误里，\
         第 1 条「忘记 join」是**运行期**现象（编译能过，主线程提前退出，没有编译错误编号），\
         第 2 条「闭包缺 move」报 `error[E0373]`，第 3 条「Rc 跨线程」报 `error[E0277]`；\
         再往后的锁中毒、死锁同样是运行期问题。",
        || {
            eq_slice(
                &exercise_17_09_diagnose_errors(),
                &["忘 join（运行期，无编译编号）", "E0373", "E0277"],
                "按课程示例 9 的注释顺序：第 1 条是运行期问题（没有编译编号）、\
                 第 2 条缺 move 是 E0373、第 3 条 Rc 跨线程是 E0277",
            );
        },
    );
}

/// 【待实现】写出前三条「线程常见错误」的诊断标识。
///
/// 场景（与课程示例 9 的注释逐条对应，**按注释顺序**）：
///   1. `thread::spawn(|| ...)` 之后没有 `join` —— 主线程可能先结束，子线程连一次都没跑完；
///   2. `let data = vec![1, 2, 3]; thread::spawn(|| println!("{data:?}"));` —— 闭包少了 `move`；
///   3. `let rc = Rc::new(1); thread::spawn(move || println!("{rc}"));` —— 把 `Rc` 送进线程。
/// 注意：第 1 条是**运行期**问题（编译能过，没有 `E....` 编号），所以答案写成
/// `"忘 join（运行期，无编译编号）"`；第 2、3 条才各有一个编译错误编号。
/// 后面两条（锁中毒、死锁）也是运行期问题，本题不涉及。
///
/// 实现要求：返回 3 个字符串，顺序与上面一致：第 1 条写运行期说明，第 2、3 条写编译错误编号；
///   这两个编号在课程示例 9 的注释里逐字写着（示例 7 里也重复了「Rc 跨线程」那条）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["忘 join（运行期，无编译编号）", "E0373", "E0277"]
/// ```
fn exercise_17_09_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_17_09_diagnose_errors",
        "返回 [\"忘 join（运行期，无编译编号）\", \"E0373\", \"E0277\"]（忘 join / 缺 move / Rc 跨线程）",
        (),
    )
}
