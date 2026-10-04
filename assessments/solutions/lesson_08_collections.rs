//! assessments/lesson_08_collections.rs —— 考核：常见集合（对应 lesson_08）
//!
//! - 对应课程：`src/tutorial/lesson_08_collections.rs`
//! - 知识点出处：`src/tutorial/README.md` 第三阶段「08 常见集合」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_08_collections         # 只考这一课
//!   cargo test                                      # 考全部 18 课
//!   cargo run --bin assessment_report               # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_08_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_08_collections`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」；
//! - 本课所有需要返回集合的地方都**必须先排序**再返回：`HashMap` 的遍历顺序不固定，
//!   不排序就无法写出确定的期望值（这是集合这一课最容易踩的坑）。
//!
//! # 本课常见错误速查（对应课程示例 10）
//!
//! | 代码 | 报错 / 现象 | 修正方法 |
//! | --- | --- | --- |
//! | `let v = Vec::new(); v.push("hi");` | `error[E0596]` cannot borrow as mutable | 补上 `mut`：`let mut v = Vec::new();` |
//! | `let v = Vec::new(); println!("{}", v.len());` | `error[E0282]` type annotations needed | 显式写 `Vec<i32>`，或先用 `push` 给出元素类型 |
//! | `let x = v[3];`（长度 3） | 编译通过，运行期 `index out of bounds` panic | 用 `get` 得到 `Option`，或先判断下标范围 |
//! | `let ch = s[0];`（`s` 是 `String`） | `error[E0277]` `the type str cannot be indexed by {integer}` | `s.chars().nth(0)`，或按 `char_indices` 的边界切片 |
//! | `let part = &s[0..1];`（`s = "你好"`） | 编译通过，运行期 `not a char boundary` panic | 用 `char_indices()` 找到合法的字节边界（0 与 3） |
//! | `let text: &str = String::from("abc");` | `error[E0308]` mismatched types | 借用：`&String::from("abc")` 或 `.as_str()` |
//! | `let c = a + b;`（`a`、`b` 都是 `String`） | `error[E0308]` expected `&str`, found `String` | 写 `a + &b`，或改用 `format!` |
//! | `m.get("k") += 1;` | `error[E0368]` / `error[E0067]` | 用 `entry("k").or_insert(0)` 拿到 `&mut`，或先 `copied()` |
//!
//! # 集合的正确性要点
//!
//! - `Vec`：`push`/`pop`/`insert`/`remove`/`retain`/`iter_mut` 的所有权与复杂度；
//! - `String`：`len()` 是**字节数**，`chars().count()` 才是**字符数**（UTF-8 变长编码）；
//! - `HashMap`：不保证遍历顺序，需要确定输出时必须排序；`entry` 一次查找完成「查 + 改」。

use std::collections::HashMap;

use assessment_harness::{Kind, assess, eq, eq_slice};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_08";

// ===========================================================================
// kp_08_01 Vec 的创建与 push：由切片构造，再追加一个元素
// ===========================================================================

/// 知识点考核：由切片构造 `Vec` 并 `push` 追加元素。
#[test]
fn kp_08_01_vec_create_and_push() {
    assess(
        M,
        "kp_08_01",
        "Vec 的创建与 push：`to_vec()` 复制出拥有所有权的向量，`push` 在尾部追加",
        Kind::Basic,
        "复习 lesson_08 示例 1（vec_create_and_push）：`Vec::new` / `vec![]` / `Vec::with_capacity` \
         三种创建方式，以及 `push` 把元素追加到末尾（必要时扩容）。",
        || {
            // 正常用例：切片 [1, 2, 3] 追加 4
            eq(
                exercise_08_01_vec_create_and_push(&[1, 2, 3], 4),
                vec![1, 2, 3, 4],
                "由切片复制出 [1, 2, 3] 后 push(4)，应得到 [1, 2, 3, 4]",
            );
            // 正常用例：负数元素照常追加，顺序不变
            eq(
                exercise_08_01_vec_create_and_push(&[-1, 0, 1], 2),
                vec![-1, 0, 1, 2],
                "元素是负数也不影响 push：应得到 [-1, 0, 1, 2]",
            );
            // 边界：空切片只能得到「只有 extra」的向量
            eq(
                exercise_08_01_vec_create_and_push(&[], 7),
                vec![7],
                "空切片时结果只有 extra 一个元素，即 [7]（不要 panic）",
            );
            // 边界：单元素 + i32 极值
            eq(
                exercise_08_01_vec_create_and_push(&[i32::MIN], i32::MAX),
                vec![i32::MIN, i32::MAX],
                "单元素切片也要正确复制：应得到 [i32::MIN, i32::MAX]",
            );
        },
    );
}

/// 【待实现】由切片构造 `Vec`，追加 `extra` 后返回。
///
/// 实现要求：
///   - 用 `input.to_vec()`（或 `input.iter().copied().collect()`）从切片**复制**出 `Vec<i32>`：
///     切片只是借用，不能直接当 `Vec` 使用，必须先复制成拥有所有权的 `Vec`
///     （`to_vec()` 会分配新的堆内存，与原切片不共享缓冲区）；
///   - 把这个向量声明成 `let mut`，调用 `push(extra)` 追加到末尾
///     （漏写 `mut` 时 `push` 会报 `error[E0596]: cannot borrow ... as mutable`）；
///   - 返回这个 `Vec<i32>`；`input` 为空时返回只含 `extra` 的向量。
///
/// 示例输入：
/// ```text
/// input = [1, 2, 3]
/// extra = 4
/// ```
/// 示例输出：
/// ```text
/// [1, 2, 3, 4]
/// ```
fn exercise_08_01_vec_create_and_push(input: &[i32], extra: i32) -> Vec<i32> {
    // 切片只是借用：先 to_vec() 复制出拥有所有权的 Vec，再 push 追加
    let mut values = input.to_vec();
    values.push(extra);
    values
}

// ===========================================================================
// kp_08_02 Vec 的安全访问：get 返回 Option，[] 越界会 panic
// ===========================================================================

/// 知识点考核：用 `get` 安全访问下标，用 `first` 取首元素。
#[test]
fn kp_08_02_vec_access() {
    assess(
        M,
        "kp_08_02",
        "Vec 的索引与安全访问：`get` / `first` 返回 `Option`，`[]` 越界会 panic",
        Kind::Core,
        "复习 lesson_08 示例 2（vec_access_and_modify）：`v[i]` 越界在运行期 panic \
         （index out of bounds），`v.get(i)` 把「可能越界」表达成 `Option`，`v.first()` 同理。",
        || {
            // 正常用例：合法下标取到 Some，首元素同时取出
            eq(
                exercise_08_02_vec_access(&[88, 92, 76], 1),
                (Some(92), 88),
                "get(1) 应得到 Some(92)；first() 应得到 88",
            );
            // 正常用例：下标 0 就是首元素
            eq(
                exercise_08_02_vec_access(&[5], 0),
                (Some(5), 5),
                "只有一个元素时 get(0) = Some(5)，first() = 5",
            );
            // 边界：下标越界 → None，首元素仍然有效
            eq(
                exercise_08_02_vec_access(&[1, 2, 3], 3),
                (None, 1),
                "长度 3 时下标 3 越界：get(3) 必须是 None（而 `v[3]` 会 panic）",
            );
            // 边界：空切片 → (None, 0)，first() 缺失时回退成 0
            eq(
                exercise_08_02_vec_access(&[], 0),
                (None, 0),
                "空切片时 get(0) = None；first() 也是 None，配合 unwrap_or(0) 得到 0",
            );
        },
    );
}

/// 【待实现】返回「按下标查询的结果」与「首元素」。
///
/// 实现要求：
///   - 第一个分量：`input.get(index).copied()`，类型是 `Option<i32>`
///     （`get` 返回 `Option<&i32>`，`i32` 是 `Copy`，所以用 `.copied()` 把它变成值）；
///   - 第二个分量：`input.first().copied().unwrap_or(0)`，类型是 `i32`；
///   - 返回 `(第一个分量, 第二个分量)`；空切片时必须是 `(None, 0)`，**不得 panic**——
///     对比一下 `input[index]`：`[]` 越界会直接 panic
///     （`index out of bounds: the len is 3 but the index is 3`），
///     所以「下标来自外部输入」时必须用 `get`。
///
/// 示例输入：
/// ```text
/// input = [88, 92, 76]
/// index = 1
/// ```
/// 示例输出：
/// ```text
/// (Some(92), 88)
/// ```
fn exercise_08_02_vec_access(input: &[i32], index: usize) -> (Option<i32>, i32) {
    // get 返回 Option<&i32>，copied() 变成值：越界只是 None，不会 panic
    let found = input.get(index).copied();
    let first = input.first().copied().unwrap_or(0);
    (found, first)
}

// ===========================================================================
// kp_08_03 Vec 的原地修改：retain 过滤 + iter_mut 就地翻倍
// ===========================================================================

/// 知识点考核：用 `retain` 删除负数、用 `iter_mut` 把剩余元素翻倍。
#[test]
fn kp_08_03_vec_mutate() {
    assess(
        M,
        "kp_08_03",
        "Vec 的原地修改：`retain` 按条件删除、`iter_mut` 解引用修改元素",
        Kind::Core,
        "复习 lesson_08 示例 1 与示例 3（vec_iterate_and_capacity）：`retain` 就地保留满足条件的\
         元素（比倒序 `remove` 直观），`for value in v.iter_mut() { *value *= 2; }` 才能修改元素。",
        || {
            // 正常用例：删掉负数 -1，剩下的 2、3、4 翻倍
            eq(
                exercise_08_03_vec_mutate(&mut vec![3, -1, 2, 4]),
                (3usize, 18i32),
                "retain 后剩 [3, 2, 4]（长度 3），翻倍后 [6, 4, 8] 之和为 18",
            );
            // 正常用例：0 不是负数，必须保留（0 翻倍仍是 0，但长度要算进去）
            eq(
                exercise_08_03_vec_mutate(&mut vec![0, -5, 1]),
                (2usize, 2i32),
                "0 不是负数要保留：retain 后是 [0, 1]，长度 2，翻倍后之和 0 + 2 = 2",
            );
            // 边界：全是负数 → 全部删光
            eq(
                exercise_08_03_vec_mutate(&mut vec![-1, -2, -3]),
                (0usize, 0i32),
                "全是负数时 retain 之后为空：应返回 (0, 0)，且空向量的 sum 就是 0",
            );
            // 边界：本来就是空向量
            eq(
                exercise_08_03_vec_mutate(&mut Vec::new()),
                (0usize, 0i32),
                "空向量不需要任何特殊处理：retain 与 iter_mut 都不会 panic",
            );
        },
    );
}

/// 【待实现】把 `values` 里的负数删掉，剩余元素翻倍，返回 `(新长度, 元素之和)`。
///
/// 实现要求：
///   - `values.retain(|value| *value >= 0);` —— **原地**删除所有负数
///     （`retain` 的闭包参数是 `&i32`，所以比较时要写 `*value >= 0`；
///     注意 0 不是负数，必须保留）；
///   - `for value in values.iter_mut() { *value *= 2; }` —— 用 `iter_mut` 就地翻倍
///     （`*value` 解引用后才能赋值；直接写 `for value in values` 会移动整个 `Vec`，
///     这里必须用 `iter_mut()` 借用遍历）；
///   - 返回 `(values.len(), values.iter().sum::<i32>())`；空向量时是 `(0, 0)`。
///
/// 示例输入：
/// ```text
/// values = [3, -1, 2, 4]
/// ```
/// 示例输出：
/// ```text
/// (3, 18)
/// ```
fn exercise_08_03_vec_mutate(values: &mut Vec<i32>) -> (usize, i32) {
    // retain 的闭包参数是 &i32，所以要 *value 比较；0 不是负数，必须保留
    values.retain(|value| *value >= 0);
    // iter_mut 借用遍历，解引用后才能就地修改元素
    for value in values.iter_mut() {
        *value *= 2;
    }
    (values.len(), values.iter().sum::<i32>())
}

// ===========================================================================
// kp_08_04 String 与 UTF-8：len() 是字节数，chars().count() 是字符数
// ===========================================================================

/// 知识点考核：区分字符串的字节长度与字符个数。
#[test]
fn kp_08_04_string_utf8() {
    assess(
        M,
        "kp_08_04",
        "String 与 UTF-8：`len()` 是字节数，`chars().count()` 是字符数",
        Kind::Core,
        "复习 lesson_08 示例 4（string_basics_and_utf8）：中文一个字占 3 字节，emoji 占 4 字节，\
         所以 `len()` 常大于 `chars().count()`；按字节下标切分不在字符边界上会 panic。",
        || {
            // 边界：空串
            eq(
                exercise_08_04_string_utf8(""),
                (0usize, 0usize),
                "空串的字节数与字符数都是 0",
            );
            // 正常用例：纯 ASCII 时两者相等
            eq(
                exercise_08_04_string_utf8("rust"),
                (4usize, 4usize),
                "ASCII 每个字符 1 字节：\"rust\" 是 4 字节 4 字符",
            );
            // 正常用例：中文 3 字节/字 + ASCII 1 字节/字
            eq(
                exercise_08_04_string_utf8("中文abc"),
                (9usize, 5usize),
                "「中文」占 6 字节，「abc」占 3 字节 → 9 字节；字符数是 2 + 3 = 5",
            );
            // 边界：emoji 是 4 字节的单个字符
            eq(
                exercise_08_04_string_utf8("🦀"),
                (4usize, 1usize),
                "螃蟹 emoji 是 4 字节、1 个字符（这正是「不能按字节下标切字符串」的原因）",
            );
            // 边界：带重音符号的拉丁字母是 2 字节
            eq(
                exercise_08_04_string_utf8("é"),
                (2usize, 1usize),
                "\"é\" 是 2 字节的单个字符：len() = 2，chars().count() = 1",
            );
        },
    );
}

/// 【待实现】返回 `(字节长度, 字符个数)`。
///
/// 实现要求：
///   - 第一个分量：`text.len()`（**字节**数，`len()` 从来不是字符数）；
///   - 第二个分量：`text.chars().count()`（Unicode 标量值个数，也就是「肉眼看到的字数」）；
///   - 返回这个元组；空串时是 `(0, 0)`；
///   - UTF-8 是变长编码（ASCII 1 字节、中文 3 字节、`🦀` 4 字节），所以**不能**写
///     `text.len() / 某个固定值` 去猜字符数；也**不能**用 `text[0]` 取字符
///     （`error[E0277]`：`str` 不能用整数下标）；要按字符取请用 `.chars().nth(n)`，
///     按字节切分请用 `char_indices()` 给出的合法边界。
///
/// 示例输入：
/// ```text
/// text = "中文abc"
/// ```
/// 示例输出：
/// ```text
/// (9, 5)
/// ```
fn exercise_08_04_string_utf8(text: &str) -> (usize, usize) {
    // len() 是 UTF-8 字节数，chars().count() 才是字符个数
    (text.len(), text.chars().count())
}

// ===========================================================================
// kp_08_05 字符串拼接的三种方式及其所有权语义
// ===========================================================================

/// 知识点考核：用 `push_str` / `+` / `format!` 三种方式拼出同一个字符串。
#[test]
fn kp_08_05_concat_three_ways() {
    assess(
        M,
        "kp_08_05",
        "字符串拼接三种方式：`push_str` 原地追加、`+` 消耗左侧、`format!` 不消耗任何操作数",
        Kind::Core,
        "复习 lesson_08 示例 5（string_concat_and_format）：`push_str` 不转移所有权；\
         `a + &b` 里 `a` 的所有权被消耗（之后不能再使用 `a`）；`format!` 返回新 `String`，参数都还可用。",
        || {
            // 正常用例：三种方式结果必须完全一致
            let (pushed, added, formatted) =
                exercise_08_05_concat_three_ways(&["Hello", ", ", "world", "!"]);
            eq(
                pushed,
                String::from("Hello, world!"),
                "push_str 循环拼接应得到 \"Hello, world!\"",
            );
            eq(
                added,
                String::from("Hello, world!"),
                "`+` 运算符拼接应得到 \"Hello, world!\"",
            );
            eq(
                formatted,
                String::from("Hello, world!"),
                "format! 拼接应得到 \"Hello, world!\"",
            );
            // 边界：空切片 → 三个空串（三者仍然相等）
            let (pushed, added, formatted) = exercise_08_05_concat_three_ways(&[]);
            eq(
                pushed,
                String::new(),
                "片段为空时 push_str 循环一次都不执行，结果是空 String",
            );
            eq(
                added,
                String::new(),
                "片段为空时 `+` 的初始累加值就是最终结果：空 String",
            );
            eq(
                formatted,
                String::new(),
                "片段为空时 format! 也必须给出空 String（不要 panic）",
            );
            // 边界：多字节字符按片段拼接，字节长度随之增长
            let (pushed, added, formatted) = exercise_08_05_concat_three_ways(&["中", "文"]);
            eq(
                pushed,
                String::from("中文"),
                "多字节片段同样只是追加字节：应得到 \"中文\"",
            );
            eq(added, String::from("中文"), "`+` 方式也要支持多字节片段");
            eq(
                formatted,
                String::from("中文"),
                "format! 方式也要支持多字节片段（\"中文\" 是 6 字节 2 字符）",
            );
        },
    );
}

/// 【待实现】用三种方式拼出同一个字符串，返回 `(push_str 版, + 版, format! 版)`。
///
/// 实现要求（三种方式都要真的用到）：
///   1. `push_str`：`let mut out = String::new(); for part in parts { out.push_str(part); }`
///      —— 原地追加，`out` 的所有权没有移动；
///   2. `+` 运算符：先把初始值变成 `String`，再逐个 `acc = acc + part;`
///      —— 注意 `+` 会**消耗左侧**的所有权，所以习惯写法是用遮蔽
///      （`let acc = acc + part;`）让旧绑定自然失效；`+` 的右侧必须是 `&str` 或 `&String`
///      （自动解引用），两个 `String` 直接相加会报
///      `error[E0308]: expected &str, found String`；
///   3. `format!`：例如 `format!("{}", parts.concat())`，或先拼好再 `format!` 一次
///      —— `format!` 不消耗任何操作数，返回新的 `String`；
///   - 返回 `(第一种, 第二种, 第三种)`；三个字符串内容必须完全相同。
///
/// 示例输入：
/// ```text
/// parts = ["Hello", ", ", "world", "!"]
/// ```
/// 示例输出：
/// ```text
/// ("Hello, world!", "Hello, world!", "Hello, world!")
/// ```
fn exercise_08_05_concat_three_ways(parts: &[&str]) -> (String, String, String) {
    // 一：push_str 原地追加，out 的所有权没有移动
    let mut pushed = String::new();
    for part in parts {
        pushed.push_str(part);
    }
    // 二：+ 会消耗左侧所有权，所以左侧是 mut 的累加器；右侧必须是 &str
    let mut added = String::new();
    for part in parts {
        added = added + *part;
    }
    // 三：format! 不消耗任何操作数，返回新的 String
    let formatted = format!("{}", parts.concat());
    (pushed, added, formatted)
}

// ===========================================================================
// kp_08_06 HashMap 基础：插入、重复键覆盖、按键排序输出
// ===========================================================================

/// 知识点考核：用 `HashMap` 收集键值对，重复键保留最后一次的值，按键排序后返回。
#[test]
fn kp_08_06_hashmap_basics() {
    assess(
        M,
        "kp_08_06",
        "HashMap 基础：`insert` 覆盖旧值并返回旧值；遍历顺序不固定，输出前必须排序",
        Kind::Core,
        "复习 lesson_08 示例 6（hashmap_basics）：`HashMap` 不保证遍历顺序，\
         所以「需要确定输出」时必须先排序；`insert` 对已存在的键会覆盖旧值（并返回被覆盖的旧值）。",
        || {
            // 正常用例：两个不同的键，按键字典序输出
            eq(
                exercise_08_06_hashmap_basics(&[("b", 2), ("a", 1)]),
                vec![(String::from("a"), 1), (String::from("b"), 2)],
                "插入顺序是 b、a，但输出必须按键字典序排序：[(\"a\", 1), (\"b\", 2)]",
            );
            // 正常用例：重复键只保留最后一次的值，且 map 长度不增加
            eq(
                exercise_08_06_hashmap_basics(&[("x", 1), ("y", 5), ("x", 9)]),
                vec![(String::from("x"), 9), (String::from("y"), 5)],
                "重复键 x 只保留最后一次的 9（长度仍是 2），所以是 [(\"x\", 9), (\"y\", 5)]",
            );
            // 边界：空输入 → 空 Vec
            eq(
                exercise_08_06_hashmap_basics(&[]),
                Vec::new(),
                "没有键值对时应返回空 Vec，不要 panic",
            );
            // 边界：多个键的字典序必须稳定（先比较首字母，再逐字符比较）
            eq(
                exercise_08_06_hashmap_basics(&[("ab", 1), ("aa", 2), ("", 3)]),
                vec![
                    (String::from(""), 3),
                    (String::from("aa"), 2),
                    (String::from("ab"), 1),
                ],
                "空字符串键排在最前，然后按逐字符比较：\"\" < \"aa\" < \"ab\"",
            );
        },
    );
}

/// 【待实现】把 `pairs` 收进 `HashMap`，按键字典序排序后返回。
///
/// 实现要求：
///   - 建一个 `HashMap<String, i32>`（键要从 `&str` 转成拥有所有权的 `String`）；
///   - 遍历 `pairs` 逐个 `insert`：**重复键由后面的插入覆盖前面的值**（这是 `insert` 的语义）；
///   - 把 `(String, i32)` 收集成 `Vec`，用 `sort_by` / `sort_unstable_by` **按键字典序**排序，
///     例如 `rows.sort_unstable_by(|a, b| a.0.cmp(&b.0));`；
///   - **千万不要**直接返回 `map.into_iter().collect()`——`HashMap` 的遍历顺序每次运行
///     都可能不同，期望值就无从写起；
///   - 返回这个 `Vec`；`pairs` 为空时返回空 `Vec`。
///
/// 示例输入：
/// ```text
/// pairs = [("b", 2), ("a", 1)]
/// ```
/// 示例输出：
/// ```text
/// [("a", 1), ("b", 2)]
/// ```
fn exercise_08_06_hashmap_basics(pairs: &[(&str, i32)]) -> Vec<(String, i32)> {
    // 键要转成拥有所有权的 String；重复键由后面的 insert 覆盖旧值
    let mut map: HashMap<String, i32> = HashMap::new();
    for (key, value) in pairs {
        map.insert((*key).to_string(), *value);
    }
    // HashMap 的遍历顺序不固定：先按键字典序排序，输出才是确定的
    let mut rows: Vec<(String, i32)> = map.into_iter().collect();
    rows.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    rows
}

// ===========================================================================
// kp_08_07 entry API 统计词频：按次数降序、同次数按字典序
// ===========================================================================

/// 知识点考核：用 `entry` API 统计词频，并按「次数降序 + 词字典序」输出。
#[test]
fn kp_08_07_word_frequency() {
    assess(
        M,
        "kp_08_07",
        "entry API 统计词频：`entry(word).or_insert(0)` 一次查找完成「查 + 改」，输出前排序",
        Kind::Hard,
        "复习 lesson_08 示例 7 与示例 8（hashmap_entry_api / word_frequency_scenario）：\
         用 `*counts.entry(word).or_insert(0) += 1;` 统计；排序规则是\
         `b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))`（次数多的在前，次数相同按字典序）。",
        || {
            // 正常用例：rust 出现 3 次排第一；次数相同按字典序
            eq(
                exercise_08_07_word_frequency("rust makes rust safe rust makes"),
                vec![
                    (String::from("rust"), 3),
                    (String::from("makes"), 2),
                    (String::from("safe"), 1),
                ],
                "rust=3 排第一；makes=2 排第二；safe=1 排最后",
            );
            // 正常用例：大小写要统一成小写，否则 Rust 与 rust 会被算成两个词
            eq(
                exercise_08_07_word_frequency("Rust RUST rust"),
                vec![(String::from("rust"), 3)],
                "统一转小写后三个词是同一个：只应有一个键 rust = 3",
            );
            // 边界：空串 → 空 Vec
            eq(
                exercise_08_07_word_frequency(""),
                Vec::new(),
                "空文本没有任何词，必须返回空 Vec（不要 panic）",
            );
            // 边界：只有空白字符 → 空 Vec（split_whitespace 不会产生空片段）
            eq(
                exercise_08_07_word_frequency("   \t\n  "),
                Vec::new(),
                "只有空白时 split_whitespace 一个片段都不产生，应返回空 Vec",
            );
            // 边界：次数完全相同 → 完全由字典序决定顺序
            eq(
                exercise_08_07_word_frequency("b a c"),
                vec![
                    (String::from("a"), 1),
                    (String::from("b"), 1),
                    (String::from("c"), 1),
                ],
                "三个词各出现 1 次，顺序只能是字典序 a、b、c（不能依赖 HashMap 的遍历顺序）",
            );
        },
    );
}

/// 【待实现】统计词频，返回按「次数降序 + 词字典序」排好序的向量。
///
/// 实现要求：
///   - 用 `text.split_whitespace()` 切词（它会自动忽略连续空白，比 `split(' ')` 健壮）；
///   - 每个词先 `to_lowercase()` 统一小写，避免 "Rust" 与 "rust" 被算成两个词；
///   - 统计必须用 **`entry` API**：`*counts.entry(word).or_insert(0) += 1;`
///     （`entry` 返回 `&mut usize`，所以计数要写 `*... += 1`（显式解引用）；
///     不要写「先 `contains_key` 再 `insert`」，那样是两次哈希查找）；
///   - 收集成 `Vec<(String, usize)>` 后排序：**次数降序**，次数相同时**词字典序升序**
///     （`b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))`）；不能只按次数排——次数相同的词
///     如果顺序不定，输出就不是确定的；
///   - 返回排序后的向量；空串或只有空白时返回空 `Vec`。
///
/// 示例输入：
/// ```text
/// text = "rust makes rust safe rust makes"
/// ```
/// 示例输出：
/// ```text
/// [("rust", 3), ("makes", 2), ("safe", 1)]
/// ```
fn exercise_08_07_word_frequency(text: &str) -> Vec<(String, usize)> {
    // entry 一次哈希查找完成「查 + 改」：返回 &mut usize，所以要显式解引用
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    // 次数降序；次数相同时按词字典序升序，保证输出确定
    let mut rows: Vec<(String, usize)> = counts.into_iter().collect();
    rows.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    rows
}

// ===========================================================================
// kp_08_08 借用陷阱：先复制出值，再插入新键（E0502）
// ===========================================================================

/// 知识点考核：先取值、后插入，避开「不可变借用与可变借用重叠」。
#[test]
fn kp_08_08_borrow_trap() {
    assess(
        M,
        "kp_08_08",
        "借用陷阱：`get` 拿到引用期间不能 `insert`（E0502），先把值拷贝出来即可",
        Kind::Hard,
        "复习 lesson_08 示例 9（ownership_and_borrow_traps）陷阱三：\
         `let value = counter.get(\"hits\").unwrap(); counter.insert(\"misses\", 1);` 会报\
         `error[E0502]: cannot borrow as mutable because it is also borrowed as immutable`；\
         修正就是先 `copied()` 把值取出来，让不可变借用立刻结束。",
        || {
            // 正常用例：取到 a 的值 10，插入 b = 11 后 map 有两个键
            let mut map = HashMap::new();
            map.insert(String::from("a"), 10);
            eq(
                exercise_08_08_borrow_trap(&mut map),
                (10i32, 2usize),
                "取到 a = 10；插入 b = 10 + 1 = 11 之后 len 变成 2",
            );
            // 边界：键 "a" 不存在时按 0 处理，插入的 b = 1
            let mut empty = HashMap::new();
            eq(
                exercise_08_08_borrow_trap(&mut empty),
                (0i32, 1usize),
                "a 缺失时取到的值按 0 算，插入 b = 1，此时 len 是 1",
            );
            // 边界："b" 已经存在时必须被 insert 覆盖（len 不增加）
            let mut with_b = HashMap::new();
            with_b.insert(String::from("a"), -3);
            with_b.insert(String::from("b"), 100);
            eq(
                exercise_08_08_borrow_trap(&mut with_b),
                (-3i32, 2usize),
                "取到 a = -3，b 被覆盖成 -2；len 仍然是 2（insert 覆盖不新增键）",
            );
            // 边界：空 map 只调用一次也要能得到 (0, 1)
            let mut again = HashMap::new();
            eq(
                exercise_08_08_borrow_trap(&mut again),
                (0i32, 1usize),
                "空 map 的结果必须是 (0, 1)，顺序不能颠倒",
            );
        },
    );
}

/// 【待实现】把 `"a"` 的值拷出来，再插入 `("b", 值 + 1)`，返回 `(值, 插入后的 len)`。
///
/// 实现要求：
///   - 第一步：用 `map.get("a")` 取**不可变借用**，立刻 `.copied().unwrap_or(0)` 得到 `i32`
///     （键缺失时按 0，返回类型是 `i32` 而不是 `&i32`）；
///   - 第二步：`map.insert(String::from("b"), value + 1);` —— 这次是**可变借用**，
///     因为第一步的借用已经结束，所以完全合法；
///   - 两个步骤不能合并成「先持有引用、再插入」：`let value = map.get("a").unwrap();`
///     得到的 `&i32` 会一直借用 `map`，此时再 `map.insert(...)` 就会报
///     `error[E0502]: cannot borrow `map` as mutable because it is also borrowed as immutable`；
///     修正口诀是**先把值拷出来，再改集合**（`Copy` 类型用 `copied()`，
///     非 `Copy` 类型用 `cloned()` 或 `to_string()`）；
///   - 返回 `(value, map.len())`（注意顺序：先返回值，再返回插入之后的长度）。
///
/// 示例输入：
/// ```text
/// map = {"a": 10}
/// ```
/// 示例输出：
/// ```text
/// (10, 2)
/// ```
fn exercise_08_08_borrow_trap(map: &mut HashMap<String, i32>) -> (i32, usize) {
    // 先把值拷出来（不可变借用立刻结束），之后才能可变借用 insert，避开 E0502
    let value = map.get("a").copied().unwrap_or(0);
    map.insert(String::from("b"), value + 1);
    (value, map.len())
}

// ===========================================================================
// kp_08_09 常见错误诊断：读得懂编译器报错，才改得动集合代码
// ===========================================================================

/// 知识点考核：写出课程示例 10 前三个错误场景对应的编译器错误编号。
#[test]
fn kp_08_09_diagnose_errors() {
    assess(
        M,
        "kp_08_09",
        "常见错误诊断：E0596 / E0282 / E0277 分别对应哪类集合错误",
        Kind::Hard,
        "复习 lesson_08 示例 10（common_mistakes）：错误 1 是「`Vec::new()` 忘了 mut 就 push」\
         （E0596），错误 2 是「元素类型完全没有来源」的 `Vec::new()`（E0282），\
         错误 3 是「用整数下标访问 `String`」（E0277：`str` 不能用 `{integer}` 索引）。",
        || {
            eq_slice(
                &exercise_08_09_diagnose_errors(),
                &["E0596", "E0282", "E0277"],
                "顺序必须是：忘写 mut（E0596）、推断不出元素类型（E0282）、字符串整数下标（E0277）",
            );
        },
    );
}

/// 【待实现】写出三个集合场景对应的编译器错误编号。
///
/// 场景（与课程示例 10 的错误 1 / 2 / 3 一致，按课程注释顺序）：
///   1. `let values = Vec::new(); values.push("hi");`
///      —— 没有声明 `mut` 就对 `Vec` 调用 `push`；
///   2. `let values = Vec::new(); println!("{}", values.len());`
///      —— 元素类型没有任何来源，编译器推断不出 `Vec<_>` 的类型；
///   3. `let s = String::from("你好"); let ch = s[0];`
///      —— 用整数下标访问 `String` / `str`。
///
/// 实现要求：返回 3 个错误编号字符串（形如 `"E0596"`），顺序与上面一致
/// （记忆口诀：0596 = 你没写 `mut`，0282 = 我不知道元素是什么类型，
/// 0277 = `str` 不能用整数下标）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0596", "E0282", "E0277"]
/// ```
fn exercise_08_09_diagnose_errors() -> [&'static str; 3] {
    // 顺序：忘写 mut（E0596）/ 推断不出元素类型（E0282）/ str 用整数下标（E0277）
    ["E0596", "E0282", "E0277"]
}
