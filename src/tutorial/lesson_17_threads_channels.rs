//! lesson_17_threads_channels.rs —— 主题：线程与通道（std::thread / mpsc / Arc<Mutex> / Send+Sync）
//!
//! 学习目标：
//!   1. 会用 std::thread::spawn + join 创建并等待线程，理解 join 返回值的意义；
//!   2. 掌握 move 闭包在线程中的作用：所有权转移是编译器防数据竞争的手段；
//!   3. 会用 mpsc 通道在多线程间传递消息，理解「多生产者、单消费者」模型；
//!   4. 会用 Arc<Mutex<T>> 在线程间共享可变状态，理解锁的获取与释放时机；
//!   5. 分清 Send / Sync 两个标记 trait，知道为什么 Rc 不能跨线程而 Arc 可以。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_17_threads_channels.rs -o lesson_17_threads_channels && ./lesson_17_threads_channels
//!   或在本项目根目录执行：cargo run --bin lesson_17_threads_channels
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       为了保证「预期输出」逐字节可核对，所有跨线程输出都通过 join/通道
//!       收回主线程后排序打印 —— 这也是让并发程序输出确定的常用手法。
//!       异步版本（tokio::spawn / async 通道）见 demoweb；本课只讲标准库线程。

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("========== lesson_17_threads_channels：线程与通道 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_spawn_and_join();
    demo_2_move_ownership();
    demo_3_mpsc_single_producer();
    demo_4_mpsc_multi_producer();
    demo_5_arc_mutex_shared_state();
    demo_6_scoped_threads();
    demo_7_send_and_sync();
    demo_8_scenario_parallel_sum();
    demo_9_common_mistakes();
}

// ---------------------------------------------------------------------------
// 示例 1：spawn + join —— 创建线程并等待结果
// ---------------------------------------------------------------------------

fn demo_1_spawn_and_join() {
    println!("--- 示例 1：spawn 与 join ---");

    // spawn 的参数是一个闭包（见 lesson_14），返回 JoinHandle<i32>；
    // 闭包的返回值会通过 join() 交还给主线程
    let handle = thread::spawn(|| {
        let doubled: Vec<i32> = (1..=5).map(|x| x * 2).collect();
        doubled
    });

    // join 阻塞当前线程，直到子线程结束；返回 Result<闭包返回值>，
    // 子线程若 panic，join 会得到 Err（见示例 9）
    let result = handle.join().unwrap();
    println!("子线程返回 = {result:?}");
    println!("// 预期输出：子线程返回 = [2, 4, 6, 8, 10]");

    // 不调用 join 会怎样：主线程提前退出，子线程可能「话还没说完」就被整体终止 ——
    // 想让子线程跑完，就必须在合适的位置 join。
    println!("join = 在这里等待子线程结束并取回返回值");
}

// ---------------------------------------------------------------------------
// 示例 2：move 闭包 —— 用所有权转移杜绝数据竞争
// ---------------------------------------------------------------------------

fn demo_2_move_ownership() {
    println!("--- 示例 2：move 与所有权转移 ---");

    let data = vec![String::from("甲"), String::from("乙")];

    // move 把 data 的所有权整体搬进子线程：
    //   * 子线程独占数据，主线程无法再碰它 —— 所以不可能产生数据竞争；
    //   * 子线程结束后，数据随闭包一起被释放。
    let handle = thread::spawn(move || {
        for item in &data {
            println!("子线程处理：{item}");
        }
        data.len()
    });

    // data 在这里已经不可用（被 move 走了）——下面这行会编译失败：
    // println!("{data:?}"); // error[E0382]: borrow of moved value: `data`

    // join 在这里等待子线程跑完：甲、乙两行输出在此之前已完成
    let length = handle.join().unwrap();
    println!("子线程交回长度 = {length}");

    // 断言块：子线程两行 + 交回长度一行（单子线程，输出顺序确定）
    println!("// 预期输出：子线程处理：甲");
    println!("// 预期输出：子线程处理：乙");
    println!("// 预期输出：子线程交回长度 = 2");

    println!("move 交给线程 = 「独占换安全」，编译器保证没有数据竞争");
}

// ---------------------------------------------------------------------------
// 示例 3：mpsc 通道 —— 单生产者、单消费者
// ---------------------------------------------------------------------------

fn demo_3_mpsc_single_producer() {
    println!("--- 示例 3：mpsc 通道（单生产者） ---");

    // mpsc = multi-producer, single-consumer：多发送端、单一接收端
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // 发送端把消息逐个送进通道；接收端按发送顺序收到
        for message in ["第一条", "第二条", "第三条"] {
            tx.send(message).unwrap();
        }
        // tx 在这里被 drop：通道关闭，接收端的 for 循环随之结束
    });

    // rx 是一个迭代器：recv 直到所有发送端都关闭
    for message in rx {
        println!("收到：{message}");
    }
    println!("// 预期输出：收到：第一条");
    println!("// 预期输出：收到：第二条");
    println!("// 预期输出：收到：第三条");

    // 通道 = 线程间的「传送带」：不需要锁，所有权随消息转移。
    println!("通道传消息 = 无锁的线程间通信，所有权随消息走");
}

// ---------------------------------------------------------------------------
// 示例 4：mpsc 通道 —— 多生产者（克隆发送端）
// ---------------------------------------------------------------------------

fn demo_4_mpsc_multi_producer() {
    println!("--- 示例 4：mpsc 通道（多生产者） ---");

    let (tx, rx) = mpsc::channel();

    // 克隆出两个发送端，分别交给两个线程；每个线程只发固定内容
    for worker_id in 1..=2 {
        let tx = tx.clone();
        thread::spawn(move || {
            for n in 1..=3 {
                tx.send(format!("worker-{worker_id} 消息{n}")).unwrap();
            }
            // 注意：这个克隆的 tx 在闭包结束时自动 drop
        });
    }

    // 主线程的 tx 必须先丢弃：否则 rx 的 for 永远等不到「所有发送端关闭」
    drop(tx);

    // 多线程交错发送，到达顺序不确定 —— 全部收回后排序，输出就确定了
    let mut received: Vec<String> = rx.iter().collect();
    received.sort();
    for message in &received {
        println!("{message}");
    }
    println!("// 预期输出：worker-1 消息1");
    println!("// 预期输出：worker-1 消息2");
    println!("// 预期输出：worker-1 消息3");
    println!("// 预期输出：worker-2 消息1");
    println!("// 预期输出：worker-2 消息2");
    println!("// 预期输出：worker-2 消息3");
    println!("共收到 {} 条消息（顺序不确定，排序后打印）", received.len());
    println!("// 预期输出：共收到 6 条消息（顺序不确定，排序后打印）");

    println!("多生产者 = clone 发送端；收集后排序让输出可复现");
}

// ---------------------------------------------------------------------------
// 示例 5：Arc<Mutex<T>> —— 线程间共享可变状态
// ---------------------------------------------------------------------------

fn demo_5_arc_mutex_shared_state() {
    println!("--- 示例 5：Arc<Mutex<T>> 共享可变状态 ---");

    // Arc = 原子引用计数的 Rc（线程安全版）；Mutex = 互斥锁，保护内部数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for worker_id in 1..=4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // lock() 返回 Result<互斥守卫>：拿到守卫才能读写；
            // 守卫离开作用域时自动解锁（RAII，不存在「忘了解锁」）
            let mut count = counter.lock().unwrap();
            *count += 25;
            *count * worker_id // 返回点只是演示，与累加结果无关
        }));
    }

    // 一次性 join 全部子线程
    for handle in handles {
        handle.join().unwrap();
    }

    println!("4 个线程各加 25，最终计数 = {}", *counter.lock().unwrap());
    println!("// 预期输出：4 个线程各加 25，最终计数 = 100");
    println!("持有者数量 = {}", Arc::strong_count(&counter));
    println!("// 预期输出：持有者数量 = 1");

    // 读法拆解：
    //   * Arc 负责「多个线程都指向同一份数据」（原子计数）；
    //   * Mutex 负责「同一时刻只有一个线程能改」（互斥守卫）；
    //   * lock() 包着全部访问，临界区越小越好 —— 拿到值尽快 drop 守卫。
    println!("Arc 管共享，Mutex 管互斥，守卫 Drop 即解锁");
}

// ---------------------------------------------------------------------------
// 示例 6：scoped 线程 —— 子线程借用局部变量（不需要 move）
// ---------------------------------------------------------------------------

fn demo_6_scoped_threads() {
    println!("--- 示例 6：thread::scope 借用局部数据 ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // scope 保证：块结束时所有子线程都已 join，因此子线程可以安全借用
    // 「没有 move 的」局部变量 —— 这是它与 spawn 最大的区别
    let partial_sums: Vec<i32> = thread::scope(|scope| {
        let chunk_size = 4;
        let mut handles = Vec::new();
        for chunk in numbers.chunks(chunk_size) {
            // 每个子线程借用 numbers 的一个切片（&[i32]），不是拿走所有权
            handles.push(scope.spawn(move || chunk.iter().sum::<i32>()));
        }
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    println!("各块之和 = {partial_sums:?}");
    println!("// 预期输出：各块之和 = [10, 26]");

    // scope 结束后 numbers 依然归主线程所有
    println!("scope 后 numbers = {numbers:?}");
    println!("// 预期输出：scope 后 numbers = [1, 2, 3, 4, 5, 6, 7, 8]");

    println!("需要借用、不想转移所有权 => thread::scope；跨函数/长生命周期 => spawn + move");
}

// ---------------------------------------------------------------------------
// 示例 7：Send 与 Sync —— 两个标记 trait
// ---------------------------------------------------------------------------

/// 空结构体只作为「类型演示」的载体
struct OnlyMainThread;

fn demo_7_send_and_sync() {
    println!("--- 示例 7：Send 与 Sync ---");

    // 两个自动派生的标记 trait（没有任何方法，只是编译期的「许可」）：
    //   Send     : 该类型的值【可以转移】到另一个线程（所有权跨线程移动）
    //   Sync     : 该类型的值【可以被引用】&T 从多个线程同时访问
    // 关系：T: Sync 等价于 &T: Send；绝大多数类型两者都满足。

    let plain = vec![1, 2, 3];
    let handle = thread::spawn(move || plain.iter().sum::<i32>()); // Vec<i32>: Send ✔
    println!("Vec<i32> 是 Send，跨线程求和 = {}", handle.join().unwrap());
    println!("// 预期输出：Vec<i32> 是 Send，跨线程求和 = 6");

    // 反例（注释里给出编译错误，不实际运行）：
    // let rc_data = std::rc::Rc::new(5);
    // thread::spawn(move || println!("{rc_data}"));
    // error[E0277]: `Rc<i32>` cannot be sent between threads safely
    //   the trait `Send` is not implemented for `Rc<i32>`
    // 原因：Rc 的引用计数是普通加减法，两个线程同时 clone/drop 就会算错；
    //       Arc 的计数是原子操作，所以 Arc<i32>: Send + Sync。
    //
    // 常见类型速查：
    //   Send + Sync   : i32 / String / Vec<T> / Arc<T>（当 T: Send + Sync）
    //   仅 Sync 非 Send：MutexGuard（锁守卫不该被转移，防止把锁带去别的线程解锁）
    //   都不是        : Rc / RefCell（非原子、非线程安全的内部可变性）
    println!("Send 管「转移」，Sync 管「共享引用」，编译器据此拦截危险代码");

    // 未经任何同步的自定义类型（如 OnlyMainThread）默认也满足 Send + Sync，
    // 因为它只是普通数据；真正需要关心的是「含内部可变性/裸指针」的类型。
    let _marker = OnlyMainThread;
    println!("自定义普通结构体默认 Send + Sync（安全数据的默认许可）");
}

// ---------------------------------------------------------------------------
// 示例 8：典型场景 —— 并行求和（分块 + 合并）
// ---------------------------------------------------------------------------

fn demo_8_scenario_parallel_sum() {
    println!("--- 示例 8：场景：并行求和 ---");

    let numbers: Vec<u64> = (1..=1000).collect();
    let worker_count = 4;
    let chunk_size = (numbers.len() + worker_count - 1) / worker_count; // 250

    // 分块：每个线程算自己那一段（用 scope 直接借用，不需要 move）
    let partial_sums: Vec<u64> = thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in numbers.chunks(chunk_size) {
            handles.push(scope.spawn(|| chunk.iter().sum::<u64>()));
        }
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    // 合并：把各线程的部分和再求和
    let total: u64 = partial_sums.iter().sum();
    println!("分块部分和 = {partial_sums:?}");
    println!("// 预期输出：分块部分和 = [31375, 93875, 156375, 218875]");
    println!("1..=1000 并行求和 = {total}");
    println!("// 预期输出：1..=1000 并行求和 = 500500");

    // 这个「分而治之」结构正是 demoweb 里批量任务处理的骨架：
    // 切分数据 → 并发执行 → 收集合并；差异只在把线程换成 async 任务。
    println!("并行模式：切分 -> 并发 -> 收集合并");
}

// ---------------------------------------------------------------------------
// 示例 9：常见错误对照（全部注释掉，取消注释即可看到真实报错/结果）
// ---------------------------------------------------------------------------

fn demo_9_common_mistakes() {
    println!("--- 示例 9：常见错误 ---");

    // 错误 1：忘记 join —— 主线程可能先于子线程结束
    // thread::spawn(|| println!("可能永远没机会打印"));
    // println!("主线程先退出了");
    // 现象：子线程的输出可能一次都不出现（进程随主线程退出而终止）。
    // 修正：保存 JoinHandle 并 join；或用通道把结果送回主线程。

    // 错误 2：闭包没有 move，却引用了栈上变量
    // let data = vec![1, 2, 3];
    // thread::spawn(|| println!("{data:?}"));
    // error[E0373]: closure may outlive the current function, but it borrows `data`
    //   （子线程可能活得比 data 久 —— 悬垂借用，编译器直接拒绝）
    // 修正：move || 拿走所有权；或借用场景改用 thread::scope（示例 6）。

    // 错误 3：直接把 Rc 跨线程发送
    // let rc = std::rc::Rc::new(1);
    // thread::spawn(move || println!("{rc}"));
    // error[E0277]: `Rc<i32>` cannot be sent between threads safely（见示例 7）
    // 修正：Arc；需要可变再配 Mutex：Arc<Mutex<T>>。

    // 错误 4：锁中毒（持有锁的线程 panic 后，锁进入「中毒」状态）
    // let m = Arc::new(Mutex::new(0));
    // let m2 = Arc::clone(&m);
    // let _ = thread::spawn(move || { let _g = m2.lock().unwrap(); panic!("带着锁崩了"); }).join();
    // let g = m.lock();  // 返回 Err(PoisonError)：数据本身没坏，但需要显式处理
    // 修正：lock().unwrap() 或 lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    //       —— 确认数据一致后强行取出继续使用。

    // 错误 5：死锁 —— 两个线程以相反顺序获取两把锁（能编译，运行后永久卡住）
    // 线程 A：lock(m1) 成功后去 lock(m2)；
    // 线程 B：lock(m2) 成功后去 lock(m1)。
    // 结果：谁也等不到对方的锁。修正：全程序约定「统一的加锁顺序」；
    //       或缩小临界区、按需逐个加锁释放。
    println!("线程高频问题：忘 join / 缺 move / Rc 跨线程 / 锁中毒 / 死锁");
    println!("原则：能传所有权就别共享，能共享只读就别可变，必须可变就上锁且临界区最小");
}

// 本文件示例按 lesson-conventions.md 约定编写：
// 「// 预期输出：」为字面量断言，可用 .dsh/check_expected_output.ps1 一键核对。
