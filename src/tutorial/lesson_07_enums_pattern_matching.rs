//! lesson_07_enums_pattern_matching.rs —— 主题：枚举与模式匹配（Enum & Pattern Matching）
//!
//! 学习目标：
//!   1. 会定义带数据的枚举变体，并理解"枚举 = 同一类东西的若干可能形态"。
//!   2. 掌握 `Option<T>` 与 `Result<T, E>` 的构造、判断与取值方式。
//!   3. 会用 `match` 穷尽所有分支，并会用通配 `_` 兜底。
//!   4. 会用匹配守卫（guard）、`@` 绑定、`|` 或模式精确表达条件。
//!   5. 会用 `if let` / `while let` / `let else` 简化单分支与解构场景。
//!   6. 能读懂常见的模式匹配编译错误并知道如何修正。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_07_enums_pattern_matching.rs -o lesson_07 && ./lesson_07
//!   或在本项目根目录执行：cargo run --bin lesson_07_enums_pattern_matching
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       `?` 运算符的完整用法属于"错误处理"章节（见 lesson_10_error_handling），本文件仅在示例 3 做最小演示。

fn main() {
    println!("========== lesson_07_enums_pattern_matching：枚举与模式匹配 ==========\n");

    demo_1_enum_with_data();
    demo_2_option();
    demo_3_result();
    demo_4_match_basics_and_wildcard();
    demo_5_guards_and_bindings();
    demo_6_if_let_and_while_let();
    demo_7_let_else();
    demo_8_typical_scenario_command_parser();
    demo_9_common_mistakes();
}

/// 枚举：一条命令可能有四种形态，各有各的数据。
///
/// 用枚举表示"命令"比用字符串 + 一堆可选字段安全得多：
/// 编译器会强制你在处理时覆盖所有形态。
#[derive(Debug, Clone, PartialEq)]
enum Command {
    /// 无数据变体
    Quit,
    /// 带一个 String 的变体
    Echo(String),
    /// 带两个数据的变体：设置项名称与新值
    Set { key: String, value: i32 },
    /// 带多个数据的变体：从 a 移动到 b
    Move { from: u32, to: u32 },
}

/// 示例 1：枚举变体携带数据
///
/// 要点：不同变体可以携带不同类型、不同数量的数据；
/// 枚举在同一时刻只能是其中一个变体。
fn demo_1_enum_with_data() {
    println!("--- 示例 1：枚举变体携带数据 ---");

    // 构造四种不同形态的命令。
    let quit = Command::Quit;
    let echo = Command::Echo(String::from("你好，枚举"));
    let set = Command::Set {
        key: String::from("音量"),
        value: 11,
    };
    let moving = Command::Move { from: 3, to: 8 };

    // Debug 打印：能直观看到"当前是哪个变体、带了什么数据"。
    println!("quit = {quit:?}");
    println!("// 预期输出：quit = Quit");
    println!("echo = {echo:?}");
    println!("// 预期输出：echo = Echo(\"你好，枚举\")");
    println!("set = {set:?}");
    println!("// 预期输出：set = Set {{ key: \"音量\", value: 11 }}");
    println!("moving = {moving:?}");
    println!("// 预期输出：moving = Move {{ from: 3, to: 8 }}");

    // 同一个 Vec 可以装下不同变体，因为它们都是 Command 类型。
    // 这里用 clone 放入副本，好让前面的变量在之后还能继续使用。
    let script = vec![echo.clone(), set, moving, quit];
    println!("脚本共 {} 条命令", script.len());
    println!("// 预期输出：脚本共 4 条命令");

    // 用方法配合 match 读取数据（match 的细节见示例 4）。
    // describe() 的输出不含多余空格：全角冒号"："本身已经占一个字符宽度。
    for (index, command) in script.iter().enumerate() {
        // 这里返回值 String，避免把借用带出循环之外。
        println!("脚本第 {} 条：{}", index + 1, command.describe());
    }
    println!("// 预期输出：脚本第 1 条：回显：你好，枚举");
    println!("// 预期输出：脚本第 2 条：设置 音量 = 11");
    println!("// 预期输出：脚本第 3 条：从 3 移动到 8");
    println!("// 预期输出：脚本第 4 条：退出程序");

    // PartialEq 是派生来的，可以比较两个枚举值。
    println!("echo == Quit ? {}", echo == Command::Quit);
    println!("// 预期输出：echo == Quit ? false");
}

impl Command {
    /// 用 match 从枚举里取出数据；返回 String 而不是 &str，
    /// 这样调用者可以自由决定怎么用（是本示例里最省心的做法）。
    fn describe(&self) -> String {
        match self {
            // 无数据变体：直接匹配变体名。
            Command::Quit => String::from("退出程序"),
            // 元组变体：把内部 String 绑定到 text 上（self 是 &Command，所以 text 是 &String）。
            Command::Echo(text) => format!("回显：{text}"),
            // 结构体变体：按字段名绑定。
            Command::Set { key, value } => format!("设置 {key} = {value}"),
            Command::Move { from, to } => format!("从 {from} 移动到 {to}"),
        }
    }
}

/// 示例 2：Option<T>
///
/// 要点：Option 把"可能没有值"写进类型里，逼着你处理 None 分支。
fn demo_2_option() {
    println!("--- 示例 2：Option ---");

    // Some(值) / None 是 Option<T> 的两个变体。
    let found = find_even(&[1, 3, 5, 6, 7]);
    let missing = find_even(&[1, 3, 5]);
    println!("found = {found:?}, missing = {missing:?}");
    println!("// 预期输出：found = Some(6), missing = None");

    // match 是处理 Option 最完整的写法：两个分支都必须写。
    match found {
        Some(n) => println!("找到偶数：{n}，它的两倍是 {}", n * 2),
        None => println!("没有找到偶数"),
    }
    println!("// 预期输出：找到偶数：6，它的两倍是 12");

    match missing {
        Some(n) => println!("找到偶数：{n}"),
        None => println!("没有找到偶数，走 None 分支"),
    }
    println!("// 预期输出：没有找到偶数，走 None 分支");

    // Option 自带许多链式方法，避免层层嵌套判断。
    println!("found.map(|n| n + 1) = {:?}", found.map(|n| n + 1));
    println!("// 预期输出：found.map(|n| n + 1) = Some(7)");
    println!("missing.unwrap_or(0) = {}", missing.unwrap_or(0));
    println!("// 预期输出：missing.unwrap_or(0) = 0");
    println!("missing.is_none() = {}", missing.is_none());
    println!("// 预期输出：missing.is_none() = true");

    // 只有在逻辑上绝不可能为 None 时才用 unwrap/expect，并写清原因。
    let definitely_some = Some(42);
    // 这里的 unwrap 是安全的：值刚刚由字面量构造出来，不可能是 None。
    println!("definitely_some.unwrap() = {}", definitely_some.unwrap());
    println!("// 预期输出：definitely_some.unwrap() = 42");

    // 可能 panic 的 unwrap：unwrap 本身是**安全函数**（调用它不需要 unsafe），
    // 但当值为 None 时它会直接 panic，所以只能用在"逻辑上不可能失败"的地方。
    // let boom = missing.unwrap(); // 运行期 panic: called `Option::unwrap()` on a `None` value
    // 修正方法：用 match / unwrap_or / unwrap_or_else / if let 处理 None。
    println!("对可能为 None 的值直接 unwrap 会 panic，应改用 match 或 unwrap_or");
    println!("// 预期输出：对可能为 None 的值直接 unwrap 会 panic，应改用 match 或 unwrap_or");
}

/// 工具函数：返回切片里第一个偶数的 Option。
fn find_even(values: &[i32]) -> Option<i32> {
    for &value in values {
        // 找到就提前返回 Some，函数结束时自然返回 None。
        if value % 2 == 0 {
            return Some(value);
        }
    }
    None
}

/// 示例 3：Result<T, E>
///
/// 要点：Result 的 Err 分支携带"为什么失败"的信息，
/// 比 Option 更适合可预期会失败的解析类操作。
fn demo_3_result() {
    println!("--- 示例 3：Result ---");

    // Ok(值) / Err(错误值) 是两个变体；这里的错误值用 String 表示。
    let good = parse_age("30");
    let not_a_number = parse_age("abc");
    let too_large = parse_age("200");
    println!("parse_age(\"30\") = {good:?}");
    println!("// 预期输出：parse_age(\"30\") = Ok(30)");
    println!("parse_age(\"abc\") = {not_a_number:?}");
    println!("// 预期输出：parse_age(\"abc\") = Err(\"不是合法整数\")");
    println!("parse_age(\"200\") = {too_large:?}");
    println!("// 预期输出：parse_age(\"200\") = Err(\"年龄超出合理范围\")");

    // match 分别处理成功与失败，并从 Err 里取出原因。
    // 这里匹配 &good（借用），以免把 good 里的 String 移动出去。
    match &good {
        Ok(age) => println!("解析成功：年龄 {age} 岁"),
        Err(reason) => println!("解析失败：{reason}"),
    }
    println!("// 预期输出：解析成功：年龄 30 岁");

    match &too_large {
        Ok(age) => println!("解析成功：年龄 {age} 岁"),
        Err(reason) => println!("解析失败：{reason}"),
    }
    println!("// 预期输出：解析失败：年龄超出合理范围");

    // is_ok / is_err / unwrap_or 是常见快捷判断。
    println!(
        "good.is_ok() = {}, not_a_number.is_err() = {}",
        good.is_ok(),
        not_a_number.is_err()
    );
    println!("// 预期输出：good.is_ok() = true, not_a_number.is_err() = true");
    println!(
        "not_a_number.as_ref().unwrap_or(&0) = {}",
        not_a_number.as_ref().unwrap_or(&0)
    );
    println!("// 预期输出：not_a_number.as_ref().unwrap_or(&0) = 0");

    // 说明：`?` 运算符能让函数在 Err 时提前返回，
    // 但它要求函数自身返回 Result / Option，因此本章的正文示例都用
    // match 与 unwrap_or 这类不涉及提前返回的写法；`?` 只在下一小节做最小演示。
    println!("提示：? 运算符需要函数返回 Result/Option，完整用法见错误处理章节");
    println!("// 预期输出：提示：? 运算符需要函数返回 Result/Option，完整用法见错误处理章节");

    // `?` 的最小演示（只演示语法，完整讲解见 lesson_10 错误处理）：
    // 函数返回 Result 时，`expr?` 会在 Ok 时取出里面的值继续往下走，
    // 在 Err 时立刻把错误返回给调用者。try_with_question_mark 内部没有任何 match，
    // 只靠一个 `?` 加一个 return Err 就把失败路径交代清楚了。
    println!("用 `?` 试两个输入：");
    println!("// 预期输出：用 `?` 试两个输入：");
    println!("\"30\" -> {:?}", try_with_question_mark("30"));
    println!("// 预期输出：\"30\" -> Ok(30)");
    println!("\"200\" -> {:?}", try_with_question_mark("200"));
    println!("// 预期输出：\"200\" -> Err(\"年龄超出合理范围\")");
}

/// 工具函数：`?` 运算符的最小演示（完整讲解见 lesson_10 错误处理）。
///
/// `?` 的三条硬性要求在这里都能看到：
///   1. 只能用在返回 Result / Option 的函数里（本函数返回 Result<u32, Box<dyn Error>>）；
///   2. 被 `?` 处理的值，其错误类型必须能通过 `From` 转成函数的错误类型：
///      parse 失败时产生的是 `ParseIntError`，靠 `?` 自动转成 `Box<dyn Error>`；
///      若返回类型写死成 `Result<u32, String>`，这一步会直接报 E0277（String 没有
///      `From<ParseIntError>` 的实现），这正是初学 `?` 时最常踩的坑；
///   3. 写 `return Err(...)` 时要自己把错误类型转好（下面用 `.into()`）。
fn try_with_question_mark(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let age: u32 = input.parse::<u32>()?; // parse 失败 -> 立刻 return Err
    if age > 150 {
        return Err(String::from("年龄超出合理范围").into());
    }
    Ok(age) // 成功路径照常返回 Ok
}

/// 工具函数：把字符串解析成"合理范围内的年龄"。
///
/// 错误类型用 String 表示：本示例只关心"能读懂原因"，
/// 自定义错误类型与 From 转换属于错误处理章节的内容。
fn parse_age(input: &str) -> Result<u32, String> {
    // match 处理 parse 的 Result：只用 Ok/Err 两个分支，不提前返回。
    let age: u32 = match input.parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Err(String::from("不是合法整数")),
    };
    if age > 150 {
        // 业务校验失败也走 Err 分支，把原因交给调用者。
        Err(String::from("年龄超出合理范围"))
    } else {
        Ok(age)
    }
}

/// 枚举：交通灯状态，用来演示 match 的穷尽性与通配兜底。
#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
    /// 带数据的变体：倒计时秒数
    Blinking(u8),
}

/// 示例 4：match 基础、穷尽性与通配 `_`
///
/// 要点：match 必须覆盖所有可能；用 `_` 兜底会牺牲"新增变体时被编译器提醒"的好处。
fn demo_4_match_basics_and_wildcard() {
    println!("--- 示例 4：match 与通配 _ ---");

    let lights = [
        TrafficLight::Red,
        TrafficLight::Yellow,
        TrafficLight::Green,
        TrafficLight::Blinking(3),
    ];

    // 穷尽匹配：每个变体都写出来，未来新增变体会直接编译报错提醒你补分支。
    for light in lights {
        let advice = match light {
            TrafficLight::Red => "停车等待",
            TrafficLight::Yellow => "减速准备停车",
            TrafficLight::Green => "可以通行",
            TrafficLight::Blinking(seconds) => {
                // 分支体可以是块，里面能写多条语句。
                if seconds <= 3 {
                    "黄灯闪烁即将变红"
                } else {
                    "信号灯异常，谨慎通行"
                }
            }
        };
        println!("{light:?} -> {advice}");
    }
    println!("// 预期输出：Red -> 停车等待");
    println!("// 预期输出：Yellow -> 减速准备停车");
    println!("// 预期输出：Green -> 可以通行");
    println!("// 预期输出：Blinking(3) -> 黄灯闪烁即将变红");

    // 通配 `_`：只关心少数情况时用它兜底（这里把所有 Blinking 都归为一类）。
    for light in lights {
        let urgent = match light {
            TrafficLight::Red => "需要立即处理",
            _ => "暂时无需处理",
        };
        println!("{light:?} 是否需要处理：{urgent}");
    }
    println!("// 预期输出：Red 是否需要处理：需要立即处理");
    println!("// 预期输出：Yellow 是否需要处理：暂时无需处理");
    println!("// 预期输出：Green 是否需要处理：暂时无需处理");
    println!("// 预期输出：Blinking(3) 是否需要处理：暂时无需处理");

    // 用 `|` 把多个模式合并，等价于写多个相同分支。
    let code = 404;
    let kind = match code {
        200 | 201 | 204 => "成功",
        301 | 302 => "重定向",
        400 | 401 | 403 | 404 => "客户端错误",
        // 用范围模式兜底，避免写出 100..599 的所有值。
        500..=599 => "服务端错误",
        _ => "未知状态码",
    };
    println!("状态码 {code} 属于：{kind}");
    println!("// 预期输出：状态码 404 属于：客户端错误");

    // match 也可以直接解构元组，把多个值一起判断。
    let point = (0, -5);
    let axis = match point {
        (0, 0) => "原点",
        (0, _) => "y 轴上",
        (_, 0) => "x 轴上",
        _ => "普通象限点",
    };
    println!("{point:?} 位于：{axis}");
    println!("// 预期输出：(0, -5) 位于：y 轴上");
}

/// 示例 5：匹配守卫与 `@` 绑定
///
/// 要点：守卫是在模式之后追加的 `if 条件`；
/// `@` 既能做范围/结构检查，又能把值绑定下来使用。
fn demo_5_guards_and_bindings() {
    println!("--- 示例 5：匹配守卫与 @ 绑定 ---");

    // 匹配守卫：同一个变体可以根据条件走不同分支。
    for n in [0, 7, 10, 42, -3] {
        let label = match n {
            0 => String::from("零"),
            // 守卫让 "正偶数" 与 "正奇数" 得以区分。
            n if n > 0 && n % 2 == 0 => format!("正偶数 {n}"),
            n if n > 0 => format!("正奇数 {n}"),
            // 兜底分支必须放在最后。
            _ => format!("负数 {n}"),
        };
        println!("n = {n} -> {label}");
    }
    println!("// 预期输出：n = 0 -> 零");
    println!("// 预期输出：n = 7 -> 正奇数 7");
    println!("// 预期输出：n = 10 -> 正偶数 10");
    println!("// 预期输出：n = 42 -> 正偶数 42");
    println!("// 预期输出：n = -3 -> 负数 -3");

    // 守卫配合枚举：同一个变体按携带的数据分流。
    let commands = [
        Command::Move { from: 1, to: 2 },
        Command::Move { from: 5, to: 5 },
        Command::Echo(String::from("hello")),
    ];
    for command in &commands {
        let status = match command {
            // 守卫里可以直接使用已绑定的 from / to。
            Command::Move { from, to } if from == to => String::from("原地未移动，忽略"),
            Command::Move { from, to } => format!("移动距离 {} 步", to - from),
            Command::Echo(text) if text.is_empty() => String::from("空回显，忽略"),
            Command::Echo(text) => format!("回显 {text}（{} 个字符）", text.chars().count()),
            _ => String::from("其它命令"),
        };
        println!("{command:?} -> {status}");
    }
    println!("// 预期输出：Move {{ from: 1, to: 2 }} -> 移动距离 1 步");
    println!("// 预期输出：Move {{ from: 5, to: 5 }} -> 原地未移动，忽略");
    println!("// 预期输出：Echo(\"hello\") -> 回显 hello（5 个字符）");

    // `@` 绑定：既用范围判断，又把实际值绑下来。
    for score in [58, 60, 85, 100] {
        let grade = match score {
            // 把 0..=59 的成绩绑定为 s，既能判断范围又能打印原值。
            s @ 0..=59 => format!("{s} 分：不及格"),
            s @ 60..=84 => format!("{s} 分：良好"),
            s @ 85..=100 => format!("{s} 分：优秀"),
            // 超出范围时提示数据有问题，避免静默接受非法输入。
            s => format!("{s} 分：成绩超出 0..=100"),
        };
        println!("{grade}");
    }
    println!("// 预期输出：58 分：不及格");
    println!("// 预期输出：60 分：良好");
    println!("// 预期输出：85 分：优秀");
    println!("// 预期输出：100 分：优秀");

    // `@` 绑定也可以绑定部分结构：把整个元组绑定下来。
    let pair = (3, 4);
    let summary = match pair {
        // 匹配"第一个是 3"的元组，同时把整个元组绑定为 p。
        p @ (3, _) => format!("{p:?} 的第一个元素是 3"),
        (a, b) => format!("({a}, {b}) 的第一个元素不是 3"),
    };
    println!("{summary}");
    println!("// 预期输出：(3, 4) 的第一个元素是 3");
}

/// 示例 6：if let 与 while let
///
/// 要点：只关心一个分支时，`if let` 比 match 更短；
/// `while let` 适合"不断取出直到取空"的循环模式。
fn demo_6_if_let_and_while_let() {
    println!("--- 示例 6：if let 与 while let ---");

    let maybe_name = Some("Rust");

    // if let：只处理 Some 的情况，忽略 None。
    if let Some(name) = maybe_name {
        println!("欢迎，{name}！");
        println!("// 预期输出：欢迎，Rust！");
    }

    // if let + else：等价于两分支 match 的简写。
    let maybe_empty: Option<i32> = None;
    if let Some(value) = maybe_empty {
        println!("拿到值 {value}");
    } else {
        println!("没有值，走 else 分支");
    }
    println!("// 预期输出：没有值，走 else 分支");

    // if let 也能用来解构枚举的某个变体。
    let command = Command::Set {
        key: String::from("音量"),
        value: 5,
    };
    if let Command::Set { key, value } = &command {
        println!("检测到设置命令：{key} = {value}");
        println!("// 预期输出：检测到设置命令：音量 = 5");
    }

    // while let：配合会"消耗数据"的方法，一次取出一个元素直到 None。
    let mut queue = MessageQueue::new();
    queue.push(String::from("第一条"));
    queue.push(String::from("第二条"));
    queue.push(String::from("第三条"));

    while let Some(message) = queue.pop() {
        // 每次 pop 都把队首 String 的所有权交出来，队列最终变空。
        println!("处理消息：{message}（剩余 {} 条）", queue.len());
    }
    println!("// 预期输出：处理消息：第一条（剩余 2 条）");
    println!("// 预期输出：处理消息：第二条（剩余 1 条）");
    println!("// 预期输出：处理消息：第三条（剩余 0 条）");
    println!("队列处理完毕，长度为 {}", queue.len());
    println!("// 预期输出：队列处理完毕，长度为 0");
}

/// 消息队列：用 Vec 做内部存储，pop 返回 Option 便于配合 while let。
struct MessageQueue {
    messages: Vec<String>,
}

impl MessageQueue {
    /// 关联函数：创建空队列。
    fn new() -> Self {
        MessageQueue {
            messages: Vec::new(),
        }
    }

    /// &mut self：入队。
    fn push(&mut self, message: String) {
        self.messages.push(message);
    }

    /// &mut self：出队；空队列时返回 None（而不是 panic）。
    fn pop(&mut self) -> Option<String> {
        if self.messages.is_empty() {
            None
        } else {
            // remove(0) 返回被移除的元素，所有权交给调用者。
            Some(self.messages.remove(0))
        }
    }

    /// &self：当前长度。
    fn len(&self) -> usize {
        self.messages.len()
    }
}

/// 示例 7：let else
///
/// 要点：`let ... else { ... }` 要求 else 分支必须发散
/// （panic!、return、continue、break 等），
/// 因此成功路径上拿到的是"已被解包并绑定好的值"，无需再嵌套。
fn demo_7_let_else() {
    println!("--- 示例 7：let else ---");

    // 成功路径直接用 port，失败路径必须离开当前流程。
    let port_text = "8080";
    let Some(port) = parse_port(port_text) else {
        println!("端口配置非法：{port_text}，使用默认值 80");
        return_demo_marker();
        return;
    };
    println!("监听端口 = {port}");
    println!("// 预期输出：监听端口 = 8080");

    // 也可以写成 never-return 之外的写法：用 panic! 结束流程。
    let bad_text = "not-a-port";
    // 下面这行一旦执行就会 panic，所以放在演示里只做注释说明：
    // let Some(_) = parse_port(bad_text) else { panic!("端口非法"); };
    println!("非法输入 {bad_text} 会让 let else 的 else 分支执行（本示例不真的 panic）");
    println!(
        "// 预期输出：非法输入 not-a-port 会让 let else 的 else 分支执行（本示例不真的 panic）"
    );

    // let else 在结构体/枚举解构中同样好用：从引用里拿到字段。
    let command = Command::Move { from: 2, to: 9 };
    let Command::Move { from, to } = &command else {
        // 这里用 unreachable! 表示"这个分支不该发生"；它会 panic，属发散表达式。
        unreachable!("本示例只传入 Move 命令");
    };
    println!("从命令中解构出 from = {from}, to = {to}");
    println!("// 预期输出：从命令中解构出 from = 2, to = 9");

    // 循环里的 let else：失败就跳过本轮，成功则继续往下写。
    let inputs = ["12", "x", "34"];
    let mut total = 0u32;
    for input in inputs {
        let Ok(value) = input.parse::<u32>() else {
            println!("跳过无法解析的输入：{input}");
            continue; // continue 也是发散表达式，满足 let else 的要求
        };
        total += value;
    }
    println!("累加结果 total = {total}");
    println!("// 预期输出：跳过无法解析的输入：x");
    println!("// 预期输出：累加结果 total = 46");
}

/// 工具函数：把字符串解析成合法端口（1..=65535），非法时返回 None。
fn parse_port(text: &str) -> Option<u16> {
    match text.parse::<u16>() {
        // 端口 0 没有实际意义，这里当作非法输入。
        Ok(port) if port > 0 => Some(port),
        _ => None,
    }
}

/// 示例 7 辅助：让演示 return 分支有可见痕迹（避免"静默返回"难以理解）。
fn return_demo_marker() {
    println!("// （这里演示了 else 分支用 return 提前离开函数）");
}

/// 示例 8：典型使用场景 —— 命令解析与状态机
///
/// 要点：枚举 + match 是 Rust 里实现"解析文本 → 结构化命令"的经典组合；
/// 状态机同样可以用枚举 + match 表达得非常清晰。
fn demo_8_typical_scenario_command_parser() {
    println!("--- 示例 8：典型使用场景 —— 命令解析与状态机 ---");

    // 真实场景：把一行行文本配置解析成命令。
    let raw_lines = [
        "echo hello world",
        "set volume 11",
        "move 3 8",
        "quit",
        "unknown do something",
    ];
    for line in raw_lines {
        // parse_line 返回 Result，用 match 处理成功/失败两条路。
        match parse_line(line) {
            Ok(command) => println!("解析成功：{line:?} -> {}", command.describe()),
            Err(reason) => println!("解析失败：{line:?} -> {reason}"),
        }
    }
    println!("// 预期输出：解析成功：\"echo hello world\" -> 回显：hello world");
    println!("// 预期输出：解析成功：\"set volume 11\" -> 设置 volume = 11");
    println!("// 预期输出：解析成功：\"move 3 8\" -> 从 3 移动到 8");
    println!("// 预期输出：解析成功：\"quit\" -> 退出程序");
    println!("// 预期输出：解析失败：\"unknown do something\" -> 未知命令：unknown");

    // 状态机：订单状态用枚举表示，状态迁移用 match 表示，非法迁移直接拒绝。
    let mut state = OrderState::Created;
    println!("初始状态 = {state:?}");
    println!("// 预期输出：初始状态 = Created");
    for event in [Event::Pay, Event::Ship, Event::Deliver, Event::Ship] {
        // next_state 返回 Option：None 表示这个事件在当前状态下非法。
        match state.next(event) {
            Some(next) => {
                state = next;
                println!("事件 {event:?} -> 新状态 {state:?}");
            }
            None => println!("事件 {event:?} 在状态 {state:?} 下非法，已忽略"),
        }
    }
    println!("// 预期输出：事件 Pay -> 新状态 Paid");
    println!("// 预期输出：事件 Ship -> 新状态 Shipped");
    println!("// 预期输出：事件 Deliver -> 新状态 Delivered");
    println!("// 预期输出：事件 Ship 在状态 Delivered 下非法，已忽略");

    // 支付成功/失败两种结果也用枚举表达，避免用 i32 约定返回值。
    for result in [PayResult::Success(9_900), PayResult::Failure("余额不足")] {
        let text = render_pay_result(&result);
        println!("{text}");
    }
    println!("// 预期输出：支付成功：99.00 元");
    println!("// 预期输出：支付失败：余额不足");
}

/// 解析一行文本为 Command。
///
/// 返回 Result：解析失败时用 Err(String) 说明原因，调用者用 match 处理。
fn parse_line(line: &str) -> Result<Command, String> {
    // split_whitespace 天然处理多余空格。
    let mut parts = line.split_whitespace();
    let Some(head) = parts.next() else {
        // let else：空行直接返回 Err，后面的逻辑无需再判空。
        return Err(String::from("空命令行"));
    };
    match head {
        "quit" => Ok(Command::Quit),
        "echo" => {
            // rest 可能为空，此时回显空字符串。
            let text = parts.collect::<Vec<&str>>().join(" ");
            Ok(Command::Echo(text))
        }
        "set" => {
            // 用元组 + Option 一次性拿到两个参数。
            match (parts.next(), parts.next()) {
                (Some(key), Some(value_text)) => match value_text.parse::<i32>() {
                    Ok(value) => Ok(Command::Set {
                        key: key.to_string(),
                        value,
                    }),
                    Err(_) => Err(format!("第二个参数不是整数：{value_text}")),
                },
                _ => Err(String::from("set 需要两个参数：key value")),
            }
        }
        "move" => match (parts.next(), parts.next()) {
            (Some(from_text), Some(to_text)) => {
                match (from_text.parse::<u32>(), to_text.parse::<u32>()) {
                    (Ok(from), Ok(to)) => Ok(Command::Move { from, to }),
                    _ => Err(format!("move 参数必须是无符号整数：{from_text} {to_text}")),
                }
            }
            _ => Err(String::from("move 需要两个参数：from to")),
        },
        other => Err(format!("未知命令：{other}")),
    }
}

/// 订单状态机的状态。
#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderState {
    Created,
    Paid,
    Shipped,
    Delivered,
}

/// 触发状态迁移的事件。
#[derive(Debug, Clone, Copy)]
enum Event {
    Pay,
    Ship,
    Deliver,
}

impl OrderState {
    /// 根据事件计算下一个状态；非法迁移返回 None。
    fn next(self, event: Event) -> Option<OrderState> {
        // 同时匹配"当前状态 + 事件"的组合，可读性远好于一堆 if。
        match (self, event) {
            (OrderState::Created, Event::Pay) => Some(OrderState::Paid),
            (OrderState::Paid, Event::Ship) => Some(OrderState::Shipped),
            (OrderState::Shipped, Event::Deliver) => Some(OrderState::Delivered),
            // 其它组合都非法：显式列出关键非法项，其余用 _ 兜底。
            (OrderState::Created, Event::Ship) => None,
            (OrderState::Delivered, _) => None,
            _ => None,
        }
    }
}

/// 支付结果枚举：成功带金额（分），失败带原因。
#[derive(Debug)]
enum PayResult<'a> {
    Success(u32),
    Failure(&'a str),
}

/// 把支付结果渲染成给用户看的文本。
fn render_pay_result(result: &PayResult<'_>) -> String {
    match result {
        PayResult::Success(cents) => format!("支付成功：{}.{:02} 元", cents / 100, cents % 100),
        PayResult::Failure(reason) => format!("支付失败：{reason}"),
    }
}

/// 示例 9：常见错误示例（错误代码全部注释掉，只保留修正后的写法）
///
/// 要点：match 的报错集中在"分支没写全""模式和值类型不匹配"，
/// if let / let else 的报错集中在"绑定没被使用""else 分支不发散"。
fn demo_9_common_mistakes() {
    println!("--- 示例 9：常见错误示例 ---");

    // 错误 1：match 分支没写全。
    // let light = TrafficLight::Red;
    // let advice = match light {
    //     TrafficLight::Green => "通行",
    // };
    // error[E0004]: non-exhaustive patterns: `TrafficLight::Red`, `TrafficLight::Yellow`
    //                and `TrafficLight::Blinking(_)` not covered
    // （编译器打印的是带枚举名限定的完整路径，这里为排版折行）
    // 修正方法：补全所有变体，或用 `_ => ...` 兜底。
    let light = TrafficLight::Red;
    let advice = match light {
        TrafficLight::Green => "通行",
        // 兜底分支：代价是新增变体时编译器不会再提醒。
        _ => "停车或等待",
    };
    println!("修正 1：补齐分支 -> {light:?} => {advice}");
    println!("// 预期输出：修正 1：补齐分支 -> Red => 停车或等待");

    // 错误 2：模式与值的类型不一致。
    // let n = 5;
    // match n {
    //     "5" => println!("字符串"),
    //     _ => {}
    // }
    // error[E0308]: mismatched types: expected integer, found `&str`
    // 修正方法：模式里的字面量类型必须与匹配值一致（这里应写 5 或加守卫）。
    let n = 5;
    match n {
        5 if n.to_string() == "5" => println!("修正 2：模式类型与值类型保持一致（整数对整数）"),
        _ => println!("修正 2：其余情况"),
    }
    println!("// 预期输出：修正 2：模式类型与值类型保持一致（整数对整数）");

    // 错误 3：把不可反驳模式用在 let 位置（let 只能接必然成立的模式）。
    // let Some(x) = Some(1); // error[E0005]: refutable pattern in local binding
    // 修正方法：改用 let else 提供失败分支。
    let Some(x) = Some(1) else {
        // else 分支必须发散；这里用 panic! 表示"逻辑上不可能"。
        panic!("这个分支不会执行");
    };
    println!("修正 3：let 后面要用不可反驳模式，否则改成 let ... else -> x = {x}");
    println!("// 预期输出：修正 3：let 后面要用不可反驳模式，否则改成 let ... else -> x = 1");

    // 错误 4：let else 的 else 分支不发散。
    // let Some(y) = Some(2) else { println!("没有值"); };
    // error[E0308]: `else` clause of `let...else` does not diverge
    // 修正方法：在 else 块里 return / continue / break / panic!。
    let Some(y) = Some(2) else {
        println!("这里必须先发散（return 等）才能通过编译");
        return;
    };
    println!("修正 4：else 分支用 return/panic!/continue 发散 -> y = {y}");
    println!("// 预期输出：修正 4：else 分支用 return/panic!/continue 发散 -> y = 2");

    // 错误 5：if let 绑定后只写单分支导致"变量未使用"警告。
    // if let Some(value) = Some(3) { } // warning: unused variable: `value`
    // 修正方法：真的不用就写 `_`，要用就在分支里使用它。
    if let Some(value) = Some(3) {
        println!(
            "修正 5：绑定就要用，或者写成 Some(_) -> value 的两倍 = {}",
            value * 2
        );
    }
    println!("// 预期输出：修正 5：绑定就要用，或者写成 Some(_) -> value 的两倍 = 6");

    // 错误 6：对「引用」做模式匹配时又显式写 ref —— edition 2024 明确禁止这种双重借用。
    // let command = Command::Echo(String::from("hi"));
    // match &command {                          // 被匹配的值已经是 &Command
    //     Command::Echo(ref text) => println!("{text}"), // ref 又被禁止
    //     _ => {}
    // }
    // error: cannot explicitly borrow within an implicitly-borrowing pattern（该错误没有 E 编号）
    // 注意区分：如果被匹配的是**拥有所有权**的值（如 `match command`），
    // 此时默认绑定模式是 move，写 `Command::Echo(ref text)` 是完全合法的。
    // 只有像上面这样"已经借用了还再写 ref"才会报错。
    // 修正方法：用 & 模式让绑定自动成为引用，或直接按值匹配。
    let command = Command::Echo(String::from("hi"));
    match &command {
        // 对 &Command 匹配，text 自动是 &String，无需写 ref。
        Command::Echo(text) => println!("修正 6：用 & 模式代替 ref -> {text}"),
        _ => println!("修正 6：其它命令"),
    }
    println!("// 预期输出：修正 6：用 & 模式代替 ref -> hi");

    // 错误 7：用 unwrap 处理可预期的失败，导致运行期 panic。
    // let value = parse_port("0").unwrap(); // panic: called `Option::unwrap()` on a `None` value
    // 修正方法：用 match / if let / unwrap_or 明确处理失败路径。
    let port = parse_port("0").unwrap_or(80);
    println!("修正 7：用 unwrap_or 兜底 -> port = {port}");
    println!("// 预期输出：修正 7：用 unwrap_or 兜底 -> port = 80");
}
