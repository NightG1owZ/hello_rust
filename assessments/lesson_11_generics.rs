//! assessments/lesson_11_generics.rs —— 考核：泛型（对应 lesson_11）
//!
//! - 对应课程：`src/tutorial/lesson_11_generics.rs`
//! - 知识点出处：`src/tutorial/README.md` 第四阶段「11 泛型」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_11_generics           # 只考这一课
//!   cargo test                                     # 考全部 18 课
//!   cargo run --bin assessment_report              # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_11_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；有几题要求你在**函数体内**定义泛型结构体 / 枚举 /
//!    辅助函数，具体写法写在「实现要求」里；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_11_generics`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部类型、局部函数与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 本课常见错误速查（对应课程示例 12）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `fn add<T>(a: T, b: T) -> T { a + b }` | `error[E0369]` cannot add `T` to `T` | 补约束 `T: std::ops::Add<Output = T>` |
//! | `fn show<T>(value: T) { println!("{value}"); }` | `error[E0277]` `T` doesn't implement `std::fmt::Display` | 加 `T: std::fmt::Display`；只想调试打印就改 `{:?}` 并约束 `T: Debug` |
//! | `struct Wrapper<T> { value: i32 }` | `error[E0392]` type parameter `T` is never used | 删掉 `<T>`，或让字段真的用上 `T`（`value: T`） |
//! | `MixedPoint::<i32> { x: 1, y: 2 }` | `error[E0107]` struct takes 2 generic arguments but 1 generic argument was supplied | 写全 `MixedPoint::<i32, &str>`，或省略 turbofish 让编译器推断 |
//! | `Point { x: 1, y: 2.5 }`（单个 `T`） | `error[E0308]` mismatched types | 同一个 `T` 只能是一种具体类型，想混用就加第二个类型参数 |
//! | 把运行期变量当 const 泛型实参 | `error[E0435]` attempt to use a non-constant value in a constant | 写 `Buffer::<4>::new()`，或先声明 `const N: usize = 4;` |

use assessment_harness::{Kind, assess, eq, eq_slice, is_true, none, some};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_11";

// ===========================================================================
// kp_11_01 泛型函数：一份代码适配多种类型
// ===========================================================================

/// 知识点考核：返回最大元素**引用**的泛型函数。
#[test]
fn kp_11_01_largest() {
    assess(
        M,
        "kp_11_01",
        "泛型函数：`fn largest<T: PartialOrd>(list: &[T]) -> Option<&T>` 一份代码适配多种类型",
        Kind::Basic,
        "复习 lesson_11 示例 1（generic_function）：类型参数 T 代表「待定的类型」，\
         `<T: PartialOrd>` 是 trait bound，告诉编译器 T 支持比较；空切片必须返回 None。",
        || {
            // 正常用例：i32 切片
            let numbers = [34, 7, 91, 25];
            eq(
                *some(
                    exercise_11_01_largest(&numbers),
                    "非空切片必须返回 Some(最大值)",
                ),
                91i32,
                "largest(&[34, 7, 91, 25]) 的最大值是 91",
            );

            // 正常用例：同一个泛型函数换成 &str 切片也能用
            let words = ["apple", "pear", "fig"];
            eq(
                *some(
                    exercise_11_01_largest(&words),
                    "同一份泛型代码也要能处理 &str 切片（T = &str）",
                ),
                "pear",
                "字符串按字典序比较：pear > fig > apple",
            );

            // 边界：单元素切片，最大值就是它自己
            let single = [5];
            eq(
                *some(
                    exercise_11_01_largest(&single),
                    "只有一个元素时它就是最大值",
                ),
                5i32,
                "单元素切片属于边界情况：不要写成 list[1..] 之类会 panic 的代码",
            );

            // 边界：空切片必须返回 None，而不是像课程示例那样 list[0] 直接 panic
            let empty: [i32; 0] = [];
            none(
                exercise_11_01_largest(&empty),
                "空切片没有最大值：必须返回 None，绝不能 panic",
            );

            // 边界：相等元素要返回**最先出现**的那个（用严格大于比较）
            let ties = [3, 7, 7, 2];
            let max = some(exercise_11_01_largest(&ties), "有重复元素时也要返回 Some");
            eq(*max, 7i32, "最大值是 7");
            is_true(
                std::ptr::eq(max, &ties[1]),
                "相等元素要保留最先出现的那个：应指向下标 1 的 7（用 `>` 而不是 `>=`）",
            );
        },
    );
}

/// 【待实现】返回切片中最大元素的引用。
///
/// 实现要求：
///   - 返回的是**引用**（`Option<&T>`），不要拷贝或克隆元素；
///   - 空切片 → `None`；
///   - 非空时把 `&list[0]` 当作当前最大值的起点，遍历其余元素，
///     只有**严格大于**当前最大值才替换（这样相等元素会保留最先出现的那个）；
///   - 返回最终的那个引用；
///   - 用 `list.first()?` 拿到起点、`list.iter().skip(1)` 遍历其余元素，可以避免 `list[0]`
///     在空切片上 panic；即使 `T: Copy` 也不要改成返回 `T`，请按签名返回引用。
///
/// 示例输入：
/// ```text
/// list = [34, 7, 91, 25]
/// ```
/// 示例输出：
/// ```text
/// Some(91)
/// ```
fn exercise_11_01_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    assessment_harness::todo_exercise(
        "exercise_11_01_largest",
        "返回最大元素的引用：空切片 → None；相等元素保留最先出现的那个",
        (list,),
    )
}

// ===========================================================================
// kp_11_02 泛型结构体：Point<T> 的两个具体实例是不同类型
// ===========================================================================

/// 知识点考核：用同一个泛型结构体定义生成 `Point<i32>` 与 `Point<f64>`。
#[test]
fn kp_11_02_generic_struct() {
    assess(
        M,
        "kp_11_02",
        "泛型结构体：`struct Point<T>` 能生成 Point<i32>、Point<f64> 等互相独立的具体类型",
        Kind::Core,
        "复习 lesson_11 示例 2（generic_struct）：字段类型可以是类型参数；\
         `Point<i32>` 与 `Point<f64>` 是两个完全不同的类型，彼此不能互相赋值。",
        || {
            let (int_x, float_y) = exercise_11_02_generic_struct();

            // 正常用例：Point<i32> 的字段
            eq(int_x, 1i32, "Point<i32> 实例的 x 字段应为 1");

            // 边界：Point<f64> 的字段（2.5 是二进制可精确表示的浮点数，用 eq 比较是安全的）
            eq(
                float_y,
                2.5f64,
                "Point<f64> 实例的 y 字段应为 2.5：浮点比较只在值可精确表示时才用 eq",
            );
        },
    );
}

/// 【待实现】在函数体内定义并使用泛型结构体。
///
/// 实现要求（全部在函数体内完成）：
///   - 定义 `struct Point<T> { x: T, y: T }`；
///   - 用 i32 实例化一个点（`x: 1, y: 2`），用 f64 实例化一个点（`x: 1.5, y: 2.5`）；
///   - 返回 `(整数点的 x, 浮点点的 y)`，即 `(1, 2.5)`；
///   - `Point { x: 1, y: 2 }` 会被推断成 `Point<i32>`，`Point { x: 1.5, y: 2.5 }` 是
///     `Point<f64>`；写成 `Point { x: 1, y: 2.5 }` 会报 `error[E0308]`——同一个 T
///     不能既是 i32 又是 f64，那种场景要用两个类型参数（见 kp_11_07）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (1, 2.5)
/// ```
fn exercise_11_02_generic_struct() -> (i32, f64) {
    assessment_harness::todo_exercise(
        "exercise_11_02_generic_struct",
        "函数内定义 struct Point<T>，用 i32 与 f64 各实例化一次，返回 (1, 2.5)",
        (),
    )
}

// ===========================================================================
// kp_11_03 泛型枚举：自己定义的枚举 ↔ 标准库的 Option / Result
// ===========================================================================

/// 知识点考核：`MyOption<T>` / `MyResult<T, E>` 与标准库类型的一一对应转换。
#[test]
fn kp_11_03_generic_enum() {
    assess(
        M,
        "kp_11_03",
        "泛型枚举：标准库的 Option<T> / Result<T, E> 就是泛型枚举，自己定义的也能互相转换",
        Kind::Core,
        "复习 lesson_11 示例 3（generic_enum）：`enum MyOption<T> { Some(T), None }` 与\
         `enum MyResult<T, E> { Ok(T), Err(E) }` 的定义方式与标准库完全一致；\
         每个变体都能无损地转成标准库对应变体。",
        || {
            let (option, result) = exercise_11_03_generic_enum();

            // 正常用例：Some 分支 → 标准库的 Some
            eq(
                option,
                Some(7i32),
                "MyOption::Some(7) 应转成标准库的 Some(7)",
            );

            // 边界：错误/空分支同样要一一对应（Err(404) → 标准库的 Err(404)）
            eq(
                result,
                Err(404i32),
                "MyResult::Err(404) 应转成标准库的 Err(404)：错误分支也要能对应上",
            );
        },
    );
}

/// 【待实现】定义泛型枚举，并把它们转成标准库类型。
///
/// 实现要求（全部在函数体内完成）：
///   - 定义 `enum MyOption<T> { Some(T), None }` 与 `enum MyResult<T, E> { Ok(T), Err(E) }`；
///   - 构造 `MyOption::Some(7)`，用 `match` 转成标准库的 `Some(7)`；
///   - 构造 `MyResult::<String, i32>::Err(404)`，用 `match` 转成标准库的 `Err(404)`；
///   - 返回 `(转换后的 Option, 转换后的 Result)`；
///   - `match` 必须写全所有变体（编译器会强制穷尽），这正是「不会漏掉 None / Err 分支」的保证；
///   - `MyResult::<String, i32>::Err(404)` 里的 turbofish 用来说明两个类型参数分别是什么
///     （Ok 侧是 String，Err 侧是 i32）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (Some(7), Err(404))
/// ```
fn exercise_11_03_generic_enum() -> (Option<i32>, Result<String, i32>) {
    assessment_harness::todo_exercise(
        "exercise_11_03_generic_enum",
        "函数内定义 MyOption<T> / MyResult<T, E>，把 Some(7) 与 Err(404) 转成标准库类型后返回",
        (),
    )
}

// ===========================================================================
// kp_11_04 泛型方法：impl 块上的约束
// ===========================================================================

/// 知识点考核：为泛型结构体写带约束的 `impl` 与方法。
#[test]
fn kp_11_04_generic_method() {
    assess(
        M,
        "kp_11_04",
        "泛型方法：impl<T: Copy + Add<Output = T>> Pair<T> 让 sum() 对两种数值类型都可用",
        Kind::Core,
        "复习 lesson_11 示例 4（generic_method）：`impl<T> Point<T>` 表示「对任意 T 都提供\
         这些方法」；约束写在 impl 块上时对块内所有方法生效。",
        || {
            let (int_sum, float_sum) = exercise_11_04_generic_method();

            // 正常用例：i32 对的和
            eq(int_sum, 7i32, "Pair { a: 3, b: 4 }.sum() = 7");

            // 边界：f64 对的和（1.5 与 2.5 都能被二进制精确表示，用 eq 比较安全）
            eq(
                float_sum,
                4.0f64,
                "Pair { a: 1.5, b: 2.5 }.sum() = 4.0：同一个 sum() 也适用于 f64",
            );
        },
    );
}

/// 【待实现】在函数体内定义泛型结构体并写一个泛型方法。
///
/// 实现要求（全部在函数体内完成）：
///   - 定义 `struct Pair<T> { a: T, b: T }`；
///   - 为它写 `impl<T: Copy + std::ops::Add<Output = T>> Pair<T> { fn sum(&self) -> T }`，
///     方法体返回 `self.a + self.b`；
///   - 用 `Pair { a: 3, b: 4 }`（i32）与 `Pair { a: 1.5, b: 2.5 }`（f64）各调用一次 `sum`；
///   - 返回两个和，即 `(7, 4.0)`；
///   - `Add<Output = T>` 这个「输出还是 T」的约束不能省——否则 `a + b` 的输出类型未必是 T；
///   - `Copy` 是为了让 `&self` 上的字段能按位复制出来做加法（否则要把字段 move 走）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (7, 4.0)
/// ```
fn exercise_11_04_generic_method() -> (i32, f64) {
    assessment_harness::todo_exercise(
        "exercise_11_04_generic_method",
        "函数内定义 Pair<T> 与 impl<T: Copy + Add<Output = T>> Pair<T> 的 sum()，返回 (7, 4.0)",
        (),
    )
}

// ===========================================================================
// kp_11_05 trait bound：泛型参数可以不止一个、类型可以不同
// ===========================================================================

/// 知识点考核：两个类型参数各自独立，只需要都满足 `Display`。
#[test]
fn kp_11_05_display_pair() {
    assess(
        M,
        "kp_11_05",
        "trait bound 用起来：`<T: Display, U: Display>` 让两种不同类型的值拼成一句话",
        Kind::Core,
        "复习 lesson_11 示例 6（trait_bounds_syntax）：bound 表达「使用方对类型的要求」，\
         编译器会拿每一个具体类型去检查；`Display` 是 `{}` 格式化所需要的约束。",
        || {
            // 正常用例：i32 与 &str 是两种不同的类型
            eq(
                exercise_11_05_display_pair(1, "x"),
                String::from("1-x"),
                "T = i32、U = &str：两个参数类型不同，但都实现了 Display",
            );

            // 正常用例：换成 bool 与 f64 依然可用
            eq(
                exercise_11_05_display_pair(true, 2.5),
                String::from("true-2.5"),
                "换成 bool 与 f64 同样可用：函数体一个字都不用改（这才是泛型的价值）",
            );

            // 边界：空字符串也是合法的 Display 值，要原样拼进去
            eq(
                exercise_11_05_display_pair(-1, ""),
                String::from("-1-"),
                "边界：不要自作主张 trim 或判空——空字符串也是合法的 Display 值",
            );
        },
    );
}

/// 【待实现】把两个泛型值用一个短横线拼起来。
///
/// 实现要求：
///   - 签名已经是 `fn exercise_11_05_display_pair<T: std::fmt::Display, U: std::fmt::Display>`
///     （注意 T 与 U 是两个**独立**的类型参数）；
///   - 返回 `format!("{a}-{b}")`；
///   - 如果没有 `Display` 约束，`{a}` 会报 `error[E0277]`: `T` doesn't implement
///     `std::fmt::Display`；只用 `println!` 调试时可以改成 `{a:?}` 并约束 `T: Debug`。
///
/// 示例输入：
/// ```text
/// a = 1
/// b = "x"
/// ```
/// 示例输出：
/// ```text
/// "1-x"
/// ```
fn exercise_11_05_display_pair<T: std::fmt::Display, U: std::fmt::Display>(a: T, b: U) -> String {
    assessment_harness::todo_exercise(
        "exercise_11_05_display_pair",
        "返回 format!(\"{a}-{b}\")：两个类型参数各自独立，只需都满足 Display",
        (a, b),
    )
}

// ===========================================================================
// kp_11_06 where 子句：约束换个位置写，语义完全一样
// ===========================================================================

/// 知识点考核：用 `where` 子句写约束，返回最小元素的引用。
#[test]
fn kp_11_06_where_clause() {
    assess(
        M,
        "kp_11_06",
        "where 子句：约束放在函数签名下方，比挤在尖括号里更好读",
        Kind::Core,
        "复习 lesson_11 示例 7（where_clause）：`fn largest_where<T>(list: &[T]) -> T where\
         T: PartialOrd + Copy` 与尖括号写法语义等价，只是排版不同（rustfmt 也偏好它）。",
        || {
            // 正常用例：最小值是 61
            let scores = [88, 95, 61, 79, 61];
            let min = some(
                exercise_11_06_where_clause(&scores),
                "非空切片必须返回 Some(最小值)",
            );
            eq(*min, 61i32, "最小值是 61");

            // 边界：相等元素要保留最先出现的那个（用严格小于比较）
            is_true(
                std::ptr::eq(min, &scores[2]),
                "相等元素要保留最先出现的那个：应指向下标 2 的 61（用 `<` 而不是 `<=`）",
            );

            // 边界：单元素切片
            let single = [42];
            eq(
                *some(
                    exercise_11_06_where_clause(&single),
                    "只有一个元素时它就是最小值",
                ),
                42i32,
                "单元素切片：最小值就是它自己",
            );

            // 边界：空切片 → None
            let empty: [&str; 0] = [];
            none(
                exercise_11_06_where_clause(&empty),
                "空切片没有最小值：必须返回 None，而不是 panic",
            );
        },
    );
}

/// 【待实现】用 `where` 子句写约束，返回最小元素的引用。
///
/// 实现要求：
///   - 约束**必须**写成下面的 `where` 子句形式，不要挤进尖括号里；
///   - 空切片 → `None`；非空时以 `&items[0]` 为起点，只有**严格小于**才替换
///     （这样相等元素保留最先出现的那个）；
///   - 返回最小元素的引用；
///   - 签名已经写好了 `where T: PartialOrd`，你只需要补函数体；这和 kp_11_01 求最大值是
///     同一套逻辑，只把比较方向反过来。
///
/// 示例输入：
/// ```text
/// items = [88, 95, 61, 79, 61]
/// ```
/// 示例输出：
/// ```text
/// Some(61)
/// ```
fn exercise_11_06_where_clause<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    assessment_harness::todo_exercise(
        "exercise_11_06_where_clause",
        "用 where 子句写约束，返回最小元素的引用：空切片 → None；相等元素保留最先出现的",
        (items,),
    )
}

// ===========================================================================
// kp_11_07 多个泛型参数：K 与 V 各自独立
// ===========================================================================

/// 知识点考核：两个类型参数拼成 `key=value`。
#[test]
fn kp_11_07_multiple_params() {
    assess(
        M,
        "kp_11_07",
        "多个泛型参数：`<K: Display, V: Display>` 让键与值各自是不同类型",
        Kind::Core,
        "复习 lesson_11 示例 8（multiple_type_params）：类型参数可以有两个及以上、各自独立；\
         泛型结构体的字段可以分别使用它们（MixedPoint<T, U> 就是例子）。",
        || {
            // 正常用例：K = i32，V = &str
            eq(
                exercise_11_07_multiple_params(1, "x"),
                String::from("1=x"),
                "K = i32、V = &str：拼成 \"1=x\"",
            );

            // 正常用例：K = &str，V = u8
            eq(
                exercise_11_07_multiple_params("count", 7u8),
                String::from("count=7"),
                "K = &str、V = u8：拼成 \"count=7\"，两个类型参数互不影响",
            );

            // 边界：两个空字符串拼出来只剩分隔符
            eq(
                exercise_11_07_multiple_params("", ""),
                String::from("="),
                "边界：空 key 与空 value 是合法输入，结果就是 \"=\"（不要额外判空报错）",
            );
        },
    );
}

/// 【待实现】用两个泛型参数拼出 `key=value`。
///
/// 实现要求：
///   - 返回 `format!("{key}={value}")`（签名里已经写好两个 `Display` 约束）；
///   - `K` 与 `V` 是彼此独立的类型参数，所以 `(1, "x")`、`("count", 7u8)` 都能调用同一个
///     函数；这也正是 `HashMap<K, V>` 的写法来源。
///
/// 示例输入：
/// ```text
/// key = 1
/// value = "x"
/// ```
/// 示例输出：
/// ```text
/// "1=x"
/// ```
fn exercise_11_07_multiple_params<K: std::fmt::Display, V: std::fmt::Display>(
    key: K,
    value: V,
) -> String {
    assessment_harness::todo_exercise(
        "exercise_11_07_multiple_params",
        "返回 format!(\"{key}={value}\")：两个泛型参数各自独立",
        (key, value),
    )
}

// ===========================================================================
// kp_11_08 const 泛型：把长度也变成类型的一部分
// ===========================================================================

/// 知识点考核：`const N: usize` 让数组长度参与泛型。
#[test]
fn kp_11_08_const_generics() {
    assess(
        M,
        "kp_11_08",
        "const 泛型：`<const N: usize>` 把编译期常量也作为参数，数组长度是类型的一部分",
        Kind::Edge,
        "复习 lesson_11 示例 9（const_generics）：`Buffer<4>` 与 `Buffer<8>` 是不同类型；\
         函数上的 `fn filled<T: Copy, const N: usize>(value: T) -> [T; N]` 也是同一原理。",
        || {
            // 正常用例：N = 3
            eq(
                exercise_11_08_const_generics([1, 2, 3]),
                (3usize, 6i32),
                "N=3：[1, 2, 3] 应返回 (长度 3, 和 6)",
            );

            // 正常用例：负数也要正确求和
            eq(
                exercise_11_08_const_generics([-1, -2, 5]),
                (3usize, 2i32),
                "-1 + -2 + 5 = 2：负数的加法不能漏",
            );

            // 边界：N = 1
            eq(
                exercise_11_08_const_generics([10]),
                (1usize, 10i32),
                "N=1 是边界：和就是那个元素本身",
            );

            // 边界：N = 0 —— 空数组同样是合法的 const 泛型实参，和必须是 0
            eq(
                exercise_11_08_const_generics([]),
                (0usize, 0i32),
                "N=0 的空数组也是合法实参：长度 0、和为 0，绝不能 panic（不能用 sums[0] 起步）",
            );
        },
    );
}

/// 【待实现】用 const 泛型求数组长度与元素之和。
///
/// 实现要求：
///   - 返回 `(N, 元素之和)`；
///   - `N == 0` 时和必须是 `0`（用 `items.iter().sum()` 这类不依赖首元素的写法自然成立）；
///   - 注意 `N` 是 `usize`，返回值的第一项也必须是 `usize`；
///   - 不要写成 `let mut total = items[0];`——N 可能是 0，那样会 panic（`N` 是编译期常量，
///     编译器允许 `[i32; 0]` 这种实参）；`items.iter().sum::<i32>()` 或 `for` 循环都能从 0 起步。
///
/// 示例输入：
/// ```text
/// items = [1, 2, 3]
/// ```
/// 示例输出：
/// ```text
/// (3, 6)
/// ```
fn exercise_11_08_const_generics<const N: usize>(items: [i32; N]) -> (usize, i32) {
    assessment_harness::todo_exercise(
        "exercise_11_08_const_generics",
        "返回 (N, 元素之和)；N = 0 时和必须为 0",
        (items,),
    )
}

// ===========================================================================
// kp_11_09 单态化：泛型在编译期被展开成具体类型
// ===========================================================================

/// 知识点考核：同一个泛型辅助函数被两种具体类型实例化。
#[test]
fn kp_11_09_monomorphization() {
    assess(
        M,
        "kp_11_09",
        "单态化（monomorphization）：泛型在编译期展开，运行期没有类型判断开销",
        Kind::Edge,
        "复习 lesson_11 示例 10（monomorphization）：编译器为每个用到的具体类型各生成一份\
         专用代码（相当于自动写出 size_of_val_i32、size_of_val_f64），\
         代价是二进制体积与编译时间增加。",
        || {
            let (int_size, float_size) = exercise_11_09_monomorphization();

            // 正常用例：i32 的大小是 Rust 保证的 4 字节
            eq(
                int_size,
                4usize,
                "size_of::<i32>() 恒为 4：Rust 保证基本类型的大小，与本机平台无关",
            );

            // 边界：f64 恒为 8 字节（不是 4，也不是「和 i32 一样」）
            eq(
                float_size,
                8usize,
                "size_of::<f64>() 恒为 8：同一份泛型代码被实例化了两次，大小可以不同",
            );

            // 这正是单态化的直观结果：T 在编译期已确定，size_of::<T>() 才是编译期常量
            is_true(
                int_size < float_size,
                "两个具体类型的大小不同，说明编译器确实为 i32 与 f64 各生成了一份代码（单态化）",
            );
        },
    );
}

/// 【待实现】用泛型辅助函数观察单态化。
///
/// 实现要求（全部在函数体内完成）：
///   - 定义泛型辅助函数 `fn size_of_val<T>(_: &T) -> usize { std::mem::size_of::<T>() }`；
///   - 分别用 `i32` 与 `f64` 的值各调用一次（例如 `size_of_val(&1i32)` 与 `size_of_val(&1.0f64)`）；
///   - 返回两个值，即 `(4, 8)`；
///   - `size_of::<T>()` 只有在 T 编译期确定时才能求值——它能写成泛型函数，本身就证明了
///     「每个具体类型都有一份专用代码」；另外标准库已有 `std::mem::size_of_val`，
///     这里要求你手写一份泛型版本，正是为了体会这一点。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (4, 8)
/// ```
fn exercise_11_09_monomorphization() -> (usize, usize) {
    assessment_harness::todo_exercise(
        "exercise_11_09_monomorphization",
        "函数内定义 fn size_of_val<T>(_: &T) -> usize，用 i32 与 f64 各调用一次，返回 (4, 8)",
        (),
    )
}

// ===========================================================================
// kp_11_10 常见错误诊断：泛型相关的错误编号
// ===========================================================================

/// 知识点考核：读懂课程示例 12 的注释，写出前三个错误的编号。
#[test]
fn kp_11_10_diagnose_errors() {
    assess(
        M,
        "kp_11_10",
        "常见错误诊断：泛型里直接相加、用 {} 打印未约束 Display、类型参数没被使用",
        Kind::Hard,
        "复习 lesson_11 示例 12（common_mistakes）：泛型代码的错误几乎都指向「约束不足」或\
         「类型不匹配」；读错误信息时先看是哪一类，再决定补约束还是改类型参数。",
        || {
            eq_slice(
                &exercise_11_10_diagnose_errors(),
                &["E0369", "E0277", "E0392"],
                "按课程示例 12 的注释顺序，错误 1 / 2 / 3 的编号依次是：\
                 泛型里直接 `a + b`（E0369）、用 `{}` 打印未约束 Display 的 T（E0277）、\
                 类型参数声明了却没用上（E0392）",
            );
        },
    );
}

/// 【待实现】写出课程示例 12 里前三个错误的编译器错误编号。
///
/// 场景（与课程示例 12 的注释一一对应，顺序也一致）：
///   1. `fn add<T>(a: T, b: T) -> T { a + b }` —— 泛型参数上直接做加法，编译器不知道 T 支持 `+`；
///   2. `fn show<T>(value: T) { println!("{value}"); }` —— 用 `{}` 打印泛型值，但 T 不一定
///      实现 `Display`；
///   3. `struct Wrapper<T> { value: i32 }` —— 类型参数 `T` 声明了却从未被使用。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0369"`），顺序与上面一致；
///   - 三个编号在课程示例 12 的注释里都写着；记住它们能让你一眼看懂「泛型报错」属于哪一类。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0369", "E0277", "E0392"]
/// ```
fn exercise_11_10_diagnose_errors() -> [&'static str; 3] {
    assessment_harness::todo_exercise(
        "exercise_11_10_diagnose_errors",
        "返回课程示例 12 前三个错误的编号（顺序一致）：[\"E0369\", \"E0277\", \"E0392\"]",
        (),
    )
}
