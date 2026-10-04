//! lesson_16_smart_pointers.rs —— 主题：智能指针（Box / Rc / RefCell / Weak）
//!
//! 学习目标：
//!   1. 理解智能指针 = 「结构体 + Deref/Drop」，知道 Box 为什么能让递归类型合法；
//!   2. 掌握 Rc 共享所有权：clone 不复制数据、引用计数的变化与时机；
//!   3. 掌握 RefCell 内部可变性：把借用检查从编译期挪到运行期，及其 panic 边界；
//!   4. 会组合 `Rc<RefCell<T>>` 解决「多个所有者 + 需要修改」的问题，会用 Weak 打破循环引用；
//!   5. 建立「按需求选指针」的决策直觉，为线程版 Arc/Mutex（lesson_17）打好基础。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_16_smart_pointers.rs -o lesson_16_smart_pointers && ./lesson_16_smart_pointers
//!   或在本项目根目录执行：cargo run --bin lesson_16_smart_pointers
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       demoweb 的 `Arc<AppState>`（连接池、计数器）是本课 Arc 的实战版；
//!       Box<dyn Trait> 的动态分发已在 lesson_12_traits 讲过，本课不再重复。

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

fn main() {
    println!("========== lesson_16_smart_pointers：智能指针 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_box_and_recursive_type();
    demo_2_box_basics();
    demo_3_deref_coercion();
    demo_4_rc_shared_ownership();
    demo_5_refcell_interior_mutability();
    demo_6_rc_refcell_combo();
    demo_7_weak_breaks_cycle();
    demo_8_scenario_shared_cache();
    demo_9_common_mistakes();
}

// ---------------------------------------------------------------------------
// 示例 1：Box 让递归类型合法 —— 编译器需要「已知大小」
// ---------------------------------------------------------------------------

enum List {
    Cons(i32, Box<List>), // Box 把「下一节点的无限大小」变成一个固定大小的指针
    Nil,
}

use List::{Cons, Nil};

/// 收集链表里的所有值（递归遍历）
fn list_values(list: &List) -> Vec<i32> {
    match list {
        Cons(value, tail) => {
            let mut values = list_values(tail);
            values.insert(0, *value); // 头插，保持顺序 1,2,3
            values
        }
        Nil => Vec::new(),
    }
}

fn demo_1_box_and_recursive_type() {
    println!("--- 示例 1：Box 与递归类型 ---");

    // 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表值 = {:?}", list_values(&list));
    println!("// 预期输出：链表值 = [1, 2, 3]");
    println!("List 占用 {} 字节（64 位平台）", std::mem::size_of::<List>());
    println!("// 预期输出：List 占用 16 字节（64 位平台）");

    // 如果不写 Box 会怎样？编译器直接拒绝：
    // enum List { Cons(i32, List), Nil }
    // error[E0072]: recursive type `List` has infinite size
    //   help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    // 原因：List 的大小 = i32 + List 的大小 + ...，编译期算不尽；
    // Box<T> 是固定 8 字节指针，把「无限」变成「有限」。
    println!("Box = 一层指针间接，让编译期大小计算停下来");
}

// ---------------------------------------------------------------------------
// 示例 2：Box 基本用法 —— 堆分配与手动解引用
// ---------------------------------------------------------------------------

fn demo_2_box_basics() {
    println!("--- 示例 2：Box 基础 ---");

    // 大数组放堆上（栈上只留一个指针），避免拷贝与栈溢出
    let big = Box::new([0i32; 1000]);
    println!("big[0] = {}（索引访问自动穿透 Box）", big[0]);
    println!("// 预期输出：big[0] = 0（索引访问自动穿透 Box）");

    // Box 实现了 Deref：*boxed 相当于 *(boxed.deref())
    let boxed_num = Box::new(42);
    println!("*boxed_num = {}", *boxed_num);
    println!("// 预期输出：*boxed_num = 42");

    // Box 离开作用域自动释放堆内存（Drop），不需要手动 free
    println!("Box 离开作用域自动释放（Drop）");
}

// ---------------------------------------------------------------------------
// 示例 3：Deref 强转 —— &Box<String> 能直接当 &str 用
// ---------------------------------------------------------------------------

/// 参数写成 &str（最通用的形式）：&String、&Box<String> 都能自动转进来
fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn demo_3_deref_coercion() {
    println!("--- 示例 3：Deref 自动强转 ---");

    let boxed = Box::new(String::from("hello"));

    // 编译器按 Deref 链自动强转：&Box<String> → &String → &str
    // （String 实现了 Deref<Target = str>，Box 实现了 Deref<Target = T>，层层穿透）
    println!("char_count(&Box<String>) = {}", char_count(&boxed));
    println!("// 预期输出：char_count(&Box<String>) = 5");

    // 直接传 &String 同理成立 —— 所以函数参数尽量写 &str / &[T] 这类「切片形式」
    let plain = String::from("你好");
    println!("char_count(&String) = {}", char_count(&plain));
    println!("// 预期输出：char_count(&String) = 2");

    println!("Deref 强转让智能指针「像」它包裹的类型一样好用");
}

// ---------------------------------------------------------------------------
// 示例 4：Rc —— 引用计数的共享所有权（单线程）
// ---------------------------------------------------------------------------

fn demo_4_rc_shared_ownership() {
    println!("--- 示例 4：Rc 共享所有权 ---");

    // Rc::clone 只增加计数、复制指针，不复制堆上的字符串
    let a = Rc::new(String::from("共享数据"));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    println!("三个所有者，strong_count = {}", Rc::strong_count(&a));
    println!("// 预期输出：三个所有者，strong_count = 3");
    println!("b 读到的内容 = {b}");
    println!("// 预期输出：b 读到的内容 = 共享数据");

    // 释放一个所有者：计数减 1，数据仍然活着
    drop(c);
    println!("drop(c) 后 strong_count = {}", Rc::strong_count(&a));
    println!("// 预期输出：drop(c) 后 strong_count = 2");

    // 最后一个所有者（a、b 都离开作用域）离开时数据才真正释放 ——
    // 这就是「共享所有权」：有几个所有者就计数到几，归零即释放。
    println!("Rc::clone 只加计数，不复制数据；计数归零才释放");
}

// ---------------------------------------------------------------------------
// 示例 5：RefCell —— 内部可变性（把借用检查挪到运行期）
// ---------------------------------------------------------------------------

fn demo_5_refcell_interior_mutability() {
    println!("--- 示例 5：RefCell 内部可变性 ---");

    let cell = RefCell::new(5);
    // cell 本身不可变（没有 mut），却能在内部改值 —— 这就是「内部可变性」：
    // 编译期不再拦「共享引用下的修改」，改为运行期动态记录借用数
    *cell.borrow_mut() += 1;
    *cell.borrow_mut() += 1;
    println!("两次 borrow_mut 后 = {}", cell.borrow());
    println!("// 预期输出：两次 borrow_mut 后 = 7");

    // 运行期借用规则与编译期完全一致：
    //   * 同时可以有多个 borrow()（共享）；
    //   * borrow_mut()（可变）存在时，任何其他借用都会 panic：
    //     let reader = cell.borrow();
    //     let writer = cell.borrow_mut(); // panic! already borrowed: BorrowMutError
    //     drop(reader);                   // 修正：先释放共享借用再申请可变借用
    //
    // 何时值得用：多个所有者（Rc）下需要修改、或「逻辑上只读但内部要缓存/惰性初始化」时。
    // 代价：把编译期错误变成运行期 panic —— 能不用就不用，能缩小范围就缩小范围。
    println!("RefCell = 运行期借用检查：违规不是编译错误而是 panic");
}

// ---------------------------------------------------------------------------
// 示例 6：Rc<RefCell<T>> 组合 —— 多个所有者 + 需要修改
// ---------------------------------------------------------------------------

fn demo_6_rc_refcell_combo() {
    println!("--- 示例 6：Rc<RefCell<T>> 组合 ---");

    // 场景：一份成绩单，多个「持有者」都能往里追加分数
    let scores = Rc::new(RefCell::new(vec![10, 20]));
    let second_owner = Rc::clone(&scores);

    // 每个持有者都只有「不可变」的 Rc，但 RefCell 允许内部修改
    second_owner.borrow_mut().push(30);
    scores.borrow_mut()[0] += 5; // 第一名加 5 分

    println!("最终成绩 = {:?}", scores.borrow());
    println!("// 预期输出：最终成绩 = [15, 20, 30]");
    println!("持有者数量 = {}", Rc::strong_count(&scores));
    println!("// 预期输出：持有者数量 = 2");

    // 读法：Rc 管「几个所有者」（编译期共享），RefCell 管「能不能改」（运行期借用）。
    // 两层各司其职 —— 这是单线程下最常见的组合，也是理解 Arc<Mutex<T>> 的跳板。
    println!("Rc 管所有者数量，RefCell 管可变性");
}

// ---------------------------------------------------------------------------
// 示例 7：Weak —— 打破循环引用，防止内存泄漏
// ---------------------------------------------------------------------------

struct Node {
    name: String,
    parent: RefCell<Weak<Node>>, // 用 Weak 指回父节点：不增加强计数
}

fn make_node(name: &str, parent: Weak<Node>) -> Rc<Node> {
    Rc::new(Node {
        name: String::from(name),
        parent: RefCell::new(parent),
    })
}

fn demo_7_weak_breaks_cycle() {
    println!("--- 示例 7：Weak 打破循环引用 ---");

    let parent = make_node("父节点", Weak::new());
    // 子节点用 Rc::downgrade 拿到父节点的 Weak：strong_count 不会变成 2
    let child = make_node("子节点", Rc::downgrade(&parent));

    {
        let upgraded = child.parent.borrow().upgrade();
        println!(
            "child 找到的父节点 = {}",
            upgraded
                .as_ref()
                .map(|n| n.name.as_str())
                .unwrap_or("（已被释放）")
        );
        println!("// 预期输出：child 找到的父节点 = 父节点");
    }
    println!(
        "parent 的 strong_count = {}（Weak/upgrade 不计入）",
        Rc::strong_count(&parent)
    );
    println!("// 预期输出：parent 的 strong_count = 1（Weak/upgrade 不计入）");

    // 父节点释放后，子节点的 Weak 还在，但 upgrade 返回 None —— 引用不悬垂
    drop(parent);
    let upgraded = child.parent.borrow().upgrade();
    println!("父节点释放后 upgrade 是 None = {}", upgraded.is_none());
    println!("// 预期输出：父节点释放后 upgrade 是 None = true");

    // 反面教材：如果 child 用 Rc 持有 parent、parent 又持有 child（互为 strong），
    // 两者计数永不归零 → 数据永远不会释放（内存泄漏）。规则：回指/环上一律用 Weak。
    println!("原则：正向持有用 Rc，回指一律用 Weak");
}

// ---------------------------------------------------------------------------
// 示例 8：典型场景 —— 多模块共享一份可变缓存
// ---------------------------------------------------------------------------

/// 缓存的「访问句柄」：调用方各自持有 Rc 克隆，共享同一份 RefCell<HashMap>
fn demo_8_scenario_shared_cache() {
    println!("--- 示例 8：场景：共享可变缓存 ---");

    let cache = Rc::new(RefCell::new(HashMap::new()));

    // 模拟两个模块各自拿到缓存的句柄
    let module_a = Rc::clone(&cache);
    let module_b = Rc::clone(&cache);

    module_a
        .borrow_mut()
        .insert(String::from("user:name"), 1);
    module_b
        .borrow_mut()
        .insert(String::from("user:age"), 2);
    // module_a 再补一条并修改 module_b 写入的值
    module_a.borrow_mut().insert(String::from("user:age"), 18);

    // 读取时按键名排序，保证输出确定
    let mut keys: Vec<String> = cache.borrow().keys().cloned().collect();
    keys.sort();
    for key in &keys {
        println!("{key} = {}", cache.borrow()[key]);
    }
    println!("// 预期输出：user:age = 18");
    println!("// 预期输出：user:name = 1");

    println!("共享句柄各自修改，看到的都是同一份数据");
}

// ---------------------------------------------------------------------------
// 示例 9：常见错误对照（全部注释掉，取消注释即可看到真实报错）
// ---------------------------------------------------------------------------

fn demo_9_common_mistakes() {
    println!("--- 示例 9：常见错误 ---");

    // 错误 1：想直接改 Rc 里的值
    // let a = Rc::new(5);
    // *a += 1;   // error[E0596]: cannot borrow data in an `Rc` as mutable
    //            //   trait `DerefMut` is required to modify through a dereference
    // 修正：Rc<RefCell<T>>（本课示例 6）；跨线程则 Arc<Mutex<T>>（lesson_17）。

    // 错误 2：RefCell 借用重叠 panic（编译通过，运行时崩溃）
    // let cell = RefCell::new(String::from("数据"));
    // let first = cell.borrow();
    // let second = cell.borrow_mut(); // panic! already borrowed: BorrowMutError
    //                                 //   （borrow() 还没释放就申请可变借用）
    // 修正：缩小每个借用的作用域，用完立刻 drop；不要把 Ref 在手里跨函数传递。

    // 错误 3：循环引用导致内存泄漏（不崩溃，但内存永远不会释放）
    // struct Parent { child: Option<Rc<Node>> }   // 两个节点互持 Rc（都是强引用）
    // struct Node   { parent: Option<Rc<Parent>> }
    // 结果：离开作用域后双方计数都是 1 → 永不归零 → 泄漏。
    // 修正：回指方向改用 Weak（本课示例 7）。

    // 错误 4：把 Rc 传给另一个线程
    // let data = Rc::new(5);
    // std::thread::spawn(move || println!("{data}"));
    // error[E0277]: `Rc<i32>` cannot be sent between threads safely
    //   （非原子的引用计数，并发增减可能算错；详见 lesson_17 的 Send/Sync 与 Arc）
    // 修正：跨线程用 Arc；Arc<RefCell<T>> 仍然不行，配 Mutex/RwLock。

    // 错误 5：递归类型忘记加间接层
    // enum Tree { Leaf, Node(Tree, Tree) }
    // error[E0072]: recursive type `Tree` has infinite size
    // 修正：Node(Box<Tree>, Box<Tree>)（本课示例 1）。
    println!("智能指针高频错误：E0596 / E0277 / E0072 / RefCell panic / 循环泄漏");
    println!("决策：独占堆 Box；共享只读 Rc；共享可变 Rc<RefCell>；跨线程 Arc<Mutex>");
}

// 本文件示例按 lesson-conventions.md 约定编写：
// 「// 预期输出：」为字面量断言，可用 .dsh/check_expected_output.ps1 一键核对。
