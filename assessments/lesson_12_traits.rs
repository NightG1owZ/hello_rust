//! assessments/lesson_12_traits.rs —— 考核：Trait（对应 lesson_12）
//!
//! - 对应课程：`src/tutorial/lesson_12_traits.rs`
//! - 知识点出处：`src/tutorial/README.md` 第四阶段「12 Trait」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_12_traits     # 只考这一课
//!   cargo test                             # 考全部 18 课
//!   cargo run --bin assessment_report      # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_12_xx_xxx` 练习函数（kp_12_01 / kp_12_08 是 `impl` 块里的
//!    方法），它的函数体里只有一行 `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_12_traits`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 上面的「提供给你的类型与 trait」小节**不要修改**：里面已经放好了考核要用的类型，
//!   你只需要为它们补上 `impl`（kp_12_01、kp_12_08）；
//! - 只修改 `exercise_*` 练习函数与对应 `impl` 块里方法的函数体，可以按需增加局部类型、
//!   局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 13）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | 把没实现 trait 的类型传进约束位置 | `error[E0277]` the trait bound `Plain: Summary` is not satisfied | 为它写 `impl Summary for Plain { ... }` |
//! | `impl Summary for Report {}` 漏写必需方法 | `error[E0046]` not all trait items implemented, missing: `summarize` | 补上必需方法，或在 trait 里给它一个默认实现 |
//! | 同一类型写两份 `impl Summary for Report` | `error[E0119]` conflicting implementations of trait | 同一个 trait 对同一类型只能有一份 `impl` |
//! | trait 含泛型方法 / `-> Self` 却想做成 `dyn` | `error[E0038]` the trait `ObjectSafe` is not dyn compatible | 改成具体参数类型，或把 `-> Self` 改成 `-> Box<dyn Trait>` |
//! | `Box<dyn Container>` 没写出关联类型 | `error[E0191]` the value of the associated type `Item` must be specified | 写成 `Box<dyn Container<Item = i32>>` |
//! | `-> impl Display` 的两个分支返回不同类型 | `error[E0308]` `if` and `else` have incompatible types | 返回 `Box<dyn Display>` |
//! | 泛型参数版本 trait 的调用推不出类型 | `error[E0283]` type annotations needed | 显式标注（`let x: String = temp.convert_to();`），或改用关联类型 |

use assessment_harness::{Kind, approx, assess, eq, eq_slice, is_false, is_true};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_12";

// ===========================================================================
// ===== 提供给你的类型与 trait（不要修改）=====
// ===========================================================================
//
// 这些类型与 trait 是考核的「地基」：
//   * kp_12_01 / kp_12_08 需要你为它们补上 `impl`（本文件中已经放好了空的骨架 impl）；
//   * kp_12_05 / kp_12_06 / kp_12_09 / kp_12_10 直接使用它们。
// 请不要改动字段名、字段类型或 trait 的方法签名，否则后面的练习与测试都无法编译。

/// 可计算面积的图形（由学员为下面两个结构体实现）。
trait Area {
    fn area(&self) -> f64;
}

/// 圆形：只保存半径。
#[derive(Debug)]
struct Circle {
    radius: f64,
}

/// 正方形：只保存边长。
#[derive(Debug)]
struct Square {
    side: f64,
}

/// 生产者（关联类型由学员在 impl 中指定）。
trait Producer {
    type Item;

    fn produce(&self) -> Self::Item;
}

/// 简单的计数器生产者：每次产出 `limit`。
struct CounterProducer {
    limit: u32,
}

/// Debug / Clone / PartialEq / Default 已派生，供 kp_12_10 使用。
#[derive(Debug, Clone, PartialEq, Default)]
struct Version {
    major: u32,
    minor: u32,
}

// ===========================================================================
// kp_12_01 为自定义类型实现 trait：impl Area for Circle / Square
// ===========================================================================

/// 知识点考核：给两个结构体实现同一个 trait，用统一的 `area()` 调用。
#[test]
fn kp_12_01_impl_area_for_shapes() {
    assess(
        M,
        "kp_12_01",
        "为自定义类型实现 trait：impl Area for Circle 与 impl Area for Square",
        Kind::Core,
        "复习 lesson_12 示例 1（定义 trait 并为自定义类型实现）与示例 10（Shape 的 area 实现）：\
         trait 只声明「能做什么」，`impl Trait for 类型 { ... }` 才给出「怎么做」；\
         圆的面积是 πr²（π 用 `std::f64::consts::PI`），正方形是边长²。",
        || {
            let circle = Circle { radius: 1.0 };
            let square = Square { side: 3.0 };
            // 读一次字段：确认测试夹具本身正确，同时让 radius / side 在骨架态不触发 dead_code
            is_true(
                circle.radius == 1.0 && square.side == 3.0,
                "两个图形的尺寸必须原样保存在字段里（圆半径 1.0、正方形边长 3.0）",
            );
            approx(
                circle.area(),
                std::f64::consts::PI,
                1e-9,
                "半径 1.0 的圆面积 = π ≈ 3.14159265…（浮点比较用近似断言）",
            );
            approx(
                square.area(),
                9.0,
                1e-9,
                "边长 3.0 的正方形面积 = 3.0 * 3.0 = 9.0",
            );
            // 边界：退化图形（半径为 0）的面积必须是 0.0，而不是 NaN
            approx(
                Circle { radius: 0.0 }.area(),
                0.0,
                1e-9,
                "边界：半径 0 的圆面积是 0.0（公式里没有除法，不应该出现 NaN）",
            );
        },
    );
}

/// 【待实现】为 `Circle` 实现 `Area`（kp_12_01 的第一个 impl）。
///
/// 实现要求：
///   - 只实现必需方法 `fn area(&self) -> f64`；
///   - 圆面积 = π * r * r，π 用标准库常量 `std::f64::consts::PI`；
///   - 读半径请用 `self.radius`，不要改动结构体定义；
///   - `trait` 只声明能力，`impl Trait for 类型` 才给出实现：漏写 impl 时，调用
///     `circle.area()` 会报 `error[E0599]: no method named `area` found`。
///
/// 示例输入：
/// ```text
/// radius = 1.0
/// ```
/// 示例输出：
/// ```text
/// 3.141592653589793
/// ```
impl Area for Circle {
    fn area(&self) -> f64 {
        assessment_harness::todo_exercise(
            "impl Area for Circle 的 area()",
            "返回 std::f64::consts::PI * self.radius * self.radius",
            (&self.radius,),
        )
    }
}

/// 【待实现】为 `Square` 实现 `Area`（kp_12_01 的第二个 impl）。
///
/// 实现要求：
///   - 只实现必需方法 `fn area(&self) -> f64`；
///   - 正方形面积 = side * side，读边长请用 `self.side`；
///   - 同一个 trait 换一个类型实现一次即可，方法名与签名必须与 trait 完全一致，
///     写错签名（例如漏了 `&self`）会报 `error[E0053]`。
///
/// 示例输入：
/// ```text
/// side = 3.0
/// ```
/// 示例输出：
/// ```text
/// 9.0
/// ```
impl Area for Square {
    fn area(&self) -> f64 {
        assessment_harness::todo_exercise(
            "impl Area for Square 的 area()",
            "返回 self.side * self.side",
            (&self.side,),
        )
    }
}

// ===========================================================================
// kp_12_02 默认方法：实现者只写必需方法即可获得默认行为
// ===========================================================================

/// 知识点考核：在函数内定义带默认方法的 trait，并只实现必需方法。
#[test]
fn kp_12_02_default_method() {
    assess(
        M,
        "kp_12_02",
        "默认方法：只实现必需方法，就能得到 shout() 的默认行为",
        Kind::Core,
        "复习 lesson_12 示例 2（默认方法与覆盖）：trait 里带方法体的方法就是默认方法，\
         实现者不写也能用；默认方法内部可以调用必需方法，例如 \
         `fn shout(&self) -> String { format!(\"{}!\", self.summarize()) }`。",
        || {
            let text = exercise_12_02_default_method();
            // 边界：默认方法只负责「拼一次 + 末尾加一个感叹号」，多一个空格或感叹号都会失败
            is_true(
                text.starts_with("Rust 学习笔记")
                    && text.ends_with('!')
                    && text.matches('!').count() == 1,
                "边界：shout() 的默认实现是 format!(\"{}!\", self.summarize())，\
                 必须原样以 summarize() 的结果开头，且末尾只加**一个**感叹号",
            );
            // 正常用例：完整字符串
            eq(
                text,
                String::from("Rust 学习笔记!"),
                "必需方法 summarize() 返回「Rust 学习笔记」，默认方法 shout() 在它后面补一个感叹号",
            );
        },
    );
}

/// 【待实现】在函数体内定义 trait、类型与 impl，体会「默认方法」。
///
/// 实现要求（全部在函数体内完成）：
///   - 定义 trait：`trait Summary { fn summarize(&self) -> String; fn shout(&self) -> String
///     { format!("{}!", self.summarize()) } }`（`summarize` 是必需方法，`shout` 是默认方法）；
///   - 定义一个本地类型（例如 `struct Note;` 或 `struct Note { title: &'static str }`）
///     并写 `impl Summary for Note`，只实现必需方法 `summarize()`，
///     让它返回 `String::from("Rust 学习笔记")`（想用字段就用 `format!("{} 学习笔记", self.title)`，
///     并让 `title` 取 `"Rust"`）；
///   - **不要**重写 `shout()`（本题就要考「默认方法不重写也能用」）；
///   - 构造该类型的实例并返回 `note.shout()` 的结果，期望 `"Rust 学习笔记!"`；
///   - 默认方法只能被「不实现它」的实现者继承，默认方法内部可以自由调用必需方法
///     （`self.summarize()`）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// "Rust 学习笔记!"
/// ```
fn exercise_12_02_default_method() -> String {
    assessment_harness::todo_exercise(
        "exercise_12_02_default_method",
        "在函数内定义带默认方法 shout() 的 trait Summary 与本地类型，只实现必需方法 summarize()，\
         返回 note.shout()，即 \"Rust 学习笔记!\"",
        (),
    )
}

// ===========================================================================
// kp_12_03 trait 作为参数：impl Trait 写法
// ===========================================================================

/// 知识点考核：`item: &impl Area` 接收任意实现了 Area 的类型。
#[test]
fn kp_12_03_impl_trait_arg() {
    assess(
        M,
        "kp_12_03",
        "trait 作为参数（写法一）：item: &impl Area 接收任意实现者",
        Kind::Core,
        "复习 lesson_12 示例 4（trait 作为参数：impl Trait）：`&impl Area` 是\
         「一个匿名类型参数 + Area 约束」的语法糖，同样是静态分发（单态化）。",
        || {
            approx(
                exercise_12_03_impl_trait_arg(&Circle { radius: 1.0 }),
                2.0 * std::f64::consts::PI,
                1e-9,
                "半径 1.0 的圆面积是 π，题目要求返回面积 × 2.0，即 2π ≈ 6.2831853…",
            );
            // 正常用例（第二种具体类型）：同一个函数也能接收 Square
            approx(
                exercise_12_03_impl_trait_arg(&Square { side: 3.0 }),
                18.0,
                1e-9,
                "边长 3.0 的正方形面积 9.0，×2.0 得 18.0（impl Trait 参数对每种具体类型各生成一份代码）",
            );
            // 边界：面积为 0 的退化图形
            approx(
                exercise_12_03_impl_trait_arg(&Square { side: 0.0 }),
                0.0,
                1e-9,
                "边界：边长为 0 时面积是 0.0，乘 2.0 之后仍然是 0.0",
            );
        },
    );
}

/// 【待实现】用 `impl Trait` 作为参数接收图形。
///
/// 实现要求：
///   - 参数写成 `item: &impl Area`（把 `impl Trait` 直接写在参数类型的位置上）；
///   - 返回 `item.area() * 2.0`；
///   - `impl Trait` 参数**没有名字**，函数体里无法用类型名指代它，调用处也不能写
///     `exercise_12_03_impl_trait_arg::<Circle>(...)` 这类 turbofish；需要显式指定类型参数时，
///     改用 kp_12_04 的泛型写法。
///
/// 示例输入：
/// ```text
/// item = Square { side: 3.0 }
/// ```
/// 示例输出：
/// ```text
/// 18.0
/// ```
fn exercise_12_03_impl_trait_arg(item: &impl Area) -> f64 {
    assessment_harness::todo_exercise(
        "exercise_12_03_impl_trait_arg",
        "返回 item.area() * 2.0（参数写成 &impl Area）",
        (item,),
    )
}

// ===========================================================================
// kp_12_04 trait 作为参数：泛型参数 + trait bound
// ===========================================================================

/// 知识点考核：`<T: Area>` 泛型写法，并演示 turbofish 显式指定类型。
#[test]
fn kp_12_04_generic_bound() {
    assess(
        M,
        "kp_12_04",
        "trait 作为参数（写法二）：泛型参数 + trait bound，可 turbofish",
        Kind::Core,
        "复习 lesson_12 示例 3（trait 作为参数：泛型 + trait bound）：\
         `fn f<T: Area>(item: &T)` 与 `fn f(item: &impl Area)` 都是静态分发；\
         区别是泛型参数**有名字**，调用处可以写 `f::<Circle>(&c)` 显式指定类型，\
         函数体里也能用 `T` 做别的事。",
        || {
            approx(
                exercise_12_04_generic_bound(&Circle { radius: 2.0 }),
                8.0 * std::f64::consts::PI,
                1e-9,
                "半径 2.0 的圆面积是 4π，×2.0 得 8π ≈ 25.1327…",
            );
            // 正常用例：泛型写法允许 turbofish 显式指定类型参数（impl Trait 参数做不到）
            approx(
                exercise_12_04_generic_bound::<Square>(&Square { side: 1.5 }),
                4.5,
                1e-9,
                "边长 1.5 的正方形面积 2.25，×2.0 得 4.5；`::<Square>` 是泛型参数独有的能力",
            );
            // 边界：面积为 0
            approx(
                exercise_12_04_generic_bound(&Square { side: 0.0 }),
                0.0,
                1e-9,
                "边界：边长为 0 时面积是 0.0，乘 2.0 仍是 0.0",
            );
        },
    );
}

/// 【待实现】用泛型参数 + trait bound 接收图形。
///
/// 实现要求：
///   - 签名写成 `fn exercise_12_04_generic_bound<T: Area>(item: &T) -> f64`；
///   - 返回 `item.area() * 2.0`；
///   - 与 kp_12_03 的 `impl Trait` 写法**功能等价**，取舍是：`impl Trait` 更短、适合「只接收」；
///     泛型参数有名字，能 turbofish、能写进结构体字段、能调用 `T::` 上的关联函数。两者都是
///     静态分发，想要「一个容器放多种类型」必须用 `dyn`。
///
/// 示例输入：
/// ```text
/// item = Square { side: 1.5 }
/// ```
/// 示例输出：
/// ```text
/// 4.5
/// ```
fn exercise_12_04_generic_bound<T: Area>(item: &T) -> f64 {
    assessment_harness::todo_exercise(
        "exercise_12_04_generic_bound",
        "返回 item.area() * 2.0（泛型参数 T: Area）",
        (item,),
    )
}

// ===========================================================================
// kp_12_05 返回 impl Trait：把具体类型藏起来
// ===========================================================================

/// 知识点考核：返回 `impl Area`，调用方只能使用 trait 的方法。
#[test]
fn kp_12_05_return_impl_trait() {
    assess(
        M,
        "kp_12_05",
        "返回 impl Trait：调用方只依赖 Area，看不到具体类型",
        Kind::Core,
        "复习 lesson_12 示例 6（返回 impl Trait）：`-> impl Area` 表示「返回某个实现了 Area 的\
         具体类型，但调用方不需要知道它是谁」；代价是所有 return 分支必须是同一个具体类型。",
        || {
            let shape = exercise_12_05_return_impl_trait(2.0);
            approx(
                shape.area(),
                4.0,
                1e-9,
                "边长 2.0 的方形面积是 4.0（背后是 Square，但这里只能调用 Area 的方法）",
            );
            // 边界：边长 0 的退化图形
            approx(
                exercise_12_05_return_impl_trait(0.0).area(),
                0.0,
                1e-9,
                "边界：边长为 0 时面积是 0.0，不能 panic、也不能返回 NaN",
            );
        },
    );
}

/// 【待实现】返回 `impl Area`（隐藏具体类型）。
///
/// 实现要求：
///   - 函数体返回 `Square { side }`：即 `fn exercise_12_05_return_impl_trait(side: f64)
///     -> impl Area { Square { side } }`；
///   - 不要改返回类型（测试只能看到 `Area` 的能力，这正是本题的考点）；
///   - `-> impl Trait` 的返回值虽然「看不见类型」，但它仍然是**静态分发**、零开销；
///     两个分支要返回不同类型（例如 Circle 与 Square）时会报
///     `error[E0308]: `if` and `else` have incompatible types`，那时必须改用 kp_12_06 的
///     `Box<dyn Area>`。
///
/// 示例输入：
/// ```text
/// side = 2.0
/// ```
/// 示例输出：
/// ```text
/// area() 返回 4.0
/// ```
fn exercise_12_05_return_impl_trait(side: f64) -> impl Area {
    // 骨架态说明：`-> impl Area` 要求编译器推断出一个**具体**的隐藏类型，而 `todo_exercise`
    // 的返回类型是 `!`；如果把占位调用直接当返回值，隐藏类型会被推断成 `!` 并报
    // `error[E0277]: the trait bound `!: Area` is not satisfied`。
    // 所以这里用一个返回 `Square`（kp_12_01 已经为它实现了 Area）的内层函数把类型钉住。
    // 学员实现时：删掉内层函数与下面那行调用，直接写 `Square { side }` 即可。
    fn placeholder(side: f64) -> Square {
        assessment_harness::todo_exercise(
            "exercise_12_05_return_impl_trait",
            "返回 Square { side }，用 impl Area 隐藏具体类型",
            (side,),
        )
    }
    placeholder(side)
}

// ===========================================================================
// kp_12_06 返回 Box<dyn Area>：返回类型不唯一时必须用 dyn
// ===========================================================================

/// 知识点考核：按运行期数据（`kind`）返回不同的具体类型。
#[test]
fn kp_12_06_box_dyn() {
    assess(
        M,
        "kp_12_06",
        "返回 Box<dyn Area>：分支返回不同类型时必须用 trait 对象",
        Kind::Hard,
        "复习 lesson_12 示例 7（返回 Box<dyn Trait>）：`dyn Trait` 的大小在编译期未知，\
         必须装在指针后面（`Box<dyn Trait>` / `&dyn Trait`）；\
         分支各自返回不同类型的唯一办法就是先统一装箱成同一种 `dyn` 类型。",
        || {
            let circle = exercise_12_06_box_dyn("circle", 1.0);
            approx(
                circle.area(),
                3.141_592_653_589_793,
                1e-9,
                "kind 为 \"circle\" 时返回 Box<dyn Area> 里的 Circle（半径 1.0 → 面积 π）",
            );
            // 正常用例：另一种 kind 走 `_` 分支
            let square = exercise_12_06_box_dyn("square", 3.0);
            approx(
                square.area(),
                9.0,
                1e-9,
                "其它 kind（这里是 \"square\"）一律走 `_` 分支返回 Square（边长 3.0 → 面积 9.0）",
            );
            // 边界：空字符串 kind 与 0 尺寸
            approx(
                exercise_12_06_box_dyn("", 0.0).area(),
                0.0,
                1e-9,
                "边界：kind 是空串时同样走 `_` 分支；尺寸为 0 时面积是 0.0，不应 panic",
            );
        },
    );
}

/// 【待实现】按 `kind` 返回不同的具体类型（装箱成 `Box<dyn Area>`）。
///
/// 实现要求：
///   - 返回类型固定为 `Box<dyn Area>`；
///   - `kind == "circle"` 时返回 `Box::new(Circle { radius: size })`；
///   - 其它任何 `kind`（包括空串）都返回 `Box::new(Square { side: size })`；
///   - 这里不能写 `-> impl Area`：两个分支返回的是**不同**的具体类型，`impl Trait` 只能代表
///     一种类型，会报 `error[E0308]: `if` and `else` have incompatible types`；`dyn Area`
///     把类型擦掉，运行期通过虚表查表调用 `area()`（动态分发），因为大小在编译期未知，
///     必须装在 `Box` 或引用后面。
///
/// 示例输入：
/// ```text
/// kind = "circle", size = 1.0
/// ```
/// 示例输出：
/// ```text
/// area() 返回 3.141592653589793
/// ```
fn exercise_12_06_box_dyn(kind: &str, size: f64) -> Box<dyn Area> {
    assessment_harness::todo_exercise(
        "exercise_12_06_box_dyn",
        "kind == \"circle\" 时返回 Box::new(Circle { radius: size })，其它一律返回 \
         Box::new(Square { side: size })",
        (kind, size),
    )
}

// ===========================================================================
// kp_12_07 多个 trait bound：Area + Debug
// ===========================================================================

/// 知识点考核：用 `+` 组合多个约束，打印 Debug 形式并带上面积。
#[test]
fn kp_12_07_multiple_bounds() {
    assess(
        M,
        "kp_12_07",
        "多个 trait bound：T: Area + Debug，既能算面积又能 {:?} 打印",
        Kind::Core,
        "复习 lesson_12 示例 5（trait bound 组合与 +）：约束用 `+` 叠加，\
         `T: Area + std::fmt::Debug` 表示「既要能算面积，又要有派生的 Debug 实现」；\
         约束叠得越多，能进来的类型越少，所以要按需叠加。",
        || {
            eq(
                exercise_12_07_multiple_bounds(&Circle { radius: 3.0 }),
                String::from("Circle { radius: 3.0 } 的面积是 28.27"),
                "Debug 输出 `Circle { radius: 3.0 }` + 面积 π*9 ≈ 28.2743…，按 {:.2} 保留两位小数得 28.27",
            );
            // 正常用例：换一种具体类型
            eq(
                exercise_12_07_multiple_bounds(&Square { side: 2.5 }),
                String::from("Square { side: 2.5 } 的面积是 6.25"),
                "Debug 输出 `Square { side: 2.5 }` + 面积 6.25（{:.2} 正好两位）",
            );
            // 边界：四舍五入的临界值 —— 0.7853981… 必须进位成 0.79
            eq(
                exercise_12_07_multiple_bounds(&Circle { radius: 0.5 }),
                String::from("Circle { radius: 0.5 } 的面积是 0.79"),
                "边界：π*0.25 = 0.7853…，{:.2} 要四舍五入成 0.79（截断成 0.78 就是错的）",
            );
        },
    );
}

/// 【待实现】组合 `Area` 与 `Debug` 两个约束。
///
/// 实现要求：
///   - 签名写成 `fn exercise_12_07_multiple_bounds<T: Area + std::fmt::Debug>(item: &T) -> String`；
///   - 返回 `format!("{item:?} 的面积是 {:.2}", item.area())`（注意 `{item:?}` 与 `{:.2}` 的位置参数）；
///   - 约束用 `+` 叠加：`Circle` / `Square` 已经 `#[derive(Debug)]`，直接就能用 `{:?}` 打印；
///     只写 `T: Area` 时 `{item:?}` 会报
///     `error[E0277]: `T` doesn't implement `std::fmt::Debug``。
///
/// 示例输入：
/// ```text
/// item = Circle { radius: 3.0 }
/// ```
/// 示例输出：
/// ```text
/// "Circle { radius: 3.0 } 的面积是 28.27"
/// ```
fn exercise_12_07_multiple_bounds<T: Area + std::fmt::Debug>(item: &T) -> String {
    assessment_harness::todo_exercise(
        "exercise_12_07_multiple_bounds",
        "返回 format!(\"{item:?} 的面积是 {:.2}\", item.area())",
        (item,),
    )
}

// ===========================================================================
// kp_12_08 关联类型：Producer::Item 由实现者一次性确定
// ===========================================================================

/// 知识点考核：实现带关联类型的 trait，并比较「关联类型 vs 泛型参数」。
#[test]
fn kp_12_08_associated_type() {
    assess(
        M,
        "kp_12_08",
        "关联类型：type Item = u32，调用处不需要写类型标注",
        Kind::Hard,
        "复习 lesson_12 示例 8（关联类型 vs 泛型参数）：`type Item = u32;` 表达\
         「这个生产者天生只产出一种类型」，所以调用处不用标注；\
         泛型参数的版本（`ConvertTo<T>`）允许同一类型有多份实现，但调用处必须标注，\
         否则报 `error[E0283]: type annotations needed`。",
        || {
            // 构造一次提供的生产者并读字段：既确认 limit 的语义，也让 CounterProducer 的字段不触发 dead_code
            let producer = CounterProducer { limit: 4 };
            eq(
                producer.limit,
                4u32,
                "limit 就是 produce() 要产出的那个值（这个实例是 4；练习函数里固定用 5）",
            );
            let produced = exercise_12_08_associated_type();
            eq_slice(
                &produced,
                &[5u32, 5, 5],
                "连续调用 3 次 produce()，每次都应产出 CounterProducer 的 limit（实现要求里固定为 5）",
            );
            // 边界：关联类型让首元素的类型是确定的，取第一个元素不需要任何标注
            eq(
                produced.first().copied(),
                Some(5u32),
                "边界：`Producer::Item` 已由 impl 确定为 u32，所以 `first()` 拿到的就是 u32，无需 turbofish",
            );
            // 用完全限定语法直接调用 trait 方法：这条断言同时让 `Producer` 这个 trait
            // 在骨架态就被真正使用（否则 rustc 会报 `trait `Producer` is never used`）
            eq(
                Producer::produce(&producer),
                4u32,
                "Producer::produce 的返回值类型由关联类型 Item 决定（u32），它应当原样返回 limit=4",
            );
        },
    );
}

/// 【待实现】实现带关联类型的 `Producer`（kp_12_08 的 impl）。
///
/// 实现要求：
///   - 关联类型固定为 `type Item = u32;`（这一行骨架态已经写好：删掉它 impl 就不完整，
///     会报 `error[E0046]: not all trait items implemented, missing: `Item``）；
///   - 实现 `fn produce(&self) -> u32`，返回 `self.limit`；
///   - 关联类型是「实现者一次性确定」的类型占位符，因此 `Producer::Item` 在调用处不需要任何
///     标注；反过来，如果你希望同一个类型能产出**多种**类型，就必须把 trait 写成泛型参数版本
///     `trait Producer<T> { fn produce(&self) -> T; }`，代价是调用时要标注（或用 turbofish）
///     指出是哪一个实现。
///
/// 示例输入：
/// ```text
/// limit = 4
/// ```
/// 示例输出：
/// ```text
/// 4
/// ```
impl Producer for CounterProducer {
    type Item = u32;

    fn produce(&self) -> u32 {
        assessment_harness::todo_exercise(
            "impl Producer for CounterProducer 的 produce()",
            "返回 self.limit（关联类型 Item = u32）",
            (&self.limit,),
        )
    }
}

/// 【待实现】用实现好的 `Producer` 产出几个值。
///
/// 实现要求：
///   - 在函数内构造 `CounterProducer { limit: 5 }`；
///   - 连续调用 3 次 `produce()`，返回 `vec![5, 5, 5]`
///     （例如 `let value = producer.produce(); vec![value, value, value]`）；
///   - `produce()` 每次只产出一个值，所以「3 个元素」必须来自 3 次调用；调用处不需要写
///     `produce::<u32>()` 这类标注 —— 这正是关联类型的好处。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// [5, 5, 5]
/// ```
fn exercise_12_08_associated_type() -> Vec<u32> {
    assessment_harness::todo_exercise(
        "exercise_12_08_associated_type",
        "构造 CounterProducer { limit: 5 }，调用 3 次 produce()，返回 vec![5, 5, 5]",
        (),
    )
}

// ===========================================================================
// kp_12_09 dyn 集合：一个 Vec 装多种图形并按面积排序
// ===========================================================================

/// 知识点考核：用 `Vec<Box<dyn Area>>` 装不同具体类型，按面积升序返回。
#[test]
fn kp_12_09_dyn_collection() {
    assess(
        M,
        "kp_12_09",
        "dyn trait 对象集合：Vec<Box<dyn Area>> 排序后返回面积列表",
        Kind::Hard,
        "复习 lesson_12 示例 9 与示例 10（dyn trait 对象、按面积排序）：\
         `Vec<Box<dyn Area>>` 能同时装 Circle / Square / 你自定义的 Rectangle，\
         排序用 `sort_by(|a, b| a.area().total_cmp(&b.area()))`（f64 用 total_cmp 得到全序，\
         避免 partial_cmp 返回 None）。",
        || {
            let areas = exercise_12_09_dyn_collection();
            eq(
                areas.len(),
                3usize,
                "集合里共有 3 个图形，返回的面积也应有 3 个",
            );
            approx(
                areas[0],
                3.141_592_653_589_793,
                1e-9,
                "面积最小的排在最前：半径 1.0 的圆（π ≈ 3.14…）",
            );
            approx(
                areas[1],
                6.25,
                1e-9,
                "第二小：边长 2.5 的正方形（2.5 * 2.5 = 6.25）",
            );
            approx(
                areas[2],
                12.0,
                1e-9,
                "最大：3.0 × 4.0 的长方形（12.0）—— 说明排序真的生效了，不是原样返回",
            );
            // 边界：集合里只有一个图形时，排序逻辑同样要成立（首项即末项）
            let mut single: Vec<Box<dyn Area>> = vec![Box::new(Square { side: 2.0 })];
            single.sort_by(|left, right| left.area().total_cmp(&right.area()));
            approx(
                single[0].area(),
                4.0,
                1e-9,
                "边界：只装一个图形的 dyn 集合排序后仍是它自己（边长 2.0 → 面积 4.0），\
                 排序代码不能假设元素个数 ≥ 2",
            );
        },
    );
}

/// 【待实现】把多种图形装进 `Vec<Box<dyn Area>>`、排序、返回面积列表。
///
/// 实现要求：
///   - 在函数内定义 `struct Rectangle { width: f64, height: f64 }`，
///     并为它写 `impl Area for Rectangle`（局部类型 + 局部 impl 都是合法的），面积 = width * height；
///   - 构造 `Vec<Box<dyn Area>>`，至少放入 3 个图形：`Rectangle { width: 3.0, height: 4.0 }`、
///     `Circle { radius: 1.0 }`、`Square { side: 2.5 }`；
///   - 用 `sort_by(|a, b| a.area().total_cmp(&b.area()))` 按面积**升序**排序；
///   - 返回排序后每个图形的面积：`vec![π, 6.25, 12.0]`（π ≈ 3.141592653589793）；
///   - `Vec<Box<dyn Area>>` 里的元素类型被擦成了 `dyn Area`，所以每个元素的大小不必相同；
///     遍历时 `for shape in &shapes { shape.area() }` 会自动解引用到 `dyn Area` 并查虚表。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// [3.141592653589793, 6.25, 12.0]
/// ```
fn exercise_12_09_dyn_collection() -> Vec<f64> {
    assessment_harness::todo_exercise(
        "exercise_12_09_dyn_collection",
        "用 Vec<Box<dyn Area>> 装 Rectangle{3.0,4.0} / Circle{1.0} / Square{2.5}，\
         按面积升序排序后返回面积列表 vec![π, 6.25, 12.0]",
        (),
    )
}

// ===========================================================================
// kp_12_10 派生标准库 trait：Debug / Clone / PartialEq / Default
// ===========================================================================

/// 知识点考核：使用已派生的 Debug / PartialEq / Default。
#[test]
fn kp_12_10_derived_traits() {
    assess(
        M,
        "kp_12_10",
        "派生标准库 trait：Default 置零、PartialEq 逐字段比较、Debug 的固定格式",
        Kind::Core,
        "复习 lesson_12 示例 11（标准库 trait 的派生）：\
         `#[derive(Debug, Clone, PartialEq, Default)]` 生成的实现都是「逐字段」的；\
         Debug 的输出格式固定为 `Version { major: 1, minor: 2 }`，Default 把 u32 字段全部置 0。",
        || {
            // 读一次字段：确认夹具正确，也让 major / minor 在骨架态不触发 dead_code
            let sample = Version { major: 1, minor: 2 };
            is_true(
                sample.major == 1 && sample.minor == 2,
                "Version 的两个字段就是版本号的两段：本例是 1 与 2",
            );
            let (equal, debug) = exercise_12_10_derived_traits();
            eq(
                equal,
                true,
                "Version::default() 的两个字段都是 0，应与 Version { major: 0, minor: 0 } 相等",
            );
            eq(
                debug,
                String::from("Version { major: 1, minor: 2 }"),
                "派生 Debug 的输出格式固定：`Version { major: 1, minor: 2 }`（类型名、花括号、逗号后的空格）",
            );
            // 边界：派生 PartialEq 逐字段比较，只有 minor 不同也必须判为不相等
            is_false(
                Version { major: 1, minor: 2 } == Version { major: 1, minor: 3 },
                "边界：只有 minor 不同也要判为不相等（不能只比 major，也不能只比其中一个字段）",
            );
        },
    );
}

/// 【待实现】使用提供的 `Version`，体会派生的 Debug / PartialEq / Default。
///
/// 实现要求：
///   - 返回一个二元组 `(bool, String)`；
///   - 第一项是 `Version::default() == Version { major: 0, minor: 0 }`（Default 把字段置 0，
///     派生的 PartialEq 逐字段比较，结果应为 `true`）；
///   - 第二项是 `format!("{:?}", Version { major: 1, minor: 2 })`
///     （派生的 Debug，结果应为 `"Version { major: 1, minor: 2 }"`）；
///   - `Version` 已经在「提供给你的类型」小节里派生了 Debug / Clone / PartialEq / Default，
///     所以这三件事各只需调用一次即可，不要手写 impl（会与派生实现冲突，报 `error[E0119]`）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (true, "Version { major: 1, minor: 2 }")
/// ```
fn exercise_12_10_derived_traits() -> (bool, String) {
    assessment_harness::todo_exercise(
        "exercise_12_10_derived_traits",
        "返回 (Version::default() == Version { major: 0, minor: 0 }, \
         format!(\"{:?}\", Version { major: 1, minor: 2 }))",
        (),
    )
}

// ===========================================================================
// kp_12_11 常见错误诊断：读得懂编译器报错，才改得动代码
// ===========================================================================

/// 知识点考核：按课程示例 13 的注释顺序写出前三个错误编号。
#[test]
fn kp_12_11_diagnose_errors() {
    assess(
        M,
        "kp_12_11",
        "常见错误诊断：trait bound 未满足 / impl 漏写必需方法 / 重复实现",
        Kind::Hard,
        "复习 lesson_12 示例 13（common_mistakes）：本课列的 7 个坑里，\
         前三个分别是「类型没有实现 trait」「impl 块漏写必需方法」「同一个类型重复实现同一个 trait」。",
        || {
            let codes = exercise_12_11_diagnose_errors();
            eq_slice(
                &codes,
                &["E0277", "E0046", "E0119"],
                "顺序必须是：trait bound 未满足（E0277）、impl 漏写必需方法（E0046）、重复实现同一个 trait（E0119）",
            );
            // 边界：错误编号的书写格式必须规范（E + 4 位数字），例如 "E0277" 而不是 "277"
            is_true(
                codes
                    .iter()
                    .all(|code| code.starts_with('E') && code.len() == 5),
                "边界：每一项都要写成 `E` 加 4 位数字（如 \"E0277\"），不能省略前缀或前导零",
            );
        },
    );
}

/// 【待实现】写出三个场景对应的编译器错误编号。
///
/// 场景（与课程示例 13 的前三个错误一致）：
///   1. `notify(&Plain);` —— `Plain` 没有实现 `Summary`（trait bound 未满足）；
///   2. `impl Summary for Report {}` —— impl 块里漏写了必需方法 `summarize`；
///   3. 对同一个 `Report` 写两份 `impl Summary for Report` —— 实现冲突。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0277"`），顺序与上面一致；
///   - 这些编号在课程示例 13 的注释里都能找到（错误 1 / 2 / 3 各对应一个编号），
///     记住它们能让你一眼看懂 trait 相关报错。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0277", "E0046", "E0119"]
/// ```
fn exercise_12_11_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_12_11_diagnose_errors",
        "返回 [\"E0277\", \"E0046\", \"E0119\"]（trait bound 未满足 / 漏写必需方法 / 重复实现）",
        (),
    )
}
