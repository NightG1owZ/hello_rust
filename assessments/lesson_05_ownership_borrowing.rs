//! assessments/lesson_05_ownership_borrowing.rs —— 考核：所有权系统（对应 lesson_05）
//!
//! - 对应课程：`src/tutorial/lesson_05_ownership_borrowing.rs`
//! - 知识点出处：`src/tutorial/README.md` 第二阶段「05 所有权系统」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_05_ownership_borrowing     # 只考这一课
//!   cargo test                                          # 考全部 18 课
//!   cargo run --bin assessment_report                   # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_05_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_05_ownership_borrowing`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 10）
//!
//! | 场景 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `let t = s;` 之后再用 `s` | `error[E0382]` borrow of moved value | 确实要两份数据就显式 `s.clone()`；只想读就借出去 `&s` |
//! | 同一时刻两个 `&mut` | `error[E0499]` cannot borrow as mutable more than once | 让两次可变借用**串行**（用完一个再用下一个，见示例 10 错误 2） |
//! | 不可变借用还没用完就 `push` | `error[E0502]` cannot borrow as mutable because it is also borrowed as immutable | 先把借用来的数据取出来（Copy 类型直接取值），再改原集合 |
//! | 返回局部变量的引用 | `error[E0515]` cannot return reference to local variable（先用 `&String` 会先撞上 `error[E0106]` missing lifetime specifier） | 返回所有权（`String`），或返回由**参数**派生出的切片 |
//! | 通过 `&T` 修改数据 | `error[E0596]` cannot borrow as mutable, as it is behind a `&` reference | 绑定加 `mut`，参数类型改成 `&mut T` |

use assessment_harness::{Kind, assess, eq, eq_slice};
use std::cell::RefCell;

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_05";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// ===========================================================================

thread_local! {
    /// drop 日志：每个 Tracer 被 drop 时把自己的名字压进来
    static DROP_LOG: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
}

/// 探针：被 drop 时把名字记入日志（Drop 实现由考核框架提供，你只需要安排作用域）
struct Tracer(&'static str);

impl Tracer {
    fn new(name: &'static str) -> Self {
        Tracer(name)
    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        DROP_LOG.with(|log| log.borrow_mut().push(self.0));
    }
}

/// 取出并清空 drop 日志
fn take_drop_log() -> Vec<&'static str> {
    DROP_LOG.with(|log| log.borrow_mut().drain(..).collect())
}

// ===========================================================================
// kp_05_01 move 语义：所有权移交之后，原变量立刻失效
// ===========================================================================

/// 知识点考核：把 `String` move 进函数再交还，验证所有权「进出」的路径。
#[test]
fn kp_05_01_move_semantics() {
    assess(
        M,
        "kp_05_01",
        "move 语义：String 传参即移交所有权，函数通过返回值把所有权交还",
        Kind::Core,
        "复习 lesson_05 示例 1（three_rules）与示例 2（move_vs_copy）：`String` 没有实现 Copy，\
         所以 `let t = s;` 或把 `s` 传进函数都是 move；move 之后原变量**不可再使用**，\
         否则报 `error[E0382]: borrow of moved value`。本题的练习函数按值接收 String、\
         再把同一个 String 装进元组返回，所有权于是回到调用方。",
        || {
            // 正常用例：借用检查器的要求是——move 进去的值必须被还回来
            let (returned, len) = exercise_05_01_move_semantics(String::from("rust"));
            // move 之后调用方的那个变量已经失效（E0382），
            // 所以这里只能用函数返回的新绑定 `returned`，不能再用原来的变量：
            //     let text = String::from("rust");
            //     let (returned, len) = exercise_05_01_move_semantics(text);
            //     println!("{text}"); // error[E0382]: borrow of moved value: `text`
            eq(
                returned,
                String::from("rust"),
                "函数应当把同一个 String 交还回来，内容仍是 \"rust\"",
            );
            eq(len, 4usize, "\"rust\" 的字节长度是 4（字节数，不是字符数）");

            // 正常用例：带空格的字符串，长度按字节算
            let (returned, len) = exercise_05_01_move_semantics(String::from("hello world"));
            eq(
                returned,
                String::from("hello world"),
                "内容应与传入时完全一致，没有被 trim 或改写",
            );
            eq(len, 11usize, "\"hello world\" 含 1 个空格共 11 字节");

            // 边界：空 String
            let (returned, len) = exercise_05_01_move_semantics(String::new());
            eq(returned, String::new(), "空字符串也要原样交还，不能 panic");
            eq(len, 0usize, "空字符串的字节长度是 0");

            // 边界：多字节 UTF-8（String::len() 返回字节数，不是字符数）
            let (returned, len) = exercise_05_01_move_semantics(String::from("所有权"));
            eq(
                returned,
                String::from("所有权"),
                "中文 String 应原样交还（move 与内容编码无关）",
            );
            eq(
                len,
                9usize,
                "「所有权」是 3 个汉字、每个占 3 字节，共 9 字节——len() 数的是字节",
            );
        },
    );
}

/// 【待实现】把 String move 进函数、量好长度再交还。
///
/// 实现要求：
///   - 参数 `text: String` 是按值接收，也就是**取得所有权**（一次 move）；
///   - 计算它的字节长度（`text.len()`，返回 `usize`）；
///   - 返回 `(text, len)`：把同一个 String 连同长度一起交回调用方——函数拿走所有权，
///     再用返回值把所有权还给调用方；如果只返回长度而不返回 String，
///     调用方传进来的那个 String 就在函数结束时被 drop 了，这正是所有权设计的本意。
///
/// 示例输入：
/// ```text
/// text = "rust"
/// ```
/// 示例输出：
/// ```text
/// ("rust", 4)
/// ```
fn exercise_05_01_move_semantics(text: String) -> (String, usize) {
    assessment_harness::todo_exercise(
        "exercise_05_01_move_semantics",
        "按值接收 String，返回 (同一个 String, 它的字节长度)",
        (text,),
    )
}

// ===========================================================================
// kp_05_02 Copy 与 Clone：i32 可以随便用，String 必须显式 clone
// ===========================================================================

/// 知识点考核：同一个函数里同时体会「按位复制」与「显式克隆」。
#[test]
fn kp_05_02_copy_vs_clone() {
    assess(
        M,
        "kp_05_02",
        "Copy 与 Clone：i32 赋值是复制、String 必须显式 .clone()",
        Kind::Core,
        "复习 lesson_05 示例 2（move_vs_copy）与示例 3（clone_explicit）：\
         `i32` 实现了 `Copy`，赋值/传参只是复制 4 个字节，原变量照样能用，\
         所以 `n` 可以在同一个表达式里出现两次；`String` 没有 Copy，\
         想要「一份留着、一份交出去」只能写 `.clone()`，它在堆上真分配、真拷贝。",
        || {
            // 正常用例：n 被用两次（乘法）+ text 被 clone 一次
            let (doubled, cloned, original_len) =
                exercise_05_02_copy_vs_clone(21, String::from("rust"));
            eq(
                doubled,
                42,
                "i32 是 Copy 类型，n 可以在 `2 * n` 里被读多次，结果为 42",
            );
            eq(
                cloned,
                String::from("rust"),
                "clone 出来的字符串内容应与原串一致",
            );
            eq(
                original_len,
                4usize,
                "原串是 \"rust\"，字节长度为 4（clone 不会改变原串）",
            );

            // 正常用例：另一个普通输入
            let (doubled, cloned, original_len) =
                exercise_05_02_copy_vs_clone(0, String::from("hello world"));
            eq(doubled, 0, "0 乘 2 还是 0，Copy 语义下不涉及所有权问题");
            eq(
                cloned,
                String::from("hello world"),
                "克隆出来的副本内容应完全一致",
            );
            eq(original_len, 11usize, "\"hello world\" 含空格共 11 字节");

            // 边界：取一个「两倍之后仍在 i32 范围内」的极大值。
            // 这里刻意不用 i32::MAX / i32::MIN：它们的两倍必然溢出 i32
            // （`i32::MAX * 2` 甚至直接是编译期 arithmetic_overflow 错误），
            // 而溢出行为是 lesson_02 的考点，不是本课要考的内容。
            let (doubled, cloned, original_len) =
                exercise_05_02_copy_vs_clone(1_073_741_823, String::new());
            eq(
                doubled,
                2_147_483_646,
                "1_073_741_823 被复制一份后参与运算得到 2_147_483_646\
                 （= i32::MAX - 1），说明 Copy 值可以安全地被使用多次而不会 move",
            );
            eq(cloned, String::new(), "空串克隆后仍是空串");
            eq(original_len, 0usize, "空串的字节长度是 0");

            // 边界：多字节字符串的克隆
            let (doubled, cloned, original_len) =
                exercise_05_02_copy_vs_clone(3, String::from("所有权"));
            eq(doubled, 6, "3 的两倍是 6");
            eq(
                cloned,
                String::from("所有权"),
                "中文串克隆后内容一致（clone 按字节复制，不会破坏 UTF-8）",
            );
            eq(
                original_len,
                9usize,
                "「所有权」占 9 字节，原串长度不受克隆影响",
            );
        },
    );
}

/// 【待实现】同时演示「Copy 类型随便用」与「非 Copy 类型要 clone」。
///
/// 实现要求：
///   - `n: i32` 是 Copy 类型：直接算 `let doubled = n * 2;`，
///     也可以在同一条表达式里把 `n` 用两次（`n + n`），这完全合法；
///   - `text: String` 不是 Copy：先写 `let cloned = text.clone();`（显式深拷贝），
///     再写 `let original_len = text.len();` 继续使用原串；
///   - 顺序很关键：先 clone、后取原串长度，两者互不影响；
///   - 如果省掉 `.clone()` 直接写 `let cloned = text;`，那是一次 move，
///     后面的 `text.len()` 会报 `error[E0382]: borrow of moved value: text`——
///     这正是「Copy 是特例、move 才是默认」的直观体现；
///   - 返回 `(doubled, cloned, original_len)`。
///
/// 示例输入：
/// ```text
/// n = 21
/// text = "rust"
/// ```
/// 示例输出：
/// ```text
/// (42, "rust", 4)
/// ```
fn exercise_05_02_copy_vs_clone(n: i32, text: String) -> (i32, String, usize) {
    assessment_harness::todo_exercise(
        "exercise_05_02_copy_vs_clone",
        "返回 (2 * n, text.clone() 得到的字符串, 原 text 的字节长度)",
        (n, text),
    )
}

// ===========================================================================
// kp_05_03 共享借用：同一时刻允许多个不可变引用
// ===========================================================================

/// 知识点考核：在同一作用域里同时持有两个不可变借用。
#[test]
fn kp_05_03_shared_borrows() {
    assess(
        M,
        "kp_05_03",
        "共享借用：同一时刻可以存在任意多个 &T（只读共享是安全的）",
        Kind::Core,
        "复习 lesson_05 示例 5（borrow_rules）：`let a: &String = &text; let b: &String = &text;` \
         两个不可变借用同时存在完全合法，因为只读、不会有人看到「写了一半」的数据；\
         反过来说，只要出现一个 `&mut`，就要求「没有其它任何借用同时活着」。",
        || {
            // 正常用例：两个借用都指向同一段数据，长度相同
            eq(
                exercise_05_03_shared_borrows("borrow"),
                (6usize, 6usize, 12usize),
                "\"borrow\" 长 6：两个不可变借用各读到 6，和为 12",
            );
            // 正常用例：带空格的句子
            eq(
                exercise_05_03_shared_borrows("rust borrow checker"),
                (19usize, 19usize, 38usize),
                "\"rust borrow checker\" 长 19：两个借用的读数必须一致，和为 38",
            );
            // 边界：空串（借用一个空切片也是合法的）
            eq(
                exercise_05_03_shared_borrows(""),
                (0usize, 0usize, 0usize),
                "空串长度为 0，两个借用各自读到 0，和为 0",
            );
            // 边界：多字节 UTF-8
            eq(
                exercise_05_03_shared_borrows("你好"),
                (6usize, 6usize, 12usize),
                "「你好」是 2 个汉字共 6 字节（len() 数字节），和为 12",
            );
        },
    );
}

/// 【待实现】同时持有两个不可变借用并测量长度。
///
/// 实现要求：
///   - `text` 已经是 `&str`（本身就是借用，不需要再 `&`）；
///   - 在同一作用域里创建**两个**不可变借用，例如
///     `let first: &str = text;` 与 `let second: &str = text;`；
///   - 两个 `&str` 同时存在是合法的（共享借用），这一点与 `&mut` 完全不同：
///     同一时刻只能有一个 `&mut`，且有 `&mut` 时不能有任何 `&`；
///     两次借用之间**不要**插入 `text.clone()` 之类的多余操作——
///     本题就是要你亲手验证「多个不可变借用共存」这件事；
///   - 分别取 `len()`，返回 `(len1, len2, len1 + len2)`。
///
/// 示例输入：
/// ```text
/// text = "borrow"
/// ```
/// 示例输出：
/// ```text
/// (6, 6, 12)
/// ```
fn exercise_05_03_shared_borrows(text: &str) -> (usize, usize, usize) {
    assessment_harness::todo_exercise(
        "exercise_05_03_shared_borrows",
        "在同一作用域持有两个不可变借用，返回 (len1, len2, len1 + len2)",
        (text,),
    )
}

// ===========================================================================
// kp_05_04 可变借用：串行的两次 &mut 是合法的（NLL）
// ===========================================================================

/// 知识点考核：用两次**不重叠**的可变借用往 Vec 里 push。
#[test]
fn kp_05_04_mutable_borrow() {
    assess(
        M,
        "kp_05_04",
        "可变借用：两次 &mut 只要不重叠就合法（每次借用用完即结束）",
        Kind::Hard,
        "复习 lesson_05 示例 6（mutable_borrow）与示例 7（nll）：`v.push(1); v.push(2);` \
         看起来是两次可变借用，但第一次借用在 `push` 返回时就结束了，\
         所以第二次借用不冲突——这就是 NLL（非词法生命周期）；\
         反过来，`let r1 = &mut v; let r2 = &mut v;` 两个引用同时活着就报 `error[E0499]`。",
        || {
            // 正常用例：从非空向量开始，最终是 [1, 2, 3, 4] → 10
            let mut v = vec![3, 4];
            eq(
                exercise_05_04_mutable_borrow(&mut v),
                10,
                "原有 3 + 4，push 进来的 1 + 2，总和是 10",
            );
            // 复用同一个向量再调一次：每次调用都应再 push 两个元素
            eq(
                exercise_05_04_mutable_borrow(&mut v),
                13,
                "再 push 两个元素后向量是 [3, 4, 1, 2, 1, 2]，和为 13",
            );

            // 边界：初始为空时，结果就是被 push 进来的两个元素之和
            let mut empty = Vec::new();
            eq(
                exercise_05_04_mutable_borrow(&mut empty),
                3,
                "空向量 + push(1) + push(2) → 只有 1 + 2 = 3",
            );
            eq(
                empty.len(),
                2usize,
                "函数必须真的往向量里追加了 2 个元素（长度从 0 变 2）",
            );

            // 边界：负数元素同样参与求和
            let mut negatives = vec![-10];
            eq(
                exercise_05_04_mutable_borrow(&mut negatives),
                -7,
                "-10 + 1 + 2 = -7，说明求和是对全部元素做的",
            );
        },
    );
}

/// 【待实现】用可变引用往 Vec 里追加两个元素并求和。
///
/// 实现要求：
///   - 通过 `v.push(1);` 与 `v.push(2);` 追加两个元素（`v: &mut Vec<i32>`，
///     可以直接调用 `push`，方法调用会自动重借用）；
///   - 不要写 `let r1 = &mut *v; let r2 = &mut *v;` 这种「两个可变引用同时活着」的写法，
///     会报 `error[E0499]: cannot borrow as mutable more than once at a time`；
///     顺序调用 `v.push(…)` 之所以合法，是因为每次借用在语句结束时就没了（NLL）；
///   - 返回向量中**所有**元素之和（`v.iter().sum()`，返回类型是 `i32`，
///     `sum()` 需要能推断出来）。
///
/// 示例输入：
/// ```text
/// v = [3, 4]
/// ```
/// 示例输出：
/// ```text
/// 10
/// ```
fn exercise_05_04_mutable_borrow(v: &mut Vec<i32>) -> i32 {
    assessment_harness::todo_exercise(
        "exercise_05_04_mutable_borrow",
        "往 v 里 push 1 和 2，返回全部元素之和",
        (v,),
    )
}

// ===========================================================================
// kp_05_05 返回切片：&str 借用入参，而不是复制出 String
// ===========================================================================

/// 知识点考核：实现课程示例 9 的经典函数 `longest_word`。
#[test]
fn kp_05_05_longest_word() {
    assess(
        M,
        "kp_05_05",
        "返回切片：最长单词是原句的一部分，返回 &str 而不是 String",
        Kind::Core,
        "复习 lesson_05 示例 9（typical_scenario_longest_word）：返回值写成 `&str` 并在\
         省略生命周期的情况下由编译器绑定到唯一的引用参数 `text` 上；\
         实现上用 `longest.len()` 做「严格大于」比较，长度相同时保留**先出现**的那个单词，\
         而 `longest` 的初值必须是 `\"\"`，这样空串才能安全返回空切片。",
        || {
            // 正常用例：课程示例 9 的句子
            eq(
                exercise_05_05_longest_word("Rust ownership makes memory safety possible"),
                "ownership",
                "\"ownership\"（9 字节）最长，应返回这个切片",
            );
            // 正常用例：普通句子
            eq(
                exercise_05_05_longest_word("keep it simple and readable"),
                "readable",
                "\"readable\"（8 字节）比 \"simple\" 长，应返回它",
            );

            // 边界：空串不能 panic，必须返回空切片
            eq(
                exercise_05_05_longest_word(""),
                "",
                "空串没有任何单词，应返回 \"\" 而不是 panic（初值 \"\" 是关键）",
            );
            // 边界：纯空格串（split_whitespace 不产生任何单词）
            eq(
                exercise_05_05_longest_word("   \t\n  "),
                "",
                "只有空白时 split_whitespace() 一个单词都不产出，应返回 \"\"",
            );
            // 边界：前后多余空白不影响结果
            eq(
                exercise_05_05_longest_word("   rust   ownership  "),
                "ownership",
                "首尾与中间的空白都只是分隔符，最长单词仍是 \"ownership\"",
            );
            // 边界：多个同长单词取第一个
            eq(
                exercise_05_05_longest_word("alpha bravo charlie"),
                "charlie",
                "\"charlie\" 长 7，比 \"alpha\"、\"bravo\" 都长",
            );
            eq(
                exercise_05_05_longest_word("abcd wxyz"),
                "abcd",
                "两个单词都是 4 字节，必须返回**先出现**的 \"abcd\"（比较用严格大于）",
            );
            // 边界：只有一个单词
            eq(
                exercise_05_05_longest_word("solo"),
                "solo",
                "只有一个单词时它就是最长的",
            );
        },
    );
}

/// 【待实现】返回句子中最长的那个单词（切片）。
///
/// 实现要求：
///   - 用 `let mut longest = "";` 做**切片**初值（类型是 `&str`，生命周期与入参绑定）；
///   - 用 `for word in text.split_whitespace() { … }` 遍历单词；
///   - 只有 `word.len() > longest.len()` 时才替换（严格大于 → 同长取第一个）；
///   - 返回类型是 `&str` 而不是 `String`——最长单词本来就是入参的一部分，
///     返回切片既不复制也不取得所有权（这是示例 9 的核心）；
///   - 初值写 `""` 而不是 `text`，空串与纯空格串才能安全返回空切片、不 panic；
///     千万别写 `&text[0..]` 之类的下标切片，遇到多字节字符会切在字符中间而 panic；
///   - 返回 `longest`。
///
/// 示例输入：
/// ```text
/// text = "Rust ownership makes memory safety possible"
/// ```
/// 示例输出：
/// ```text
/// "ownership"
/// ```
fn exercise_05_05_longest_word(text: &str) -> &str {
    assessment_harness::todo_exercise(
        "exercise_05_05_longest_word",
        "用 split_whitespace + 严格大于比较返回最长单词的切片；空串与纯空格返回 \"\"",
        (text,),
    )
}

// ===========================================================================
// kp_05_06 Vec 的 len 与 capacity：已有元素数 vs 已分配容量
// ===========================================================================

/// 知识点考核：观察 `with_capacity` 与 `push` 前后 `len` / `capacity` 的变化。
#[test]
fn kp_05_06_vec_len_capacity() {
    assess(
        M,
        "kp_05_06",
        "len 与 capacity：len 是已有元素个数，capacity 是已分配的坑位数",
        Kind::Basic,
        "复习 lesson_05 示例 4（stack_and_heap）：`Vec::with_capacity(10)` 只预留坑位、\
         不放元素，所以 `len() == 0` 而 `capacity() == 10`；`push(1)` 放了一个元素，\
         `len()` 变成 1，而预留的容量足够，`capacity()` 仍是 10（不会被无故放大）。",
        || {
            eq(
                exercise_05_06_vec_len_capacity(),
                (0usize, 10usize, 1usize, 10usize),
                "with_capacity(10) 之后应为 (len 0, capacity 10)；push(1) 之后应为 (len 1, capacity 10)",
            );
        },
    );
}

/// 【待实现】观察 `Vec` 的 `len` 与 `capacity`。
///
/// 实现要求：
///   - 创建 `let mut v: Vec<i32> = Vec::with_capacity(10);`；
///   - 先取 `v.len()` 与 `v.capacity()` 存下来（此时是 0 和 10）；
///   - 再 `v.push(1);`，重新取一次 `len()` 与 `capacity()`（此时是 1 和 10）；
///   - `with_capacity` 只是**预留**内存，不创建元素，所以 len 是 0；
///     因为容量足够，`push` 不会触发扩容，capacity 仍然是 10——
///     如果写 `let mut v = Vec::new();` 再 push，capacity 通常是 4，结果就对不上了；
///   - 返回 `(len_before, capacity_before, len_after, capacity_after)`。
///
/// 示例输入：
/// ```text
/// v = Vec::with_capacity(10)
/// ```
/// 示例输出：
/// ```text
/// (0, 10, 1, 10)
/// ```
fn exercise_05_06_vec_len_capacity() -> (usize, usize, usize, usize) {
    assessment_harness::todo_exercise(
        "exercise_05_06_vec_len_capacity",
        "返回 (with_capacity(10) 的 len, 其 capacity, push(1) 后的 len, 其 capacity) = (0, 10, 1, 10)",
        (),
    )
}

// ===========================================================================
// kp_05_07 NLL 边界：不可变借用一结束，就能立刻可变借用
// ===========================================================================

/// 知识点考核：先只读取出最后一个元素，再可变借用 push。
#[test]
fn kp_05_07_nll_boundary() {
    assess(
        M,
        "kp_05_07",
        "NLL 边界：先把值取出来（借用结束），再可变借用修改",
        Kind::Edge,
        "复习 lesson_05 示例 7（nll_non_lexical_lifetime）：借用的有效范围由**最后一次使用**决定；\
         本题中 `last` 是一个 `i32`（Copy 类型），`*v.last().unwrap()` 一取值，\
         不可变借用就结束了，所以后面 `v.push(99)` 完全不冲突；\
         如果写成 `let last = v.last().unwrap();`（保留 `&i32`）再 push，就会报 `error[E0502]`。",
        || {
            // 正常用例
            let mut v = vec![1, 2, 3];
            eq(
                exercise_05_07_nll_boundary(&mut v),
                (3, 4usize),
                "取到最后一个元素 3，push(99) 之后长度变为 4",
            );
            eq(
                &v,
                &vec![1, 2, 3, 99],
                "push 必须真的执行：99 应出现在向量末尾\
                 （这里写成 `&v` 只是为了比较后还能继续使用 v，否则 eq(v, …) 会把 v move 走）",
            );

            // 复用同一个向量再调一次：&mut 只是借用，v 的所有权始终在测试函数手里
            eq(
                exercise_05_07_nll_boundary(&mut v),
                (99, 5usize),
                "第二次调用取到的是上一次 push 进去的 99，长度变为 5",
            );
            eq(
                &v,
                &vec![1, 2, 3, 99, 99],
                "两次调用后向量应是 [1, 2, 3, 99, 99]（&mut 借用不会夺走所有权）",
            );

            // 边界：只有一个元素
            let mut single = vec![7];
            eq(
                exercise_05_07_nll_boundary(&mut single),
                (7, 2usize),
                "单元素向量：取到 7，push 之后长度是 2",
            );
            eq(single, vec![7, 99], "单元素向量也要被正确追加 99");

            // 边界：包含负数与重复值
            let mut mixed = vec![-1, -1, 0];
            eq(
                exercise_05_07_nll_boundary(&mut mixed),
                (0, 4usize),
                "最后一个元素是 0（不是负数也不是 -1），长度变为 4",
            );
            eq(
                mixed.len(),
                4usize,
                "负数元素不影响 push：[-1, -1, 0] 追加 99 后长度为 4",
            );
        },
    );
}

/// 【待实现】先用不可变借用取值，再用可变借用追加元素（NLL 演示）。
///
/// 实现要求：
///   - 先用不可变借用取最后一个元素，并且**立刻把它变成自有的 `i32`**：
///     `let last = *v.last().unwrap();`（解引用后借用就结束了）；
///   - 关键差别在「`let last = *v.last().unwrap();`（拿到值，借用结束）」与
///     「`let last = v.last().unwrap();`（拿到 `&i32`，借用还活着，push 会报 E0502）」；
///     本文件约定入参保证非空，所以 `unwrap()` 不会失败，但你要理解它为什么会成功；
///   - 再 `v.push(99);`（此时没有任何未结束的借用，可变借用合法）；
///   - 返回 `(last, v.len())`。
///
/// 示例输入：
/// ```text
/// v = [1, 2, 3]
/// ```
/// 示例输出：
/// ```text
/// (3, 4)
/// ```
fn exercise_05_07_nll_boundary(v: &mut Vec<i32>) -> (i32, usize) {
    assessment_harness::todo_exercise(
        "exercise_05_07_nll_boundary",
        "先解引用取出最后一个元素（借用结束），再 push(99)，返回 (取到的值, 新长度)",
        (v,),
    )
}

// ===========================================================================
// kp_05_08 常见错误诊断：E0382 / E0499 / E0502
// ===========================================================================

/// 知识点考核：写出课程示例 10 前三个错误场景对应的编译器错误编号。
#[test]
fn kp_05_08_diagnose_errors() {
    assess(
        M,
        "kp_05_08",
        "常见错误诊断：E0382（move 后使用）/ E0499（两个可变借用）/ E0502（不可变与可变重叠）",
        Kind::Hard,
        "复习 lesson_05 示例 10（common_mistakes）：错误 1 是「move 之后继续用原变量」（E0382），\
         错误 2 是「同一时刻两个 `&mut`」（E0499），错误 3 是「不可变借用与可变借用重叠」（E0502）；\
         这三个编号覆盖了初学者 90% 的所有权编译失败。",
        || {
            eq_slice(
                &exercise_05_08_diagnose_errors(),
                &["E0382", "E0499", "E0502"],
                "顺序必须是：move 后使用（E0382）、两个可变借用（E0499）、不可变与可变重叠（E0502）",
            );
        },
    );
}

/// 【待实现】写出三个所有权场景对应的编译器错误编号。
///
/// 场景（与课程示例 10 的错误 1 / 2 / 3 一致，按课程注释顺序）：
///   1. `let s = String::from("x"); let t = s; println!("{s}");`
///      —— move 之后继续使用原变量；
///   2. `let r1 = &mut v; let r2 = &mut v;` —— 同一时刻两个可变借用；
///   3. `let first = &w[0]; w.push(4);` —— 不可变借用还没结束就可变借用。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0382"`），顺序与上面一致；
///   - 这三个编号在课程示例 10 的注释里都能找到；
///     记忆口诀：0382 = 值被移走了，0499 = 可变借太多了，0502 = 又读又写撞车了。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0382", "E0499", "E0502"]
/// ```
fn exercise_05_08_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_05_08_diagnose_errors",
        "返回 [\"E0382\", \"E0499\", \"E0502\"]（move 后使用 / 两个可变借用 / 不可变与可变重叠）",
        (),
    )
}

// ===========================================================================
// kp_05_09 Drop 顺序：值离开作用域时按「逆序」被释放
// ===========================================================================

/// 知识点考核：用 `Tracer` 验证变量的 drop 顺序。
#[test]
fn kp_05_09_drop_order() {
    assess(
        M,
        "kp_05_09",
        "Drop 顺序：作用域结束时后声明的先 drop，内层作用域先于外层",
        Kind::Hard,
        "复习 lesson_05 示例 1（three_rules）的规则三与示例 4（stack_and_heap）：\
         值离开作用域时自动 drop；同一个作用域内**后声明的先 drop**（逆序），\
         内层块整体先于外层块结束。本题期望的顺序是 [\"b\", \"c\", \"a\"]。",
        || {
            eq_slice(
                &exercise_05_09_drop_order(),
                &["b", "c", "a"],
                "期望 drop 顺序为 [\"b\", \"c\", \"a\"]：内层块里的 b 先走，\
                 然后同层的 c（后声明先 drop）先于 a",
            );
        },
    );
}

/// 【待实现】用 `Tracer` 安排作用域，得到 `["b", "c", "a"]` 的 drop 顺序。
///
/// 实现要求：
///   - 用已提供的 `Tracer::new("名字")` 创建探针，并把它绑定到以 `_` 开头的变量
///     （例如 `let _a = Tracer::new("a");`，下划线前缀才不会触发 unused_variables 警告）；
///   - 顺序：先在外层创建 `_a`；再开一个**内层块**创建 `_b`（块结束时 `_b` 先 drop）；
///     内层块之后再创建 `_c`；最后让外层作用域结束；
///   - drop 的顺序是「后声明先 drop」（栈式逆序），所以外层里 `_c` 比 `_a` 先 drop；
///     内层块里的 `_b` 最早 drop——合起来正好是 `["b", "c", "a"]`；
///     必须**真正**创建这几个 Tracer 并使用 `take_drop_log()`，
///     否则提供的类型会因从未被使用而报 dead_code 警告；
///   - 作用域全部结束后返回 `take_drop_log()`。
///
/// 示例输入：
/// ```text
/// 探针 Tracer::new("a")（外层）；Tracer::new("b")（内层块）；之后 Tracer::new("c")（外层）
/// ```
/// 示例输出：
/// ```text
/// ["b", "c", "a"]
/// ```
fn exercise_05_09_drop_order() -> Vec<&'static str> {
    assessment_harness::todo_exercise(
        "exercise_05_09_drop_order",
        "用 Tracer 安排作用域（外层 a；内层块 b；之后 c），返回 take_drop_log()，期望 [\"b\", \"c\", \"a\"]",
        // 传入一个「引用到上面提供的类型与函数」的零参数闭包：这样骨架态下
        // `Tracer` / `Tracer::new` / `take_drop_log` 都算「被使用过」，
        // 不会出现 dead_code 警告；学员实现时直接删除下面这一行即可。
        || {
            let _ = (Tracer::new, take_drop_log);
        },
    )
}
