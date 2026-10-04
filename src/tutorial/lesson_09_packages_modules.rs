//! lesson_09_packages_modules.rs —— 主题：包和模块（crate / package / mod / 可见性 / use 与路径）
//!
//! 学习目标：
//!   1. 分清 package（Cargo 包）、crate（编译单元）、module（模块）三个概念
//!   2. 会在同一文件内用嵌套 `mod` 组织代码，掌握 `pub` / `pub(crate)` / `pub(super)` 的可见性层级
//!   3. 会用 `use` 引入路径、用 `as` 重命名、用 `pub use` 重导出
//!   4. 会用 `crate::` / `self::` / `super::` 三种相对与绝对路径
//!   5. 知道真实项目如何把模块拆成 `xxx/mod.rs` + 子文件（本文件仅用注释演示，不创建额外文件）
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_09_packages_modules.rs -o lesson_09 && ./lesson_09
//!   或在本项目根目录执行：cargo run --bin lesson_09_packages_modules
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       本文件演示的是「同一个文件内的嵌套模块」，原因见示例 6 的目录树说明。

fn main() {
    println!("========== lesson_09_packages_modules：包和模块 ==========\n");

    demo_1_module_basics();
    demo_2_visibility_levels();
    demo_3_use_and_rename();
    demo_4_pub_use_reexport();
    demo_5_module_paths();
    demo_6_file_and_directory_modules();
    demo_7_common_mistakes();
    demo_8_mini_library_scenario();
}

// ============================================================================
// 模块树总览（本文件真正实现的部分）：
//
//   crate（本文件编译出来的可执行文件）
//   └── lessons                  （用 inline mod 写的“分类”模块）
//       ├── calc                 （计算器迷你库：pub / pub(crate) / pub(super) 三种可见性）
//       │   └── shapes           （calc 的子模块，演示 super:: 访问父模块私有个体）
//       ├── logger               （日志计数模块：演示模块内私有静态状态）
//       ├── stats                （统计模块：由若干 impl 块组成的一个完整小模块）
//       └── api                  （门面模块：用 pub use 把内部实现重导出为“公开 API”）
// ============================================================================

/// 顶层模块：把本课所有示例模块收纳在一个「分类」模块里。
///
/// 要点：inline 模块用 `mod 名字 { ... }` 写在与 main 同一个文件中，
/// 适合「示例集中在一个文件」的教学场景；真实项目通常拆成独立文件（见示例 6）。
mod lessons {
    /// 日志计数模块：演示模块内部的私有状态（模块外只能通过公开函数访问）。
    pub mod logger {
        // 模块可以有自己的 use；本模块只需要格式化，所以只引入 fmt
        use std::sync::atomic::{AtomicUsize, Ordering};

        /// 计数器是模块私有项：外部只能用 log_info/log_error 间接影响它
        static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

        /// 记录一条 INFO 日志（演示用，只累加计数并返回消息）
        pub fn log_info(message: &str) -> String {
            record(); // 调用同模块内的私有函数，外部无法直接调用 record
            format!("[INFO] {message}")
        }

        /// 记录一条 ERROR 日志
        pub fn log_error(message: &str) -> String {
            record();
            format!("[ERROR] {message}")
        }

        /// 查询累计日志条数：用 `pub(crate)` 表示「整个 crate 可见，但不对外暴露」
        pub(crate) fn count() -> usize {
            LOG_COUNT.load(Ordering::Relaxed)
        }

        /// 私有函数：只有 logger 模块及其子模块能调用
        fn record() {
            LOG_COUNT.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// 计算器迷你库：用一个模块演示三种可见性。
    pub mod calc {
        // 只引入本模块需要的一项，避免把整个 prelude 风格的东西带进来
        use std::fmt;

        /// `pub`：任何地方都能访问（这里演示模块常量）
        pub const PI_APPROX: f64 = 3.141_592_653_589_793;

        /// `pub(crate)`：只在本 crate 内可见，适合「内部共享但不打算公开」的常量
        pub(crate) const MAX_INPUT_LEN: usize = 64;

        /// 私有常量：只有 calc 模块内部可用，用来演示「私有项可以被同级/子级使用」
        const INTERNAL_TAG: &str = "calc";

        /// `pub` 函数：加法，任何地方都能调用。
        /// 注意它本身只用到了两个参数；圆周率常量 PI_APPROX 是在子模块
        /// `shapes::circle_area` 里通过 `super::PI_APPROX` 使用的。
        pub fn add(left: f64, right: f64) -> f64 {
            left + right
        }

        /// `pub(super)` 函数：只对「父模块 lessons」及其下面的模块可见。
        /// `main` 处在 crate 根，不是 lessons 的子模块，因此 main 不能直接调用 sub，
        /// 必须在 lessons 内部（例如 calc::shapes）转发。
        pub(super) fn sub(left: f64, right: f64) -> f64 {
            guard(left - right) // 私有函数 guard 只能被本模块内部调用
        }

        /// `pub(crate)` 函数：crate 内任何模块都能调用（示例 2 会从 lessons 里调用它）
        pub(crate) fn mul(left: f64, right: f64) -> f64 {
            left * right
        }

        /// 私有函数：演示「私有项不能被外部模块调用」
        fn guard(value: f64) -> f64 {
            if value.is_nan() { 0.0 } else { value } // NaN 兜底，避免脏数据扩散
        }

        /// 子模块：演示 `super::` 访问父模块的项（包括私有项与常量）
        pub mod shapes {
            /// 圆的面积：用 `super::PI_APPROX` 取父模块（calc）的常量，
            /// 再用 `super::mul` 调用父模块的 `pub(crate)` 函数
            pub fn circle_area(radius: f64) -> f64 {
                super::mul(super::PI_APPROX, radius * radius)
            }

            /// 演示子模块能访问父模块的**私有**项：`super::INTERNAL_TAG`
            pub fn tag() -> &'static str {
                super::INTERNAL_TAG
            }

            /// 演示把父模块的 `pub(super)` 函数重新暴露给 crate 根：
            /// 子模块里 `sub` 是可见的（因为 super 就是它的父模块），
            /// 这里用一个 `pub` 包装转发，main 就能间接用到它。
            pub fn difference(left: f64, right: f64) -> f64 {
                super::sub(left, right)
            }
        }

        /// 演示 `impl fmt::Display` 给本模块的类型用（类型与实现都放在同一模块内）
        pub struct Ratio {
            /// 分子
            pub top: f64,
            /// 分母
            pub bottom: f64,
        }

        impl fmt::Display for Ratio {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // 分母为 0 时显示 "∞"，否则正常输出小数
                if self.bottom == 0.0 {
                    write!(f, "∞")
                } else {
                    write!(f, "{:.3}", self.top / self.bottom)
                }
            }
        }
    }

    /// 统计模块：演示一个模块里放类型 + 多个 impl 块 + 关联函数。
    pub mod stats {
        /// 运行统计结果：字段全部私有，只能通过方法访问
        #[derive(Debug)]
        pub struct Stats {
            sum: f64,   // 总和
            count: u32, // 样本数
            len: u32,   // 预期的样本容量（来自构造参数，演示「私有字段也要被用到」）
        }

        impl Stats {
            /// 关联函数（不带 self）：按预期容量创建统计器
            pub fn with_capacity(len: u32) -> Self {
                // 字段初始化简写 + 需要显式给出的字段
                Stats {
                    sum: 0.0,
                    count: 0,
                    len,
                }
            }

            /// 旁路构造：直接给出总和与样本数。
            /// 注意可见性陷阱：这里若写成 `pub(super)`，`super` 是 stats 的父模块 lessons，
            /// 于是 crate 根的 main 就调用不到它（会报 E0624：associated function is private）。
            /// 因此这里用 `pub(crate)`——「整个 crate 可见」才是本示例需要的范围。
            pub(crate) fn from_totals(sum: f64, count: u32) -> Self {
                Stats {
                    sum,
                    count,
                    len: count, // 这里容量就等于样本数
                }
            }

            /// 插入一个样本值
            pub fn push(&mut self, value: f64) {
                self.sum += value;
                self.count += 1;
            }

            /// 平均值：没有样本时返回 None，而不是用 0 掩盖问题
            pub fn mean(&self) -> Option<f64> {
                if self.count == 0 {
                    None
                } else {
                    Some(self.sum / f64::from(self.count))
                }
            }

            /// 已收集样本数
            pub fn count(&self) -> u32 {
                self.count
            }

            /// 是否已装满（用到私有字段 len，演示内部一致性校验）
            pub fn is_full(&self) -> bool {
                self.count >= self.len
            }

            /// 清空重新计数（`pub`，main 会直接调用，验证它确实可用）
            pub fn reset(&mut self) {
                self.sum = 0.0;
                self.count = 0;
            }
        }
    }

    /// 门面模块：只做重导出，把「内部分散的实现」汇总成一条公开 API。
    ///
    /// 重导出的可见性规则（本文件实际踩到过的坑）：`pub use` 的目标必须是 `pub` 项。
    /// 若把 `pub(crate)` 的函数重导出成 `pub`，编译器会报
    /// `error[E0364]: ... is only public within the crate, and cannot be re-exported outside`。
    /// 所以这里只重导出真正 `pub` 的项，并顺便演示「重导出时改名」。
    pub mod api {
        // 说明：这里不用 `use ... as _` 之类的技巧，全部重导出并在示例中被真实调用
        pub use super::calc::{PI_APPROX as PI, Ratio, add};
        pub use super::logger::{log_error, log_info};
        pub use super::stats::Stats;
    }

    /// 演示 `crate::` 绝对路径：从 crate 根出发写完整路径，任何模块里都能用。
    /// 注意：即使本文件是单文件二进制，`crate::` 依然指向「本文件的根模块」。
    pub fn call_via_crate_path(left: f64, right: f64) -> f64 {
        crate::lessons::calc::add(left, right)
    }

    /// 演示 `self::` 路径：`self` 指当前模块（lessons），
    /// 所以 `self::logger::log_info` 与 `logger::log_info` 等价。
    pub fn call_via_self_path(message: &str) -> String {
        self::logger::log_info(message)
    }
}

/// 示例 1：模块基础 —— 用 `mod` 组织代码并调用
///
/// 要点：模块名用蛇形命名，路径用 `::` 分隔；模块内的 `pub fn` 才能从外部调用。
fn demo_1_module_basics() {
    println!("--- 示例 1：模块基础 ---");

    // 从 crate 根写完整路径调用 lessons::calc 里的函数
    let sum = lessons::calc::add(1.5, 2.5);
    println!("lessons::calc::add(1.5, 2.5) = {sum}");
    println!("// 预期输出：lessons::calc::add(1.5, 2.5) = 4");

    // 乘法的可见性是 pub(crate)（crate 内可见），从 main 调用同样合法
    let product = lessons::calc::mul(3.0, 4.0);
    println!("lessons::calc::mul(3.0, 4.0) = {product}");
    println!("// 预期输出：lessons::calc::mul(3.0, 4.0) = 12");

    // 模块常量：pub 常量任何位置都能读
    println!(
        "lessons::calc::PI_APPROX = {}（pub 常量）",
        lessons::calc::PI_APPROX
    );
    println!("// 预期输出：lessons::calc::PI_APPROX = 3.141592653589793（pub 常量）");
    // pub(crate) 常量同样能在 crate 内读到；外部 crate 就不能了
    println!(
        "lessons::calc::MAX_INPUT_LEN = {}（pub(crate)：本 crate 内可见）",
        lessons::calc::MAX_INPUT_LEN
    );
    println!("// 预期输出：lessons::calc::MAX_INPUT_LEN = 64（pub(crate)：本 crate 内可见）");

    // 多级嵌套：calc::shapes 是 calc 的子模块，路径会一层层变长
    println!(
        "半径 1 的圆面积 = {:.6}",
        lessons::calc::shapes::circle_area(1.0)
    );
    println!("// 预期输出：半径 1 的圆面积 = 3.141593");
    println!(
        "子模块通过 super:: 读到父模块私有常量 INTERNAL_TAG = {}",
        lessons::calc::shapes::tag()
    );
    println!("// 预期输出：子模块通过 super:: 读到父模块私有常量 INTERNAL_TAG = calc");

    // 模块内的类型 + impl：Display 实现在 calc 模块里
    let ratio = lessons::calc::Ratio {
        top: 22.0,
        bottom: 7.0,
    };
    println!("Ratio 的 Display = {ratio}（约等于圆周率近似值）");
    println!("// 预期输出：Ratio 的 Display = 3.143（约等于圆周率近似值）");

    // 日志模块：私有静态计数器只能通过公开函数间接改变
    println!("{}", lessons::logger::log_info("模块可以隐藏内部状态"));
    println!("// 预期输出：[INFO] 模块可以隐藏内部状态");
    println!(
        "{}",
        lessons::logger::log_error("模块内部状态只能被模块代码修改")
    );
    println!("// 预期输出：[ERROR] 模块内部状态只能被模块代码修改");
    println!(
        "累计日志条数（通过 pub(crate) 函数 count 查询）= {}",
        lessons::logger::count()
    );
    println!("// 预期输出：累计日志条数（通过 pub(crate) 函数 count 查询）= 2");
}

/// 示例 2：可见性层级 —— pub / pub(crate) / pub(super) / 私有
///
/// 要点：Rust 的可见性是「从定义处向外的范围」：
///   - `pub`        任何地方都可见
///   - `pub(crate)` 只在当前 crate 内可见
///   - `pub(super)` 只在父模块及其后代中可见
///   - 不加修饰     只有本模块及其子模块可见
fn demo_2_visibility_levels() {
    println!("--- 示例 2：可见性层级 ---");

    // pub：从 crate 根调用完全合法
    println!("pub 函数 add(2, 3) = {}", lessons::calc::add(2.0, 3.0));
    println!("// 预期输出：pub 函数 add(2, 3) = 5");

    // pub(crate)：crate 根也在「本 crate 内」，所以合法
    println!(
        "pub(crate) 函数 mul(2, 3) = {}",
        lessons::calc::mul(2.0, 3.0)
    );
    println!("// 预期输出：pub(crate) 函数 mul(2, 3) = 6");

    // pub(super)：直接写 lessons::calc::sub(...) 会编译失败（见本函数注释中的错误示例），
    // 正确做法是通过同处于 lessons 内部的 shapes::difference 转发
    println!(
        "pub(super) 函数 sub 经由 shapes::difference 转发 = {}",
        lessons::calc::shapes::difference(10.0, 4.0)
    );
    println!("// 预期输出：pub(super) 函数 sub 经由 shapes::difference 转发 = 6");

    // 私有函数 guard 完全无法从外部访问，只能被 calc 模块内部（例如 sub）调用。
    // 它只做一件事：值为 NaN 时返回 0.0，其余情况原样返回。
    // 注意：上面的 difference(10, 4) = 6 是普通减法的结果，并没有走到 NaN 兜底分支。

    // pub(crate) 的另一种用法：在 lessons 内部调用 crate 可见的统计构造函数
    let mut stats = lessons::stats::Stats::with_capacity(2);
    stats.push(1.0);
    stats.push(3.0);
    println!(
        "with_capacity(2) 的统计结果 count = {}，mean = {:?}，is_full = {}",
        stats.count(),
        stats.mean(),
        stats.is_full()
    );
    println!(
        "// 预期输出：with_capacity(2) 的统计结果 count = 2，mean = Some(2.0)，is_full = true"
    );

    // pub(crate) 关联函数 from_totals：整个 crate（包括 crate 根的 main）都能直接调用
    let seeded = lessons::stats::Stats::from_totals(9.0, 3);
    println!("from_totals(9.0, 3) 的 mean = {:?}", seeded.mean());
    println!("// 预期输出：from_totals(9.0, 3) 的 mean = Some(3.0)");

    // logger::record 是私有的，外部无法调用；只能通过 log_info / log_error 间接触发
    let _ = lessons::logger::log_info("可见性演示：外部无法直接调用私有的 record()");
    println!(
        "日志条数增加到 {}（每次 log_* 调用内部私有 record 一次）",
        lessons::logger::count()
    );
    println!("// 预期输出：日志条数增加到 3（每次 log_* 调用内部私有 record 一次）");

    // 下面这些写法都会编译失败，仅作说明：
    // lessons::calc::guard(1.0);
    // error[E0603]: function `guard` is private
    // lessons::calc::INTERNAL_TAG;
    // error[E0603]: constant `INTERNAL_TAG` is private
    // lessons::logger::record();
    // error[E0603]: function `record` is private
    // lessons::calc::sub(1.0, 2.0);
    // error[E0603]: function `sub` is private（pub(super) 对 crate 根不可见，故提示为 private）
}

/// 示例 3：`use` 引入与 `as` 重命名
///
/// 要点：`use` 只是「路径别名」，不改变可见性；`as` 可以解决同名冲突或起更短的别名。
/// 注意 `use` 写在函数体内时作用域仅限该函数，写在模块顶层时作用于整个模块。
fn demo_3_use_and_rename() {
    println!("--- 示例 3：use 与 as 重命名 ---");

    // 函数作用域内的 use：只在本次调用内有效，不会污染整个文件
    use lessons::calc::Ratio as CalcRatio;
    use lessons::calc::add as plus;
    use lessons::stats::Stats;

    // 别名调用：plus 就是 add
    println!("use ... as plus 调用 plus(5, 6) = {}", plus(5.0, 6.0));
    println!("// 预期输出：use ... as plus 调用 plus(5, 6) = 11");

    // 类型别名：CalcRatio 就是 lessons::calc::Ratio
    let r = CalcRatio {
        top: 1.0,
        bottom: 8.0,
    };
    println!("CalcRatio 别名实例 = {r}");
    println!("// 预期输出：CalcRatio 别名实例 = 0.125");

    // 同名项冲突时用 as 区分：这里把模块本身也起了别名
    use lessons::logger as log;
    println!(
        "模块别名 log::log_info -> {}",
        log::log_info("模块也能起别名")
    );
    println!("// 预期输出：模块别名 log::log_info -> [INFO] 模块也能起别名");

    // 大括号分组写多个 use 项，减少重复前缀
    use lessons::calc::{MAX_INPUT_LEN, PI_APPROX};
    println!("分组引入的常量 PI_APPROX = {PI_APPROX:.4}，MAX_INPUT_LEN = {MAX_INPUT_LEN}");
    println!("// 预期输出：分组引入的常量 PI_APPROX = 3.1416，MAX_INPUT_LEN = 64");

    // 用 use 引入的 Stats 直接构造，路径更短
    let mut stats = Stats::with_capacity(4);
    for value in [1.0, 2.0, 3.0, 4.0] {
        stats.push(value);
    }
    println!(
        "用 use 引入后 Stats 的 mean = {:?}，count = {}",
        stats.mean(),
        stats.count()
    );
    println!("// 预期输出：用 use 引入后 Stats 的 mean = Some(2.5)，count = 4");

    // 通配引入 `use lessons::calc::*;` 也能工作，但会降低可读性、容易撞名，
    // 因此本课一律使用显式引入（若确实需要，可写成 use lessons::calc::*;）。
}

/// 示例 4：`pub use` 重导出 —— 把内部实现包装成公开 API
///
/// 要点：`pub use` 让外部使用者只记住一条短路径，内部可以自由重构模块结构。
/// 这是库设计里最常见的「门面（facade）」手法。
fn demo_4_pub_use_reexport() {
    println!("--- 示例 4：pub use 重导出 ---");

    // api 模块本身只是一堆 pub use，但用起来就像一条独立的公开 API
    println!(
        "api::add(7, 8) = {}（实际实现位于 lessons::calc::add）",
        lessons::api::add(7.0, 8.0)
    );
    println!("// 预期输出：api::add(7, 8) = 15（实际实现位于 lessons::calc::add）");
    println!(
        "calc::mul(7, 8) = {}（pub(crate) 项不能被 pub use 到 crate 外，故这里走原路径）",
        lessons::calc::mul(7.0, 8.0)
    );
    println!(
        "// 预期输出：calc::mul(7, 8) = 56（pub(crate) 项不能被 pub use 到 crate 外，故这里走原路径）"
    );
    println!(
        "api::PI = {}（重导出时用 as 改了名字：PI_APPROX -> PI）",
        lessons::api::PI
    );
    println!("// 预期输出：api::PI = 3.141592653589793（重导出时用 as 改了名字：PI_APPROX -> PI）");

    // 类型重导出：调用方不需要知道 Ratio 原本住在 lessons::calc 里
    let r = lessons::api::Ratio {
        top: 1.0,
        bottom: 3.0,
    };
    println!("api::Ratio 实例 = {r}");
    println!("// 预期输出：api::Ratio 实例 = 0.333");

    // 函数重导出：日志三件套都挂在 api 上
    println!("{}", lessons::api::log_info("经 api 门面输出"));
    println!("// 预期输出：[INFO] 经 api 门面输出");
    println!("{}", lessons::api::log_error("经 api 门面输出错误"));
    println!("// 预期输出：[ERROR] 经 api 门面输出错误");
    println!(
        "lessons::logger::count() = {}（pub(crate) 项可直接调用，但不能被 pub use 出去）",
        lessons::logger::count()
    );
    println!(
        "// 预期输出：lessons::logger::count() = 6（pub(crate) 项可直接调用，但不能被 pub use 出去）"
    );

    // 想给一个「内部名字」起更漂亮的对外名字时用 pub use ... as ...，例如：
    // pub use super::logger::count as log_count;   // 若 count 是 pub(crate)，这句会得到：
    // error[E0364]: `count` is only public within the crate, and cannot be re-exported outside
    // 修正 A：把 count 改成 pub 再重导出；修正 B：不改可见性，直接使用原路径（本文件采用 B）

    // 结构体重导出
    let mut stats = lessons::api::Stats::with_capacity(3);
    stats.push(2.0);
    stats.push(4.0);
    stats.push(6.0);
    println!(
        "api::Stats 的 mean = {:?}，full = {}",
        stats.mean(),
        stats.is_full()
    );
    println!("// 预期输出：api::Stats 的 mean = Some(4.0)，full = true");
    stats.reset(); // reset 是 pub，重导出后同样可用
    println!(
        "reset 之后 count = {}，mean = {:?}",
        stats.count(),
        stats.mean()
    );
    println!("// 预期输出：reset 之后 count = 0，mean = None");

    // 重导出的好处（注释说明）：内部把 calc 拆成 calc::basic 与 calc::advanced 后，
    // 只要 api 里补一行 `pub use super::calc::basic::add;`，调用方代码完全不用改。
}

/// 示例 5：路径 crate / self / super
///
/// 要点：`crate::` 是绝对路径（crate 根），`self::` 是当前模块，`super::` 是父模块。
/// 三者都能出现在 `use` 语句与表达式路径中。
fn demo_5_module_paths() {
    println!("--- 示例 5：crate / self / super 路径 ---");

    // crate:: 绝对路径：lessons 模块里的函数内部就是这么写的
    println!(
        "crate::lessons::calc::add 的结果 = {}",
        lessons::call_via_crate_path(20.0, 22.0)
    );
    println!("// 预期输出：crate::lessons::calc::add 的结果 = 42");

    // self:: 相对路径：self 指 lessons，于是等价于 lessons::logger::log_info
    println!(
        "self::logger::log_info 的结果 = {}",
        lessons::call_via_self_path("self 指当前模块")
    );
    println!("// 预期输出：self::logger::log_info 的结果 = [INFO] self 指当前模块");

    // 从 crate 根用绝对路径读常量
    println!(
        "crate::lessons::calc::PI_APPROX = {:.4}",
        crate::lessons::calc::PI_APPROX
    );
    println!("// 预期输出：crate::lessons::calc::PI_APPROX = 3.1416");

    // super:: 在 main 这一层不适用（main 处于 crate 根，没有父模块），
    // 因此这里演示「在 lessons::calc::shapes 内部」用 super:: 访问父模块的结果：
    println!(
        "shapes 内部 super::mul 计算圆面积 = {:.6}",
        lessons::calc::shapes::circle_area(2.0)
    );
    println!("// 预期输出：shapes 内部 super::mul 计算圆面积 = 12.566371");

    // 混用：在同一表达式里既用绝对路径又用 use 别名
    use lessons::calc::add;
    println!(
        "use 别名 add(1, 1) = {}，绝对路径 crate::lessons::calc::add(1, 1) = {}",
        add(1.0, 1.0),
        crate::lessons::calc::add(1.0, 1.0)
    );
    println!("// 预期输出：use 别名 add(1, 1) = 2，绝对路径 crate::lessons::calc::add(1, 1) = 2");

    // 路径也可以用在本模块中声明的类型上：请注意 main 所在模块（crate 根）的 self 就是 crate
    println!(
        "self::lessons::calc::MAX_INPUT_LEN = {}",
        self::lessons::calc::MAX_INPUT_LEN
    );
    println!("// 预期输出：self::lessons::calc::MAX_INPUT_LEN = 64");
}

/// 示例 6：把模块拆分为文件与目录模块（本文件只做注释演示，不创建额外文件）
///
/// 要点：教学示例把模块写在同一个文件里，方便一次编译阅读；真实项目会把每个模块
/// 放进独立文件。规则只有两条：
///   1. `mod foo;` 声明后，编译器去 `foo.rs` 或 `foo/mod.rs` 找 `foo` 模块的内容；
///   2. 文件里的内容等价于写在 `mod foo { ... }` 的大括号里，**子模块声明再套一层目录**。
fn demo_6_file_and_directory_modules() {
    println!("--- 示例 6：文件与目录模块（目录树见注释） ---");

    // 下面是本项目若真的拆分模块时的标准目录结构（推荐 mod.rs 风格，全部以注释形式给出）：
    //
    // src/
    // ├── tutorial/
    // │   ├── lesson_09_packages_modules.rs <- 本文件（可继续保留 inline 模块用于教学）
    // │   └── lessons_lib/               <- 目录模块：与「模块名」同名，用 mod.rs 作入口
    // │       ├── mod.rs                 <- 声明子模块 + 重导出：
    // │       │                          //     pub mod calc;
    // │       │                          //     pub mod logger;
    // │       │                          //     pub mod stats;
    // │       │                          //     pub mod api;
    // │       │                          //     pub use calc::add;   // 门面重导出
    // │       ├── calc.rs                <- 对应「同一个文件内嵌套模块」里的 mod calc { ... }
    // │       ├── calc/
    // │       │   └── shapes.rs          <- calc 的子模块：写成 calc/shapes.rs
    // │       ├── logger.rs
    // │       ├── stats.rs
    // │       └── api.rs
    // └── main.rs
    //
    // 在 lesson_09_packages_modules.rs 里引用它们只需要（注意：本文件并未真的创建这些文件）：
    // mod lessons_lib;
    // use lessons_lib::calc::add;
    //
    // 两种风格的等价关系（重点）：
    //   inline：mod calc { pub fn add(..) {} pub mod shapes { .. } }
    //   文件：  calc.rs           里写 pub fn add(..) 与 pub mod shapes;
    //           calc/shapes.rs    里写 shapes 的内容
    //   —— 也就是说，**目录名必须等于父模块名**，`mod.rs` 是父模块自己的内容。
    //
    // 另一种（2018 之后同样合法、现在更常用）的写法是不用 mod.rs，而是并列放
    // calc.rs + calc/ 目录：
    //   src/tutorial/lessons_lib/calc.rs
    //   src/tutorial/lessons_lib/calc/shapes.rs
    // 两种风格不要混用同一个模块，否则会出现「file for module found at both ...」错误。

    // 用调用真实模块的方式验证「文件模块 = inline 模块」这一等价关系：
    // 本文件的 inline 模块 lessons::calc::add 就扮演了 lessons_lib/calc.rs 的角色。
    let via_inline = lessons::calc::add(3.0, 0.14);
    println!("inline 模块 lessons::calc::add(3.0, 0.14) = {via_inline}");
    println!("// 预期输出：inline 模块 lessons::calc::add(3.0, 0.14) = 3.14");
    println!(
        "若拆成文件，同样的调用会写成 lessons_lib::calc::add(3.0, 0.14) = {}",
        via_inline
    );
    println!("// 预期输出：若拆成文件，同样的调用会写成 lessons_lib::calc::add(3.0, 0.14) = 3.14");
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("可见性规则不变（pub / pub(crate) / pub(super) / 私有），只是路径首段换成了模块名");
}

/// 示例 7：常见错误示例（错误代码全部注释掉）
///
/// 要点：模块系统相关的编译错误大多集中在「可见性」和「路径写错」两类，
/// 记住错误编号 E0603（私有项）与 E0432/E0433（路径解析）即可快速定位。
fn demo_7_common_mistakes() {
    println!("--- 示例 7：常见错误示例（代码已注释，仅作说明） ---");

    // 错误 1：调用私有函数 / 读取私有常量
    // lessons::calc::guard(1.0);
    // error[E0603]: function `guard` is private
    // lessons::calc::INTERNAL_TAG;
    // error[E0603]: constant `INTERNAL_TAG` is private
    // 修正：把项改成 pub / pub(crate)，或提供一个公开的包装函数转发调用
    println!(
        "修正方式示例 —— 通过公开的 shapes::tag() 读到私有常量 = {}",
        lessons::calc::shapes::tag()
    );
    println!("// 预期输出：修正方式示例 —— 通过公开的 shapes::tag() 读到私有常量 = calc");

    // 错误 2：pub(super) 项被更外层的模块调用
    // lessons::calc::sub(1.0, 2.0);
    // error[E0603]: function `sub` is private
    // 说明：pub(super) 的「super」是 calc 的父模块 lessons；crate 根（main）不在范围内
    // 修正 A：改成 pub(crate)；修正 B：在 lessons 内部提供一个 pub 包装（本文件用的是 B）
    println!(
        "修正方式示例 —— shapes::difference(10, 4) = {}",
        lessons::calc::shapes::difference(10.0, 4.0)
    );
    println!("// 预期输出：修正方式示例 —— shapes::difference(10, 4) = 6");

    // 错误 3：忘记 pub 就声明子模块，外部路径解析不到
    // mod outer {
    //     mod inner { pub fn hi() {} }
    //     pub fn call() { inner::hi(); } // 模块内部可见，这里没问题
    // }
    // outer::inner::hi();
    // error[E0603]: module `inner` is private
    // 修正：`pub mod inner { pub fn hi() {} }`
    // 本文件的 lessons 是 `mod lessons`（私有），但 main 与它同处 crate 根，
    // 因此在 crate 根仍然可以访问；若要给外部 crate 用，就必须写成 pub mod lessons。

    // 错误 4：路径写错（少了层级或多写了 crate::）
    // lessons::add(1.0, 2.0);
    // error[E0425]: cannot find function `add` in module `lessons`
    // 修正：add 定义在 lessons::calc 里，应写 lessons::calc::add(1.0, 2.0)
    println!(
        "修正方式示例 —— 写全路径 lessons::calc::add(1, 2) = {}",
        lessons::calc::add(1.0, 2.0)
    );
    println!("// 预期输出：修正方式示例 —— 写全路径 lessons::calc::add(1, 2) = 3");

    // 错误 5：use 引入的项与实际使用不匹配，导致「未使用导入」警告
    // use lessons::calc::mul;   // 若函数体里从未使用 mul，会得到：
    // warning: unused import: `lessons::calc::mul`
    // 修正：删除未使用的 use，或改成通配/别名后真正使用；
    //       本文件所有 use 都被真实使用，因此 rustc 编译为零警告。

    // 错误 6：use 与本地定义同名，出现二义性
    // use lessons::calc::add;
    // fn add(a: f64, b: f64) -> f64 { a + b }
    // error[E0255]: the name `add` is defined multiple times
    // 修正：用 as 起别名 `use lessons::calc::add as calc_add;`，或删除本地多余定义

    // 错误 7：mod 声明了却没有对应文件
    // mod calc;              // `mod calc;` 会在**本文件所在目录**寻找 calc.rs 或 calc/mod.rs
    //                        // 示例里没有创建这些文件，于是报：
    //                        // error[E0583]: file not found for module `calc`
    // 修正：补上同名文件（calc.rs 或 calc/mod.rs）后即可正常编译——
    //       文件模块在 rustc 单文件编译下同样可用，前提是文件确实存在于同一目录；
    //       本文件为了自包含，选择用 inline mod（内联模块）的写法。

    // 错误 8：可见性不能「放大」——不能把 crate 内部的项重导出给外部使用
    // mod a {
    //     pub(crate) struct Secret;   // 只在 crate 内可见
    //     pub mod b {
    //         pub use super::Secret;  // error[E0364]: `Secret` is only public within the crate,
    //     }                           //        and cannot be re-exported outside
    // }
    // 说明：E0364 用于「项」的可见性放大；如果是私有**模块**被重导出则报 E0365。
    // 修正：先把类型本身改成 pub，再重导出

    // 错误 9：`use` 放在函数内部却想给整个模块用
    // fn helper() { use lessons::calc::add; }
    // fn other() { add(1.0, 2.0); }   // error[E0425]: cannot find function `add`
    // 修正：把 use 提到模块顶层，或在使用它的函数内各自 use

    // 错误 10：edition 2024 里把 `gen` 当标识符使用
    // mod gen { pub fn run() {} }
    // error: expected identifier, found reserved keyword `gen`
    // 修正：改名（例如 generator），因为 Rust 2024 起 `gen` 是保留字
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("以上的错误写法全部被注释掉，本程序依然零错误零警告运行到这里");
}

/// 示例 8（典型使用场景）：在单文件内用嵌套模块模拟一个迷你库并提供公开 API
///
/// 要点：一个「库」需要的是稳定、简短的公开 API（api 模块），内部实现细节
/// （calc / logger / stats 的具体布局、私有字段、私有函数）随时可以重构。
fn demo_8_mini_library_scenario() {
    println!("--- 示例 8：典型使用场景 —— 单文件内的迷你库 ---");

    // 场景：一个成绩统计小库。调用方只使用 api 门面，完全不需要知道内部模块划分。
    let mut stats = lessons::api::Stats::with_capacity(5);

    // 用 api 暴露的计算函数把「原始分」换算成百分制（这里的换算只是演示）
    let raw_scores = [80.0, 92.5, 76.0, 88.5, 95.0];
    for raw in raw_scores {
        let normalized = lessons::calc::mul(raw, 1.0); // 换算：走 crate 内可见的内部函数
        stats.push(normalized);
    }

    // 平均值可能不存在，所以用 Option 处理，而不是 unwrap 赌它一定有值
    match stats.mean() {
        Some(mean) => {
            println!("5 个分数的平均分 = {mean:.2}");
            println!("// 预期输出：5 个分数的平均分 = 86.40")
        }
        None => println!("// 预期输出：没有样本，无法计算平均分"),
    }
    println!(
        "样本数 = {}，是否已装满容量 5 = {}",
        stats.count(),
        stats.is_full()
    );
    println!("// 预期输出：样本数 = 5，是否已装满容量 5 = true");

    // 用公开的计算函数验证一下平均分（api::add 做一次对照计算）
    let check = lessons::api::add(0.0, 86.4);
    println!("api::add(0.0, 86.4) = {check}（演示门面函数的可组合性）");
    println!("// 预期输出：api::add(0.0, 86.4) = 86.4（演示门面函数的可组合性）");

    // 日志也走门面：调用方不需要知道 logger 内部用的是原子计数器
    println!("{}", lessons::api::log_info("统计完成"));
    println!("// 预期输出：[INFO] 统计完成");
    println!(
        "{}",
        lessons::api::log_error("示例：出现一条演示用错误日志")
    );
    println!("// 预期输出：[ERROR] 示例：出现一条演示用错误日志");
    println!(
        "本次运行累计日志 = {} 条（log_* 均通过私有 record 累加）",
        lessons::logger::count()
    );
    println!("// 预期输出：本次运行累计日志 = 9 条（log_* 均通过私有 record 累加）");

    // 对外展示的常量：调用方通过 api::PI 拿到精度友好的别名
    println!("api::PI = {:.6}", lessons::api::PI);
    println!("// 预期输出：api::PI = 3.141593");

    // 复用：reset 后统计器可以重新开始（说明公开 API 是完整可用的）
    stats.reset();
    println!(
        "reset 后 count = {}，mean = {:?}（用 Option 表达「无数据」）",
        stats.count(),
        stats.mean()
    );
    println!("// 预期输出：reset 后 count = 0，mean = None（用 Option 表达「无数据」）");
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("本示例证明 —— 单文件内的嵌套模块 + pub use 门面，已经能表达一个完整迷你库");
}
