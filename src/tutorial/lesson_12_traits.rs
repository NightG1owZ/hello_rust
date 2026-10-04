//! lesson_12_traits.rs —— 主题：Trait（定义、实现、默认方法、作为参数与返回值、dyn 动态分发）
//!
//! 学习目标：
//!   1. 会定义 trait、为自定义类型实现 trait，理解「必需的关联方法」与「默认方法」；
//!   2. 分清 trait 作为参数的两种写法：`impl Trait` 与「泛型参数 + trait bound」，并知道各自取舍；
//!   3. 理解返回 `impl Trait` 与返回 `Box<dyn Trait>` 的差别，以及什么时候必须用 dyn；
//!   4. 掌握「关联类型 vs 泛型参数」的取舍，理解 `&dyn Trait` / `Box<dyn Trait>` 的动态分发原理；
//!   5. 熟悉标准库常用 trait（Debug、Clone、PartialEq、PartialOrd、Default、Display）的派生与手写实现。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_12_traits.rs -o lesson_12_traits && ./lesson_12_traits
//!   或在本项目根目录执行：cargo run --bin lesson_12_traits
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       trait bound 在泛型上的语法细节见 lesson_11_generics.rs；引用与生命周期的完整规则见
//!       lesson_13_lifetimes.rs（本文件中出现的 `&self`、`Formatter<'_>` 都只做最简使用）。

use std::cmp::Ordering;
use std::fmt;

fn main() {
    println!("========== lesson_12_traits：Trait ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_define_and_implement();
    demo_2_default_methods();
    demo_3_trait_as_param_generic();
    demo_4_trait_as_param_impl_trait();
    demo_5_multiple_bounds();
    demo_6_return_impl_trait();
    demo_7_return_box_dyn();
    demo_8_associated_type_vs_generic();
    demo_9_dyn_trait_objects();
    demo_10_scenario_shapes();
    demo_11_derived_std_traits();
    demo_12_manual_std_traits();
    demo_13_common_mistakes();
}

/// 示例 1 用到的 trait：一组「行为契约」
///
/// 要点：trait 只声明「能做什么」，不关心数据怎么存；
///       每个方法要么只有签名（实现者必须写），要么给默认实现（实现者可不写）。
trait Summary {
    /// 只有签名、没有方法体，末尾是分号：这是必须由实现者提供的行为
    fn summarize(&self) -> String;
}

/// 数据仍然由普通结构体持有，trait 只是附加的能力
struct Article {
    title: String,
    author: String,
    words: usize,
}

/// `impl trait名 for 类型名` 就是「为某个类型实现某个 trait」
impl Summary for Article {
    fn summarize(&self) -> String {
        // 结构体字段在这里被读取，方法体由实现者自由决定
        format!("《{}》/ {} / {} 字", self.title, self.author, self.words)
    }
}

/// 完全不同的数据类型，也能实现同一个 trait
struct NewsFlash {
    headline: String,
    location: String,
}

impl Summary for NewsFlash {
    fn summarize(&self) -> String {
        format!("[{}] {}", self.location, self.headline)
    }
}

/// 后续多个示例共用的构造辅助函数，避免重复写字段
fn sample_article() -> Article {
    Article {
        title: String::from("泛型与 trait"),
        author: String::from("小明"),
        words: 1200,
    }
}

fn sample_news() -> NewsFlash {
    NewsFlash {
        headline: String::from("今日有雨"),
        location: String::from("上海"),
    }
}

fn demo_1_define_and_implement() {
    println!("--- 示例 1：定义 trait 并为自定义类型实现 ---");

    let article = sample_article();
    // 方法调用与普通方法完全一样：编译器根据 article 的类型找到对应的 impl
    println!("{}", article.summarize());
    println!("// 预期输出：《泛型与 trait》/ 小明 / 1200 字");

    let news = sample_news();
    println!("{}", news.summarize());
    println!("// 预期输出：[上海] 今日有雨");

    // 同一个 trait 有很多实现者时，调用处只依赖 trait，不依赖具体类型（见示例 3、4）
    println!("Article 与 NewsFlash 没有任何继承关系，却都能被当作 Summary 使用");
}

/// 示例 2 用到的 trait：带默认方法
trait Greet {
    /// 必需方法：默认方法可以调用它，所以实现者只需提供最少的信息
    fn name(&self) -> String;

    /// 默认方法：有方法体，实现者可以什么都不写直接获得这个行为；
    /// 默认方法内部还能调用「必需方法」和「默认钩子」，从而把可定制点暴露给实现者。
    fn greet(&self) -> String {
        // 先调用默认钩子：不覆盖它的实现者得到「什么都不做」，覆盖了的实现者可以插入自己的逻辑
        self.before_greet();
        format!("你好，我是{}", self.name())
    }

    /// 默认「钩子」：默认什么都不做，实现者按需覆盖；这是标准库里常见的可选项写法
    fn before_greet(&self) {}
}

struct Robot {
    id: u32,
}

impl Greet for Robot {
    fn name(&self) -> String {
        format!("机器人-{}", self.id)
    }

    /// 只覆盖钩子，greet 继续用默认实现
    fn before_greet(&self) {
        println!("(机器人自检完成)");
    }
}

struct Human {
    name: String,
}

impl Greet for Human {
    fn name(&self) -> String {
        // clone 一份 String：方法签名返回的是拥有所有权的 String，不能直接挪走字段
        self.name.clone()
    }

    /// 覆盖默认方法：完全替换默认行为，连钩子也不再被调用
    fn greet(&self) -> String {
        format!("嗨，我是{}", self.name())
    }
}

fn demo_2_default_methods() {
    println!("--- 示例 2：默认方法与覆盖 ---");

    let robot = Robot { id: 7 };
    // 默认的 greet 内部先调用 before_greet（钩子），再拼接问候语。
    // Robot 覆盖了钩子，所以会先看到 "(机器人自检完成)" 这一行，再看到问候语。
    println!("{}", robot.greet());
    println!("// 预期输出：(机器人自检完成)");
    println!("// 预期输出：你好，我是机器人-7");

    let human = Human {
        name: String::from("小红"),
    };
    // Human 覆盖了 greet，所以输出格式与 Robot 不同
    println!("{}", human.greet());
    println!("// 预期输出：嗨，我是小红");

    // 必需方法依然可以被直接调用
    println!("Human 的 name() = {}", human.name());
    println!("// 预期输出：Human 的 name() = 小红");
}

/// 示例 3：trait 作为参数（写法一：泛型参数 + trait bound）
///
/// 要点：这是「静态分发」——编译期为每个具体类型生成一份专用函数（单态化），
///       运行期没有查表开销；代价是每种用到的类型都会让二进制更大。
fn notify<T: Summary>(item: &T) -> String {
    format!("快讯：{}", item.summarize())
}

/// 只有一个类型参数 T，因此两个实参必须是同一种具体类型
fn notify_pair_same<T: Summary>(first: &T, second: &T) -> String {
    format!("{} ｜ {}", first.summarize(), second.summarize())
}

fn demo_3_trait_as_param_generic() {
    println!("--- 示例 3：trait 作为参数（泛型 + trait bound） ---");

    let article = sample_article();
    println!("{}", notify(&article));
    println!("// 预期输出：快讯：《泛型与 trait》/ 小明 / 1200 字");

    // 泛型写法允许 turbofish 显式指定类型参数
    println!("{}", notify::<Article>(&article));
    println!("// 预期输出：快讯：《泛型与 trait》/ 小明 / 1200 字");

    let another = Article {
        title: String::from("生命周期"),
        author: String::from("小红"),
        words: 800,
    };
    // 两个参数都是 Article，满足 T 唯一的要求
    println!("{}", notify_pair_same(&article, &another));
    println!("// 预期输出：《泛型与 trait》/ 小明 / 1200 字 ｜ 《生命周期》/ 小红 / 800 字");

    let news = sample_news();
    println!("{}", notify(&news));
    println!("// 预期输出：快讯：[上海] 今日有雨");

    // notify_pair_same(&article, &news); 会编译失败，因为 T 只能是一种类型：
    // error[E0308]: mismatched types（expected `&Article`, found `&NewsFlash`）
    // 需要混合类型时用示例 4 的 impl Trait 写法，或者两个独立的类型参数
    println!("同一个泛型函数可以服务多种类型，但一次调用里 T 只能是一种具体类型");
}

/// 示例 4：trait 作为参数（写法二：impl Trait）
///
/// 要点：`item: &impl Summary` 是语法糖，等价于「一个匿名的类型参数 + Summary 约束」，
///       同样是静态分发；区别在于：不能 turbofish 指定，也没法在函数体里用名字指代这个类型。
fn notify_impl(item: &impl Summary) -> String {
    format!("快讯：{}", item.summarize())
}

/// 每个 `impl Trait` 都是一个独立的匿名类型参数，所以两个参数可以是不同类型
fn notify_pair_any(first: &impl Summary, second: &impl Summary) -> String {
    format!("{} ｜ {}", first.summarize(), second.summarize())
}

fn demo_4_trait_as_param_impl_trait() {
    println!("--- 示例 4：trait 作为参数（impl Trait） ---");

    let news = sample_news();
    println!("{}", notify_impl(&news));
    println!("// 预期输出：快讯：[上海] 今日有雨");

    let article = sample_article();
    // 两个参数是不同具体类型（Article 与 NewsFlash），impl Trait 写法允许
    println!("{}", notify_pair_any(&article, &news));
    println!("// 预期输出：《泛型与 trait》/ 小明 / 1200 字 ｜ [上海] 今日有雨");

    // notify_impl::<NewsFlash>(&news); 不能编译：impl Trait 参数没有名字，无法 turbofish
    // 取舍小结：
    //   * 只做「接收任意实现者」这件事 —— impl Trait 更短，参数多、类型各不相同时最舒服；
    //   * 需要显式指定类型参数、需要把类型存进结构体、需要在函数体里调用 T 的关联函数
    //     （如 T::default()）—— 必须用泛型参数写法；
    //   * 两者都是静态分发；想要「一个容器装多种类型」必须用示例 9 的 dyn。
    println!("impl Trait 适合简单接收，泛型参数适合需要指代类型的场景");
}

/// 示例 5 用到的类型：同时实现 Summary 与 Display
struct Report {
    id: u32,
    title: String,
}

impl Summary for Report {
    fn summarize(&self) -> String {
        format!("报告#{} {}", self.id, self.title)
    }
}

/// Display 不能派生，必须手写；写 `{}` 时就会调用这里
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formatter<'_> 里的 '_ 是生命周期省略写法，完整规则见 lesson_13_lifetimes.rs
        write!(f, "Report({})", self.id)
    }
}

/// 多个约束用 `+` 组合：既要能摘要，又要能用 {} 打印
fn announce<T: Summary + fmt::Display>(item: &T) -> String {
    format!("{item} => {}", item.summarize())
}

/// impl Trait 位置同样支持 `+` 组合，注意括号包住整个 `impl ...` 类型
fn announce_impl(item: &(impl Summary + fmt::Display)) -> String {
    format!("{item} => {}", item.summarize())
}

fn demo_5_multiple_bounds() {
    println!("--- 示例 5：trait bound 组合与 + ---");

    let report = Report {
        id: 42,
        title: String::from("季度总结"),
    };

    println!("{}", announce(&report));
    println!("// 预期输出：Report(42) => 报告#42 季度总结");

    println!("{}", announce_impl(&report));
    println!("// 预期输出：Report(42) => 报告#42 季度总结");

    // announce(&sample_article()); 会编译失败：Article 没有实现 Display
    // error[E0277]: the trait bound `Article: std::fmt::Display` is not satisfied
    // 修正：为 Article 手写 impl fmt::Display，或把约束放宽为只想打印的部分
    println!("约束叠加得越多，能用上的类型越少，所以按需叠加");
}

/// 示例 6：返回 impl Trait —— 把具体类型藏起来
///
/// 要点：调用方只知道「返回值实现了 Summary」，不知道也不用关心它是 NewsFlash；
///       代价是所有 return 分支必须是同一个具体类型。
fn breaking_news(headline: &str) -> impl Summary {
    NewsFlash {
        headline: headline.to_string(),
        location: String::from("本地"),
    }
}

fn demo_6_return_impl_trait() {
    println!("--- 示例 6：返回 impl Trait ---");

    // flash 的具体类型是 NewsFlash，但调用处看不到，只能用 Summary 的能力
    let flash = breaking_news("地铁新线路今日开通");
    println!("{}", flash.summarize());
    println!("// 预期输出：[本地] 地铁新线路今日开通");

    // 下面这种「按条件返回不同类型」的写法无法用 impl Trait：
    // fn make_summary(kind: bool) -> impl Summary {
    //     if kind {
    //         NewsFlash { headline: String::from("A"), location: String::from("B") }
    //     } else {
    //         Article { title: String::from("C"), author: String::from("D"), words: 1 }
    //     }
    // }
    // error[E0308]: `if` and `else` have incompatible types
    // 修正：返回类型改成 Box<dyn Summary>，见示例 7
    println!("impl Trait 返回值只有一种具体类型，需要分支返回不同类型时改用 Box<dyn Trait>");
}

/// 示例 7：返回 Box<dyn Trait> —— 运行期决定具体类型
///
/// 要点：`dyn Summary` 是「类型被擦除的 trait 对象」，它的大小在编译期未知，
///       所以不能直接返回或放进变量，必须装在指针后面：`Box<dyn Summary>` 或 `&dyn Summary`。
fn make_summary(kind: &str) -> Box<dyn Summary> {
    match kind {
        // 三个分支返回三种不同具体类型，但都被 Box 擦除成同一个 dyn 类型
        "article" => Box::new(sample_article()),
        "news" => Box::new(sample_news()),
        _ => Box::new(Report {
            id: 0,
            title: String::from("未知类型"),
        }),
    }
}

/// 接收 &dyn Summary：不关心指针是 Box 还是普通引用，也不关心背后是谁
fn describe_dyn(item: &dyn Summary) -> String {
    format!("动态分发调用：{}", item.summarize())
}

fn demo_7_return_box_dyn() {
    println!("--- 示例 7：返回 Box<dyn Trait> ---");

    // kinds 里的每一项都由运行期数据决定走哪个分支，编译期无法统一成一种类型
    for kind in ["article", "news", "unknown"] {
        let item = make_summary(kind);
        println!("{kind} -> {}", item.summarize());
    }
    println!("// 预期输出：article -> 《泛型与 trait》/ 小明 / 1200 字");
    println!("// 预期输出：news -> [上海] 今日有雨");
    println!("// 预期输出：unknown -> 报告#0 未知类型");

    let boxed = make_summary("news");
    // 取出借用得到 &dyn Summary：Box<dyn Summary> 的 Deref::Target 就是 dyn Summary，
    // 所以 `&*boxed` 与 `boxed.as_ref()` 都行；注意不能直接写 describe_dyn(&boxed)，
    // 那会被当成「把 Box<dyn Summary> 本身当成实现者」而报
    // error[E0277]: the trait bound `Box<dyn Summary>: Summary` is not satisfied
    println!("{}", describe_dyn(boxed.as_ref()));
    println!("// 预期输出：动态分发调用：[上海] 今日有雨");

    // 取舍小结：
    //   * `-> impl Trait`  ：静态分发、零开销、可内联；只能返回一种具体类型；
    //   * `-> Box<dyn Trait>`：动态分发、有指针与查表开销；能返回不同类型、能放进同一个集合。
    println!("impl Trait 更快，Box<dyn Trait> 更灵活");
}

/// 示例 8 用到的 trait：关联类型
///
/// 要点：关联类型（`type Item`）是「由实现者一次性确定」的类型占位符；
///       与泛型参数相比，它的语义是「这个类型天生只有一种元素类型」，因此调用处不用标注类型。
trait Container {
    /// 关联类型：具体是什么，由 impl 块里的 `type Item = ...` 决定
    type Item;

    fn first(&self) -> Option<&Self::Item>;

    fn count(&self) -> usize;

    /// 默认方法里可以直接使用 Self::Item，不需要额外约束
    fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

impl Container for Vec<i32> {
    type Item = i32;

    fn first(&self) -> Option<&i32> {
        // 调用切片自己的 first，避免与 trait 方法同名造成无限递归
        self.as_slice().first()
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl Container for Vec<String> {
    type Item = String;

    fn first(&self) -> Option<&String> {
        self.as_slice().first()
    }

    fn count(&self) -> usize {
        self.len()
    }
}

struct Temperature {
    celsius: f64,
}

/// 泛型参数版本：同一个类型可以为不同的 T 各实现一次
trait ConvertTo<T> {
    fn convert_to(&self) -> T;
}

impl ConvertTo<String> for Temperature {
    fn convert_to(&self) -> String {
        format!("{:.1} 摄氏度", self.celsius)
    }
}

impl ConvertTo<f64> for Temperature {
    fn convert_to(&self) -> f64 {
        // 摄氏转开尔文
        self.celsius + 273.15
    }
}

/// 同样的需求换成关联类型：每种类型只能有一种输出
trait IntoOne {
    type Output;

    fn into_one(&self) -> Self::Output;
}

impl IntoOne for Temperature {
    type Output = String;

    fn into_one(&self) -> String {
        format!("{:.1}℃", self.celsius)
    }
}

fn demo_8_associated_type_vs_generic() {
    println!("--- 示例 8：关联类型 vs 泛型参数 ---");

    let numbers = vec![10, 20, 30];
    // 关联类型让调用处不需要写任何类型标注：Self::Item 已经是 i32。
    // 这里用「完全限定语法」Container::first(&numbers) 明确调用 trait 方法，
    // 因为 Vec / 切片本身也有同名方法 first、is_empty，直接写 numbers.first() 会优先选中固有方法。
    match Container::first(&numbers) {
        Some(value) => println!("整数容器第一个元素：{value}"),
        None => println!("整数容器为空"),
    }
    println!("// 预期输出：整数容器第一个元素：10");

    println!("整数容器元素个数：{}", Container::count(&numbers));
    println!("// 预期输出：整数容器元素个数：3");

    // is_empty 是 trait 的默认方法，直接使用了实现者提供的 count
    println!("整数容器为空吗？{}", Container::is_empty(&numbers));
    println!("// 预期输出：整数容器为空吗？false");

    let words = vec![String::from("一"), String::from("二")];
    match Container::first(&words) {
        Some(value) => println!("字符串容器第一个元素：{value}"),
        None => println!("字符串容器为空"),
    }
    println!("// 预期输出：字符串容器第一个元素：一");

    let empty: Vec<String> = Vec::new();
    println!("空容器为空吗？{}", Container::is_empty(&empty));
    println!("// 预期输出：空容器为空吗？true");

    let temp = Temperature { celsius: 25.0 };
    // 泛型参数版本：必须靠类型标注选实现，否则会有歧义
    let text: String = temp.convert_to();
    println!("ConvertTo<String> 的结果：{text}");
    println!("// 预期输出：ConvertTo<String> 的结果：25.0 摄氏度");

    let kelvin: f64 = temp.convert_to();
    println!("ConvertTo<f64> 的结果：{kelvin}");
    println!("// 预期输出：ConvertTo<f64> 的结果：298.15");

    // 关联类型版本：不需要标注，编译器知道只有一种 Output
    println!("IntoOne 的结果：{}", temp.into_one());
    println!("// 预期输出：IntoOne 的结果：25.0℃");

    // 泛型参数版本的 trait 也能做 trait 对象，因为 T 已经被写成具体类型了
    let boxed: Box<dyn ConvertTo<String>> = Box::new(Temperature { celsius: 0.0 });
    println!("dyn ConvertTo<String> 的结果：{}", boxed.convert_to());
    println!("// 预期输出：dyn ConvertTo<String> 的结果：0.0 摄氏度");

    // 泛型参数版本的歧义错误示例：
    // let ambiguous = temp.convert_to();
    // error[E0283]: type annotations needed
    // note: multiple `impl`s satisfying `Temperature: ConvertTo<_>` found
    // 修正：写成 `let x: String = temp.convert_to();`；如果本来就只该有一种结果，用关联类型
    // 关联类型版本的 dyn 需要把关联类型写出来（见示例 9 与示例 13）：
    // let boxed: Box<dyn Container> = Box::new(numbers);
    // error[E0191]: the value of the associated type `Item` in `Container` must be specified
    // 修正：`Box<dyn Container<Item = i32>>`
    println!("关联类型语义唯一、调用简洁；泛型参数可多重实现、需要标注");
}

/// 示例 9：dyn trait 对象与动态分发
///
/// 要点：`dyn Summary` 是「大小未知的类型」，只能用 `&dyn Summary`（借用，两字宽胖指针：
///       数据指针 + 虚表指针）或 `Box<dyn Summary>`（拥有所有权）来使用；
///       调用方法时运行期查虚表，因此叫动态分发。
fn print_summaries(items: &[&dyn Summary]) {
    for (index, item) in items.iter().enumerate() {
        // item 是 &&dyn Summary，自动解引用后调用虚表里的 summarize
        println!("{}. {}", index + 1, item.summarize());
    }
}

fn demo_9_dyn_trait_objects() {
    println!("--- 示例 9：dyn trait 对象与动态分发 ---");

    let article = sample_article();
    let news = sample_news();
    let report = Report {
        id: 9,
        title: String::from("周报"),
    };

    // 一个切片里同时持有三种不同的具体类型：类型被擦除成 &dyn Summary
    let items: Vec<&dyn Summary> = vec![&article, &news, &report];
    print_summaries(&items);
    println!("// 预期输出：1. 《泛型与 trait》/ 小明 / 1200 字");
    println!("// 预期输出：2. [上海] 今日有雨");
    println!("// 预期输出：3. 报告#9 周报");

    // Box<dyn Summary> 拥有数据，可以放进 Vec 并在函数之间传递（甚至返回，见示例 7）
    let boxed: Vec<Box<dyn Summary>> = vec![make_summary("news"), make_summary("article")];
    for item in &boxed {
        // &Box<dyn Summary> 先解引用到 Box，再 Deref 到 dyn Summary
        println!("盒装：{}", item.summarize());
    }
    println!("// 预期输出：盒装：[上海] 今日有雨");
    println!("// 预期输出：盒装：《泛型与 trait》/ 小明 / 1200 字");

    // 为什么「泛型方法」和「返回 Self 的关联函数」不能用于 dyn：
    // trait ObjectSafe {
    //     fn generic_method<T>(&self, value: T); // 泛型方法：每个 T 都是一份新代码
    //     fn create() -> Self;                   // 返回 Self：dyn 类型大小未知，无法返回
    // }
    // let obj: Box<dyn ObjectSafe> = ...;
    // error[E0038]: the trait `ObjectSafe` is not dyn compatible
    // （旧版 rustc 的提示语是 the trait `ObjectSafe` cannot be made into an object）
    // 原因：虚表必须在编译期固定下来，而泛型方法会随 T 无限展开，Self 的大小对 dyn 也未知。
    // 修正：把泛型方法改成 `fn concrete_method(&self, value: i32)`，把 `-> Self` 改成
    //       `-> Box<dyn ObjectSafe>`（由具体实现者提供），或者干脆改用泛型静态分发。
    println!("dyn 的虚表在编译期固定，所以泛型方法与返回 Self 的方法进不了 trait 对象");
}

/// 示例 10 用到的 trait：几何图形
trait Shape {
    /// 名称：返回借用到 self 的 &str（省略写法，详见 lesson_13_lifetimes.rs）
    fn name(&self) -> &str;

    fn area(&self) -> f64;

    fn perimeter(&self) -> f64;

    /// 默认方法：等周商 4πA/P²，圆形为 1，越接近 1 越圆润
    fn roundness(&self) -> f64 {
        let perimeter = self.perimeter();
        if perimeter == 0.0 {
            // 退化图形（周长为 0）单独处理，避免除以 0 得到 NaN
            0.0
        } else {
            4.0 * std::f64::consts::PI * self.area() / (perimeter * perimeter)
        }
    }

    /// 默认方法里用 &dyn Shape 接收另一个形状：可以和任意实现者比较
    fn is_rounder_than(&self, other: &dyn Shape) -> bool {
        self.roundness() > other.roundness()
    }
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn name(&self) -> &str {
        "圆形"
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &str {
        "长方形"
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Square {
    fn name(&self) -> &str {
        "正方形"
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

fn demo_10_scenario_shapes() {
    println!("--- 示例 10：典型使用场景 —— trait 抽象图形集合并按面积排序 ---");

    // 真实场景：图形来源不同、类型各异，但都要参与「统一遍历 + 排序 + 统计」，
    // 于是用 Box<dyn Shape> 把它们的类型擦掉，统一放进一个 Vec。
    let mut shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.5 }),
    ];

    // total_cmp 给 f64 一个全序，避免 partial_cmp 可能返回 None 而被迫 unwrap
    shapes.sort_by(|a, b| a.area().total_cmp(&b.area()));

    for shape in &shapes {
        println!(
            "{} 面积 = {:.2}，周长 = {:.2}，圆润度 = {:.3}",
            shape.name(),
            shape.area(),
            shape.perimeter(),
            // 这里没有实现 roundness，走的是 trait 的默认方法
            shape.roundness()
        );
    }
    println!("// 预期输出：圆形 面积 = 3.14，周长 = 6.28，圆润度 = 1.000");
    println!("// 预期输出：正方形 面积 = 6.25，周长 = 10.00，圆润度 = 0.785");
    println!("// 预期输出：长方形 面积 = 12.00，周长 = 14.00，圆润度 = 0.769");

    // 排序后索引 0 最小、2 最大
    println!("面积最小：{}（{:.2}）", shapes[0].name(), shapes[0].area());
    println!("// 预期输出：面积最小：圆形（3.14）");

    println!("面积最大：{}（{:.2}）", shapes[2].name(), shapes[2].area());
    println!("// 预期输出：面积最大：长方形（12.00）");

    // 用 trait 的默认方法比较两个 trait 对象：圆形(1.000) 比 正方形(0.785) 圆润
    println!(
        "圆形比正方形更圆润吗？{}",
        shapes[0].is_rounder_than(shapes[1].as_ref())
    );
    println!("// 预期输出：圆形比正方形更圆润吗？true");

    println!(
        "正方形比圆形更圆润吗？{}",
        shapes[1].is_rounder_than(shapes[0].as_ref())
    );
    println!("// 预期输出：正方形比圆形更圆润吗？false");
}

/// 示例 11：可派生的标准库 trait
///
/// 要点：Debug / Clone / PartialEq / PartialOrd / Default 都能用 `#[derive(...)]` 自动生成；
///       派生出来的行为是「逐字段比较、逐字段复制」，符合大多数数据类型的直觉。
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
struct Version {
    major: u32,
    minor: u32,
}

/// Display 不能派生：人类可读格式必须自己决定
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

fn demo_11_derived_std_traits() {
    println!("--- 示例 11：标准库 trait 的派生 ---");

    let v1 = Version { major: 1, minor: 9 };
    let v2 = Version { major: 2, minor: 0 };
    let v3 = v1.clone();

    // Display：手写实现，用 {} 打印
    println!("v1 = {v1}");
    println!("// 预期输出：v1 = 1.9");

    // Debug：派生实现，用 {:?} 打印
    println!("v1 的 Debug 形式：{v1:?}");
    println!("// 预期输出：v1 的 Debug 形式：Version {{ major: 1, minor: 9 }}");

    // PartialEq + Clone：v3 是 v1 的副本，因此相等
    println!("v1 == v3 ？{}", v1 == v3);
    println!("// 预期输出：v1 == v3 ？true");

    // PartialOrd：派生实现按字段顺序（先 major 再 minor）比较
    println!("v1 < v2 ？{}", v1 < v2);
    println!("// 预期输出：v1 < v2 ？true");

    // 需要显式拿到 Ordering 时调用 partial_cmp
    match v1.partial_cmp(&v2) {
        Some(Ordering::Less) => println!("partial_cmp 结果：v1 更小"),
        Some(Ordering::Equal) => println!("partial_cmp 结果：两者相等"),
        Some(Ordering::Greater) => println!("partial_cmp 结果：v1 更大"),
        None => println!("partial_cmp 结果：不可比较"),
    }
    println!("// 预期输出：partial_cmp 结果：v1 更小");

    // Default：派生实现把所有字段置零
    let zero = Version::default();
    println!("Version::default() = {zero}");
    println!("// 预期输出：Version::default() = 0.0");
}

/// 示例 12：手写标准库 trait
///
/// 要点：当默认语义不合适时可以手写实现。下面给 `Tag` 定制「忽略大小写」的相等
///       与「先比长度再比字典序」的大小关系。
#[derive(Debug)]
struct Tag(String);

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        // eq_ignore_ascii_case 是 str 提供的不区分大小写比较
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 先比长度，长度相同再比小写后的字典序；then_with 让第二段比较延迟执行
        Some(
            self.0
                .len()
                .cmp(&other.0.len())
                .then_with(|| self.0.to_lowercase().cmp(&other.0.to_lowercase())),
        )
    }
}

/// Default 也可以手写：给字段一个非零的合理默认值
struct ServerConfig {
    host: String,
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}

fn demo_12_manual_std_traits() {
    println!("--- 示例 12：手写标准库 trait ---");

    let a = Tag(String::from("Rust"));
    let b = Tag(String::from("rust"));
    // 手写的 PartialEq 忽略了大小写
    println!("\"Rust\" == \"rust\"（忽略大小写）：{}", a == b);
    println!("// 预期输出：\"Rust\" == \"rust\"（忽略大小写）：true");

    // Debug 是派生的，所以能直接打印内部值
    println!("b = {b:?}");
    println!("// 预期输出：b = Tag(\"rust\")");

    let short = Tag(String::from("ab"));
    let long = Tag(String::from("abc"));
    // 手写的 PartialOrd：先比长度，所以短的小
    println!("ab < abc（先比长度）：{}", short < long);
    println!("// 预期输出：ab < abc（先比长度）：true");

    let same_len_1 = Tag(String::from("Zeta"));
    let same_len_2 = Tag(String::from("beta"));
    // 长度都是 4，于是比小写字典序：beta < zeta，所以 Zeta < beta 为假
    println!(
        "Zeta < beta（长度相同比字典序）：{}",
        same_len_1 < same_len_2
    );
    println!("// 预期输出：Zeta < beta（长度相同比字典序）：false");

    // 手写的 Default
    let config = ServerConfig::default();
    println!("ServerConfig::default() = {}:{}", config.host, config.port);
    println!("// 预期输出：ServerConfig::default() = 127.0.0.1:8080");
}

/// 示例 13：常见错误示例
///
/// 要点：trait 相关错误集中在「没实现、没实现全、实现冲突、不能做 trait 对象」四类。
///       下面每段错误代码都注释掉了，并写明错误编号与修正方法。
fn demo_13_common_mistakes() {
    println!("--- 示例 13：常见错误示例 ---");

    // 错误 1：把没有实现 trait 的类型传进去
    // struct Plain;
    // notify(&Plain);
    // error[E0277]: the trait bound `Plain: Summary` is not satisfied
    // 修正：为 Plain 写 impl Summary for Plain { ... }，或换一个已实现该 trait 的类型
    println!(
        "错误 1：类型没有实现 trait —— error[E0277] the trait bound `Plain: Summary` is not satisfied"
    );

    // 错误 2：impl 块里漏写了必需方法
    // impl Summary for Report {}
    // error[E0046]: not all trait items implemented, missing: `summarize`
    // 修正：补上 fn summarize(&self) -> String { ... }；或者把该方法在 trait 中写成带默认实现的方法
    println!(
        "错误 2：impl 漏写必需方法 —— error[E0046] not all trait items implemented, missing: `summarize`"
    );

    // 错误 3：同一个类型重复实现同一个 trait（或与空白实现 blanket impl 冲突）
    // impl Summary for Report { fn summarize(&self) -> String { String::from("A") } }
    // impl Summary for Report { fn summarize(&self) -> String { String::from("B") } }
    // error[E0119]: conflicting implementations of trait `Summary` for type `Report`
    // 修正：只保留一个实现；同一个 trait 对同一类型只能有一份 impl
    println!("错误 3：重复实现同一个 trait —— error[E0119] conflicting implementations");

    // 错误 4：trait 里有泛型方法，就无法做成 trait 对象
    // trait ObjectSafe {
    //     fn generic_method<T>(&self, value: T);
    //     fn create() -> Self;
    // }
    // let obj: Box<dyn ObjectSafe> = Box::new(Report { id: 1, title: String::from("x") });
    // error[E0038]: the trait `ObjectSafe` is not dyn compatible
    // 修正：把方法改成具体类型参数或无参方法；需要 `-> Self` 时改成 `-> Box<dyn ObjectSafe>`
    println!(
        "错误 4：trait 含泛型方法或返回 Self —— error[E0038] the trait `ObjectSafe` is not dyn compatible"
    );

    // 错误 5：dyn 关联类型时必须指定关联类型
    // let boxed: Box<dyn Container> = Box::new(vec![1, 2, 3]);
    // error[E0191]: the value of the associated type `Item` in `Container` must be specified
    // 修正：写出具体关联类型 —— Box<dyn Container<Item = i32>>
    println!(
        "错误 5：dyn 未指定关联类型 —— error[E0191] the value of the associated type `Item` must be specified"
    );

    // 错误 6：impl Trait 返回值在不同分支返回不同类型
    // fn pick(flag: bool) -> impl fmt::Display {
    //     if flag { 1 } else { String::from("一") }
    // }
    // error[E0308]: `if` and `else` have incompatible types
    // 修正：返回 Box<dyn fmt::Display>，让两个分支先各自装箱成同一种类型
    println!(
        "错误 6：impl Trait 返回值分支类型不一致 —— error[E0308] `if` and `else` have incompatible types"
    );

    // 错误 7：泛型参数版本 trait 的调用需要类型标注
    // let ambiguous = Temperature { celsius: 25.0 }.convert_to();
    // error[E0283]: type annotations needed
    // note: multiple `impl`s satisfying `Temperature: ConvertTo<_>` found
    // 修正：`let text: String = temp.convert_to();`，或把 trait 改成关联类型版本 IntoOne
    println!("错误 7：泛型参数版本 trait 调用有歧义 —— error[E0283] type annotations needed");

    println!("以上错误都能用「补实现 / 补全方法 / 去冲突 / 换成分发方式」四类手段修正");
}
