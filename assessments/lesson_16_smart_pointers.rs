//! assessments/lesson_16_smart_pointers.rs —— 考核：智能指针（对应 lesson_16）
//!
//! - 对应课程：`src/tutorial/lesson_16_smart_pointers.rs`
//! - 知识点出处：`src/tutorial/README.md` 第五阶段「16 智能指针」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_16_smart_pointers     # 只考这一课
//!   cargo test                                      # 考全部 18 课
//!   cargo run --bin assessment_report               # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_16_xx_xxx` 练习函数（或 `impl` 块里的练习方法），
//!    它的函数体里只有一行 `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_16_smart_pointers`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数与 `impl List` / `impl Node` 里标了「【待实现】」的方法，
//!   可以按需增加局部变量与辅助函数；
//! - 顶部「提供给你的类型（不要修改）」一节里的 `enum` / `struct` / `collect_values` /
//!   `Node::new` 都是**已经实现好**的真实代码，请原样保留（它们是骨架态零警告编译的前提）；
//! - 练习函数里需要的 `use`（例如 `std::collections::HashMap`）请写在**函数体内**，
//!   这样骨架态不会出现 unused import 警告；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 9）
//!
//! | 代码 | 报错 / 现象 | 修正方法 |
//! | --- | --- | --- |
//! | `let a = Rc::new(5); *a += 1;` | `error[E0596]` cannot borrow data in an `Rc` as mutable | 要改内部值就用 `Rc<RefCell<T>>`（示例 6）；跨线程改用 `Arc<Mutex<T>>`（lesson_17） |
//! | `let r = cell.borrow(); let w = cell.borrow_mut();` | 编译通过，运行期 panic：`already borrowed: BorrowMutError` | 缩小借用范围、用完立刻 `drop`；不要把 `Ref` 跨函数传递 |
//! | 两个节点互相持有 `Rc`（都是强引用） | 编译通过、运行期不报错，但计数永不归零 → 内存泄漏 | 回指 / 环上一律用 `Weak`（示例 7） |
//! | 把 `Rc` 送进线程 | `error[E0277]` `Rc<i32>` cannot be sent between threads safely | 跨线程用 `Arc`；需要可变再配 `Mutex`（lesson_17） |
//! | `enum Tree { Leaf, Node(Tree, Tree) }` | `error[E0072]` recursive type `Tree` has infinite size | 加一层间接：`Node(Box<Tree>, Box<Tree>)`（示例 1） |

use assessment_harness::{Kind, assess, eq, eq_slice, is_false, panics_real};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_16";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// ===========================================================================
//
// 下面这些类型与辅助函数由考核文件提供，保证骨架态就能零警告编译通过。
// `collect_values` 与 `Node::new` 是**已经实现好**的真实代码（骨架态也要用到它们，
// 所以不能留占位）；学员只需要实现标了「【待实现】」的方法与练习函数。

/// 单链表：递归类型必须用 `Box` 才「算得出大小」（`len` / `sum` 由学员实现，见 kp_16_01）。
#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// 收集链表里的所有值（递归遍历，头元素在前）。
///
/// 这是**已经实现好**的只读辅助函数：它用 `match` 把 `Cons` 的两个字段都读了一遍，
/// 所以骨架态不会出现「字段从未被读取」的 dead_code 警告；测试也先用它证明
/// 「学员构造出来的链表内容是对的」，再去检查 `len` / `sum`。
fn collect_values(list: &List) -> Vec<i32> {
    match list {
        List::Cons(value, tail) => {
            let mut values = collect_values(tail);
            values.insert(0, *value);
            values
        }
        List::Nil => Vec::new(),
    }
}

/// 环上的节点：`next` 用 `Weak` 指向下一个节点（不增加强计数，见 kp_16_07）。
#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Weak<Node>>,
}

impl Node {
    /// 提供的构造函数（**已经实现好**）：造一个 `next` 为「空 Weak」的节点。
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            next: RefCell::new(Weak::new()),
        })
    }
}

// ===========================================================================
// kp_16_01 Box 与递归类型：链表的长度与元素和
// ===========================================================================

/// 知识点考核：用 `Box` 构成递归类型，并递归遍历它。
#[test]
fn kp_16_01_recursive_list_len_sum() {
    assess(
        M,
        "kp_16_01",
        "Box 与递归类型：用 Box 把「无限大小」变成固定大小，再递归处理它",
        Kind::Core,
        "复习 lesson_16 示例 1（box_and_recursive_type）：`enum List { Cons(i32, Box<List>), Nil }` \
         靠 `Box`（固定 8 字节指针）打断「大小递归」；`len` / `sum` 两个方法都要写递归调用\
         （`1 + tail.len()` / `*value + tail.sum()`），`Nil` 分支返回 0。\
         不加 Box 会报 `error[E0072]: recursive type `List` has infinite size`。",
        || {
            // 正常用例：1 -> 2 -> 3 -> Nil
            let list = List::Cons(
                1,
                Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
            );
            // 先用提供的 collect_values 证明「链表构造正确」，再考 len / sum
            eq_slice(
                &collect_values(&list),
                &[1, 2, 3],
                "先确认链表内容：Cons(1, Cons(2, Cons(3, Nil))) 应该收集出 [1, 2, 3]",
            );
            eq(list.len(), 3usize, "正常用例：三个节点，len() 应返回 3");
            eq(
                list.sum(),
                6i32,
                "正常用例：1 + 2 + 3 = 6，sum() 要一路递归到 Nil 再返回 0",
            );

            // 边界：Nil（空链表）
            let empty = List::Nil;
            eq_slice(&collect_values(&empty), &[], "边界：Nil 收集出来是空 Vec");
            eq(
                empty.len(),
                0usize,
                "边界：Nil 的长度是 0（不能 panic、也不能算成 1）",
            );
            eq(
                empty.sum(),
                0i32,
                "边界：Nil 的元素和是 0——0 是加法的单位元，递归的「出口」就返回它",
            );

            // 边界：只有一个元素
            let single = List::Cons(7, Box::new(List::Nil));
            eq(single.len(), 1usize, "边界：单元素链表的长度是 1");
            eq(single.sum(), 7i32, "边界：单元素链表的和就是那个元素本身");
        },
    );
}

impl List {
    /// 【待实现】递归计算链表长度。
    ///
    /// 实现要求：
    ///   - `Cons(_, tail)` → `1 + tail.len()`；`Nil` → `0`（`Nil` 是递归出口，必须先写，
    ///     否则会无限递归直到栈溢出）；
    ///   - 用 `match self { ... }` 匹配（`self` 是 `&List`，匹配出来的 `tail` 是 `&Box<List>`，
    ///     直接写 `tail.len()` 会自动穿透 Box 调到本方法）；
    ///   - 返回类型是 `usize`，字面量写 `0` / `1` 即可（会被推断成 `usize`）。
    ///
    /// 示例输入：
    /// ```text
    /// list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    /// ```
    /// 示例输出：
    /// ```text
    /// 3
    /// ```
    fn len(&self) -> usize {
        assessment_harness::todo_exercise(
            "List::len",
            "递归求长度：Cons(_, tail) => 1 + tail.len()，Nil => 0",
            (self,),
        )
    }

    /// 【待实现】递归计算所有元素之和。
    ///
    /// 实现要求：
    ///   - `Cons(value, tail)` → `*value + tail.sum()`；`Nil` → `0`；
    ///   - 注意 `&self` 下匹配出来的 `value` 是 `&i32`，要解引用（`*value`）才能参与加法
    ///     （`&i32 + i32` 其实也能编译，标准库为引用实现了 `Add`，但显式解引用更能说明
    ///     「Box / 引用都只是间接层，值本身在堆上」）。
    ///
    /// 示例输入：
    /// ```text
    /// list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    /// ```
    /// 示例输出：
    /// ```text
    /// 6
    /// ```
    fn sum(&self) -> i32 {
        assessment_harness::todo_exercise(
            "List::sum",
            "递归求和：Cons(value, tail) => *value + tail.sum()，Nil => 0",
            (self,),
        )
    }
}

// ===========================================================================
// kp_16_02 Box 的堆分配与解引用
// ===========================================================================

/// 知识点考核：`Box::new` 与 `*` 解引用。
#[test]
fn kp_16_02_box_deref() {
    assess(
        M,
        "kp_16_02",
        "Box<T>：唯一所有权 + 堆分配，`*b` 解引用取出里面的值",
        Kind::Basic,
        "复习 lesson_16 示例 2（box_basics）与示例 3（deref_coercion）：`Box::new(5)` 把 5 分配到堆上，\
         变量本身只是一个固定大小的指针（在栈上）；`*b` 通过 `Deref` 取出堆上的值，\
         所以 `&Box<String>` 还能自动强转成 `&str`——这就是 Deref 强制转换。",
        || {
            let (deref, plus_one) = exercise_16_02_box_deref();

            // 正常用例
            eq(
                deref,
                5i32,
                "正常用例：`*b` 解引用 `Box::new(5)`，取到的就是堆上的 5",
            );
            eq(
                plus_one,
                6i32,
                "正常用例：`*b + 1` = 6——解引用之后它就是一个普通的 i32",
            );

            // 边界：两个分量必须真的不同（防「+1 没写」这类笔误）
            is_false(
                deref == plus_one,
                "边界：两个分量必须不同——常见笔误是漏掉 +1，直接返回 (5, 5)",
            );
            // 边界：Box 不改变它装的值，极值也原样
            let boundary = Box::new(i32::MIN);
            eq(
                *boundary,
                i32::MIN,
                "边界：Box 不会改写它装的值——i32::MIN 放进去，解引用后仍是 i32::MIN",
            );
        },
    );
}

/// 【待实现】用 `Box::new(5)` 体会堆分配与解引用。
///
/// 实现要求：
///   - 用 `let b = Box::new(5);` 把 5 放到堆上；
///   - 返回 `(*b, *b + 1)`，也就是 `(5, 6)`；
///   - 请在实现里写一句注释说明两件事：Box =「唯一所有权 + 堆分配」（栈上只有一个指针），
///     以及 `*b` 能直接取值是因为 `Box<T>` 实现了 `Deref<Target = T>`；
///   - `*b` 就是解引用；Box 离开作用域会自动释放堆内存（Drop），不需要手动 free
///     （`Box::new(5)` 里的 5 是 `i32`，所以返回类型是 `(i32, i32)`）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (5, 6)
/// ```
fn exercise_16_02_box_deref() -> (i32, i32) {
    assessment_harness::todo_exercise(
        "exercise_16_02_box_deref",
        "用 Box::new(5) 造一个堆上的值，返回 (*b, *b + 1) = (5, 6)",
        (),
    )
}

// ===========================================================================
// kp_16_03 Rc 共享所有权：clone 只加计数，不复制数据
// ===========================================================================

/// 知识点考核：`Rc::clone` 与 `Rc::strong_count` 的变化。
#[test]
fn kp_16_03_rc_strong_count() {
    assess(
        M,
        "kp_16_03",
        "Rc 共享所有权：`Rc::clone` 只让计数 +1，不复制堆上的数据",
        Kind::Core,
        "复习 lesson_16 示例 4（rc_shared_ownership）：`Rc::clone(&a)` 复制的只是一个「指向同一份堆数据的指针」，\
         `Rc::strong_count` 因此从 1 变成 2；`drop` 掉一个所有者后计数回到 1；\
         只有计数归零（最后一个所有者离开）时，堆上的数据才真正释放。",
        || {
            let (after_clone, after_drop) = exercise_16_03_rc_counts();

            // 正常用例
            eq(
                after_clone,
                2usize,
                "正常用例：Rc::new 一个 + Rc::clone 一个 → strong_count = 2",
            );
            eq(
                after_drop,
                1usize,
                "正常用例：drop 掉那个克隆之后，计数回到 1",
            );
            // 边界：只丢掉一个所有者，计数不会归零、数据仍然活着
            is_false(
                after_drop == 0,
                "边界：只丢掉一个所有者，计数不会归零——归零意味着堆上的数据已经被释放，\
                 而这里明明还有一个所有者活着",
            );
        },
    );
}

/// 【待实现】观察 `Rc::strong_count` 的变化。
///
/// 实现要求：
///   - `let first = Rc::new(String::from("shared"));`
///   - `let second = Rc::clone(&first);`，先记下 `Rc::strong_count(&first)`（应为 2）；
///   - `drop(second);`，再记下 `Rc::strong_count(&first)`（应为 1）；
///   - 返回 `(克隆后的计数, drop 之后的计数)`，即 `(2, 1)`；
///   - `Rc::clone` 是关联函数调用，它**只增加引用计数、复制一个指针**，堆上的那个 `String`
///     （"shared"）从头到尾只有一份——请在实现注释里点明「clone 不复制数据」，这正是
///     `Rc::clone` 与 `String::clone` 最大的差别。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (2, 1)
/// ```
fn exercise_16_03_rc_counts() -> (usize, usize) {
    assessment_harness::todo_exercise(
        "exercise_16_03_rc_counts",
        "Rc::new(String::from(\"shared\")) + Rc::clone + drop(second)，返回 (2, 1)",
        (),
    )
}

// ===========================================================================
// kp_16_04 RefCell 内部可变性：把借用检查挪到运行期
// ===========================================================================

/// 知识点考核：不可变绑定下的内部可变性。
#[test]
fn kp_16_04_refcell_interior_mutability() {
    assess(
        M,
        "kp_16_04",
        "RefCell 内部可变性：把借用检查从编译期挪到运行期",
        Kind::Core,
        "复习 lesson_16 示例 5（refcell_interior_mutability）：`let cell = RefCell::new(0);` 没写 `mut`，\
         却可以写 `*cell.borrow_mut() += 5;`——`borrow_mut()` 在运行期动态登记「当前有一个可变借用」，\
         守卫（`RefMut`）一 drop 就归还；编译期只看到「共享引用」，所以借用检查被推迟到了运行期。",
        || {
            let total = exercise_16_04_refcell();

            // 正常用例
            eq(
                total,
                10i32,
                "正常用例：`RefCell::new(0)` 之后两次 `*cell.borrow_mut() += 5`，结果是 10",
            );
            // 边界/防呆：两次修改都必须落到同一份数据上
            is_false(
                total == 5,
                "边界：两次修改都要生效——只加一次会得到 5（常见原因是第二次 borrow_mut 之前\
                 把 RefMut 守卫提前 drop 了，或者干脆只写了一次）",
            );
        },
    );
}

/// 【待实现】用 `RefCell` 在「不可变绑定」里改值。
///
/// 实现要求：
///   - `let cell = RefCell::new(0);`（注意：**不要**写 `let mut cell`，本知识点考的就是不需要 `mut`）；
///   - 连续两次 `*cell.borrow_mut() += 5;`；
///   - 最后用 `*cell.borrow()` 把当前值返回（应为 10）；
///   - `borrow_mut()` 返回 `RefMut` 守卫，语句结束就自动归还借用；运行期的借用规则与编译期
///     完全一致——同一时刻只能有一个 `borrow_mut()`，否则 panic（kp_16_05 专门考这个边界）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_16_04_refcell() -> i32 {
    assessment_harness::todo_exercise(
        "exercise_16_04_refcell",
        "RefCell::new(0) 之后两次 *cell.borrow_mut() += 5，返回 10",
        (),
    )
}

// ===========================================================================
// kp_16_05 RefCell 的边界：借用重叠会在运行期 panic
// ===========================================================================

/// 知识点考核：故意触发 `RefCell` 的「借用重叠」panic。
#[test]
fn kp_16_05_double_borrow_panics() {
    assess(
        M,
        "kp_16_05",
        "RefCell 的运行期边界：可变借用没释放就再借用 → panic（不是编译错误）",
        Kind::Edge,
        "复习 lesson_16 示例 5 的注释与示例 9 的错误 2：`let first = cell.borrow(); \
         let second = cell.borrow_mut();` 能编译通过，运行期却 panic：\
         `already borrowed: BorrowMutError`；修正办法是缩小借用作用域、用完立刻 drop。",
        || {
            // 正常（对照）用例：先把 RefMut 守卫的作用域结束，再 borrow —— 不会 panic
            let safe = RefCell::new(0);
            {
                let mut writer = safe.borrow_mut();
                *writer += 1;
            } // writer 在这里 drop：可变借用归还
            eq(
                *safe.borrow(),
                1i32,
                "对照用例：先结束 RefMut 守卫的作用域（drop）再 borrow，只会读到 1，不会 panic",
            );

            // 边界用例：练习函数「故意」写出会 panic 的代码
            panics_real(
                || {
                    exercise_16_05_double_borrow();
                },
                "边界：`borrow_mut()` 的守卫还活着时再 `borrow()`，RefCell 必然 panic\
                 （already mutably borrowed）——这就是「内部可变性把借用检查挪到运行期」的代价",
            );
        },
    );
}

/// 【待实现】故意写出「双重借用」的代码，确认你知道边界在哪里。
///
/// 实现要求（本题就是要写错一次，看它怎么炸）：
///   - `let cell = RefCell::new(1);`
///   - `let a = cell.borrow_mut();`（`a` 是 `RefMut`，此处**故意**不让它提前 drop）；
///   - 在 `a` 还活着时写 `let b = cell.borrow();` —— 这一行会在运行期 panic；
///   - 最后返回 `*b`（这行永远执行不到，但必须通过类型检查）；
///   - 本考核用的是 `panics_real(...)` 而不是 `panics(...)`：骨架态下占位函数自己就会 panic，
///     普通 `panics` 会把「还没实现」误判成「边界写对了」，`panics_real` 则会检查 panic 信息里
///     是否含「未实现」，含则判为未通过；
///   - 修正写法（编译期一样、运行期不炸）：先把 `a` 的作用域结束，再 `borrow()`；真实项目里
///     宁可用 `try_borrow_mut()` 返回 `Result`，也不要让线上代码 panic。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// panic: already borrowed
/// ```
fn exercise_16_05_double_borrow() -> i32 {
    assessment_harness::todo_exercise(
        "exercise_16_05_double_borrow",
        "先 let a = cell.borrow_mut()，在 a 还活着时再 let b = cell.borrow()，返回 *b（必然 panic）",
        (),
    )
}

// ===========================================================================
// kp_16_06 Rc<RefCell<T>> 组合：多个所有者 + 需要修改
// ===========================================================================

/// 知识点考核：`Rc<RefCell<T>>` 组合。
#[test]
fn kp_16_06_rc_refcell_combo() {
    assess(
        M,
        "kp_16_06",
        "Rc<RefCell<T>>：Rc 管「几个所有者」，RefCell 管「能不能改」",
        Kind::Core,
        "复习 lesson_16 示例 6（rc_refcell_combo）：两个 `Rc::clone` 指向同一份 `RefCell`，\
         各自 `borrow_mut()` 改的都是**同一份**数据（不是各自的副本），所以两次 +1 得到 2；\
         同时 `Rc::strong_count` 说明确实只有 2 个所有者。",
        || {
            let (value, owners) = exercise_16_06_rc_refcell();

            // 正常用例
            eq(
                value,
                2i32,
                "正常用例：两个克隆各 `borrow_mut` +1，共享的那份数据变成 2",
            );
            eq(
                owners,
                2usize,
                "正常用例：`Rc::clone` 一次 → strong_count = 2",
            );
            // 边界/防呆：改的必须是同一份数据
            is_false(
                value == 1,
                "边界：结果是 2 而不是 1——若两个克隆改的是各自的副本，就说明它们没有共享同一份 RefCell",
            );
        },
    );
}

/// 【待实现】用 `Rc<RefCell<i32>>` 让两个所有者修改同一份数据。
///
/// 实现要求：
///   - `let first = Rc::new(RefCell::new(0));`
///   - `let second = Rc::clone(&first);`
///   - 两个所有者各写一次 `*xxx.borrow_mut() += 1;`（先后顺序随意）；
///   - 返回 `(*first.borrow(), Rc::strong_count(&first))`，即 `(2, 2)`；
///   - `Rc` 给的是「共享的不可变访问」，`RefCell` 在共享访问里开出「运行期可变借用」的口子，
///     两层各司其职——单线程下最常见的组合，也是 `Arc<Mutex<T>>`（lesson_17）的跳板；
///   - 写 `first.borrow_mut()` 就行：`Rc` 会自动解引用到内部的 `RefCell`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (2, 2)
/// ```
fn exercise_16_06_rc_refcell() -> (i32, usize) {
    assessment_harness::todo_exercise(
        "exercise_16_06_rc_refcell",
        "Rc<RefCell<i32>> 的两个克隆各 borrow_mut +1，返回 (最终值 2, strong_count 2)",
        (),
    )
}

// ===========================================================================
// kp_16_07 Weak 打破循环引用：link 与 next_value
// ===========================================================================

/// 知识点考核：用 `Weak` 让两个节点互相指向而不泄漏。
#[test]
fn kp_16_07_weak_breaks_cycle() {
    assess(
        M,
        "kp_16_07",
        "Weak 打破循环引用：`Rc::downgrade` 不加强计数，`upgrade()` 拿临时强引用",
        Kind::Hard,
        "复习 lesson_16 示例 7（weak_breaks_cycle）：正向持有用 `Rc`，回指 / 环上一律用 `Weak`——\
         `Rc::downgrade(&child)` 存进 `RefCell<Weak<Node>>`，强计数不增加；\
         要读的时候 `upgrade()` 返回 `Option<Rc<Node>>`，对方已被释放时得到 `None`（不会悬垂）。\
         如果两边都用 `Rc` 互指，计数永不归零 → 内存泄漏（示例 9 的错误 3）。",
        || {
            let a = Node::new(1);
            let b = Node::new(2);
            Node::link(&a, &b); // a.next -> b（Weak）
            Node::link(&b, &a); // b.next -> a（Weak）

            // 正常用例：Weak 不增加强计数（若用 Rc 互指，两个计数都会变成 2 → 永不释放）
            eq(
                Rc::strong_count(&a),
                1usize,
                "正常用例：a 被 b 的 Weak 指着，但 strong_count(&a) 仍然是 1",
            );
            eq(
                Rc::strong_count(&b),
                1usize,
                "正常用例：b 同理——若这里用 Rc 互指，两个计数都会是 2，谁都不释放（内存泄漏）",
            );
            eq(
                Node::next_value(&a),
                Some(2),
                "正常用例：a 通过 Weak 升级拿到 b，next_value(&a) = Some(2)",
            );
            eq(
                Node::next_value(&b),
                Some(1),
                "正常用例：反方向也成立，next_value(&b) = Some(1)",
            );

            // 边界：Weak 不延长对方生命周期——a 被释放后，b 的 Weak 升级失败
            drop(a);
            eq(
                Node::next_value(&b),
                None,
                "边界：drop(a) 之后升级失败返回 None（Weak 不会悬垂，也不会阻止对方释放）",
            );
        },
    );
}

impl Node {
    /// 【待实现】把 `parent` 的下一个节点设为 `child`（只存 `Weak`，不加强计数）。
    ///
    /// 实现要求：
    ///   - `*parent.next.borrow_mut() = Rc::downgrade(child);`
    ///     （`Rc::downgrade` 把 `&Rc<Node>` 降级成 `Weak<Node>`）；
    ///   - `parent.next` 是 `RefCell<Weak<Node>>`，要用 `borrow_mut()` 拿到运行期可变借用，
    ///     再对里面的 `Weak` 整体赋值；
    ///   - `link` 不需要返回值（返回 `()`）；这里写 `Rc::clone(child)` 类型就对不上——
    ///     `Weak` 只能来自 `Rc::downgrade` 或 `Weak::new()`。
    ///
    /// 示例输入：
    /// ```text
    /// parent = Node::new(1)
    /// child  = Node::new(2)
    /// ```
    /// 示例输出：
    /// ```text
    /// Rc::strong_count(&child) 仍为 1（只存 Weak，不增加强计数）
    /// ```
    fn link(parent: &Rc<Node>, child: &Rc<Node>) {
        assessment_harness::todo_exercise(
            "Node::link",
            "把 parent.next（RefCell<Weak<Node>>）设为 Rc::downgrade(child)",
            (parent.value, child.value),
        )
    }

    /// 【待实现】读取某个节点 `next` 指向的值。
    ///
    /// 实现要求：
    ///   - 用 `upgrade()` 把 `Weak` 升成 `Option<Rc<Node>>`，再取出里面的 `value`：
    ///     `node.next.borrow().upgrade().map(|next| next.value)`；
    ///   - `upgrade()` 返回 `None` 表示对方已经被释放——这就是 `Weak` 与裸指针的区别：
    ///     访问前会检查有效性，不可能出现悬垂引用；
    ///   - `Weak::upgrade` 接收 `&self`，所以要先 `borrow()` 借出 `Weak`，并让整条链在一个
    ///     表达式里完成，避免把 `Ref` 留在手上导致后面的借用冲突。
    ///
    /// 示例输入：
    /// ```text
    /// node = a（a.next 经 Node::link(&a, &b) 指向存活的 b，b.value = 2）
    /// ```
    /// 示例输出：
    /// ```text
    /// Some(2)
    /// ```
    fn next_value(node: &Rc<Node>) -> Option<i32> {
        assessment_harness::todo_exercise(
            "Node::next_value",
            "用 node.next.borrow().upgrade() 升成 Option<Rc<Node>>，再 map 取出 value",
            (&node.next,),
        )
    }
}

// ===========================================================================
// kp_16_08 场景：Rc<RefCell<HashMap>> 共享可变缓存 + 命中统计
// ===========================================================================

/// 知识点考核：共享可变缓存与命中统计。
#[test]
fn kp_16_08_shared_cache() {
    assess(
        M,
        "kp_16_08",
        "典型场景：`Rc<RefCell<HashMap>>` 做共享可变缓存，并统计命中次数",
        Kind::Hard,
        "复习 lesson_16 示例 8（scenario_shared_cache）：缓存的句柄用 `Rc::clone` 共享，\
         内部用 `RefCell<HashMap<..>>` 允许修改；「命中」= 查到了已有键，\
         「未命中」= 没查到、需要计算并写入。注意借用的作用域：先取值（语句结束即归还借用），\
         再考虑要不要 `borrow_mut()` 插入，否则会撞上运行期借用冲突。",
        || {
            let (value, hits) = exercise_16_08_shared_cache();

            // 正常用例
            eq(
                value,
                42i32,
                "正常用例：缓存里的值就是第一次未命中时算出来的 42",
            );
            eq(
                hits,
                1usize,
                "正常用例：两次访问里第二次命中 → 命中次数是 1（第一次未命中不计命中）",
            );
            // 边界/防呆：命中次数必须与「真的命中」一致
            is_false(
                hits == 2,
                "边界：命中次数必须与「真的命中」严格一致——把第一次未命中也算进去会得到 2",
            );
        },
    );
}

/// 【待实现】用 `Rc<RefCell<HashMap<String, i32>>>` 写一个带命中统计的缓存。
///
/// 实现要求（期望值写死，按这个来）：
///   - 在函数体内 `use std::collections::HashMap;`（本文件顶部没有导入它）；
///   - `let cache: Rc<RefCell<HashMap<String, i32>>> = Rc::new(RefCell::new(HashMap::new()));`
///     —— 缓存键用 `String::from("answer")`；
///   - 第一次访问：键不存在（未命中）→ 把 `6 * 7` 算出来写进缓存，命中次数保持 **0**；
///   - 第二次访问：键已存在（命中）→ 命中次数 +1，直接取缓存里的值（不要重算）；
///   - 返回 `(缓存里的值, 命中次数)`，即 `(42, 1)`；
///   - 借用要点是本题的关键：`let hit = cache.borrow().get(&key).copied();` 是一个**语句**，
///     借用在这个语句结束时就归还了；不要写成
///     `match cache.borrow().get(&key) { None => { cache.borrow_mut().insert(..); } ... }`，
///     因为 match 的临时借用会活到整个 match 结束，`borrow_mut()` 会在运行期 panic；
///   - 命中次数可以放在另一个 `Rc<RefCell<usize>>` 里；顶部已经导入了 `Rc` / `RefCell`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (42, 1)
/// ```
fn exercise_16_08_shared_cache() -> (i32, usize) {
    assessment_harness::todo_exercise(
        "exercise_16_08_shared_cache",
        "Rc<RefCell<HashMap<String, i32>>> 缓存：第一次未命中算 6 * 7 = 42 写入（命中 0），\
         第二次命中（命中 1），返回 (42, 1)",
        (),
    )
}

// ===========================================================================
// kp_16_09 常见错误诊断：E0596 / E0277 / E0072
// ===========================================================================

/// 知识点考核：写出三个智能指针场景对应的编译器错误编号。
#[test]
fn kp_16_09_diagnose_errors() {
    assess(
        M,
        "kp_16_09",
        "常见错误诊断：改 Rc 内部值 / Rc 跨线程 / 递归类型没加间接层，分别报什么错",
        Kind::Hard,
        "复习 lesson_16 示例 9（common_mistakes）与示例 1：示例 9 里**带编译错误编号**的三个坑\
         依次是「想直接改 `Rc` 里的值」（E0596）、「把 `Rc` 送进线程」（E0277）、\
         「递归类型忘记加 `Box`」（E0072）——示例 9 末尾那行 `println!` 直接把这三个编号连在一起打印；\
         另外两个坑（RefCell 借用重叠、循环引用泄漏）都是运行期问题，没有编译错误编号。",
        || {
            eq_slice(
                &exercise_16_09_diagnose_errors(),
                &["E0596", "E0277", "E0072"],
                "顺序必须是：改 `Rc` 内部值（E0596）、`Rc` 跨线程（E0277）、\
                 递归类型无限大小（E0072）",
            );
        },
    );
}

/// 【待实现】写出三个场景对应的编译器错误编号。
///
/// 场景（与课程示例 9 的注释一一对应，按它们在注释里出现的顺序）：
///   1. `let a = Rc::new(5); *a += 1;` —— 想直接修改 `Rc` 里的值；
///   2. `let data = Rc::new(5); thread::spawn(move || println!("{data}"));` —— 把 `Rc` 送进线程；
///   3. `enum Tree { Leaf, Node(Tree, Tree) }` —— 递归类型忘记加间接层（`Box`）。
/// 注意：示例 9 里的错误 2（RefCell 借用重叠）与错误 3（循环引用泄漏）都是**运行期**问题，
/// 没有编译错误编号，所以跳过它们。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0596"`），顺序与上面一致；
///   - 这三个编号在课程示例 9 的注释里都能找到，示例 1 里也重复讲了 E0072。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0596", "E0277", "E0072"]
/// ```
fn exercise_16_09_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_16_09_diagnose_errors",
        "返回 [\"E0596\", \"E0277\", \"E0072\"]（改 Rc 内部值 / Rc 跨线程 / 递归类型无限大小）",
        (),
    )
}
