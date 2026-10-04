//! assessments/lesson_06_structs.rs —— 考核：结构体（对应 lesson_06）
//!
//! - 对应课程：`src/tutorial/lesson_06_structs.rs`
//! - 知识点出处：`src/tutorial/README.md` 第二阶段「06 结构体」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_06_structs                 # 只考这一课
//!   cargo test                                          # 考全部 18 课
//!   cargo run --bin assessment_report                   # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_06_xx_xxx` 练习函数（或 `impl` 块里的练习方法），
//!    它的函数体里只有一行 `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_06_structs`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数/方法的函数体，可以按需增加局部变量与辅助函数；
//! - 顶部「提供给你的类型（不要修改）」一节里的 `struct` 定义与已给出的构造函数
//!   **不要修改**，它们是骨架能编译的前提；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 10）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `Rectangle { width: 10 }`（漏字段） | `error[E0063]` missing field `height` in initializer | 补齐所有字段，或用 `..other` 更新语法补齐 |
//! | `Rectangle { width: 3, hight: 4 }` | `error[E0560]` struct `Rectangle` has no field named `hight` | 字段名必须与定义逐字一致（是 `width` 不是 `hight`） |
//! | `let r = &rect; r.width = 1;` | `error[E0594]` cannot assign to `r.width`, which is behind a `&` reference | 要改字段就直接持有 `mut` 实例，或使用 `&mut` 借用 |
//! | `fn bad_scale(&self) { self.width *= 2; }` | `error[E0594]` cannot assign to `self.width`, which is behind a `&` reference | 改字段的方法接收者必须是 `&mut self` |
//! | `println!("{:?}", NoDebug { n: 1 });` | `error[E0277]` `NoDebug` doesn't implement `Debug` | 定义前加 `#[derive(Debug)]` |
//! | `println!("{report}");`（没实现 Display） | `error[E0277]` `Report` doesn't implement `Display` | 手动 `impl std::fmt::Display`，或先用 `{:?}` 调试 |
//! | `let t2 = Theme { font_size: 9, ..t1 }; println!("{t1:?}");` | `error[E0382]` borrow of moved value: `t1.name` | 给非 Copy 字段加 `clone()`，或显式写出所有字段 |

use assessment_harness::{Kind, assess, eq, is_false};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_06";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// ===========================================================================
//
// 下面这些类型由考核文件提供，保证骨架态就能编译通过。
// 学员只需要实现标了「【待实现】」的方法/关联函数/练习函数；
// 类型定义与已经给出的构造函数请原样保留（改了考核用例就编译不过了）。

/// 计数器：`new` 已提供，三个方法由学员实现（见 kp_06_03）。
#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    /// 提供的构造函数（关联函数）：`Counter::new(0)`。
    fn new(value: i32) -> Self {
        Counter { value }
    }
}

/// 二维点：已派生 `PartialEq`，所以测试里可以直接用 `eq` 比较两个点（见 kp_06_04）。
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// 温度：已派生 `Debug`（`{:?}` 可用），`Display`（`{}`）由学员实现（见 kp_06_06）。
#[derive(Debug, Clone, Copy, PartialEq)]
struct Temperature {
    celsius: f64,
}

/// 订单：`new` 已提供，用来演示「部分移动」（见 kp_06_08）。
struct Order {
    id: u32,
    note: String,
}

impl Order {
    /// 提供的构造函数：把 `&str` 参数复制成 `String` 字段。
    fn new(id: u32, note: &str) -> Self {
        Order {
            id,
            note: note.to_string(),
        }
    }
}

/// 查询构造器：字段定义已提供，三个方法由学员实现（见 kp_06_07）。
#[derive(Debug)]
struct QueryBuilder {
    table: String,
    limit: Option<u32>,
}

// ===========================================================================
// kp_06_01 定义与实例化：用点号访问字段
// ===========================================================================

/// 知识点考核：在函数内定义结构体、实例化并读取字段。
#[test]
fn kp_06_01_define_and_instantiate() {
    assess(
        M,
        "kp_06_01",
        "结构体定义与实例化：字段写全、用点号访问",
        Kind::Basic,
        "复习 lesson_06 示例 1（define_and_instantiate）：先 `struct Rectangle { width: u32, height: u32 }` \
         定义形状，再 `Rectangle { width: 30, height: 50 }` 实例化，然后用 `rect.width` / `rect.height` \
         点号读字段——实例化时字段必须写全（漏了会报 E0063）。",
        || {
            let area = exercise_06_01_define_and_instantiate();
            // 正常用例：30 × 50 的矩形面积
            eq(area, 1500u32, "30 × 50 = 1500：面积要用两个字段相乘得到");
            // 边界：常见笔误——把乘法写成加法
            is_false(
                area == 80u32,
                "边界：30 + 50 = 80 是常见笔误，面积必须是 width * height 而不是相加",
            );
        },
    );
}

/// 【待实现】在函数内定义并实例化经典结构体。
///
/// 实现要求：
///   - 在函数内定义 `struct Rectangle { width: u32, height: u32 }`；
///   - 用 `Rectangle { width: 30, height: 50 }` 创建实例，字段必须写全，
///     漏一个就会报 `error[E0063]: missing field ...`；
///   - 用点号分别访问两个字段，返回面积 `width * height`（也就是 1500）；
///     读字段只是复制（`u32` 是 Copy），不会把实例移走。
///
/// 示例输入：
/// ```text
/// width = 30, height = 50
/// ```
/// 示例输出：
/// ```text
/// 1500
/// ```
fn exercise_06_01_define_and_instantiate() -> u32 {
    assessment_harness::todo_exercise(
        "exercise_06_01_define_and_instantiate",
        "在函数内定义 struct Rectangle { width: u32, height: u32 }，实例化 30×50，\
         用点号读取字段并返回面积 1500",
        (),
    )
}

// ===========================================================================
// kp_06_02 字段初始化简写 + 结构体更新语法
// ===========================================================================

/// 知识点考核：先用简写造第一份配置，再用 `..first` 造第二份。
#[test]
fn kp_06_02_shorthand_and_update() {
    assess(
        M,
        "kp_06_02",
        "字段初始化简写与结构体更新语法（..other）",
        Kind::Core,
        "复习 lesson_06 示例 2（field_init_shorthand）与示例 3（struct_update_syntax）：\
         字段名与变量名相同时可以只写一次名字；`Config { port: 9090, ..first }` 只覆盖写出来的字段，\
         其余字段从 `first` 补齐，非 Copy 字段（String）会被 move。",
        || {
            let (first_port, second_port, host, retries) =
                exercise_06_02_shorthand_and_update(String::from("localhost"), 3306);
            // 正常用例
            eq(
                first_port,
                3306u16,
                "first 用字段初始化简写构造，port 直接来自入参 3306",
            );
            eq(
                second_port,
                9090u16,
                "second 用 `..first` 更新语法把 port 覆盖成 9090",
            );
            eq(
                host,
                String::from("localhost"),
                "host 没有被覆盖，应该从 first 搬过来（String 是 move 不是复制）",
            );
            eq(
                retries,
                3u8,
                "retries 由更新语法从 first 补齐，仍是简写时写死的 3",
            );
            // 边界：port 取 0、host 为空串时，简写与覆盖行为都不应改变
            let (zero_first, zero_second, empty_host, zero_retries) =
                exercise_06_02_shorthand_and_update(String::new(), 0);
            eq(
                zero_first,
                0u16,
                "边界：port = 0 时简写得到的就是 0（简写不会替换成默认值）",
            );
            eq(
                zero_second,
                9090u16,
                "边界：即使入参 port 是 0，second 仍被固定覆盖成 9090",
            );
            eq(
                empty_host,
                String::new(),
                "边界：空主机名必须原样保留，不能被丢掉或替换",
            );
            eq(
                zero_retries,
                3u8,
                "边界：入参再特殊，retries 也始终是 3（更新语法只覆盖列出来的字段）",
            );
        },
    );
}

/// 【待实现】字段初始化简写与结构体更新语法。
///
/// 实现要求（全部在函数体内完成）：
///   - 在函数内定义 `struct Config { host: String, port: u16, retries: u8 }`；
///   - 用**字段初始化简写**造 `first`：`Config { host, port, retries: 3 }`
///     （`host` / `port` 与字段同名，直接写变量名即可）；
///   - 用**结构体更新语法**造 `second`：`Config { port: 9090, ..first }`；
///     `..first` 必须写在字段列表的**最后**，它只覆盖 `port`，`host` 与 `retries`
///     从 `first` 补齐；
///   - 更新语法会把 `first.host`（String，非 Copy）move 进 `second`，之后不能再整体使用
///     `first`，但读 `first.port`（u16 是 Copy）仍然合法；
///   - 返回 `(first.port, second.port, second.host, second.retries)`，
///     也就是 `(port, 9090, host, 3)`：返回值要读一次 `retries`，
///     用来确认更新语法确实把它补上了。
///
/// 示例输入：
/// ```text
/// host = String::from("localhost"), port = 3306
/// ```
/// 示例输出：
/// ```text
/// (3306, 9090, "localhost", 3)
/// ```
fn exercise_06_02_shorthand_and_update(host: String, port: u16) -> (u16, u16, String, u8) {
    assessment_harness::todo_exercise(
        "exercise_06_02_shorthand_and_update",
        "函数内定义 struct Config { host: String, port: u16, retries: u8 }：\
         用简写造 first（retries = 3），用 `Config { port: 9090, ..first }` 造 second，\
         返回 (first.port, second.port, second.host, second.retries)",
        (host, port),
    )
}

// ===========================================================================
// kp_06_03 方法与三种接收者：&self / &mut self / self
// ===========================================================================

/// 知识点考核：同一个类型上三种接收者各自的作用。
#[test]
fn kp_06_03_method_receivers() {
    assess(
        M,
        "kp_06_03",
        "方法接收者：&self 读、&mut self 改、self 消费",
        Kind::Core,
        "复习 lesson_06 示例 6（methods_and_receivers）：`&self` 只读借用、`&mut self` 可写借用、\
         `self` 按值接收会把调用者的所有权 move 进方法（调用后原变量失效）。",
        || {
            // 正常用例：&mut self 累加，&self 读取，最后 self 消费
            let mut counter = Counter::new(0);
            counter.add(3);
            counter.add(4);
            eq(
                counter.value(),
                7,
                "0 + 3 + 4 = 7：&self 方法读到的就是累加后的值",
            );
            eq(
                counter.into_value(),
                7,
                "into_value(self) 按值消费自身，交出的仍是 7",
            );
            // 边界：初始值非 0 且 delta 为负数
            let mut moved = Counter::new(10);
            moved.add(-10);
            eq(
                moved.value(),
                0,
                "边界：delta 为负数也要正确累加（10 + (-10) = 0）",
            );
            eq(
                moved.into_value(),
                0,
                "边界：消费自身后交出的值是 0，而不是初始的 10",
            );
        },
    );
}

impl Counter {
    /// 【待实现】`&self`：只读借用，读取内部值。
    ///
    /// 实现要求：
    ///   - 用 `&self` 接收者返回 `value` 字段（例如 `self.value`）；
    ///   - `&self` 方法不会消耗实例，所以调用之后还能继续用同一个变量；
    ///     接收者写成 `self` 才会把实例 move 走，之后再调用会报 `error[E0382]`。
    ///
    /// 示例输入：
    /// ```text
    /// self = Counter { value: 7 }
    /// ```
    /// 示例输出：
    /// ```text
    /// 7
    /// ```
    fn value(&self) -> i32 {
        assessment_harness::todo_exercise(
            "Counter::value",
            "用 &self 接收者读取并返回 value 字段",
            (&self.value,),
        )
    }

    /// 【待实现】`&mut self`：可写借用，原地修改。
    ///
    /// 实现要求：
    ///   - 用 `&mut self` 接收者把 `delta` 累加到 `value` 上（`self.value += delta;`）；
    ///   - 调用方必须把绑定声明成 `let mut counter = ...`，否则借用检查会拒绝 `&mut`；
    ///   - 此方法返回 `()`，不需要写 `return`。
    ///
    /// 示例输入：
    /// ```text
    /// self = Counter { value: 0 }, delta = 3
    /// self = Counter { value: 3 }, delta = 4
    /// ```
    /// 示例输出：
    /// ```text
    /// 两次调用后 self.value = 7
    /// ```
    fn add(&mut self, delta: i32) {
        assessment_harness::todo_exercise(
            "Counter::add",
            "用 &mut self 接收者把 delta 累加到 value 字段上",
            (self.value, delta),
        )
    }

    /// 【待实现】`self`：按值接收，消费自身并把值交出去。
    ///
    /// 实现要求：
    ///   - 用 `self` 接收者把内部的 `value` 返回出去（例如 `self.value`）；
    ///   - `i32` 是 Copy，所以 `self.value` 在按值接收的方法里只是复制；
    ///     方法返回后 `self` 被丢弃，调用方的变量**不能再使用**。
    ///
    /// 示例输入：
    /// ```text
    /// self = Counter { value: 7 }
    /// ```
    /// 示例输出：
    /// ```text
    /// 7
    /// ```
    fn into_value(self) -> i32 {
        assessment_harness::todo_exercise(
            "Counter::into_value",
            "用 self 按值接收者消费自身，返回内部的 value",
            (self.value,),
        )
    }
}

// ===========================================================================
// kp_06_04 关联函数与 Self：Point::new / Point::origin / translate
// ===========================================================================

/// 知识点考核：没有 `self` 的关联函数，以及把 `Self` 当类型别名用。
#[test]
fn kp_06_04_associated_functions() {
    assess(
        M,
        "kp_06_04",
        "关联函数与 Self：用 `类型名::函数名` 调用，不依赖实例",
        Kind::Core,
        "复习 lesson_06 示例 7（associated_functions）：`impl` 块里不带 self 的函数是关联函数，\
         用 `Point::origin()` 调用；返回类型写 `Self` 就是当前类型的别名，等价于写 `Point`。",
        || {
            // 正常用例：关联函数返回的类型与字面量一致（用上了提供的 PartialEq）
            eq(
                Point::origin(),
                Point { x: 0, y: 0 },
                "Point::origin() 必须正好是原点 (0, 0)",
            );
            eq(
                Point::new(3, 4).translate(10, -4),
                Point { x: 13, y: 0 },
                "translate(self, dx, dy) 按值接收自身，返回平移后的新点：(3+10, 4-4)",
            );
            // 边界：平移量为 0，以及从原点出发
            eq(
                Point::new(0, 0).translate(0, 0),
                Point { x: 0, y: 0 },
                "边界：平移 (0, 0) 不应改变任何坐标",
            );
            eq(
                Point::new(-5, -6).translate(5, 6),
                Point { x: 0, y: 0 },
                "边界：负坐标平移后回到原点，说明加减方向没有写反",
            );
        },
    );
}

impl Point {
    /// 【待实现】关联函数：用两个坐标构造点。
    ///
    /// 实现要求：
    ///   - `fn new(x: i32, y: i32) -> Self`，返回 `Point { x, y }`（可用字段初始化简写）；
    ///   - 没有 `self` 参数的函数就是关联函数，只能用 `Point::new(...)` 调用；
    ///     返回类型写 `Self`（当前类型的别名）比写 `Point` 更符合惯例。
    ///
    /// 示例输入：
    /// ```text
    /// x = 3, y = 4
    /// ```
    /// 示例输出：
    /// ```text
    /// Point { x: 3, y: 4 }
    /// ```
    fn new(x: i32, y: i32) -> Self {
        assessment_harness::todo_exercise(
            "Point::new",
            "关联函数：用 (x, y) 构造 Point 并返回 Self",
            (x, y),
        )
    }

    /// 【待实现】关联函数：返回原点。
    ///
    /// 实现要求：
    ///   - `fn origin() -> Self`，返回 `Point { x: 0, y: 0 }`；
    ///   - 可以复用刚刚实现的 `Point::new(0, 0)`，体会「关联函数互调」，
    ///     也可以用结构体字面量直接构造。
    ///
    /// 示例输入：
    /// ```text
    /// （无参数）
    /// ```
    /// 示例输出：
    /// ```text
    /// Point { x: 0, y: 0 }
    /// ```
    fn origin() -> Self {
        assessment_harness::todo_exercise("Point::origin", "关联函数：返回原点 (0, 0)", ())
    }

    /// 【待实现】`self` 方法：平移后返回新点。
    ///
    /// 实现要求：
    ///   - `fn translate(self, dx: i32, dy: i32) -> Self`，
    ///     返回 `Point { x: self.x + dx, y: self.y + dy }`；
    ///   - `i32` 是 Copy，`self.x` 只是复制，所以按值接收也不会丢数据；
    ///     本方法消费旧的 `self`，调用之后的原变量不能再使用（这是「消耗型方法」的惯例）。
    ///
    /// 示例输入：
    /// ```text
    /// self = Point { x: 3, y: 4 }, dx = 10, dy = -4
    /// ```
    /// 示例输出：
    /// ```text
    /// Point { x: 13, y: 0 }
    /// ```
    fn translate(self, dx: i32, dy: i32) -> Self {
        assessment_harness::todo_exercise(
            "Point::translate",
            "按值接收 self，返回平移后的新点 Point { x: self.x + dx, y: self.y + dy }",
            (self.x, self.y, dx, dy),
        )
    }
}

// ===========================================================================
// kp_06_05 元组结构体与单元结构体
// ===========================================================================

/// 知识点考核：元组结构体的 `.0` 访问与单元结构体的零大小值。
#[test]
fn kp_06_05_tuple_and_unit_struct() {
    assess(
        M,
        "kp_06_05",
        "元组结构体（.0 位置访问）与单元结构体（零大小标记）",
        Kind::Basic,
        "复习 lesson_06 示例 4（tuple_struct）与示例 5（unit_struct）：元组结构体字段没有名字，\
         只能用 `.0` / `.1` 按位置访问；单元结构体没有任何字段，值就是类型名本身，占 0 字节。",
        || {
            let (meters, name_len, marker) = exercise_06_05_tuple_and_unit_struct();
            // 正常用例
            eq(meters, 5, "Meters(5) 用 .0 按位置取出内部值 5");
            eq(
                name_len,
                2,
                "Named(String::from(\"ab\")) 的 .0 是 String，len() 得到字节长度 2",
            );
            // 边界：单元结构体不携带任何数据，它所在的分量就是一个零大小的值
            eq(
                marker,
                (),
                "边界：单元结构体的值不携带数据，元组第三个分量就是 ()（零大小）",
            );
        },
    );
}

/// 【待实现】元组结构体与单元结构体。
///
/// 实现要求（全部在函数体内完成）：
///   - 定义元组结构体 `struct Meters(i32);` 与 `struct Named(String);`
///     （字段没有名字，只能用 `.0` / `.1` 按位置访问）；
///   - 定义单元结构体 `struct Marker;`，并真的**构造一个 `Marker` 值**
///     （例如 `let _marker = Marker;`：值就是类型名本身、不写括号，且零大小）；
///   - 单元结构体的值类型是 `Marker` 而不是 `()`，两者不能互换：
///     本题要把 `Marker` 构造出来，而元组的第三个分量按函数签名交 `()`；
///   - `Meters(5).0` 拿到的是 `i32`；`Named(...)` 的 `.0` 是 `String`，
///     它的 `.len()` 返回**字节数**（"ab" 是 2）；
///   - 返回 `(Meters(5).0, Named(String::from("ab")).0.len(), ())`，即 `(5, 2, ())`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (5, 2, ())
/// ```
fn exercise_06_05_tuple_and_unit_struct() -> (i32, usize, ()) {
    assessment_harness::todo_exercise(
        "exercise_06_05_tuple_and_unit_struct",
        "函数内定义元组结构体 Meters(i32) / Named(String) 与单元结构体 Marker，\
         构造 Marker 值，返回 (Meters(5).0, Named(String::from(\"ab\")).0.len(), ())",
        (),
    )
}

// ===========================================================================
// kp_06_06 手动实现 Display：与派生的 Debug 分工不同
// ===========================================================================

/// 知识点考核：`{}` 走自定义 Display，`{:?}` 走派生的 Debug。
#[test]
fn kp_06_06_display_and_debug() {
    assess(
        M,
        "kp_06_06",
        "手动实现 Display（{}）与派生 Debug（{:?}）的分工",
        Kind::Core,
        "复习 lesson_06 示例 8（debug_and_display）：`#[derive(Debug)]` 给的是面向开发者的 `{:?}`，\
         而 `{}` 必须手动 `impl std::fmt::Display`；Display 负责把内部数据翻译成用户语言。",
        || {
            let t = Temperature { celsius: 25.0 };
            // 正常用例：Display 面向用户，带单位与一位小数
            eq(
                format!("{t}"),
                String::from("25.0°C"),
                "Display（{{}}）要输出 \"25.0°C\"：保留一位小数并带上 °C 单位",
            );
            // Debug 由派生得到，带类型名与字段名，是开发期看的调试快照
            eq(
                format!("{t:?}"),
                String::from("Temperature { celsius: 25.0 }"),
                "Debug（{{:?}}）是派生出来的，会打印类型名与字段名（开发者视角）",
            );
            // 边界：0 度与负温度都要保留一位小数
            eq(
                format!("{}", Temperature { celsius: 0.0 }),
                String::from("0.0°C"),
                "边界：0.0 也必须输出一位小数（不能变成 0°C），这正是 {:.1} 的作用",
            );
            eq(
                format!("{}", Temperature { celsius: -3.5 }),
                String::from("-3.5°C"),
                "边界：负温度同样保留一位小数，负号不能被吞掉",
            );
        },
    );
}

impl std::fmt::Display for Temperature {
    /// 【待实现】为 `Temperature` 实现 `Display`。
    ///
    /// 实现要求：
    ///   - 用 `write!(f, "{:.1}°C", self.celsius)` 输出；`{:.1}` 是「保留一位小数」的格式
    ///     说明符，写在格式串里而不是调用处；
    ///   - 因此 25.0 → `25.0°C`、0.0 → `0.0°C`、-3.5 → `-3.5°C`（都保留一位小数）；
    ///   - `{}` 找 Display、`{:?}` 找 Debug：Debug 已由 `#[derive(Debug)]` 提供，
    ///     Display 必须自己实现，否则 `format!("{t}")` 会报 `error[E0277]`。
    ///
    /// 示例输入：
    /// ```text
    /// self = Temperature { celsius: 25.0 }
    /// ```
    /// 示例输出：
    /// ```text
    /// "25.0°C"
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assessment_harness::todo_exercise(
            "Temperature::fmt",
            "实现 Display：用 write!(f, \"{:.1}°C\", self.celsius) 输出，例如 25.0 -> 25.0°C",
            (self.celsius, f),
        )
    }
}

// ===========================================================================
// kp_06_07 链式调用：消费 self 并返回 Self 构造器
// ===========================================================================

/// 知识点考核：`new(...).limit(n).build()` 的链式调用。
#[test]
fn kp_06_07_chained_builder() {
    assess(
        M,
        "kp_06_07",
        "链式调用：构造器方法消费 self 并返回 Self",
        Kind::Hard,
        "复习 lesson_06 示例 6 的消耗型方法（`self` 接收者）与示例 7 的构造函数惯例：\
         链式调用的每一步都要把 `self` 交出去（返回 `Self`），最后一步 `build` 才产出结果；\
         中间任何一步返回 `&mut Self` 或 `()` 都会让链断掉。",
        || {
            // 正常用例：完整链式调用
            eq(
                QueryBuilder::new("users").limit(10).build(),
                String::from("SELECT * FROM users LIMIT 10"),
                "new -> limit -> build 应产出 \"SELECT * FROM users LIMIT 10\"",
            );
            eq(
                QueryBuilder::new("users").build(),
                String::from("SELECT * FROM users"),
                "没调用 limit 时不能输出 LIMIT 子句（不是 LIMIT 0）",
            );
            // 边界：显式设置 limit 为 0，与「没设置」是两回事
            eq(
                QueryBuilder::new("orders").limit(0).build(),
                String::from("SELECT * FROM orders LIMIT 0"),
                "边界：limit(0) 是「显式设置了 0」，与 None（没设置）必须区别对待",
            );
            // 边界：u32 极值不能被截断
            eq(
                QueryBuilder::new("logs").limit(u32::MAX).build(),
                String::from("SELECT * FROM logs LIMIT 4294967295"),
                "边界：u32::MAX 要完整打印出来（4294967295），不能截断或溢出",
            );
        },
    );
}

impl QueryBuilder {
    /// 【待实现】构造函数：用表名创建构造器，初始没有 LIMIT。
    ///
    /// 实现要求：
    ///   - `fn new(table: &str) -> Self`，返回
    ///     `QueryBuilder { table: table.to_string(), limit: None }`；
    ///   - 入参是 `&str`、字段是 `String`，所以必须 `.to_string()`
    ///     （或 `String::from(table)`）做一次复制；
    ///   - `limit` 用 `None` 表示「调用方没有设置 LIMIT」，这与 `Some(0)`（显式设置 0）
    ///     是两回事。
    ///
    /// 示例输入：
    /// ```text
    /// table = "users"
    /// ```
    /// 示例输出：
    /// ```text
    /// QueryBuilder { table: "users", limit: None }
    /// ```
    fn new(table: &str) -> Self {
        assessment_harness::todo_exercise(
            "QueryBuilder::new",
            "用 table.to_string() 与 limit: None 构造 QueryBuilder 并返回 Self",
            (table,),
        )
    }

    /// 【待实现】设置 LIMIT：消费自身并返回新的构造器，从而支持链式调用。
    ///
    /// 实现要求：
    ///   - `fn limit(self, n: u32) -> Self`，返回
    ///     `QueryBuilder { limit: Some(n), ..self }`（表名保持不变）；
    ///   - 返回类型必须是 `Self` 而不是 `()`，否则
    ///     `QueryBuilder::new(..).limit(..).build()` 无法编译；
    ///   - `Some(n)` 与 `None` 的区别就是「设了 0」与「没设置」的区别。
    ///
    /// 示例输入：
    /// ```text
    /// self = QueryBuilder { table: "users", limit: None }, n = 10
    /// ```
    /// 示例输出：
    /// ```text
    /// QueryBuilder { table: "users", limit: Some(10) }
    /// ```
    fn limit(self, n: u32) -> Self {
        assessment_harness::todo_exercise(
            "QueryBuilder::limit",
            "按值接收 self，返回 limit 为 Some(n)、table 不变的新 QueryBuilder",
            (self.table, self.limit, n),
        )
    }

    /// 【待实现】生成 SQL 文本。
    ///
    /// 实现要求：
    ///   - `fn build(self) -> String`，按下面两种形态输出：
    ///     `limit` 是 `Some(n)` → `format!("SELECT * FROM {} LIMIT {}", self.table, n)`；
    ///     `limit` 是 `None` → `format!("SELECT * FROM {}", self.table)`；
    ///   - 用 `match self.limit` 或 `if let Some(n) = self.limit` 区分两种情况；
    ///   - `None` 时**整段** LIMIT 子句都不要输出（不能写成 `LIMIT 0`）。
    ///
    /// 示例输入：
    /// ```text
    /// self = QueryBuilder { table: "users", limit: Some(10) }
    /// ```
    /// 示例输出：
    /// ```text
    /// "SELECT * FROM users LIMIT 10"
    /// ```
    fn build(self) -> String {
        assessment_harness::todo_exercise(
            "QueryBuilder::build",
            "按 limit 是 Some(n) / None 分别输出 \"SELECT * FROM {table} LIMIT {n}\" 与 \
             \"SELECT * FROM {table}\"",
            (self.table, self.limit),
        )
    }
}

// ===========================================================================
// kp_06_08 部分移动：把 String 字段 move 出来，同时读 Copy 字段
// ===========================================================================

/// 知识点考核：结构体的「部分移动」。
#[test]
fn kp_06_08_partial_move() {
    assess(
        M,
        "kp_06_08",
        "部分移动：move 出 String 字段的同时仍可读取 Copy 字段",
        Kind::Edge,
        "复习 lesson_06 示例 3 与示例 10 的错误 7：从结构体里把非 Copy 字段（String）move 出来之后，\
         该实例不能再用作整体，但**没被移走**的字段（如 u32 的 id）仍然可以读。",
        || {
            // 正常用例
            let (id, note) = exercise_06_08_partial_move(Order::new(7, "加急"));
            eq(id, 7u32, "id 是 Copy，读它只是复制，返回的仍是 7");
            eq(
                note,
                String::from("加急"),
                "note 是 String，必须被原样 move 出来（内容不变、不发生复制开销）",
            );
            // 边界：空备注 + id 为 0
            let (zero_id, empty_note) = exercise_06_08_partial_move(Order::new(0, ""));
            eq(
                zero_id,
                0u32,
                "边界：id = 0 时部分移动仍然可用（id 是 Copy，不受 note 被 move 的影响）",
            );
            eq(
                empty_note,
                String::new(),
                "边界：空备注也要原样搬出来，不能被替换成占位文本",
            );
        },
    );
}

/// 【待实现】部分移动：拆开订单，把两个字段分别交出去。
///
/// 实现要求：
///   - 入参 `order` 是**按值**传入的（所有权已经在这个函数里）；
///   - 把 `order.note`（String，非 Copy）直接 move 出来；
///   - 同时读取 `order.id`（u32，Copy，读取只是复制）；
///   - 返回 `(id, note)`；
///   - 这就是「部分移动」：可以写 `let id = order.id; let note = order.note;`，
///     也可以直接解构 `let Order { id, note } = order;`；`note` 被移走之后 `order`
///     不能再整体使用（会报 `error[E0382]`），但 `order.id` 依然可读。
///
/// 示例输入：
/// ```text
/// order = Order { id: 7, note: String::from("加急") }
/// ```
/// 示例输出：
/// ```text
/// (7, "加急")
/// ```
fn exercise_06_08_partial_move(order: Order) -> (u32, String) {
    assessment_harness::todo_exercise(
        "exercise_06_08_partial_move",
        "把非 Copy 的 order.note 移动出来、同时读取 Copy 的 order.id，返回 (id, note)",
        (order.id, order.note),
    )
}
