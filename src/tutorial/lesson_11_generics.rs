//! lesson_11_generics.rs —— 主题：泛型（Generics）
//!
//! 学习目标：
//!   1. 会写泛型函数、泛型结构体、泛型枚举、泛型方法，理解「一份代码适配多种类型」；
//!   2. 掌握 trait bound 的书写位置（`<T: Trait>`、`where` 子句、impl 块上的约束）与多约束组合；
//!   3. 理解单态化（monomorphization）：泛型在编译期被展开成具体类型，运行期没有额外开销；
//!   4. 掌握多个泛型参数与 const 泛型（把长度、容量这类编译期常量也作为参数）。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_11_generics.rs -o lesson_11 && ./lesson_11
//!   或在本项目根目录执行：cargo run --bin lesson_11_generics
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       本课只讲「泛型」本身；trait 的定义、默认方法、关联类型、dyn 动态分发
//!       属于 lesson_12_traits.rs 的内容，本文件里出现的 `PartialOrd` / `Debug` 等
//!       都只是标准库已有的 trait，作为「约束」使用，不去定义新 trait。

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::mem::size_of;

fn main() {
    println!("========== lesson_11_generics：泛型 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_generic_function();
    demo_2_generic_struct();
    demo_3_generic_enum();
    demo_4_generic_method();
    demo_5_impl_for_concrete_type();
    demo_6_trait_bounds_syntax();
    demo_7_where_clause();
    demo_8_multiple_type_params();
    demo_9_const_generics();
    demo_10_monomorphization();
    demo_11_scenario_generic_cache();
    demo_12_common_mistakes();
}

/// 示例 1：泛型函数
///
/// 要点：函数体只写一遍，用类型参数 `T` 代表「待定的类型」；
///       `<T: PartialOrd + Copy>` 是 trait bound：告诉编译器 T 必须支持比较（>）
///       且支持按位复制（这样 list[0] 能直接拷出来当初始最大值）。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // 先把第一个元素复制一份，作为「当前最大值」的起点
    let mut max = list[0];
    // 从第二个元素开始逐个比较；`&item` 模式把 &T 解构成 T（因为 T: Copy）
    for &item in list.iter() {
        // 只有严格更大才替换，保证结果稳定（相等时保留先出现的那个）
        if item > max {
            max = item;
        }
    }
    max
}

/// 恒等函数：最简单、也最能说明泛型函数价值的例子
fn identity<T>(value: T) -> T {
    // 这里完全不需要知道 T 是什么类型，直接原样返回
    value
}

fn demo_1_generic_function() {
    println!("--- 示例 1：泛型函数 ---");

    // 显式标注类型：编译器会用 i32 去替换 T
    let a = identity::<i32>(42);
    println!("identity::<i32>(42) = {a}");
    println!("// 预期输出：identity::<i32>(42) = 42");

    // 省略类型参数：由实参类型自动推断出 T = &str
    let b = identity("hello generics");
    println!("identity(\"hello generics\") = {b}");
    println!("// 预期输出：identity(\"hello generics\") = hello generics");

    // 同一个 largest 同时服务于 i32 / f64 / char 三种类型
    let numbers = [34, 7, 91, 25];
    println!("整数切片最大值：{}", largest(&numbers));
    println!("// 预期输出：整数切片最大值：91");

    let floats = [1.5, 9.25, 3.75];
    println!("浮点切片最大值：{}", largest(&floats));
    println!("// 预期输出：浮点切片最大值：9.25");

    // 字符按 Unicode 码点比较：'a'(97) < 'm'(109)，'Z'(90) 比它们都小
    let letters = ['a', 'Z', 'm'];
    println!("字符切片最大值：{}", largest(&letters));
    println!("// 预期输出：字符切片最大值：m");
}

/// 示例 2：泛型结构体
///
/// 要点：字段类型可以是类型参数；一个 `Point<T>` 定义能生成 `Point<i32>`、`Point<f64>` …
///       这些「具体类型」彼此完全独立，不能互相赋值。
#[derive(Debug)] // Debug 是本课唯一用到的派生：方便用 {:?} 打印观察
struct Point<T> {
    x: T,
    y: T,
}

fn demo_2_generic_struct() {
    println!("--- 示例 2：泛型结构体 ---");

    // 由字段值推断出 T = i32
    let int_point = Point { x: 3, y: 4 };
    println!("整数点：{int_point:?}");
    println!("// 预期输出：整数点：Point {{ x: 3, y: 4 }}");

    // 同一个结构体，T = f64
    let float_point = Point { x: 1.5, y: -2.5 };
    println!("浮点点：{float_point:?}");
    println!("// 预期输出：浮点点：Point {{ x: 1.5, y: -2.5 }}");

    // 显式标注类型参数：等价于上面第一行，但类型一目了然
    let explicit: Point<u8> = Point { x: 10, y: 20 };
    println!("显式标注的点：{explicit:?}");
    println!("// 预期输出：显式标注的点：Point {{ x: 10, y: 20 }}");
}

/// 示例 3：泛型枚举
///
/// 要点：标准库的 `Option<T>`、`Result<T, E>` 就是泛型枚举；
///       我们自己定义时，类型参数写法和泛型结构体完全一致。
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

/// 一个带泛型载荷的事件枚举：不同事件可以携带不同类型的数据
#[derive(Debug)]
enum Event<T> {
    Started,
    Data(T),
    Stopped,
}

/// 泛型枚举也能拥有泛型方法
impl<T> MyOption<T> {
    /// 判断是否为 Some，只借用不消耗
    fn is_some(&self) -> bool {
        // matches! 宏把「模式匹配 + 返回 bool」压缩成一行
        matches!(self, MyOption::Some(_))
    }

    /// 按值取出内容，若是 None 就用调用者给的兜底值
    fn unwrap_or(self, fallback: T) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback,
        }
    }
}

fn demo_3_generic_enum() {
    println!("--- 示例 3：泛型枚举 ---");

    let some_number: MyOption<i32> = MyOption::Some(7);
    println!("some_number = {some_number:?}");
    println!("// 预期输出：some_number = Some(7)");

    println!("some_number.is_some() = {}", some_number.is_some());
    println!("// 预期输出：some_number.is_some() = true");

    let none_text: MyOption<String> = MyOption::None;
    println!("none_text.is_some() = {}", none_text.is_some());
    println!("// 预期输出：none_text.is_some() = false");

    // unwrap_or 消耗掉枚举，取出内部值
    println!("some_number.unwrap_or(0) = {}", some_number.unwrap_or(0));
    println!("// 预期输出：some_number.unwrap_or(0) = 7");

    println!(
        "none_text.unwrap_or(\"默认\".to_string()) = {}",
        none_text.unwrap_or(String::from("默认"))
    );
    println!("// 预期输出：none_text.unwrap_or(\"默认\".to_string()) = 默认");

    // MyResult<T, E> 有两个彼此独立的类型参数
    let ok: MyResult<u32, String> = MyResult::Ok(200);
    let err: MyResult<u32, String> = MyResult::Err(String::from("连接超时"));
    // 模式匹配时用 & 借用，避免把 ok / err 移动走（edition 2024 下默认绑定模式即可）
    match &ok {
        MyResult::Ok(code) => println!("成功，状态码 {code}"),
        MyResult::Err(message) => println!("失败：{message}"),
    }
    println!("// 预期输出：成功，状态码 200");

    match &err {
        MyResult::Ok(code) => println!("成功，状态码 {code}"),
        MyResult::Err(message) => println!("失败：{message}"),
    }
    println!("// 预期输出：失败：连接超时");

    // 事件流：Started 不携带数据，Data 携带 T，Stopped 结束
    let events = [Event::Started, Event::Data(3.5_f64), Event::Stopped];
    for event in &events {
        let text = match event {
            Event::Started => String::from("开始"),
            Event::Data(value) => format!("数据 {value}"),
            Event::Stopped => String::from("结束"),
        };
        println!("事件：{text}");
    }
    println!("// 预期输出：事件：开始");
    println!("// 预期输出：事件：数据 3.5");
    println!("// 预期输出：事件：结束");
}

/// 示例 4：泛型方法
///
/// 要点：`impl<T> Point<T>` 表示「对任意 T 都提供这些方法」；
///       方法自己还能再引入新的类型参数（`fn map<U, F>`），
///       所以一个 impl 块里可以同时出现 T、U、F 三种参数。
impl<T> Point<T> {
    /// 关联函数（没有 self），用于构造
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// 返回字段的引用：注意生命周期省略规则让签名保持简洁（详见 lesson 13）
    fn x(&self) -> &T {
        &self.x
    }

    /// 方法级别的泛型参数 U、F：把 T 映射成 U
    /// `F: Fn(T) -> U` 表示 F 是一个「吃 T 吐 U」的闭包
    fn map<U, F>(self, f: F) -> Point<U>
    where
        F: Fn(T) -> U,
    {
        // self 被消耗，字段按值取出后交给闭包转换
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }

    /// 用另一个类型的点拼出一个双类型点：T 来自 self，U 来自 other
    fn mixup<U>(self, other: Point<U>) -> MixedPoint<T, U> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn demo_4_generic_method() {
    println!("--- 示例 4：泛型方法 ---");

    let p = Point::new(3, 4);
    println!("p = {p:?}");
    println!("// 预期输出：p = Point {{ x: 3, y: 4 }}");

    println!("p.x() = {}", p.x());
    println!("// 预期输出：p.x() = 3");

    // map 把 Point<i32> 变成 Point<i32>（这里 U 恰好也是 i32）
    let doubled = Point::new(1, 2).map(|value| value * 10);
    println!("doubled = {doubled:?}");
    println!("// 预期输出：doubled = Point {{ x: 10, y: 20 }}");

    // map 也能换类型：Point<i32> -> Point<String>
    let labelled = Point::new(7, 9).map(|value| format!("值-{value}"));
    println!("labelled = {labelled:?}");
    println!("// 预期输出：labelled = Point {{ x: \"值-7\", y: \"值-9\" }}");

    // mixup：两个不同类型参数各自保留自己的类型
    let mixed = Point::new(1, 2).mixup(Point::new("左", "右"));
    println!("mixed = {mixed:?}");
    println!("// 预期输出：mixed = MixedPoint {{ x: 1, y: \"右\" }}");
}

/// 示例 5：只为某个具体类型实现方法
///
/// 要点：`impl Point<f64>` 不是泛型 impl，它只作用于 T = f64 的那一种实例；
///       `Point<i32>` 上没有 distance_from_origin 方法。
impl Point<f64> {
    /// 到原点的欧氏距离，只有浮点坐标才谈得上开方
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn demo_5_impl_for_concrete_type() {
    println!("--- 示例 5：只为具体类型实现方法 ---");

    let float_point = Point::new(3.0_f64, 4.0_f64);
    // 3-4-5 直角三角形：距离正好是 5
    println!("到原点距离：{}", float_point.distance_from_origin());
    println!("// 预期输出：到原点距离：5");

    // 下面这行如果取消注释会编译失败，说明泛型 impl 与具体类型 impl 的区别：
    // let int_point = Point::new(3, 4);
    // int_point.distance_from_origin();
    // error[E0599]: no method named `distance_from_origin` found for struct `Point<{integer}>`
    // 修正：改为 Point::<f64>::new(3.0, 4.0)，或者把 impl 写成 impl<T: Into<f64>> Point<T> 并自行转换
}

/// 示例 6：trait bound 的三种书写位置
///
/// 要点：约束（bound）可以写在函数、结构体、impl 块上；
///       bound 表达的是「使用方对类型的要求」，编译器据此检查每一个具体类型。
#[derive(Debug)]
struct Labeled<T> {
    value: T,
    tag: String,
}

impl<T: Debug> Labeled<T> {
    /// impl 块上的约束对块内所有方法生效
    fn describe(&self) -> String {
        // {:?} 需要 T: Debug，这正是 impl 块上写约束的原因
        format!("[{}] {:?}", self.tag, self.value)
    }

    /// 方法自己也可以追加约束（U 是方法级参数）
    fn pair_with<U: Debug>(&self, other: &U) -> String {
        format!("{} <-> {:?}", self.describe(), other)
    }
}

/// 函数级约束：`T: Debug` 写在尖括号里，适合只有一个约束的简单情形
fn debug_label<T: Debug>(value: &T) -> String {
    format!("{value:?}")
}

fn demo_6_trait_bounds_syntax() {
    println!("--- 示例 6：trait bound 语法与位置 ---");

    println!("debug_label(&2025) = {}", debug_label(&2025));
    println!("// 预期输出：debug_label(&2025) = 2025");

    let labeled = Labeled {
        value: vec![1, 2, 3],
        tag: String::from("列表"),
    };
    println!("labeled.describe() = {}", labeled.describe());
    println!("// 预期输出：labeled.describe() = [列表] [1, 2, 3]");

    println!(
        "labeled.pair_with(&\"附加\") = {}",
        labeled.pair_with(&"附加")
    );
    println!("// 预期输出：labeled.pair_with(&\"附加\") = [列表] [1, 2, 3] <-> \"附加\"");

    // 常见错误演示：若把 impl 块上的 T: Debug 约束去掉，报错位置不在构造语句，
    // 而在真正需要 Debug 的地方 —— 也就是 describe() 的实现体与这里的调用点：
    // let plain = Labeled { value: 1, tag: String::from("没有约束") }; // 构造本身没问题
    // println!("{}", plain.describe());   // ← 报错发生在这里（impl 内部用到了 {:?}）
    // error[E0277]: `T` doesn't implement `std::fmt::Debug`
    // 修正：把约束补上 —— impl<T: Debug> Labeled<T>
    println!("上面的 Labeled<i32> 之所以能打印，是因为 impl 块上写了 T: Debug");
}

/// 示例 7：where 子句
///
/// 要点：约束多、或者约束本身很长时，where 子句比挤在尖括号里更易读；
///       两者语义完全等价，只是排版不同（rustfmt 也偏好这种换行风格）。
fn largest_where<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    // 与示例 1 的 largest 是同一份逻辑，只换了约束的书写位置
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

/// 多参数 + 多个约束：where 子句可以每行一个约束，非常清晰
fn describe_pair<T, U>(first: &T, second: &U) -> String
where
    T: Debug,
    U: Debug,
{
    format!("{first:?} 和 {second:?}")
}

fn demo_7_where_clause() {
    println!("--- 示例 7：where 子句 ---");

    let scores = [88, 95, 61, 79];
    println!("积分最高者：{}", largest_where(&scores));
    println!("// 预期输出：积分最高者：95");

    println!("describe_pair(&1, &\"一\") = {}", describe_pair(&1, &"一"));
    println!("// 预期输出：describe_pair(&1, &\"一\") = 1 和 \"一\"");

    println!(
        "describe_pair(&true, &(1.5, 2)) = {}",
        describe_pair(&true, &(1.5, 2))
    );
    println!("// 预期输出：describe_pair(&true, &(1.5, 2)) = true 和 (1.5, 2)");
}

/// 示例 8：多个泛型参数
///
/// 要点：类型参数可以有两个及以上，各自独立；泛型结构体的字段可以分别使用它们；
///       元组结构体同样支持泛型。
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

/// 两个类型参数各有归属：x 是 T，y 是 U，互不干扰
impl<T, U> MixedPoint<T, U> {
    /// 读取 x 字段：返回 &T，调用方不必知道 U 是什么
    fn x(&self) -> &T {
        &self.x
    }

    /// 读取 y 字段：返回 &U，与 x 的类型无关
    fn y(&self) -> &U {
        &self.y
    }
}

/// 泛型元组结构体：适合做「一对东西」的轻量载体
#[derive(Debug)]
struct Pair<A, B>(A, B);

impl<A, B> Pair<A, B> {
    fn new(first: A, second: B) -> Self {
        Self(first, second)
    }

    /// 交换两个位置的类型：返回值是 Pair<B, A>
    fn swap(self) -> Pair<B, A> {
        Pair(self.1, self.0)
    }

    /// 拆成普通元组，避免调用方依赖本类型
    fn into_tuple(self) -> (A, B) {
        (self.0, self.1)
    }
}

fn demo_8_multiple_type_params() {
    println!("--- 示例 8：多个泛型参数 ---");

    let mixed = MixedPoint { x: 42, y: "答案" };
    println!("mixed = {mixed:?}");
    println!("// 预期输出：mixed = MixedPoint {{ x: 42, y: \"答案\" }}");

    // 分别取两个字段：x 是 i32，y 是 &str，说明两个类型参数各自独立
    println!("mixed.x() = {}", mixed.x());
    println!("// 预期输出：mixed.x() = 42");

    println!("mixed.y() = {}", mixed.y());
    println!("// 预期输出：mixed.y() = 答案");

    let pair = Pair::new(1, "一");
    println!("pair = {pair:?}");
    println!("// 预期输出：pair = Pair(1, \"一\")");

    // swap 之后类型参数顺序互换：Pair<i32, &str> -> Pair<&str, i32>
    let swapped = Pair::new(1, "一").swap();
    println!("swapped = {swapped:?}");
    println!("// 预期输出：swapped = Pair(\"一\", 1)");

    let tuple = pair.into_tuple();
    println!("tuple = {tuple:?}");
    println!("// 预期输出：tuple = (1, \"一\")");
}

/// 示例 9：const 泛型
///
/// 要点：类型参数不只可以是类型，还可以是「编译期常量」，写作 `const N: usize`；
///       数组长度 `[u8; N]` 因此可以被泛型结构体参数化，且长度是类型的一部分：
///       `Buffer<4>` 与 `Buffer<8>` 是两种不同的类型。
#[derive(Debug)]
struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> Buffer<N> {
    /// 编译期就把容量 N 固定下来，构造时不需要再传容量参数
    fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }

    /// 追加一个字节；返回 false 表示缓冲区已满（不 panic，交给调用方决定）
    fn push(&mut self, byte: u8) -> bool {
        if self.len < N {
            self.data[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }

    /// 只暴露已写入的部分
    fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// 容量来自类型参数，属于编译期常量
    fn capacity(&self) -> usize {
        N
    }
}

/// 函数也能使用 const 泛型：返回一个填满 value 的定长数组
fn filled<T: Copy, const N: usize>(value: T) -> [T; N] {
    // 数组重复表达式 `[value; N]` 要求元素类型 T: Copy
    [value; N]
}

fn demo_9_const_generics() {
    println!("--- 示例 9：const 泛型 ---");

    // Buffer<4> 与 Buffer<8> 是两个不同类型，容量写在类型里
    let mut small: Buffer<4> = Buffer::new();
    println!("small 的容量：{}", small.capacity());
    println!("// 预期输出：small 的容量：4");

    // 前 4 次成功，第 5 次因为已满而返回 false
    for byte in [10u8, 20, 30, 40] {
        let accepted = small.push(byte);
        println!("压入 {byte} 是否成功：{accepted}");
    }
    println!("// 预期输出：压入 10 是否成功：true");
    println!("// 预期输出：压入 20 是否成功：true");
    println!("// 预期输出：压入 30 是否成功：true");
    println!("// 预期输出：压入 40 是否成功：true");

    println!("再压入 50 是否成功：{}", small.push(50));
    println!("// 预期输出：再压入 50 是否成功：false");

    println!("small 的内容：{:?}", small.as_slice());
    println!("// 预期输出：small 的内容：[10, 20, 30, 40]");

    // 同一个泛型结构体的另一种实例化
    let mut big: Buffer<8> = Buffer::new();
    // 忽略返回值只在确实不关心是否写满时使用；此处容量充足，必然为 true
    let _ = big.push(1);
    println!("big 的内容：{:?}", big.as_slice());
    println!("// 预期输出：big 的内容：[1]");

    // 函数上的 const 泛型：N 由调用处决定
    let threes: [u8; 3] = filled(7);
    println!("filled::<u8, 3>(7) = {:?}", threes);
    println!("// 预期输出：filled::<u8, 3>(7) = [7, 7, 7]");

    let flags: [bool; 5] = filled(true);
    println!("filled::<bool, 5>(true) = {:?}", flags);
    println!("// 预期输出：filled::<bool, 5>(true) = [true, true, true, true, true]");
}

/// 示例 10：单态化（monomorphization）
///
/// 要点：Rust 的泛型是「编译期展开」的——编译器为每个用到的具体类型
///       各生成一份专用代码（可以理解为自动写出了 largest_i32、largest_f64 …），
///       所以运行期没有类型检查/装箱的开销；代价是二进制体积和编译时间会增加。
fn describe_size<T>(label: &str) {
    // size_of::<T>() 是编译期常量：正因为 T 在编译期已确定，这才能算出来
    println!("{label} 占用 {} 字节", size_of::<T>());
}

fn demo_10_monomorphization() {
    println!("--- 示例 10：单态化原理 ---");

    // 同一个泛型函数 describe_size，以 5 种具体类型各实例化一次
    describe_size::<i32>("i32");
    println!("// 预期输出：i32 占用 4 字节");

    describe_size::<f64>("f64");
    println!("// 预期输出：f64 占用 8 字节");

    // Point<i32> = 两个 i32 = 8 字节
    describe_size::<Point<i32>>("Point<i32>");
    println!("// 预期输出：Point<i32> 占用 8 字节");

    // Point<f64> = 两个 f64 = 16 字节：同一份定义，不同的内存布局
    describe_size::<Point<f64>>("Point<f64>");
    println!("// 预期输出：Point<f64> 占用 16 字节");

    // Buffer<8> = [u8; 8] + usize，usize 需要 8 字节对齐，总大小 16 字节
    describe_size::<Buffer<8>>("Buffer<8>");
    println!("// 预期输出：Buffer<8> 占用 16 字节");

    // 运行期不存在「未知类型」：每个实例都是普通的具体函数，可直接内联优化
    println!("泛型在编译期展开，运行期没有任何类型判断开销");
}

/// 示例 11：典型使用场景 —— 泛型缓存 + 记忆化
///
/// 要点：把「键值存储」抽成与业务无关的泛型结构体，K 只要可哈希可比较、
///       V 只要可克隆就能用；再用它给递归函数做记忆化，避免重复计算。
struct Cache<K, V> {
    store: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K: Eq + Hash, V: Clone> Cache<K, V> {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// 查询：命中则计数并返回克隆值，未命中也要计数
    fn get(&mut self, key: &K) -> Option<V> {
        match self.store.get(key) {
            Some(value) => {
                // 命中：克隆一份交给调用方，缓存本身继续持有原值
                self.hits += 1;
                Some(value.clone())
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    /// 写入：调用方交出键的所有权
    fn insert(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    }

    /// 组合操作：没有就现算。F: FnOnce() -> V 表示「最多调用一次的闭包」
    fn get_or_insert_with<F>(&mut self, key: K, make: F) -> V
    where
        F: FnOnce() -> V,
    {
        // 先借用查询；命中直接返回，不用执行 make
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = make();
        // 缓存里留一份，另一份返回给调用方
        self.insert(key, value.clone());
        value
    }

    /// (命中次数, 未命中次数)
    fn stats(&self) -> (usize, usize) {
        (self.hits, self.misses)
    }
}

/// 记忆化斐波那契：缓存是泛型的，这里实例化为 Cache<String, u64>
fn memo_fib(cache: &mut Cache<String, u64>, n: u64) -> u64 {
    // 每次递归都要构造键，所以键用 String（真实项目里常改写成整数下标的 Vec）
    let key = format!("fib({n})");
    if let Some(value) = cache.get(&key) {
        return value;
    }
    // 未命中：递归计算。注意先结束上一次借用，才能把 cache 再借给递归调用
    let value = match n {
        0 => 0,
        1 => 1,
        _ => memo_fib(cache, n - 1) + memo_fib(cache, n - 2),
    };
    cache.insert(key, value);
    value
}

fn demo_11_scenario_generic_cache() {
    println!("--- 示例 11：典型使用场景 —— 泛型缓存与记忆化 ---");

    let mut fib_cache: Cache<String, u64> = Cache::new();
    println!("fib(10) = {}", memo_fib(&mut fib_cache, 10));
    println!("// 预期输出：fib(10) = 55");

    println!("fib(20) = {}", memo_fib(&mut fib_cache, 20));
    println!("// 预期输出：fib(20) = 6765");

    // fib(5) 在算 fib(10)/fib(20) 时已经存进缓存，这里应直接命中
    println!("fib(5)（应命中缓存） = {}", memo_fib(&mut fib_cache, 5));
    println!("// 预期输出：fib(5)（应命中缓存） = 5");

    let (hits, misses) = fib_cache.stats();
    // 11 个键（fib(0)..fib(10)）首次计算各未命中一次，之后重复访问都命中
    println!("缓存命中 {hits} 次，未命中 {misses} 次");
    println!("// 预期输出：缓存命中 20 次，未命中 21 次");

    // 同一份 Cache 定义，换成 Cache<i32, String> 依然直接可用
    let mut label_cache: Cache<i32, String> = Cache::new();
    let label = label_cache.get_or_insert_with(7, || String::from("七"));
    println!("key=7 得到：{label}");
    println!("// 预期输出：key=7 得到：七");

    let again = label_cache.get_or_insert_with(7, || String::from("重新计算"));
    println!("再次 key=7 得到：{again}");
    println!("// 预期输出：再次 key=7 得到：七");

    let (label_hits, label_misses) = label_cache.stats();
    println!("标签缓存命中 {label_hits} 次，未命中 {label_misses} 次");
    println!("// 预期输出：标签缓存命中 1 次，未命中 1 次");
}

/// 示例 12：常见错误示例
///
/// 要点：泛型代码的错误几乎都指向「约束不足」或「类型不匹配」。
///       下面每段错误代码都注释掉了，并写明错误编号与修正方法。
fn demo_12_common_mistakes() {
    println!("--- 示例 12：常见错误示例 ---");

    // 错误 1：泛型参数上直接做加法，编译器不知道 T 支持 `+`
    // error[E0369]: cannot add `T` to `T`
    // fn add<T>(a: T, b: T) -> T {
    //     a + b
    // }
    // 修正：补上加法约束并声明输出类型
    // fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    //     a + b
    // }
    println!("错误 1：泛型里直接 `a + b` —— error[E0369]，需 T: std::ops::Add<Output = T>");

    // 错误 2：用 {} 打印泛型值，但 T 不一定实现 Display
    // error[E0277]: `T` doesn't implement `std::fmt::Display`
    // （这条错误在别的场景下也会显示成 the trait bound `T: std::fmt::Display` is not satisfied）
    // fn show<T>(value: T) {
    //     println!("{value}");
    // }
    // 修正：加上 T: std::fmt::Display；只想调试打印就用 {:?} 并约束 T: std::fmt::Debug
    println!("错误 2：`println!(\"{{value}}\")` —— error[E0277]，需 T: std::fmt::Display");

    // 错误 3：类型参数声明了却没用上
    // error[E0392]: type parameter `T` is never used
    // struct Wrapper<T> {
    //     value: i32,
    // }
    // 修正：要么删掉 <T>，要么让某个字段真的用上 T（value: T）
    // 若确实是为了标记而保留，标准库风格是 std::marker::PhantomData<T>（本课不展开）
    println!("错误 3：`struct Wrapper<T> {{ value: i32 }}` —— error[E0392]，T 未被使用");

    // 错误 4：泛型实参个数不对
    // error[E0107]: struct takes 2 generic arguments but 1 generic argument was supplied
    // let p = MixedPoint::<i32> { x: 1, y: 2 };
    // 修正：写全 MixedPoint::<i32, &str> { x: 1, y: "二" }，或省略 turbofish 让编译器推断
    println!("错误 4：`MixedPoint::<i32>` 少给一个参数 —— error[E0107]，需 2 个泛型实参");

    // 错误 5：同一个类型参数只能是一种具体类型，x 与 y 想用不同类型必须加参数
    // error[E0308]: mismatched types
    // let p = Point { x: 1, y: 2.5 };
    // 修正：改用两个类型参数 —— MixedPoint { x: 1, y: 2.5 }
    println!("错误 5：`Point {{ x: 1, y: 2.5 }}` —— error[E0308]，同 T 不能既是 i32 又是 f64");

    // 错误 6：const 泛型的实参必须是编译期常量
    // error[E0435]: attempt to use a non-constant value in a constant
    // let n = 4;
    // let buffer: Buffer<n> = Buffer::new();
    // 修正：写成 Buffer::<4>::new()，或先声明 const N: usize = 4; 再 Buffer::<N>::new()
    println!("错误 6：把运行期变量当作 const 泛型实参 —— error[E0435]，必须是编译期常量");

    // 错误 7：泛型方法在错误的类型上调用（示例 5 已演示）
    // error[E0599]: no method named `distance_from_origin` found for struct `Point<{integer}>`
    // 修正：换成 Point<f64>，或把方法定义在 impl<T: Into<f64>> Point<T> 里
    println!("错误 7：在 Point<i32> 上调用只属于 Point<f64> 的方法 —— error[E0599]");

    println!("以上错误都可用「补约束 / 改类型参数 / 换成编译期常量」三类手段修正");
}
