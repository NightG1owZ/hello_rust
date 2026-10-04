//! lesson_18_macros.rs —— 主题：声明宏（macro_rules!）
//!
//! 学习目标：
//!   1. 分清宏与函数的差别：宏可变参数、编译期按「代码模板」展开；
//!   2. 会写带片段说明符（expr / ident 等）与重复模式（`$(...),+`）的声明宏；
//!   3. 会用 stringify! / concat! 等内建宏生成诊断信息与代码文本；
//!   4. 会写「提前返回」型宏（ensure!）与小型 DSL（schedule!），理解宏的适用边界；
//!   5. 知道宏的文本作用域规则与卫生性，能识别典型的宏编写错误。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_18_macros.rs -o lesson_18_macros && ./lesson_18_macros
//!   或在本项目根目录执行：cargo run --bin lesson_18_macros
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       本课只讲 macro_rules! 声明宏；过程宏（derive 宏）见 03_engineering_practices.md。
//!       println! / vec! / assert_eq! 这些你每天都在用的「感叹号」，就是宏。

use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// 宏定义区：macro_rules! 是「文本作用域」——必须先定义、后使用。
// 所有宏集中放在文件顶部，main 与各 demo 函数都能使用。
// ---------------------------------------------------------------------------

/// 最小示例：一个宏可以有多个「匹配臂」，按调用形状选择其一展开
macro_rules! announce {
    () => {
        println!("（无参公告）")
    };
    ($msg:expr) => {
        println!("公告：{}", $msg)
    };
}

/// 片段说明符：$a:expr 表示「这里要填一个表达式」；
/// 展开后 if/else 的两个分支类型一致，因此 max_of! 对 i32 / f64 都适用
macro_rules! max_of {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

/// 重复模式：`$(...),+` 表示「逗号分隔、至少一个」；`$(,)?` 表示「可选的尾逗号」。
/// 展开时 `$x` 会逐个代入 `total += $x;`
macro_rules! sum_all {
    ($($x:expr),+ $(,)?) => {{
        let mut total = 0;
        $( total += $x; )+
        total
    }};
}

/// 方括号调用的宏 + 零次重复（`*`）：list![] 也是合法调用
macro_rules! list {
    [ $($x:expr),* $(,)? ] => {{
        let mut items = Vec::new();
        $( items.push($x); )*
        items
    }};
}

/// stringify! 把「源代码文本」原样转成字符串 —— 错误信息/日志的利器
macro_rules! describe {
    ($e:expr) => {{
        let value = $e;
        format!("{} = {}", stringify!($e), value)
    }};
}

/// 「提前返回」型宏：条件不满足就 return Err —— demoweb 与本仓库课程
/// 里手写的参数校验都可以用它收敛成一行
macro_rules! ensure {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            return Err(String::from($msg));
        }
    };
}

/// 小型 DSL：`key => value` 重复构建 BTreeMap（有序，输出可复现）
macro_rules! schedule {
    ( $( $day:expr => $hours:expr ),+ $(,)? ) => {{
        let mut plan = BTreeMap::new();
        $( plan.insert($day, $hours); )+
        plan
    }};
}

fn main() {
    println!("========== lesson_18_macros：声明宏 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_macro_vs_function();
    demo_2_fragment_specifiers();
    demo_3_repetition();
    demo_4_stringify_and_concat();
    demo_5_ensure_early_return();
    demo_6_scenario_schedule_dsl();
    demo_7_common_mistakes();
}

// ---------------------------------------------------------------------------
// 示例 1：宏与函数的差别
// ---------------------------------------------------------------------------

fn demo_1_macro_vs_function() {
    println!("--- 示例 1：宏与函数的差别 ---");

    // println! 是可变参数宏：1 个、2 个、3 个参数都能接受 ——
    // 固定签名的函数做不到这一点，宏在编译期按「调用形状」展开成不同代码
    println!("一个参数：{}", 1);
    println!("// 预期输出：一个参数：1");
    println!("两个参数：{} 和 {}", 1, 2);
    println!("// 预期输出：两个参数：1 和 2");

    // 自定义宏同样支持多个匹配臂：无参调用走第一臂，带表达式走第二臂
    announce!();
    announce!("项目已构建");
    println!("// 预期输出：（无参公告）");
    println!("// 预期输出：公告：项目已构建");

    // 三个差别小结：
    //   1. 可变参数/任意形状：宏按匹配臂展开，函数签名固定；
    //   2. 编译期展开：宏生成的是「代码」，可以包含字面量拼接、类型选择等；
    //   3. 文本作用域：宏必须先定义后使用（函数则没有这个限制）。
    println!("宏 = 编译期代码生成器，函数 = 运行期逻辑单元");
}

// ---------------------------------------------------------------------------
// 示例 2：片段说明符 expr —— 一份模板适配多种类型
// ---------------------------------------------------------------------------

fn demo_2_fragment_specifiers() {
    println!("--- 示例 2：片段说明符 ---");

    // 同一个宏，i32 与 f64 都能用：因为展开后的 if/else 是普通表达式，
    // 类型由代入的实参决定（这与泛型函数的单态化殊途同归，见 lesson_11）
    println!("max_of!(10, 7) = {}", max_of!(10, 7));
    println!("// 预期输出：max_of!(10, 7) = 10");
    println!("max_of!(3.5, 9.25) = {}", max_of!(3.5, 9.25));
    println!("// 预期输出：max_of!(3.5, 9.25) = 9.25");

    // 实参可以是任意表达式，展开时原样代入
    println!("max_of!(2 + 3, 4) = {}", max_of!(2 + 3, 4));
    println!("// 预期输出：max_of!(2 + 3, 4) = 5");

    // 常用片段说明符速查：
    //   expr（表达式）、ident（标识符）、literal（字面量）、ty（类型）、
    //   pat（模式）、block（语句块）、tt（单棵语法树，最通用）。
    // 本课只用 expr，够覆盖大多数日常场景。
    println!("expr 说明符让宏对「任意表达式」生效");
}

// ---------------------------------------------------------------------------
// 示例 3：重复模式 —— 可变参数的本质
// ---------------------------------------------------------------------------

fn demo_3_repetition() {
    println!("--- 示例 3：重复模式 $(...),+ ---");

    // sum_all! 的展开（以 sum_all!(1, 2, 3) 为例）：
    //   { let mut total = 0; total += 1; total += 2; total += 3; total }
    println!("sum_all!(1, 2, 3) = {}", sum_all!(1, 2, 3));
    println!("// 预期输出：sum_all!(1, 2, 3) = 6");

    // `$(,)?` 允许尾逗号：与函数调用风格保持一致
    println!("sum_all!(5,) = {}", sum_all!(5,));
    println!("// 预期输出：sum_all!(5,) = 5");

    // list! 用方括号调用（宏可以用 () [] {} 任一种定界符），顺序与书写一致
    let nums = list![1, 2, 3];
    println!("list![1, 2, 3] = {nums:?}");
    println!("// 预期输出：list![1, 2, 3] = [1, 2, 3]");

    // `*` 表示零次或多次：空列表也合法（类型需显式标注，因为没有元素可推断）
    let empty: Vec<i32> = list![];
    println!("list![] = {empty:?}");
    println!("// 预期输出：list![] = []");

    println!("$(...)模式 + 分隔符 + 次数（+/*）= 宏的可变参数");
}

// ---------------------------------------------------------------------------
// 示例 4：stringify! 与 concat! —— 生成代码文本
// ---------------------------------------------------------------------------

fn demo_4_stringify_and_concat() {
    println!("--- 示例 4：stringify! 与 concat! ---");

    // describe! 展开后用 stringify! 把表达式源码变成字符串：
    // 打印日志时能同时看到「表达式长什么样」和「值是多少」
    println!("{}", describe!(1 + 2));
    println!("// 预期输出：1 + 2 = 3");
    println!("{}", describe!(40 * 2 + 2));
    println!("// 预期输出：40 * 2 + 2 = 82");

    // concat! 在编译期拼接字符串字面量
    println!("{}", concat!("cargo", " ", "run"));
    println!("// 预期输出：cargo run");

    // 环境信息宏（file!/line!）的值依赖源码与平台，这里只说明用途、不断言输出：
    // file!/line! 常用于打日志时定位代码位置，与 tracing 的日志字段配合（见 demoweb）。
    println!("stringify! 把源码文本带进错误信息，排查问题一目了然");
}

// ---------------------------------------------------------------------------
// 示例 5：ensure! —— 用宏收敛「校验 + 提前返回」样板
// ---------------------------------------------------------------------------

/// 三条校验规则：任何一条不满足都提前返回 Err
fn validate_age(age: i32) -> Result<String, String> {
    ensure!(age >= 0, "年龄不能为负");
    ensure!(age <= 150, "年龄超出范围");
    ensure!(age != 13, "13 岁暂不受理");
    Ok(format!("年龄有效：{age} 岁"))
}

fn demo_5_ensure_early_return() {
    println!("--- 示例 5：ensure! 提前返回宏 ---");

    // ensure! 展开就是一行 if !cond { return Err(...) }：
    // 三个校验规则在函数体里排成三行，可读性与手写 if 完全一致
    println!("{:?}", validate_age(30));
    println!("// 预期输出：Ok(\"年龄有效：30 岁\")");
    println!("{}", validate_age(-1).err().unwrap());
    println!("// 预期输出：年龄不能为负");
    println!("{}", validate_age(200).err().unwrap());
    println!("// 预期输出：年龄超出范围");
    println!("{}", validate_age(13).err().unwrap());
    println!("// 预期输出：13 岁暂不受理");

    // 这是 demoweb 校验请求参数的真实模式：宏消除重复样板，错误处理仍走
    // lesson_10 的 Result 链路 —— 宏不改变架构，只消除重复。
    println!("宏消除样板代码，Result 保持统一错误链路");
}

// ---------------------------------------------------------------------------
// 示例 6：典型场景 —— 用宏写一个小型 DSL
// ---------------------------------------------------------------------------

fn demo_6_scenario_schedule_dsl() {
    println!("--- 示例 6：场景：schedule! 小型 DSL ---");

    // 「周三 => 3」这种箭头写法在 Rust 函数参数里做不到，
    // 但宏可以按自己的语法形状匹配并展开成普通代码
    let plan = schedule! {
        "周一" => 2,
        "周三" => 3,
        "周五" => 1,
    };

    // BTreeMap 按键排序遍历（键为汉字时按 Unicode 码点：一 < 三 < 五）
    for (day, hours) in &plan {
        println!("{day} 排班 {hours} 小时");
    }
    println!("// 预期输出：周一 排班 2 小时");
    println!("// 预期输出：周三 排班 3 小时");
    println!("// 预期输出：周五 排班 1 小时");

    let total: i32 = plan.values().sum();
    println!("本周合计 {total} 小时");
    println!("// 预期输出：本周合计 6 小时");

    // 宏 DSL 的边界：只在「消除重复、提升表达力」时使用；
    // 复杂业务请回到结构体 + trait（lesson_12），可维护性永远优先。
    println!("宏 DSL：小而专，别让它变成另一门语言");
}

// ---------------------------------------------------------------------------
// 示例 7：常见错误对照（全部注释掉，取消注释即可看到真实报错）
// ---------------------------------------------------------------------------

fn demo_7_common_mistakes() {
    println!("--- 示例 7：常见错误 ---");

    // 错误 1：先使用、后定义 —— 宏是文本作用域，从定义处向下可见
    // fn early() { late!(); }
    // macro_rules! late { () => { println!("太晚了") }; }
    // error: cannot find macro `late` in this scope
    // 修正：把宏定义移动到首次使用之前（本文件把宏集中放在顶部的意义所在）；
    //       跨文件导出需要在宏定义上加 #[macro_export]。

    // 错误 2：实参形状与匹配臂对不上
    // max_of!(1);            // 只有一个表达式，两个匹配臂都不匹配
    // error: no rules expected the token `)`（宏没有能匹配这次调用的规则）
    // 修正：为单参数补一条匹配臂，或检查调用形状（本课 max_of! 固定两参）。

    // 错误 3：重复里的分隔符用错
    // sum_all!(1; 2);        // 匹配臂约定逗号分隔，这里写了分号
    // error: expected `,` or `)`（在重复模式里找不到约定的分隔符）
    // 修正：按匹配臂声明的分隔符传参：sum_all!(1, 2)。

    // 错误 4：期望 ident 却拿到表达式
    // macro_rules! bad { ($name:ident) => { let $name = 1; }; }
    // bad!(1 + 2);           // `1 + 2` 是表达式，不是标识符
    // error: expected identifier, found `1`
    // 修正：按实际用途选对说明符（要表达式用 $x:expr，要名字用 $x:ident）。

    // 错误 5：卫生性 —— 宏内部创建的名字不会泄漏到调用处
    // macro_rules! make {
    //     () => {{ let hidden = 42; hidden }};
    // }
    // let value = make!();
    // println!("{hidden}");  // error[E0425]: cannot find value `hidden` in this scope
    // 说明：宏展开里的 hidden 与调用处的同名变量是「不同的 hidden」（宏卫生），
    //       这是刻意设计：宏不应该悄悄污染调用方的命名空间。
    // 修正：让宏把值作为表达式的最后结果返回（本课 sum_all! 的写法）。

    // 何时不用宏：逻辑能用函数/泛型表达时就不用宏 ——
    // 宏的错误提示更难读、IDE 支持更弱；宏的价值在「可变形状 + 代码生成」。
    println!("宏高频错误：先用后定义 / 形状不匹配 / 分隔符不符 / 误用说明符");
    println!("能用函数与泛型解决时，优先不用宏");
}

// 本文件示例按 lesson-conventions.md 约定编写：
// 「// 预期输出：」为字面量断言，可用 .dsh/check_expected_output.ps1 一键核对。
