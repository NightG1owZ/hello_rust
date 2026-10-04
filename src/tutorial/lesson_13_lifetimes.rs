//! lesson_13_lifetimes.rs —— 主题：生命周期（Lifetime）
//!
//! 学习目标：
//!   1. 明白生命周期要解决的问题：借用检查器如何保证「引用永远不会悬垂」；
//!   2. 会在函数签名中写生命周期注解，理解它「只描述关系、不延长存活时间」；
//!   3. 会让结构体持有引用（`struct X<'a> { field: &'a str }`），并在 impl 块与方法上写生命周期；
//!   4. 掌握生命周期省略的三条规则，知道什么时候必须手写注解；
//!   5. 分清 `&'static T` 与 `T: 'static` 两种含义，并会把生命周期参数与 trait bound 组合使用。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_13_lifetimes.rs -o lesson_13_lifetimes && ./lesson_13_lifetimes
//!   或在本项目根目录执行：cargo run --bin lesson_13_lifetimes
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       生命周期是编译期概念，运行期没有任何额外开销；本课的「错误示例」都用注释给出，
//!       并写明 rustc 的错误编号与修正方法。

use std::fmt;

fn main() {
    println!("========== lesson_13_lifetimes：生命周期 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_why_lifetimes();
    demo_2_function_lifetime_annotation();
    demo_3_elision_rules();
    demo_4_multiple_lifetime_params();
    demo_5_struct_holding_reference();
    demo_6_impl_and_method_lifetimes();
    demo_7_static_lifetime();
    demo_8_lifetime_with_trait_bound();
    demo_9_annotation_does_not_extend();
    demo_10_scenario_config_and_longest();
    demo_11_common_mistakes();
}

/// 示例 2 用到的函数：两个输入引用、一个输出引用，必须手写生命周期注解
///
/// 要点：`<'a>` 是在声明「有一个生命周期参数叫 'a」，`&'a str` 表示「这个引用的存活区间是 'a」；
///       返回值写成 `&'a str` 的意思不是「延长它」，而是「返回的引用和这两个参数活得一样久」。
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

/// 示例 3 用到的函数：省略规则 1 + 2 让它完全不需要手写注解
///
/// 规则 1：每个引用参数各自获得一个独立的生命周期参数；
/// 规则 2：只有一个输入引用参数时，输出引用的生命周期就等于它。
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

fn demo_1_why_lifetimes() {
    println!("--- 示例 1：为什么需要生命周期 ---");

    let text = String::from("生命周期是编译期的概念");
    // 合法的借用：被借用的 text 比引用 word 活得更久
    let word = first_word(&text);
    println!("借用安全：{word}");
    println!("// 预期输出：借用安全：生命周期是编译期的概念");

    // 悬垂引用（dangling reference）：引用比它指向的数据活得更久。
    // 下面的代码会被借用检查器直接拒绝，所以只能放在注释里：
    //
    // let dangling;
    // {
    //     let temporary = String::from("临时数据");
    //     dangling = &temporary;   // 把临时变量借出去
    // }                            // temporary 在这里被释放
    // println!("{dangling}");      // 此时 dangling 指向已释放的内存
    //
    // error[E0597]: `temporary` does not live long enough
    // 修正：把 temporary 声明到与 dangling 相同或更外层的 scope，或者让函数返回拥有所有权的类型
    //       （返回 String 而不是 &str）。
    //
    // 关键结论：生命周期注解不能修复悬垂引用，它只是把「引用之间的存活关系」写清楚，
    // 让编译器能替你检查。真正的修复办法永远是调整数据与借用的作用域。
    println!("编译器用生命周期保证：引用永远指向仍然有效的数据");
}

fn demo_2_function_lifetime_annotation() {
    println!("--- 示例 2：函数签名中的生命周期注解 ---");

    let a = String::from("长字符串甲乙丙");
    let b = String::from("短");
    println!("较长的字符串：{}", longest(a.as_str(), b.as_str()));
    println!("// 预期输出：较长的字符串：长字符串甲乙丙");

    // 一个字面量 + 一个 String：'a 会被推断为两者中较短的那个
    println!("{}", longest("字面量也可以", b.as_str()));
    println!("// 预期输出：字面量也可以");

    // 返回值能活多久，取决于传给它的两个参数中较短的那个：
    let result;
    {
        let short_lived = String::from("块内的字符串丙丁戊己");
        // 'a 在这里被推断为 short_lived 的作用域（它比 a 短）
        result = longest(a.as_str(), short_lived.as_str());
        println!("块内使用：{result}");
        println!("// 预期输出：块内使用：块内的字符串丙丁戊己");
    }
    // 出了块，short_lived 已经释放，所以下面这行不能编译 —— 这正是注解在起作用的证据：
    // println!("{result}");
    // error[E0597]: `short_lived` does not live long enough
    // 说明：返回值类型是 &'a str，而 'a 被推断为两个实参中较短的那个生命周期。
    println!("注解把返回值的存活期与参数绑在一起，编译器据此拒绝越界使用");
}

fn demo_3_elision_rules() {
    println!("--- 示例 3：生命周期省略的三条规则 ---");

    let sentence = String::from("省略规则让常见签名保持简洁");
    println!("第一个单词：{}", first_word(&sentence));
    println!("// 预期输出：第一个单词：省略规则让常见签名保持简洁");

    // 规则 1：每个引用参数各自得到一个独立的生命周期参数；
    // 规则 2：只有一个输入引用参数时，输出引用的生命周期取它；
    //         所以 fn first_word(text: &str) -> &str 等价于 fn first_word<'a>(text: &'a str) -> &'a str。
    // 规则 3：如果参数里有 &self 或 &mut self，输出引用的生命周期取 self 的；
    //         所以 fn source(&self) -> &str 等价于 fn source<'s>(&'s self) -> &'s str（见示例 6）。
    //
    // 省略规则救不了「两个输入引用 + 一个输出引用」的情况，必须手写：
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }
    // error[E0106]: missing lifetime specifier
    // help: this function's return type contains a borrowed value,
    //       but the signature does not say whether it is borrowed from `x` or `y`
    // 修正：写成 fn longest<'a>(x: &'a str, y: &'a str) -> &'a str，即示例 2 的 longest。
    println!("只有「一个输入引用」或「有 &self」时才能省略，否则必须手写 'a");
}

/// 示例 4 用到的函数：两个参数各自独立的生命周期
///
/// 要点：返回值只写成 `&'a str`，意味着结果只和 first 绑定；
///       second 可以是「很短命」的引用，完全不影响返回值。
fn pick_first<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    // 这里只读取 second 的长度（usize 是 Copy 的值），没有把 second 的引用带出函数
    println!("（顺带看一眼第二段的长度：{}）", second.len());
    first
}

fn demo_4_multiple_lifetime_params() {
    println!("--- 示例 4：多个生命周期参数 ---");

    let long_lived = String::from("长期存在的数据");
    // 先声明后赋值：result 的存活区间由 pick_first 的返回值类型 &'a str 决定
    let result;
    {
        let short_lived = String::from("临时");
        // 'a = long_lived 的区间，'b = short_lived 的区间，两者互不影响
        result = pick_first(long_lived.as_str(), short_lived.as_str());
        println!("块内使用：{result}");
        println!("// 预期输出：块内使用：长期存在的数据");
    }
    // short_lived 已经释放，但 result 只借用了 long_lived，所以这里依然合法
    println!("块外依然可用：{result}");
    println!("// 预期输出：块外依然可用：长期存在的数据");

    // 反面教材：如果签名写成两个参数共用一个 'a：
    // fn pick_first<'a>(first: &'a str, second: &'a str) -> &'a str { first }
    // 那么 'a 会被压缩成两个实参中较短的那个（short_lived 的区间），
    // 块外的 println!("{result}") 就会报：
    // error[E0597]: `short_lived` does not live long enough
    // 修正：给互不相关的参数各自起一个生命周期名字（'a 与 'b）。
    //
    // 另一个常见错误：返回值想用 second，却只声明了 first 的生命周期：
    // fn longer<'a>(x: &'a str, y: &str) -> &'a str {
    //     if x.len() > y.len() { x } else { y }
    // }
    // error[E0621]: explicit lifetime required in the type of `y`
    // 修正：把 y 也写成 &'a str（要求两者活得一样久），或者给 y 单独的 'b 并只返回 x。
    println!("生命周期参数可以有很多个，各自独立，返回值只与写出来的那个绑定");
}

/// 示例 5 用到的结构体：持有引用就必须声明生命周期参数
///
/// 要点：结构体字段是引用时，实例的存活期不能超过被借用的数据，
///       所以必须写 `struct Excerpt<'a> { field: &'a str }` 把它记录下来。
struct Excerpt<'a> {
    source: &'a str,
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// 关联函数：两个参数共用 'a，保证 part 一定取自 source 这种「活得一样久」的数据
    fn new(source: &'a str, part: &'a str) -> Self {
        Self { source, part }
    }

    /// 返回字段里的引用本身：类型是 &'a str，不受 &self 这次借用的限制
    fn part(&self) -> &'a str {
        self.part
    }

    /// 返回借用到 self 的引用：省略规则 3 让它等价于 fn source<'s>(&'s self) -> &'s str
    fn source(&self) -> &str {
        self.source
    }

    fn describe(&self) -> String {
        format!("{} / {}", self.source, self.part)
    }
}

fn demo_5_struct_holding_reference() {
    println!("--- 示例 5：结构体持有引用 ---");

    let text = String::from("Rust 的生命周期是编译期概念");
    // &text[..4] 取下前 4 个字节，正好是 ASCII 的 "Rust"
    let excerpt = Excerpt::new(text.as_str(), &text[..4]);
    println!("摘要：{}", excerpt.describe());
    println!("// 预期输出：摘要：Rust 的生命周期是编译期概念 / Rust");

    // 结构体字段是引用时，字段没写生命周期注解会直接报错：
    // struct Excerpt {
    //     part: &str,
    // }
    // error[E0106]: missing lifetime specifier
    // help: consider introducing a named lifetime parameter
    //      struct Excerpt<'a> { part: &'a str }
    // 修正：struct Excerpt<'a> { part: &'a str }
    //
    // 实例活得比被借用的数据更久也不行：
    // let excerpt;
    // {
    //     let temporary = String::from("临时数据");
    //     excerpt = Excerpt::new(temporary.as_str(), temporary.as_str());
    // }
    // println!("{}", excerpt.describe());
    // error[E0597]: `temporary` does not live long enough
    // 修正：把 temporary 放到与 excerpt 同级（或更外层）的作用域里。
    println!("结构体的生命周期参数就是在声明：实例不会比它借用的数据活得更久");
}

fn demo_6_impl_and_method_lifetimes() {
    println!("--- 示例 6：impl 块与方法中的生命周期 ---");

    let text = String::from("借用检查器保证引用始终有效");
    // part 先声明、后赋值：它的存活区间由真正赋给它的那个引用决定
    let part;
    {
        // &text[..6] 取下前 6 个字节，正好是两个汉字「借用」
        let excerpt = Excerpt::new(text.as_str(), &text[..6]);
        // part() 的返回类型是 &'a str：直接把字段里的引用复制出来，与 excerpt 这个值无关
        part = excerpt.part();
        // source() 的返回类型是 &str（等价于 &'s self）：只能在 excerpt 存活期内使用
        println!("原文：{}", excerpt.source());
        println!("// 预期输出：原文：借用检查器保证引用始终有效");
        println!("摘要：{}", excerpt.describe());
        println!("// 预期输出：摘要：借用检查器保证引用始终有效 / 借用");
    }
    // excerpt 已经离开作用域，但 part 借的是 text（活得更久），所以这里仍然合法
    println!("块外依然可用：{part}");
    println!("// 预期输出：块外依然可用：借用");

    // 对比：如果把 part() 写成省略形式 fn part(&self) -> &str，
    // 返回值就会绑定到 &self，块外这行会报：
    // error[E0597]: `excerpt` does not live long enough
    // 这就是「返回 &'a str」与「返回 &self 的 &str」的差别，也是省略规则 3 的边界。
    println!("impl 块只写一次 <'a>，块内所有方法都能使用这个生命周期参数");
}

/// 示例 7 用到的常量与函数：'static 的第一种含义 —— 引用本身活到程序结束
///
/// 常量里的 `&str` 省略写法就是 `&'static str`：数据被放在二进制的只读区，全程有效。
const APP_NAME: &'static str = "hello-rust-lessons";

/// static 项本身就是程序级的存在，它的类型里省略的引用也是 'static
static APP_VERSION: &str = "1.0";

/// 返回 'static 引用：调用方拿到的东西不会因为任何局部作用域结束而失效
fn app_name() -> &'static str {
    APP_NAME
}

fn demo_7_static_lifetime() {
    println!("--- 示例 7：'static 生命周期 ---");

    // 字符串字面量直接嵌在可执行文件里，所以它的类型是 &'static str
    let literal: &'static str = "我是字符串字面量";
    println!("字面量：{literal}");
    println!("// 预期输出：字面量：我是字符串字面量");

    println!("app_name() = {}", app_name());
    println!("// 预期输出：app_name() = hello-rust-lessons");

    println!("APP_VERSION = {APP_VERSION}");
    println!("// 预期输出：APP_VERSION = 1.0");

    // String 的数据在堆上，由变量负责释放；想拿到 'static 引用，只能主动「泄漏」不再回收
    let leaked: &'static str = String::from("故意泄漏的字符串").leak();
    println!("leaked = {leaked}");
    println!("// 预期输出：leaked = 故意泄漏的字符串");

    // 'static 有两种完全不同的用法，务必分清：
    //   1) `&'static T`：引用指向的数据从程序开始到结束一直有效
    //      （字符串字面量、static 项、String::leak / Box::leak 得到的引用）；
    //   2) `T: 'static` 约束：表示「T 内部不包含任何非 'static 的借用」，
    //      并不要求这个值活到程序结束——拥有所有权的 String、i32 都满足 `String: 'static`。
    //
    // 局部的 String 永远借不出 'static：
    // fn bad() -> &'static str {
    //     let s = String::from("临时");
    //     s.as_str()
    // }
    // error[E0515]: cannot return reference to local variable `s`
    // 修正：返回拥有所有权的 String（fn good() -> String），或者用 String::leak 主动泄漏。
    println!("'static 既是「指向程序级数据」的引用，也是「内部没有短命借用」的约束");
}

/// 示例 8 用到的函数：生命周期参数与 trait bound 一起使用
///
/// 要点：`<'a, T>` 里 'a 是生命周期参数、T 是类型参数，
///       约束写在 where 子句里；返回 `Option<&'a T>` 表示结果借用自 items。
fn pick_longest<'a, T>(items: &'a [T]) -> Option<&'a T>
where
    T: AsRef<str>,
{
    // max_by_key 返回借用自切片的最长元素，因此结果自然带着 'a
    items.iter().max_by_key(|item| item.as_ref().len())
}

/// 结构体同时带生命周期参数与 trait bound：字段是引用，且被引用的类型要能打印
struct Holder<'a, T: fmt::Display> {
    label: &'a str,
    value: &'a T,
}

impl<'a, T: fmt::Display> Holder<'a, T> {
    fn new(label: &'a str, value: &'a T) -> Self {
        Self { label, value }
    }

    fn render(&self) -> String {
        format!("{} = {}", self.label, self.value)
    }
}

/// `T: 'static` 要求 T 内部没有非 'static 的借用；拥有所有权的类型都满足它
fn keep_static<T: 'static + fmt::Debug>(value: T) -> T {
    value
}

fn demo_8_lifetime_with_trait_bound() {
    println!("--- 示例 8：生命周期与 trait bound ---");

    let names = vec![
        String::from("阿一"),
        String::from("阿二二二二"),
        String::from("阿三三"),
    ];
    // 切片借用 names，返回值借用同一个切片
    match pick_longest(&names) {
        Some(value) => println!("最长的名字：{value}"),
        None => println!("列表为空"),
    }
    println!("// 预期输出：最长的名字：阿二二二二");

    // T = &str 同样满足 AsRef<str>，说明约束是按需叠加的
    let words: [&str; 3] = ["一", "三三三", "二二"];
    match pick_longest(&words) {
        Some(value) => println!("最长的词：{value}"),
        None => println!("列表为空"),
    }
    println!("// 预期输出：最长的词：三三三");

    let city = String::from("杭州");
    let holder = Holder::new("城市", &city);
    println!("{}", holder.render());
    println!("// 预期输出：城市 = 杭州");

    // keep_static 的实参是拥有所有权的 String，满足 T: 'static
    let owned = keep_static(String::from("拥有所有权的值"));
    println!("keep_static(String) = {owned}");
    println!("// 预期输出：keep_static(String) = 拥有所有权的值");

    // 实参是 &'static str（字面量），同样满足 T: 'static
    println!(
        "keep_static(字面量) = {}",
        keep_static("字面量是 &'static str")
    );
    println!("// 预期输出：keep_static(字面量) = 字面量是 &'static str");

    // 但把「指向局部变量的引用」传进去就会被拒绝：
    // let local = String::from("局部数据");
    // let borrowed = keep_static(local.as_str());
    // error[E0597]: `local` does not live long enough
    // 修正：直接把所有权交出去 —— keep_static(local)；或者改传字面量。
    println!("生命周期参数与 trait bound 可以同时出现在同一个泛型参数列表里");
}

fn demo_9_annotation_does_not_extend() {
    println!("--- 示例 9：生命周期注解不会延长存活时间 ---");

    let sentence = String::from("注解只描述关系");
    let word = first_word(&sentence);
    println!("第一个单词：{word}");
    println!("// 预期输出：第一个单词：注解只描述关系");

    // 注解做的事只有一件：把「返回值的存活区间」和「某个参数的存活区间」关联起来。
    // 它不会让任何值活得更久，也不会改变数据的真实释放时机：
    //
    // let dangling = {
    //     let temporary = String::from("临时数据");
    //     first_word(&temporary)   // 返回的引用借的是 temporary
    // };                            // temporary 在这里被释放
    // println!("{dangling}");
    // error[E0597]: `temporary` does not live long enough
    //
    // 就算把签名改成必须接收 'static 引用（注意：'static 要写在**类型**位置，
    // 不能写成 `<'static>` 这样的生命周期参数名，那样会报
    // error[E0262]: invalid lifetime parameter name: `'static`）：
    //
    // fn first_word(text: &'static str) -> &'static str { text }
    //
    // 这样只是把「参数必须活到程序结束」变成硬性要求，编译器照样直接拒绝传入 temporary：
    // error[E0597]: `temporary` does not live long enough
    // 修正思路只有一个：让被借用的数据活得更久（提升到外层作用域、改用拥有所有权的类型、
    // 或者克隆一份数据）。
    println!("注解是「约束」和「说明」，不是「延长寿命的魔法」");
}

/// 示例 10 用到的结构体：真实场景里让配置结构体借用外部的字符串
///
/// 要点：配置项常常来自命令行参数或环境变量（本来就是一个长命 String），
///       用 `&'a str` 借用可以避免为每个字段再分配一份 String。
struct Config<'a> {
    name: &'a str,
    host: &'a str,
    port: u16,
}

impl<'a> Config<'a> {
    fn new(name: &'a str, host: &'a str, port: u16) -> Self {
        Self { name, host, port }
    }

    fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// 返回较长的一项：返回类型是 &'a str，说明它借的是字段里的数据（'a），
    /// 而不是 &self 这次借用，因此结果可以比 Config 实例活得更久。
    fn longer(&self, other: &'a str) -> &'a str {
        if self.name.len() >= other.len() {
            self.name
        } else {
            other
        }
    }
}

fn demo_10_scenario_config_and_longest() {
    println!("--- 示例 10：典型使用场景 —— 借用式配置 + 返回较长字符串的引用 ---");

    // 真实场景：从命令行/环境读到的原始字符串只在这里拥有一份，配置结构体只借用它
    let name = String::from("order-service");
    let host = String::from("10.0.0.7");
    let config = Config::new(name.as_str(), host.as_str(), 8080);

    println!("服务名：{}", config.name);
    println!("// 预期输出：服务名：order-service");

    println!("端点：{}", config.endpoint());
    println!("// 预期输出：端点：10.0.0.7:8080");

    // 与一个字面量比较长度：字面量是 &'static str，可以安全地和 'a 统一
    println!("较长的名字：{}", config.longer("order-service-v2"));
    println!("// 预期输出：较长的名字：order-service-v2");

    println!("较长的名字：{}", config.longer("api"));
    println!("// 预期输出：较长的名字：order-service");

    // 独立的函数版本：返回两个字符串中较长的那个引用
    let left = String::from("左侧较长的字符串");
    let right = String::from("右侧");
    println!("longest 结果：{}", longest(left.as_str(), right.as_str()));
    println!("// 预期输出：longest 结果：左侧较长的字符串");

    // 演示「字段里的引用可以比实例本身活得更久」：
    let name_borrow;
    {
        let inner = String::from("temp-service");
        let temp_config = Config::new(inner.as_str(), inner.as_str(), 80);
        println!("临时配置端点：{}", temp_config.endpoint());
        println!("// 预期输出：临时配置端点：temp-service:80");
        // 字段是 &'a str（引用可以 Copy），拷出来之后 name_borrow 借的是 inner
        name_borrow = temp_config.name;
        // 在块内使用，所以 name_borrow 的存活区间正好停在块尾
        println!("块内读取借用：{name_borrow}");
        println!("// 预期输出：块内读取借用：temp-service");
    }
    // 下面这行会让 name_borrow 的存活区间延长到块外，于是借用检查失败：
    // println!("{name_borrow}");
    // error[E0597]: `inner` does not live long enough
    // 修正：把 inner 提到与 name_borrow 同级的作用域，或者改用拥有所有权的 String 字段。
    println!("引用可以按值复制，但复制的引用同样受被借用数据的存活期约束");
}

/// 示例 11：常见错误示例
///
/// 要点：生命周期相关的错误集中在五类——缺注解（E0106）、标注不全（E0621）、
///       返回局部变量引用（E0515）、被借用数据活得太短（E0597）、以及把注解当魔法。
///       下面每段错误代码都注释掉了，并写明错误编号与修正方法。
fn demo_11_common_mistakes() {
    println!("--- 示例 11：常见错误示例 ---");

    // 错误 1：两个输入引用 + 一个输出引用，省略规则无法推断
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }
    // error[E0106]: missing lifetime specifier
    // help: this function's return type contains a borrowed value,
    //       but the signature does not say whether it is borrowed from `x` or `y`
    // 修正：fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    println!(
        "错误 1：两个引用参数却省略了返回值的生命周期 —— error[E0106] missing lifetime specifier"
    );

    // 错误 2：结构体字段是引用却没有生命周期参数
    // struct Excerpt {
    //     part: &str,
    // }
    // error[E0106]: missing lifetime specifier
    // 修正：struct Excerpt<'a> { part: &'a str }
    println!("错误 2：结构体持有引用却没写生命周期参数 —— error[E0106] missing lifetime specifier");

    // 错误 3：想返回局部变量的引用（连注解都写不出来）
    // fn dangling() -> &String {
    //     let s = String::from("临时");
    //     &s
    // }
    // error[E0106]: missing lifetime specifier
    // 修正：返回拥有所有权的类型 —— fn dangling() -> String { String::from("临时") }
    //       如果非要返回引用，就让数据活得比函数调用更久（由调用方传入）。
    println!("错误 3：返回局部变量的引用 —— error[E0106] missing lifetime specifier");

    // 错误 4：硬写 'static 也救不了局部变量
    // fn dangling() -> &'static String {
    //     let s = String::from("临时");
    //     &s
    // }
    // error[E0515]: cannot return reference to local variable `s`
    // 修正：把局部数据改成字符串字面量（真的是 'static），或者返回拥有所有权的 String。
    println!(
        "错误 4：给局部变量标注 'static —— error[E0515] cannot return reference to local variable `s`"
    );

    // 错误 5：只给一个参数标注生命周期，却想返回另一个参数的引用
    // fn longer<'a>(x: &'a str, y: &str) -> &'a str {
    //     if x.len() > y.len() { x } else { y }
    // }
    // error[E0621]: explicit lifetime required in the type of `y`
    // 修正：把 y 写成 &'a str（两者活得一样久），或给 y 单独的 'b 并只返回 x。
    println!(
        "错误 5：漏标一个参数的生命周期 —— error[E0621] explicit lifetime required in the type of `y`"
    );

    // 错误 6：实例活得比被借用的数据更久
    // let excerpt;
    // {
    //     let temporary = String::from("临时");
    //     excerpt = Excerpt::new(temporary.as_str(), temporary.as_str());
    // }
    // println!("{}", excerpt.describe());
    // error[E0597]: `temporary` does not live long enough
    // 修正：把 temporary 提升到与 excerpt 同级的作用域，或让 Excerpt 持有 String 而不是 &str。
    println!(
        "错误 6：结构体实例比被借用的数据活得久 —— error[E0597] `temporary` does not live long enough"
    );

    // 错误 7：把生命周期注解当成「延长存活时间」的手段
    // 先记住一个语法红线：'static 不能当生命周期参数名 ——
    // fn first_word<'static>(text: &'static str) -> &'static str { text }
    // error[E0262]: invalid lifetime parameter name: `'static`
    //
    // 正确写法是把 'static 放在类型位置（下面这样），但它同样救不了局部变量：
    // fn first_word(text: &'static str) -> &'static str { text }
    // let dangling = {
    //     let temporary = String::from("临时");
    //     first_word(&temporary)
    // };
    // error[E0597]: `temporary` does not live long enough
    // 修正：注解只能表达关系，不能改变数据的释放时机；请调整作用域或改用拥有所有权的类型。
    println!("错误 7：指望注解延长存活时间 —— error[E0597] `temporary` does not live long enough");

    println!("以上错误的修正思路都是「调整作用域 / 改拥有所有权 / 补齐注解」，而不是硬写 'static");
}
