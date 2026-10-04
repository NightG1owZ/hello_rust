//! assessments/lesson_09_packages_modules.rs —— 考核：包和模块（对应 lesson_09）
//!
//! - 对应课程：`src/tutorial/lesson_09_packages_modules.rs`
//! - 知识点出处：`src/tutorial/README.md` 第三阶段「09 包和模块」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_09_packages_modules      # 只考这一课
//!   cargo test                                        # 考全部 18 课
//!   cargo run --bin assessment_report                 # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_09_xx_xxx` 练习函数，它的函数体里只有一行
//!    `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_09_packages_modules`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 只修改 `exercise_*` 练习函数的函数体，可以按需增加局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」；
//! - **本课的「实现」是在练习函数体内部组织模块**：考核只看最终返回值，
//!   但你必须真的用 `mod` / `pub` 系列可见性 / `use` / `pub use` / `super::` 这些机制去拿值，
//!   直接写一个字面量虽然也能让断言通过，却完全没有练到本课的知识点。
//!
//! # 本课常见错误速查（对应课程示例 7）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | `lessons::calc::guard(1.0);`（`fn guard` 无 `pub`） | `error[E0603]` function `guard` is private | 给项加 `pub` / `pub(crate)`，或提供一个公开的包装函数转发 |
//! | `lessons::calc::sub(1.0, 2.0);`（`sub` 是 `pub(super)`） | `error[E0603]` function `sub` is private | `pub(super)` 只对父模块及其后代可见；改成 `pub(crate)` 或在内部转发 |
//! | `outer::inner::hi();`（`mod inner` 前没有 `pub`） | `error[E0603]` module `inner` is private | 写 `pub mod inner`，模块本身也要公开 |
//! | `lessons::add(1.0, 2.0);`（漏了层级） | `error[E0425]` cannot find function `add` in module `lessons` | 写全路径 `lessons::calc::add(...)` |
//! | `use lessons::calc::mul;` 但从未使用 | `warning: unused import`（骨架态必须零 warning） | 删掉没用的 `use`，或真正使用它 |
//! | `use lessons::calc::add;` 与本地 `fn add` 同名 | `error[E0255]` the name `add` is defined multiple times | 用 `as` 起别名（`use ... as calc_add;`）或删掉重复定义 |
//! | `mod calc;` 却没有同名文件 | `error[E0583]` file not found for module `calc` | 补上 `calc.rs` 或 `calc/mod.rs` |
//! | 把 `pub(crate)` 项 `pub use` 出去 | `error[E0364]`/`E0365` cannot be re-exported outside | 先让项本身 `pub`，重导出时可见性只能「不放大」 |
//! | 函数内 `use` 却想在别的函数用 | `error[E0425]` cannot find function | `use` 的作用域只到所在块，要提到模块顶层 |
//!
//! # 三条路径的含义
//!
//! - `crate::` 绝对路径：从 **crate 根**出发，任何模块里都能用（本文件里 `crate::M` 指文件顶部的常量）；
//! - `self::` 当前模块：`self::foo` 与直接写 `foo` 等价，只是把「我就是在本模块里找」写明确了；
//! - `super::` 父模块：`super` 是**上一级**模块，`super::super::` 就是再上一级。

use assessment_harness::{Kind, assess, eq, eq_slice, is_false, is_true};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_09";

// ===========================================================================
// kp_09_01 模块基础：在函数内定义 mod 并用「模块::函数」调用
// ===========================================================================

/// 知识点考核：定义 `mod math`，用路径 `math::add` / `math::double` 调用。
#[test]
fn kp_09_01_module_basics() {
    assess(
        M,
        "kp_09_01",
        "模块基础：`mod math { pub fn ... }` + 路径 `math::add` 调用",
        Kind::Basic,
        "复习 lesson_09 示例 1（module_basics）：模块用 `mod 名字 { ... }` 声明，\
         里面的项默认私有，必须写 `pub fn` 才能从模块外用 `模块名::函数名` 调用。",
        || {
            // 正常用例：add(1, 2) + double(3) = 3 + 6 = 9
            eq(
                exercise_09_01_module_basics(),
                9i32,
                "math::add(1, 2) = 3，math::double(3) = 6，两者相加应为 9",
            );
            // 边界用例：同一题再调一次，结果必须稳定（模块里没有可变状态）
            eq(
                exercise_09_01_module_basics(),
                9i32,
                "重复调用结果必须一致：模块内是纯函数，不能依赖全局可变状态",
            );
        },
    );
}

/// 【待实现】在函数体内定义 `mod math`，返回 `math::add(1, 2) + math::double(3)`。
///
/// 实现要求：
///   - 在函数体内写 `mod math { ... }`，其中至少包含两个函数：
///     `pub fn add(a: i32, b: i32) -> i32`（返回 `a + b`）与
///     `pub fn double(x: i32) -> i32`（返回 `x * 2`）；
///   - 两个函数都必须是 `pub`，否则函数外的代码无法调用（`error[E0603]`）；
///   - 函数体内的 `mod` 是合法的内联模块，作用域只到该函数体，不会污染文件里的其他名字；
///   - 返回 `math::add(1, 2) + math::double(3)`，也就是 `3 + 6 = 9`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 9
/// ```
fn exercise_09_01_module_basics() -> i32 {
    // 内联模块：作用域只到本函数体，不污染文件里的其他名字
    mod math {
        // 模块里的项默认私有，必须写 `pub` 才能用 `math::add` 从模块外调用
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn double(x: i32) -> i32 {
            x * 2
        }
    }

    // 路径调用：math::add(1, 2) = 3，math::double(3) = 6
    math::add(1, 2) + math::double(3)
}

// ===========================================================================
// kp_09_02 可见性层级：pub(crate) / pub(super) / 私有
// ===========================================================================

/// 知识点考核：在一个模块里同时用上三种可见性，并只通过合法路径取到值。
#[test]
fn kp_09_02_visibility_levels() {
    assess(
        M,
        "kp_09_02",
        "可见性层级：`pub(crate)`（本 crate 可见）/ `pub(super)`（父模块可见）/ 私有（仅本模块可见）",
        Kind::Core,
        "复习 lesson_09 示例 2（visibility_levels）：四种可见性分别是 `pub`、`pub(crate)`、\
         `pub(super)`、不加修饰（私有，只有本模块及其子模块可见）；访问范围只能缩小不能放大。",
        || {
            // 正常用例：crate 可见的 API + 父模块可见的转发 + 子模块私有项，合计 5
            eq(
                exercise_09_02_visibility_levels(),
                5i32,
                "三部分信息合起来必须是 5：1（crate 可见）+ 3（父模块可见）+ 1（同模块私有）",
            );
            // 边界用例：再次调用结果相同，且**没有**用到任何越级访问
            eq(
                exercise_09_02_visibility_levels(),
                5i32,
                "可见性题的结果必须确定：不允许把私有项直接暴露到模块外",
            );
        },
    );
}

/// 【待实现】用三层可见性组合出返回值 5。
///
/// 实现要求（下面是一个可直接采用的方案，也可以自己设计等价的方案；三层可见性
/// 各贡献一部分，合计 `1 + 3 + 1 = 5`）：
///   - 在函数体内写 `mod outer { ... }`，其中：
///     1. `pub(crate) const BASE: i32 = 1;` —— crate 内任何地方都能读到；
///     2. `pub(crate) fn visible_to_crate() -> i32 { BASE }` —— crate 可见的入口，贡献 **1**；
///     3. `mod inner { ... }`（**不加 pub**，所以只有 outer 及其后代能访问它）；
///     4. 在 `inner` 里写 `pub(super) fn visible_to_parent() -> i32 { 3 }` —— `super` 就是
///        outer，所以只有 outer 能调用它，贡献 **3**；
///     5. 在 `inner` 里写私有 `const PRIVATE_STEP: i32 = 1;` 与私有
///        `fn visible_here_only() -> i32 { PRIVATE_STEP }` —— 两者都只能由 `inner` 自己的函数使用；
///     6. 在 `inner` 里写 `pub(super) fn collect() -> i32 { visible_here_only() }`，把私有部分
///        交给 outer，贡献 **1**（`PRIVATE_STEP` 已经算在 `visible_here_only()` 里，**不要**再额外
///        `+ PRIVATE_STEP`，那样会得到 6）；
///     7. 在 `outer` 里写 `pub(crate) fn total() -> i32`，返回
///        `visible_to_crate() + inner::visible_to_parent() + inner::collect()`；
///   - 返回 `outer::total()`，也就是 `1 + 3 + 1 = 5`：crate 可见 1 + 父模块可见 3 + 同模块私有 1；
///   - 三层的可见范围：`pub(crate)` 覆盖整个 crate，所以函数外能调用 `outer::total()`；
///     `pub(super)` 只覆盖父模块及其后代，函数外**不能**直接调用 `inner::visible_to_parent()`；
///     私有项只有本模块及其子模块可见，所以只能由 `inner` 自己的函数使用；
///   - 模块本身也要可见：`mod inner` 前没有 `pub` 时，在 `outer` 外面写 `outer::inner::...`
///     会报 `error[E0603]: module inner is private`；
///   - 实现时逐条写清每一项为什么能被 / 不能被访问。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 5
/// ```
fn exercise_09_02_visibility_levels() -> i32 {
    mod outer {
        // pub(crate)：crate 根起的任何地方都能读到；贡献 1
        pub(crate) const BASE: i32 = 1;

        pub(crate) fn visible_to_crate() -> i32 {
            BASE
        }

        // 不加 pub：inner 只有 outer 及其后代能访问（函数体里写 outer::inner 会报 E0603）
        mod inner {
            // pub(super) 的 super 就是 outer：只有 outer 能调用；贡献 3
            pub(super) fn visible_to_parent() -> i32 {
                3
            }

            // 私有项：只有 inner 自己（及其子模块）能用；贡献 1
            const PRIVATE_STEP: i32 = 1;

            fn visible_here_only() -> i32 {
                PRIVATE_STEP
            }

            // 把私有部分交给 outer 使用：PRIVATE_STEP 已算在 visible_here_only() 里
            pub(super) fn collect() -> i32 {
                visible_here_only()
            }
        }

        pub(crate) fn total() -> i32 {
            visible_to_crate() + inner::visible_to_parent() + inner::collect()
        }
    }

    // 1（crate 可见）+ 3（父模块可见）+ 1（同模块私有）= 5
    outer::total()
}

// ===========================================================================
// kp_09_03 use 引入与 as 重命名
// ===========================================================================

/// 知识点考核：用 `use ... as ...` 引入并重命名两个路径。
#[test]
fn kp_09_03_use_and_rename() {
    assess(
        M,
        "kp_09_03",
        "`use` 引入与 `as` 重命名：路径别名不改变可见性，只改调用时写的名字",
        Kind::Core,
        "复习 lesson_09 示例 3（use_and_rename）：`use 路径::项 as 别名;` 之后就能用别名调用；\
         别名可以解决同名冲突，也可以让调用点更短。",
        || {
            // 正常用例：(compute(2), other(2)) = (200, 7)
            eq(
                exercise_09_03_use_and_rename(),
                (200i32, 7i32),
                "calc(2) = 2 * 100 = 200；other(2) = 2 * 3 + 1 = 7",
            );
            // 边界用例：结果必须来自两个不同的重命名路径，顺序不能颠倒
            eq(
                exercise_09_03_use_and_rename(),
                (200i32, 7i32),
                "元组顺序固定为 (calc 的返回值, other 的返回值)：200 在前、7 在后",
            );
        },
    );
}

/// 【待实现】用 `use ... as ...` 重命名两条路径，返回两个调用结果。
///
/// 实现要求（建议方案）：
///   - 在函数体内定义 `mod inner { pub fn compute(value: i32) -> i32 { value * 100 } }`
///     与 `mod other { pub fn compute(value: i32) -> i32 { value * 3 + 1 } }`
///     （两个同名函数正好说明为什么需要 `as`）；
///   - 用两条 `use` 把同名函数重命名成不同别名，例如
///     `use inner::compute as calc;` 与 `use other::compute as other_calc;`
///     （真实项目里更常见的写法是只重命名其中一个）；
///   - 函数体内的 `use` 作用域只到该函数结束，不会污染文件其他位置；两条 `use` 若都写成
///     `use ...::compute;` 就会报 `error[E0252]: the name compute is defined multiple times`；
///   - 返回 `(calc(2), other_calc(2))`，也就是 `(200, 7)`。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (200, 7)
/// ```
fn exercise_09_03_use_and_rename() -> (i32, i32) {
    // 两个模块里有同名函数 compute：直接 `use ...::compute;` 两次会报 E0252
    mod inner {
        pub fn compute(value: i32) -> i32 {
            value * 100
        }
    }

    mod other {
        pub fn compute(value: i32) -> i32 {
            value * 3 + 1
        }
    }

    // 用 `as` 起别名：别名只改调用时写的名字，不改可见性（use 作用域只到本函数）
    use inner::compute as calc;
    use other::compute as other_calc;

    // (200, 7)
    (calc(2), other_calc(2))
}

// ===========================================================================
// kp_09_04 pub use 重导出：门面模块把内部实现暴露成公开 API
// ===========================================================================

/// 知识点考核：通过 `facade::Api` 使用被重导出的类型。
#[test]
fn kp_09_04_pub_use_reexport() {
    assess(
        M,
        "kp_09_04",
        "`pub use` 重导出：门面模块让调用方只记住一条短路径",
        Kind::Core,
        "复习 lesson_09 示例 4（pub_use_reexport）：`pub use` 把内部的项「搬」到当前模块名下，\
         内部结构怎么改都不影响调用方；重导出的可见性不能放大（`pub(crate)` 项不能被 `pub use` 到 crate 外，\
         否则报 `error[E0364]` / `error[E0365]`）。",
        || {
            // 正常用例：通过 facade::Api 构造并调用 run()
            eq(
                exercise_09_04_pub_use_reexport(),
                42i32,
                "Api::new(40) 的 run() 返回 40 + 2 = 42（类型是从 inner 重导出过来的）",
            );
            // 边界用例：结果确定，不依赖任何全局状态
            eq(
                exercise_09_04_pub_use_reexport(),
                42i32,
                "重复调用必须是 42：Api 的字段是常量种子，run() 是纯函数",
            );
        },
    );
}

/// 【待实现】定义内部实现 + 门面重导出，通过 `facade::Api` 返回 `run()` 的结果。
///
/// 实现要求（建议方案）：
///   - 先把 `inner` 与 `facade` 放进同一个外层模块 `outer`，让两者成为兄弟模块：
///     函数体本身不是模块，若把 `mod inner` 与 `mod facade` 直接写在函数体里，
///     `facade` 里的 `super::inner` 会解析到 **crate 根**，报
///     `error[E0432]: unresolved import super::inner`；
///   - `mod outer { ... }`，其中：
///     1. `pub(crate) mod inner { ... }`：
///        - `pub(crate) struct Api { value: i32 }`（字段私有，只能通过方法访问）；
///        - `impl Api { pub fn new() -> Self { Api { value: 40 } } }`；
///        - `impl Api { pub fn run(&self) -> i32 { self.value + 2 } }`；
///     2. `pub(crate) mod facade { pub(crate) use super::inner::Api; }` 做重导出：
///        `pub(crate)` 表示「本 crate 内可见，但不对外暴露」；
///        **注意**：如果这里写成 `pub use super::inner::Api;`，而 `Api` 是 `pub(crate)` 的，
///   就会报 `error[E0365]: Api is only public within the crate, and cannot be re-exported outside`；
///        也可以把 `Api` 本身写成 `pub`（此时门面用 `pub use` 与 `pub(crate) use` 都可以），
///        两种方案都允许，但必须在注释里说明你选的是哪一种、为什么；
///   - 通过**重导出后的路径**构造并调用：`outer::facade::Api::new().run()`（或分两步写）；
///   - 返回 `run()` 的结果，也就是 `42`；
///   - 这一题的重点不是算 42，而是「调用方只写 `facade::Api`，完全不知道 `inner` 的存在」：
///     以后把 `inner` 拆成更多子模块，调用方代码不用改。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 42
/// ```
fn exercise_09_04_pub_use_reexport() -> i32 {
    // inner 与 facade 必须是同一个外层模块 outer 下的兄弟模块，
    // facade 里的 super::inner 才指得到（否则 super 会解析到 crate 根，报 E0432）
    mod outer {
        pub(crate) mod inner {
            // 字段私有：只能通过方法访问
            pub(crate) struct Api {
                value: i32,
            }

            impl Api {
                pub fn new() -> Self {
                    Api { value: 40 }
                }

                pub fn run(&self) -> i32 {
                    self.value + 2
                }
            }
        }

        pub(crate) mod facade {
            // Api 本身是 pub(crate)，重导出也只能用 pub(crate)（放大成 pub 会报 E0365）
            pub(crate) use super::inner::Api;
        }
    }

    // 调用方只写 facade::Api，完全不需要知道 inner 的存在
    outer::facade::Api::new().run()
}

// ===========================================================================
// kp_09_05 模块路径：super:: / self:: / crate::
// ===========================================================================

/// 知识点考核：在嵌套模块里分别用 `super::`、`self::`、`crate::` 三种路径取值。
#[test]
fn kp_09_05_module_paths() {
    assess(
        M,
        "kp_09_05",
        "模块路径：`super::` 父模块、`self::` 当前模块、`crate::` 绝对路径（crate 根）",
        Kind::Core,
        "复习 lesson_09 示例 5（module_paths）：`crate::` 从 crate 根出发，任何模块里都能用；\
         `self::foo` 与 `foo` 等价，强调「在本模块里找」；`super::foo` 往上一级找，\
         子模块因此能访问父模块里连 `pub` 都没有的私有项。",
        || {
            // 正常用例：super:: 6 + self:: 5 + crate:: 9 = 20
            eq(
                exercise_09_05_module_paths(),
                20i32,
                "super::BASE(6) + self::LOCAL(5) + crate::M.len()(9) = 20",
            );
            // 边界用例：结果必须是编译期确定值，重复调用完全一致
            eq(
                exercise_09_05_module_paths(),
                20i32,
                "三种路径取到的都是常量与本文件常量 M（\"lesson_09\" 长 9），结果必须稳定为 20",
            );
        },
    );
}

/// 【待实现】用三种路径取三个值相加，返回 20。
///
/// 实现要求（建议方案）：
///   - 定义 `mod outer { ... }`，在 `outer` 里写 `const BASE: i32 = 6;`（私有即可）；
///   - 在 `outer` 里的嵌套模块 `mod inner { ... }` 中写一个函数：
///     - `super::BASE` —— **父模块**（outer）的私有常量也能读到，值 6；
///     - `self::LOCAL` —— **当前模块**（inner）自己的常量 `const LOCAL: i32 = 5;`；
///     - `crate::M.len() as i32` —— **crate 根**的常量 `M`（本文件顶部 `const M: &str = "lesson_09";`），
///       长度是 9（9 个 ASCII 字节 = 9 个字符）；
///   - 让 `inner` 里的函数返回这三者之和，再通过一个 `pub(crate)` 的路径从函数外调用它；
///   - 返回总和 `6 + 5 + 9 = 20`；`crate::` 在**函数体内的模块**里同样指向 crate 根，
///     也就是本文件的根模块，所以 `crate::M` 取到的就是 `"lesson_09"`；
///   - 在实现注释里逐个说明：`super::` 往上一级找、`self::` 在本模块找、`crate::` 从根找。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// 20
/// ```
fn exercise_09_05_module_paths() -> i32 {
    mod outer {
        // 私有常量：子模块用 super:: 照样读得到
        const BASE: i32 = 6;

        // 模块要 pub(crate) 才能在函数体里写 outer::inner::...
        pub(crate) mod inner {
            const LOCAL: i32 = 5;

            pub(crate) fn sum_paths() -> i32 {
                // super:: 往上一级（outer）找私有常量 6
                // self:: 明确「在本模块里找」LOCAL = 5
                // crate:: 从 crate 根找常量 M = "lesson_09"，长度 9
                super::BASE + self::LOCAL + crate::M.len() as i32
            }
        }
    }

    // 6 + 5 + 9 = 20
    outer::inner::sum_paths()
}

// ===========================================================================
// kp_09_06 迷你库：pub mod 公开 API + 私有辅助函数 + pub use 重导出
// ===========================================================================

/// 知识点考核：组织一个迷你计算器「库」，返回计算结果与版本字符串。
#[test]
fn kp_09_06_mini_library() {
    assess(
        M,
        "kp_09_06",
        "迷你库：`pub mod` 暴露 API、私有辅助函数隐藏实现、`pub use` 汇总成一条门面",
        Kind::Hard,
        "复习 lesson_09 示例 8（mini_library_scenario）：一个「库」对外只需要稳定、简短的公开 API；\
         内部模块划分、私有字段、私有辅助函数都可以随时重构，只要门面 `pub use` 的路径不变。",
        || {
            let (total, version) = exercise_09_06_mini_library();
            // 正常用例：计算部分
            eq(
                total,
                10i32,
                "facade::total(&[1, 4, 5]) 应得到 1 + 4 + 5 = 10",
            );
            // 正常用例：版本字符串必须与实现里声明的完全一致
            eq(
                version,
                String::from("mini-calc 1.0"),
                "版本字符串必须逐字符等于 \"mini-calc 1.0\"（否则版本约定就形同虚设）",
            );
            // 边界用例：版本字符串不是空串，且包含库名
            let (_, version_again) = exercise_09_06_mini_library();
            is_false(
                version_again.is_empty(),
                "版本字符串不能为空：对外发布的库必须能报出自己的版本",
            );
            is_true(
                version_again.contains("mini-calc"),
                "版本字符串里要包含库名 mini-calc，方便调用方识别",
            );
        },
    );
}

/// 【待实现】组织一个迷你计算器库，返回 `(计算结果, 版本字符串)`。
///
/// 实现要求（建议方案，模块层级如下）：
///   - `pub(crate) mod lib { ... }`，其中：
///     1. `pub(crate) mod math { ... }`：
///        - 私有辅助函数 `fn clamp(value: i32) -> i32 { value.clamp(-100, 100) }`
///          —— 模块私有，模块外无法调用（这条正是「隐藏实现细节」的演示）；
///        - `pub fn sum(values: &[i32]) -> i32 { values.iter().map(|v| clamp(*v)).sum() }`
///          —— 公开 API，内部调用私有辅助函数；
///     2. `pub mod facade { ... }`：
///        - `pub use super::math::sum as total;` —— **重导出并改名**，调用方只记 `facade::total`；
///        - `pub const VERSION: &str = "mini-calc 1.0";` —— 库对外声明的版本；
///   - 用 `lib::facade::total(&[1, 4, 5])` 得到 10，用 `String::from(lib::facade::VERSION)` 得到版本串；
///   - 返回 `(10, "mini-calc 1.0")`；
///   - `pub use` 的目标必须**至少和重导出同样可见**：`sum` 是 `pub`（在 `pub(crate) mod math` 里），
///     所以 `pub use super::math::sum as total;` 合法；如果把 `sum` 写成 `pub(crate)` 再用 `pub use`
///   导出，就会报 `error[E0364]: sum is only public within the crate, and cannot be re-exported outside`；
///   - `facade` 必须与 `math` 是**兄弟模块**（都放在 `lib` 里），`super::math` 才指得到；
///     若把 `mod math` 与 `mod facade` 直接写在函数体里，`facade` 里的 `super` 会解析到 crate 根，
///     报 `error[E0432]: unresolved import super::math`；
///   - 本题的 `clamp` 在当前数据上不会改变结果（1/4/5 都在 -100..=100 内），它的作用是演示
///     「私有辅助函数」，请不要把它删掉，也不要在 `sum` 里绕过它直接求和。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// (10, "mini-calc 1.0")
/// ```
fn exercise_09_06_mini_library() -> (i32, String) {
    // math 与 facade 都是 lib 的子模块（兄弟），facade 里的 super::math 才指得到
    pub(crate) mod lib {
        pub(crate) mod math {
            // 私有辅助函数：math 之外无法调用（演示「隐藏实现细节」）
            fn clamp(value: i32) -> i32 {
                value.clamp(-100, 100)
            }

            // 公开 API：内部始终经过私有的 clamp
            pub fn sum(values: &[i32]) -> i32 {
                values.iter().map(|v| clamp(*v)).sum()
            }
        }

        pub mod facade {
            // 重导出并改名：调用方只记 facade::total；
            // sum 是 pub，所以 pub use 合法（若 sum 是 pub(crate) 会报 E0364）
            pub use super::math::sum as total;

            pub const VERSION: &str = "mini-calc 1.0";
        }
    }

    // (1 + 4 + 5, "mini-calc 1.0")
    (
        lib::facade::total(&[1, 4, 5]),
        String::from(lib::facade::VERSION),
    )
}

// ===========================================================================
// kp_09_07 常见错误诊断：模块系统的错误编号
// ===========================================================================

/// 知识点考核：写出课程示例 7 前三个错误场景对应的编译器错误编号。
#[test]
fn kp_09_07_diagnose_errors() {
    assess(
        M,
        "kp_09_07",
        "常见错误诊断：E0603（私有项）/ E0425（路径写错）/ E0255（use 与本地定义同名）",
        Kind::Hard,
        "复习 lesson_09 示例 7（common_mistakes）：错误 1 是「调用私有函数或读取私有常量」\
         （E0603），错误 2 是「`pub(super)` 项被更外层的模块调用」——编译器同样报 E0603，\
         错误 3 是「路径写错，少了层级」的 `lessons::add`（E0425）。",
        || {
            eq_slice(
                &exercise_09_07_diagnose_errors(),
                &["E0603", "E0425", "E0255"],
                "顺序必须是：私有项不可访问（E0603）、路径里找不到名字（E0425）、\
                 use 与本地定义同名（E0255）",
            );
        },
    );
}

/// 【待实现】写出三个模块场景对应的编译器错误编号。
///
/// 场景（与课程示例 7 的错误 1 / 2、错误 4、错误 6 一致，按课程注释顺序）：
///   1. `lessons::calc::guard(1.0);` —— 调用了没有 `pub` 的私有函数（读私有常量
///      `lessons::calc::INTERNAL_TAG` 也是同一个编号）；
///   2. `lessons::calc::sub(1.0, 2.0);` —— `sub` 是 `pub(super)`，只对父模块 `lessons`
///      及其后代可见，从 crate 根调用时编译器仍按「私有」报错（**同一个编号**）；
///   3. `use lessons::calc::add;` 与本地 `fn add(a, b)` 同时存在
///      —— 名字被定义了两次。
///
/// 实现要求：返回 3 个错误编号字符串（形如 `"E0603"`），顺序与上面一致。
///   - 前两个场景编号相同并不是笔误：Rust 对「可见性不足」统一使用 E0603；
///   - 课程示例 7 的注释里逐步列出了 E0603 / E0425 / E0255 / E0364 / E0583，
///     本题只取课程注释顺序里的**前三个不同编号**：
///     E0603（错误 1、2）、E0425（错误 4）、E0255（错误 6）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0603", "E0425", "E0255"]
/// ```
fn exercise_09_07_diagnose_errors() -> [&'static str; 3] {
    // 1/2. 私有项 / pub(super) 项被外层调用 —— 可见性不足统一报 E0603
    // 3.    路径里找不到名字（少了层级，如 lessons::add）—— E0425
    // 4.    use 与本地定义同名 —— E0255
    ["E0603", "E0425", "E0255"]
}
