//! lesson_10_error_handling.rs —— 主题：错误处理（panic / Result / ? / 自定义错误 / Box<dyn Error>）
//!
//! 学习目标：
//!   1. 分清「不可恢复错误 panic!」与「可恢复错误 Result<T, E>」的适用场景
//!   2. 掌握 unwrap / expect 的语义与它们隐藏的 panic 风险
//!   3. 会用 `?` 运算符在返回 Result 的函数中传播错误，并理解它背后的 `From` 自动转换
//!   4. 会定义自定义错误类型，同时实现 `std::fmt::Display` 与 `std::error::Error`
//!   5. 会用 `Box<dyn Error>` 做统一错误出口，以及写在 `fn main() -> Result<...>` 里的写法
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_10_error_handling.rs -o lesson_10 && ./lesson_10
//!   或在本项目根目录执行：cargo run --bin lesson_10_error_handling
//!
//! 说明：
//!   - 本文件所有示例均只依赖标准库，可直接编译运行。
//!   - 为保证整程序退出码为 0，本文件中**没有任何会真正把进程打挂的 panic**：
//!     需要演示 panic 的地方，要么被 `catch_unwind` 捕获，要么被注释掉并写明错误信息。
//!   - main 的返回值是 `Result<(), Box<dyn Error>>`：失败时标准库的运行时会打印 `Error: ...`
//!     并以退出码 1 结束。因此 main 中只使用 `?` 传播错误，不混用 unwrap。

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseFloatError;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

/// 统一的「应用级错误」类型别名：把动态错误装箱，便于跨层传递。
type AppResult<T> = Result<T, Box<dyn Error>>;

/// 本课的自定义错误类型：配置解析错误。
///
/// 要点：一个合格的错误类型需要
///   1. `Debug`（这样才能放进 `Result` 并配合 `?` 与测试断言）；
///   2. `Display`（面向使用者的可读消息）；
///   3. `std::error::Error`（让自己能参与错误链、装箱与 `?` 的组合）。
#[derive(Debug)]
enum ConfigError {
    /// 配置行里没有出现分隔符 `=`
    MissingSeparator {
        /// 出错的整行原文
        line: String,
    },
    /// 键值对存在，但值是空白的
    EmptyValue {
        /// 对应的键
        key: String,
    },
    /// 值不是合法浮点数（内部持有标准库的原始错误，形成错误链）
    InvalidNumber {
        /// 对应的键
        key: String,
        /// 由 `String::parse` 返回的底层错误
        source: ParseFloatError,
    },
    /// 值不是合法整数（这里只保留错误文本，演示「不同底层错误的另一种处理方式」）
    InvalidInteger {
        /// 对应的键
        key: String,
        /// 底层错误的可读描述
        detail: String,
    },
    /// 查询了一个不存在的键
    MissingKey {
        /// 被查询的键
        key: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display 面向「人」：给出足够定位问题的上下文
        match self {
            ConfigError::MissingSeparator { line } => {
                write!(f, "配置行缺少 `=` 分隔符：{line:?}")
            }
            ConfigError::EmptyValue { key } => write!(f, "配置项 `{key}` 的值是空的"),
            ConfigError::InvalidNumber { key, source } => {
                write!(f, "配置项 `{key}` 不是合法数字：{source}")
            }
            ConfigError::InvalidInteger { key, detail } => {
                write!(f, "配置项 `{key}` 不是合法整数：{detail}")
            }
            ConfigError::MissingKey { key } => write!(f, "配置中缺少必需的键 `{key}`"),
        }
    }
}

impl Error for ConfigError {
    /// 给出「底层原因」，让调用方能顺着错误链继续排查。
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // 只有 InvalidNumber 有真正的底层错误
            ConfigError::InvalidNumber { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<ParseFloatError> for ConfigError {
    /// 关键：实现 `From` 后，`?` 就能把 `ParseFloatError` 自动上转成 `ConfigError`。
    fn from(source: ParseFloatError) -> Self {
        ConfigError::InvalidNumber {
            key: String::from("<未知键>"), // 没有键信息时给一个占位，避免丢错误
            source,
        }
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    /// 另一种底层错误类型也需要各自的 `From` 实现：只有实现了 `From<E>`，
    /// 函数的 `?` 才能接收 `E`。这里把整数解析错误转成字符串再记录，
    /// 因为它无法无损地变成 `ParseFloatError`（两者是不同的类型）。
    fn from(source: std::num::ParseIntError) -> Self {
        ConfigError::InvalidInteger {
            key: String::from("<未知键>"), // 没有键信息时给一个占位，避免丢错误
            detail: source.to_string(),
        }
    }
}

/// 可恢复的除法：除数为 0 时返回错误，而不是 panic。
fn safe_divide(top: f64, bottom: f64) -> Result<f64, String> {
    if bottom == 0.0 {
        Err(String::from("除数不能为 0"))
    } else {
        Ok(top / bottom)
    }
}

/// 解析形如 `"key=value"` 的单个配置项。
///
/// 要点：`split_once` 一次切分；失败路径全部用 `Result` 表达，不用 panic。
fn parse_key_value(line: &str) -> Result<(&str, &str), ConfigError> {
    let trimmed = line.trim(); // 去掉首尾空白，提升健壮性
    match trimmed.split_once('=') {
        Some((key, value)) => {
            let value = value.trim();
            if value.is_empty() {
                Err(ConfigError::EmptyValue {
                    key: key.trim().to_string(),
                })
            } else {
                Ok((key.trim(), value))
            }
        }
        None => Err(ConfigError::MissingSeparator {
            line: trimmed.to_string(),
        }),
    }
}

/// 解析多行配置文本为 `HashMap<String, f64>`。
///
/// 要点：这是典型的「全有或全无」解析——任何一行出错就整体返回 Err，
/// 出错信息里带上具体键名，方便使用者直接定位。空行会被跳过。
fn parse_config(text: &str) -> Result<HashMap<String, f64>, ConfigError> {
    let mut config: HashMap<String, f64> = HashMap::new();
    for raw_line in text.lines() {
        if raw_line.trim().is_empty() {
            continue; // 空行不是错误，直接忽略
        }
        let (key, value) = parse_key_value(raw_line)?; // `?`：出错立刻返回
        // 该闭包返回 Result；`map_err` 补上键名后 `?` 再上转成 ConfigError
        let number = value
            .parse::<f64>()
            .map_err(|source| ConfigError::InvalidNumber {
                key: key.to_string(),
                source,
            })?;
        config.insert(key.to_string(), number);
    }
    Ok(config)
}

/// panic 场景的辅助函数：从 `catch_unwind` 的载荷里取出可读消息。
///
/// 要点：`panic!` 只接受一个字符串载荷，它会被装箱成 `Box<dyn Any + Send>`；
/// 想读出来必须向下转型（downcast）。
fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string() // panic!("字面量") 的载荷是 &str
    } else if let Some(text) = payload.downcast_ref::<String>() {
        text.clone() // panic!("{}", 格式化) 的载荷是 String
    } else {
        String::from("<无法识别的 panic 载荷>")
    }
}

/// 在「静音 panic 输出」的保护下运行一段代码。
///
/// 要点：`catch_unwind` 只负责把 panic 变成 `Err`，默认仍会向 stderr 打印
/// `thread 'main' panicked at ...`。教学示例希望输出干净，所以临时替换 panic 钩子，
/// 运行结束后立刻恢复。返回值表示闭包是否 panic 了。
fn run_silently<F: FnOnce() -> T, T>(f: F) -> bool {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(|_info| {
        // 故意静音：panic 消息由 catch_unwind 的返回值展示
    }));
    let is_err = panic::catch_unwind(AssertUnwindSafe(f)).is_err();
    panic::set_hook(original_hook); // 恢复默认行为，避免影响后续代码
    is_err
}

/// 把自定义错误装箱成 `Box<dyn Error>`。
///
/// 要点：`Box<dyn Error>` 作为统一出口时，`?` 需要能从 `ConfigError` 转换过去；
/// 由于 `ConfigError: Error`，标准库已提供 `impl From<E> for Box<dyn Error>`，
/// 所以这里直接 `Box::new(...)` 即可，不需要手写 From。
fn boxed_error(err: ConfigError) -> Box<dyn Error> {
    Box::new(err)
}

fn main() -> AppResult<()> {
    println!("========== lesson_10_error_handling：错误处理 ==========\n");

    // 前两个示例不返回 Result：它们演示 panic 行为，把 panic 放在 catch_unwind 里观测
    demo_1_panic_terminates();
    demo_2_unwrap_and_expect();
    demo_3_result_and_match();

    // 下面这些示例返回 Result；用 `?` 逐个传播。任何一个失败，main 都会打印
    // `Error: ...` 并以退出码 1 结束 —— 这正是 main 返回 Result 的语义。
    demo_4_question_mark_operator()?;
    demo_5_custom_error_type()?;
    demo_6_from_conversion_via_question_mark()?;
    demo_7_box_dyn_error()?;
    demo_8_config_parsing_scenario()?;

    demo_9_common_mistakes();
    demo_10_when_to_panic_or_result();

    // 走到这里说明全部示例都成功；返回 Ok(()) 让进程退出码为 0
    Ok(())
}

/// 示例 1：panic! 与「线程内 panic 不会拖垮进程」
///
/// 要点：`panic!` 是**不可恢复**错误的表达方式，默认行为是打印消息并展开栈、
/// 终止当前线程。这里用 `catch_unwind` 捕获它，并把默认的 panic 打印临时关掉，
/// 以保证演示输出干净、进程退出码为 0。
fn demo_1_panic_terminates() {
    println!("--- 示例 1：panic! 与 catch_unwind ---");

    // 临时替换 panic 钩子：catch_unwind 捕获后默认仍会向 stderr 打印，
    // 这里把它静音，演示结束后再恢复原钩子（不影响程序语义）。
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(|_info| {
        // 故意什么都不打印：消息由 catch_unwind 的返回值展示
    }));

    // catch_unwind 需要闭包是「展开安全」的；示例中没有共享可变状态，可用 AssertUnwindSafe 包装
    let caught = panic::catch_unwind(AssertUnwindSafe(|| {
        panic!("演示用 panic：这一步不会被恢复，只会终止当前调用栈");
    }));
    match caught {
        // 闭包 panic 之后一定返回 Err，Ok 分支不会执行；期望值同样写成字面量
        Ok(_) => {
            println!("// 预期输出：没有捕获到 panic（不会发生），返回值 = ...（该分支不会执行）")
        }
        Err(payload) => {
            println!("捕获到 panic，载荷 = {}", panic_message(payload));
            println!(
                "// 预期输出：捕获到 panic，载荷 = 演示用 panic：这一步不会被恢复，只会终止当前调用栈"
            );
            // 补充说明（不是某个具体值，故不使用"预期输出"标记）
            println!("catch_unwind 让 panic 变成可观测的值，但原线程的后续代码不会继续执行");
        }
    }

    // panic! 的另一种形态：带格式化参数
    let reason = "配置缺失";
    let caught2 = panic::catch_unwind(AssertUnwindSafe(|| {
        panic!("无法继续：{reason}");
    }));
    match caught2 {
        Ok(()) => println!("// 预期输出：没有捕获到 panic（不会发生）"),
        Err(payload) => {
            println!("带格式化参数的 panic 载荷 = {}", panic_message(payload));
            println!("// 预期输出：带格式化参数的 panic 载荷 = 无法继续：配置缺失")
        }
    }

    // panic_any 可以携带任意类型的载荷（这里用字符串，便于跨类型演示）
    let caught3 = panic::catch_unwind(AssertUnwindSafe(|| {
        panic::panic_any(String::from("panic_any 携带的字符串载荷"));
    }));
    match caught3 {
        Ok(()) => println!("// 预期输出：没有捕获到 panic（不会发生）"),
        Err(payload) => {
            println!("panic_any 载荷 = {}", panic_message(payload));
            println!("// 预期输出：panic_any 载荷 = panic_any 携带的字符串载荷")
        }
    }

    // 在**另一个线程**里 panic：它只会终止那个线程，主线程继续运行，进程退出码仍为 0
    let child = std::thread::spawn(|| {
        panic!("子线程内部 panic：只会终止这个线程");
    });
    let joined = child.join();
    println!(
        "子线程 join 结果是 Err(...)（说明该线程 panic 了）= is_err {}",
        joined.is_err()
    );
    println!("// 预期输出：子线程 join 结果是 Err(...)（说明该线程 panic 了）= is_err true");
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("主线程继续执行到这里，说明线程崩溃不会传染给整个进程");

    panic::set_hook(original_hook); // 恢复默认 panic 行为

    // 下面是**不会**在演示中触发的真实退出写法，仅作说明：
    // panic!("致命错误");
    // 若在被 catch_unwind 捕获之外的位置调用，运行结果是：
    //   thread 'main' panicked at src/tutorial/lesson_10_error_handling.rs:NN:NN:
    //   致命错误
    //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // 并且进程退出码为 101（Windows 上表现为 0x65）。本文件刻意避免这种情况。
}

/// 示例 2：unwrap 与 expect
///
/// 要点：`unwrap()` 出错时用默认信息 panic；`expect("...")` 允许自定义信息，
/// 是「已经论证过不会失败」时更好的选择。下面只演示成功路径，
/// 失败路径全部注释说明，以保证程序不会真的 panic。
fn demo_2_unwrap_and_expect() {
    println!("--- 示例 2：unwrap 与 expect ---");

    // 成功路径：unwrap 直接取出 Ok 里的值
    let parsed: i32 = "42".parse::<i32>().unwrap(); // 教学示例：字面量必然可解析
    println!("\"42\".parse::<i32>().unwrap() = {parsed}");
    println!("// 预期输出：\"42\".parse::<i32>().unwrap() = 42");

    // expect：给出「为什么这里不会失败」的上下文，报错时更有用
    let number: f64 = "3.5".parse::<f64>().expect("字面量 3.5 一定能解析成 f64");
    println!("\"3.5\".parse::<f64>().expect(..) = {number}");
    println!("// 预期输出：\"3.5\".parse::<f64>().expect(..) = 3.5");

    // unwrap_or / unwrap_or_else / unwrap_or_default：给失败提供兜底值（不 panic）
    let fallback: i32 = "abc".parse::<i32>().unwrap_or(0);
    let fallback2: i32 = "abc".parse::<i32>().unwrap_or_else(|err| {
        // 闭包能拿到错误对象，可以顺便记录日志
        println!("unwrap_or_else 观察到错误 = {err}");
        println!("// 预期输出：unwrap_or_else 观察到错误 = invalid digit found in string");
        7
    });
    let fallback3: i32 = "abc".parse::<i32>().unwrap_or_default();
    println!(
        "unwrap_or(0) = {fallback}，unwrap_or_else(|_| 7) = {fallback2}，unwrap_or_default() = {fallback3}"
    );
    println!("// 预期输出：unwrap_or(0) = 0，unwrap_or_else(|_| 7) = 7，unwrap_or_default() = 0");

    // Option 上的 unwrap_or / unwrap_or_else（注意借用与所有权的区别）
    let maybe: Option<i32> = None;
    println!(
        "Option 的 unwrap_or(0) = {}，unwrap_or_else(|| 9) = {}",
        maybe.unwrap_or(0),
        maybe.unwrap_or_else(|| 9)
    );
    println!("// 预期输出：Option 的 unwrap_or(0) = 0，unwrap_or_else(|| 9) = 9");
    let borrowed: Option<i32> = Some(5);
    println!(
        "Some(5) 的 unwrap_or(&0) = {}（借用版本返回 &T，故默认值写成 &0）",
        borrowed.as_ref().unwrap_or(&0)
    );
    println!("// 预期输出：Some(5) 的 unwrap_or(&0) = 5（借用版本返回 &T，故默认值写成 &0）");

    // **会 panic 的写法（已注释）**：
    // let boom: i32 = "四十二".parse::<i32>().unwrap();
    // thread 'main' panicked at ...: called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    // 修正：改用 unwrap_or / match / `?`，或把失败路径显式处理
    // let boom2: i32 = "四十二".parse::<i32>().expect("分数必须是数字");
    // thread 'main' panicked at ...: 分数必须是数字: ParseIntError { kind: InvalidDigit }
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("以上两种 panic 写法已被注释，本程序不会因为 unwrap 而崩溃");

    // unwrap 也可以用在 Option 上
    let some_value: Option<&str> = Some("可用值");
    // println!("{}", None::<i32>.unwrap());
    // thread 'main' panicked at ...: called `Option::unwrap()` on a `None` value
    println!("Option 的 unwrap 成功路径 = {}", some_value.unwrap());
    println!("// 预期输出：Option 的 unwrap 成功路径 = 可用值");
}

/// 示例 3：Result<T, E> 与 match
///
/// 要点：`match` 是处理 `Result` 最完整的写法；`if let` / `is_ok` / `ok()` 等
/// 适合只关心一侧的场景。永远不要在库代码里用 `unwrap` 吞掉错误。
fn demo_3_result_and_match() {
    println!("--- 示例 3：Result 与 match ---");

    // match 显式分支：两个分支都必须处理，编译器强制你面对错误
    match safe_divide(10.0, 4.0) {
        Ok(value) => {
            println!("safe_divide(10, 4) = Ok({value})");
            println!("// 预期输出：safe_divide(10, 4) = Ok(2.5)")
        }
        // 10 / 4 是合法除法，Err 分支不会执行，期望值写成字面量
        Err(_) => println!("// 预期输出：safe_divide(10, 4) = Err(...)（该分支不会执行）"),
    }
    match safe_divide(10.0, 0.0) {
        // 除数为 0 一定走 Err，Ok 分支不会执行，期望值写成字面量
        Ok(_) => println!("// 预期输出：safe_divide(10, 0) = Ok(...)（该分支不会执行）"),
        Err(message) => {
            println!("safe_divide(10, 0) = Err({message})");
            println!("// 预期输出：safe_divide(10, 0) = Err(除数不能为 0)")
        }
    }

    // 只关心成功的一侧：if let
    if let Ok(value) = safe_divide(9.0, 3.0) {
        println!("if let Ok(value) 拿到 {value}");
        println!("// 预期输出：if let Ok(value) 拿到 3");
    }

    // 只关心失败的一侧：用 err() 转成 Option
    if let Some(message) = safe_divide(1.0, 0.0).err() {
        println!("err() 得到的错误消息 = {message}");
        println!("// 预期输出：err() 得到的错误消息 = 除数不能为 0");
    }

    // 链式风格的常用组合子
    let doubled = safe_divide(8.0, 2.0).map(|value| value * 2.0);
    let recovered = safe_divide(8.0, 0.0).or_else(|_| Ok::<f64, String>(0.0));
    let chained_error = safe_divide(8.0, 0.0).map_err(|message| format!("计算出错：{message}"));
    println!(
        "map 后 = {doubled:?}，or_else 恢复后 = {recovered:?}，map_err 后 = {chained_error:?}"
    );
    println!(
        "// 预期输出：map 后 = Ok(8.0)，or_else 恢复后 = Ok(0.0)，map_err 后 = Err(\"计算出错：除数不能为 0\")"
    );

    // is_ok / is_err 只做判断；ok() 丢弃错误只留值
    println!(
        "safe_divide(6, 3).is_ok() = {}，safe_divide(6, 0).is_err() = {}，ok() = {:?}",
        safe_divide(6.0, 3.0).is_ok(),
        safe_divide(6.0, 0.0).is_err(),
        safe_divide(6.0, 3.0).ok()
    );
    println!(
        "// 预期输出：safe_divide(6, 3).is_ok() = true，safe_divide(6, 0).is_err() = true，ok() = Some(2.0)"
    );

    // 把 Option 与 Result 互相转换：ok_or 给 None 补一个错误
    let from_option: Result<&str, String> = Some("值").ok_or_else(|| String::from("没有值"));
    let none_to_err: Result<&str, String> = None.ok_or_else(|| String::from("没有值"));
    println!("Some(...).ok_or_else -> {from_option:?}，None.ok_or_else -> {none_to_err:?}");
    println!("// 预期输出：Some(...).ok_or_else -> Ok(\"值\")，None.ok_or_else -> Err(\"没有值\")");

    // 注意 Result 也可以用 `let ... else` 提前返回（该语法自 Rust 1.65 起稳定，任何 edition 都能用）
    let Ok(value) = safe_divide(12.0, 4.0) else {
        // else 分支必须发散（这里直接 return，因为是 () 函数）
        println!("// 预期输出：let-else 进入 else 分支（本行不会执行）");
        return;
    };
    println!("let ... else 绑定成功，value = {value}");
    println!("// 预期输出：let ... else 绑定成功，value = 3");
}

/// 示例 4：`?` 运算符
///
/// 要点：`?` 在 `Ok` 时取出值继续执行，遇到 `Err` 立刻 `return Err(...)`。
/// 它只能用在返回 `Result` / `Option` 的函数里；用在 main 里则要求 main 也返回 Result。
fn demo_4_question_mark_operator() -> AppResult<()> {
    println!("--- 示例 4：? 运算符 ---");

    // 把 Result 里的值取出来：`?` 在出错时提前返回，所以后面的代码只在成功时执行
    let quotient = safe_divide(20.0, 5.0)?;
    println!("safe_divide(20, 5)? = {quotient}");
    println!("// 预期输出：safe_divide(20, 5)? = 4");

    // 把 String 错误上转成 Box<dyn Error>：String 实现了 Into<Box<dyn Error>>，
    // 因此本函数的返回类型 AppResult 可以直接用 `?` 接收安全除法的结果
    let converted: Box<dyn Error> = String::from("演示：String 也能装箱成 dyn Error").into();
    println!("String -> Box<dyn Error> 的结果 = {converted}");
    println!("// 预期输出：String -> Box<dyn Error> 的结果 = 演示：String 也能装箱成 dyn Error");

    // 链式：多个 `?` 顺序传播，任何一个失败都会立即返回
    let sum = safe_divide(100.0, 4.0)? + safe_divide(30.0, 3.0)?;
    println!("两个 `?` 结果相加 = {sum}");
    println!("// 预期输出：两个 `?` 结果相加 = 35");

    // Option 上的 `?`：返回类型是 AppResult 时不能直接用 `Option?`，
    // 但可以先 ok_or_else 转成 Result（这是最常见的转换套路）
    let values: HashMap<&str, f64> = HashMap::from([("price", 19.9)]);
    let price = values.get("price").copied().ok_or_else(|| {
        boxed_error(ConfigError::MissingKey {
            key: String::from("price"),
        })
    })?;
    println!("Option -> Result 后 `?` 取值 price = {price}");
    println!("// 预期输出：Option -> Result 后 `?` 取值 price = 19.9");

    // **错误写法（已注释）**：在返回 Result 的函数里对 Option 直接用 `?`
    // let missing = HashMap::<&str, f64>::new().get("nope")?;
    // error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s,
    //               in a function that returns `Result`
    // 修正 A：返回 Option（fn f() -> Option<T>）；修正 B：ok_or_else(...) 转成 Result
    let missing = HashMap::<&str, f64>::new()
        .get("nope")
        .copied()
        .ok_or_else(|| {
            boxed_error(ConfigError::MissingKey {
                key: String::from("nope"),
            })
        });
    println!(
        "修正后的写法（ok_or_else 转 Result）得到 {:?}",
        missing.map_err(|err| err.to_string())
    );
    println!(
        "// 预期输出：修正后的写法（ok_or_else 转 Result）得到 Err(\"配置中缺少必需的键 `nope`\")"
    );

    // **错误写法（已注释）**：在 main（无返回值版本）里用 `?`
    // fn main() { let ok: i32 = "1".parse::<i32>()?; }
    // error[E0277]: the `?` operator can only be used in a function that returns `Result`
    //               or `Option` (or another type that implements `FromResidual`)
    // 修正：把 main 写成 `fn main() -> Result<(), Box<dyn Error>>`（本文件采用此法）

    // **错误写法（已注释）**：在不返回 Result 的普通函数里用 `?`
    // fn helper() -> i32 { let value = "1".parse::<i32>()?; value }
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` ...

    Ok(()) // 显式返回成功，让调用方的 `?` 继续
}

/// 示例 5：自定义错误类型实现 Display 与 std::error::Error
///
/// 要点：自定义错误类型只要实现 `Debug + Display + Error`，就能参与
/// `?` 传播、装箱、错误链。`Error::source` 用来指向底层原因。
fn demo_5_custom_error_type() -> AppResult<()> {
    println!("--- 示例 5：自定义错误类型（Display + std::error::Error） ---");

    // 用自定义错误处理「温度解析」：输入形如 "36.6C" / "98.6F"
    fn parse_temperature(input: &str) -> Result<f64, ConfigError> {
        // 先用字节长度挡住空串与过短输入，避免后续切片越界（这里体现「显式前置检查」）
        if input.len() < 2 {
            return Err(ConfigError::MissingSeparator {
                line: input.to_string(),
            });
        }
        // split_at_checked 返回 Option：切分点落在字符边界之外时得到 None，而不是 panic
        let Some((number_text, unit)) = input.split_at_checked(input.len() - 1) else {
            return Err(ConfigError::MissingSeparator {
                line: input.to_string(),
            });
        };
        // 单位不在白名单内视为「缺少必需的键信息」
        match unit {
            "C" | "F" => {}
            _ => {
                return Err(ConfigError::MissingKey {
                    key: format!("单位 {unit}"),
                });
            }
        }
        // 这里用 map_err 补足上下文，让错误消息直接说出「哪个输入坏了」
        let value = number_text
            .parse::<f64>()
            .map_err(|source| ConfigError::InvalidNumber {
                key: input.to_string(),
                source,
            })?;
        Ok(if unit == "F" {
            (value - 32.0) * 5.0 / 9.0
        } else {
            value
        })
    }

    match parse_temperature("36.6C") {
        Ok(celsius) => {
            println!("36.6C 换算为摄氏 {celsius}");
            println!("// 预期输出：36.6C 换算为摄氏 36.6")
        }
        // "36.6C" 是合法输入，Err 分支不会执行，期望值写成字面量
        Err(_) => println!("// 预期输出：解析失败：...（该输入合法，本分支不会执行）"),
    }
    match parse_temperature("98.6F") {
        Ok(celsius) => {
            println!("98.6F 换算为摄氏 {:.4}", celsius);
            println!("// 预期输出：98.6F 换算为摄氏 37.0000")
        }
        // "98.6F" 是合法输入，Err 分支不会执行，期望值写成字面量
        Err(_) => println!("// 预期输出：解析失败：...（该输入合法，本分支不会执行）"),
    }
    match parse_temperature("abcC") {
        // "abcC" 的数字部分非法，Ok 分支不会执行，期望值写成字面量
        Ok(_) => println!("// 预期输出：意外成功：...（该输入非法，本分支不会执行）"),
        Err(err) => {
            // Display：面向使用者的消息
            println!("Display 消息 = {err}");
            println!(
                "// 预期输出：Display 消息 = 配置项 `abcC` 不是合法数字：invalid float literal"
            );
            // Debug：面向开发者的结构化表示（派生得到）
            println!("Debug 表示 = {err:?}");
            println!(
                "// 预期输出：Debug 表示 = InvalidNumber {{ key: \"abcC\", source: ParseFloatError {{ kind: Invalid }} }}"
            );
            // source：错误链的下一环
            match err.source() {
                Some(inner) => {
                    println!("底层 source = {inner}");
                    println!("// 预期输出：底层 source = invalid float literal")
                }
                None => println!("// 预期输出：该错误没有底层 source"),
            }
        }
    }

    // 把自定义错误装箱成 Box<dyn Error>，作为统一出口
    let boxed: Box<dyn Error> = Box::new(ConfigError::EmptyValue {
        key: String::from("port"),
    });
    println!(
        "装箱后 Display = {boxed}，source 是否存在 = {}",
        boxed.source().is_some()
    );
    println!("// 预期输出：装箱后 Display = 配置项 `port` 的值是空的，source 是否存在 = false");

    // 说明：`Box<dyn Error>` 等价于 `Box<dyn Error + 'static>`，而本文件的 AppResult 用的正是它；
    //       因此自定义错误与标准库错误都能通过 `?` 汇聚到同一个返回类型上。
    Ok(())
}

/// 示例 6：`?` 如何借助 `From` 完成错误上转
///
/// 要点：`expr?` 在出错时会做 `From::from(err)`，把底层错误自动转成函数返回类型
/// 的错误类型。这就是「底层用 ParseFloatError、上层用自定义错误」能无缝衔接的原因。
fn demo_6_from_conversion_via_question_mark() -> AppResult<()> {
    println!("--- 示例 6：From 自动转换（错误上转） ---");

    // 场景一：直接使用 From 的显式转换
    let parse_err = "not-a-number".parse::<f64>().unwrap_err();
    let converted = ConfigError::from(parse_err); // 走 impl From<ParseFloatError>
    println!("显式 From 转换 = {converted}");
    println!("// 预期输出：显式 From 转换 = 配置项 `<未知键>` 不是合法数字：invalid float literal");

    // 场景二：From 让 `?` 自动上转 —— 下面的函数返回 ConfigError，
    // 但 `?` 接收到的是 ParseFloatError，编译器自动插入 From::from
    fn double_config_value(value: &str) -> Result<f64, ConfigError> {
        let number: f64 = value.parse::<f64>()?; // 关键：? 触发 From 上转
        Ok(number * 2.0)
    }
    println!(
        "double_config_value(\"21\") = {:?}",
        double_config_value("21")
    );
    println!("// 预期输出：double_config_value(\"21\") = Ok(42.0)");
    println!(
        "double_config_value(\"x\") 的错误 = {}",
        double_config_value("x").unwrap_err()
    );
    println!(
        "// 预期输出：double_config_value(\"x\") 的错误 = 配置项 `<未知键>` 不是合法数字：invalid float literal"
    );

    // 场景三：`?` 的自动转换也能直接上转到 Box<dyn Error>（因为标准库有相应 From 实现）
    fn bump(text: &str) -> AppResult<u32> {
        // "12" 能解析成功；若传入 "x"，? 会用 From<ParseIntError> for Box<dyn Error> 自动装箱后返回
        let value: u32 = text.parse::<u32>()?;
        Ok(value + 1)
    }
    println!("bump(\"12\") = {:?}", bump("12"));
    println!("// 预期输出：bump(\"12\") = Ok(13)");
    println!("bump(\"x\") 的错误消息 = {}", bump("x").unwrap_err());
    println!("// 预期输出：bump(\"x\") 的错误消息 = invalid digit found in string");

    // 场景四：为自定义错误补一个「带键名」的 From 包装函数，避免占位键名
    fn with_key(key: &str, source: ParseFloatError) -> ConfigError {
        ConfigError::InvalidNumber {
            key: key.to_string(),
            source,
        }
    }
    let keyed = "port"
        .parse::<f64>()
        .map_err(|source| with_key("port", source));
    println!("带键名的错误 = {}", keyed.unwrap_err());
    println!("// 预期输出：带键名的错误 = 配置项 `port` 不是合法数字：invalid float literal");

    Ok(())
}

/// 示例 7：`Box<dyn Error>` 作为统一错误出口
///
/// 要点：当函数可能返回多种错误（IO、解析、自定义）时，`Box<dyn Error>`
/// 免去为每种组合定义枚举的成本；代价是失去按类型分支匹配的能力。
fn demo_7_box_dyn_error() -> AppResult<()> {
    println!("--- 示例 7：Box<dyn Error> 统一出口 ---");

    // 场景：读取一个有意不存在的文件，用 `?` 把 io::Error 装箱后返回
    let missing = Path::new("这是不存在的文件__lesson10.txt");
    let outcome = fs::read_to_string(missing);
    match outcome {
        // 这个文件是有意不存在的，只会走 Err 分支；Ok 分支不会执行，期望值写成字面量
        Ok(_) => println!("// 预期输出：文件内容长度 = ...（文件不存在，该分支不会执行）"),
        Err(err) => {
            // 注意：错误文案由操作系统决定（这里是中文 Windows），因此不给"预期输出"断言，
            // 只断言与语言无关的 kind 字段（见下方 NotFound）。
            println!("读取失败（预期内的失败）= {err}");
            println!("错误类型名 = {}", std::any::type_name_of_val(&err));
            println!("// 预期输出：错误类型名 = core::io::error::Error");
            println!("io::Error 的 kind = {:?}", err.kind());
            println!("// 预期输出：io::Error 的 kind = NotFound");
        }
    }

    // 用 `?` 传播到 AppResult：io::Error 会被自动装箱
    fn read_or_fail(path: &str) -> AppResult<usize> {
        let text = fs::read_to_string(path)?; // ? 自动 From<io::Error> for Box<dyn Error>
        Ok(text.len())
    }
    // 传入存在的文件（本文件自身）验证成功路径
    // 注意：这里用的是相对路径，所以是否成功取决于运行时的「当前工作目录」；
    // 请在项目根目录运行本示例（cargo run 会自动以项目根为工作目录）。
    match read_or_fail("src/tutorial/lesson_10_error_handling.rs") {
        Ok(len) => {
            // 因环境而异：字节数取决于本文件当时的字节数，因此只打印真实值、不写死数字断言。
            println!("成功读到本文件，字节数 = {len}");
        }
        // 在项目根目录运行时该文件存在，Err 分支不会执行；工作目录不同则会走到这里
        Err(_) => println!("// 预期输出：相对路径读取失败 = ...（因环境而异：与当前工作目录有关）"),
    }
    // 同样只打印真实值：错误文案随操作系统语言而变，故不做字面量断言。
    println!(
        "不存在的路径会返回 Err，这里只打印消息以免中断演示 = {}",
        read_or_fail("不存在的路径.txt").unwrap_err()
    );

    // 不同来源的错误放进同一个 Box<dyn Error> 出口
    fn mixed(source: u8) -> AppResult<String> {
        match source {
            0 => {
                // 解析错误 -> Box<dyn Error>
                let number: i32 = "oops".parse::<i32>()?;
                Ok(number.to_string())
            }
            1 => {
                // 自定义错误 -> Box<dyn Error>
                Err(Box::new(ConfigError::MissingKey {
                    key: String::from("host"),
                }))
            }
            _ => {
                // 字符串错误 -> Box<dyn Error>（String 有对应 From 实现）
                Err(String::from("未知来源").into())
            }
        }
    }
    // 期望值逐条写成字面量（三种来源都会失败），与上一行的真实输出逐字节对照
    let expected_mixed = [
        "// 预期输出：mixed(0) = Err(invalid digit found in string)",
        "// 预期输出：mixed(1) = Err(配置中缺少必需的键 `host`)",
        "// 预期输出：mixed(2) = Err(未知来源)",
    ];
    for (source, expected) in [0u8, 1, 2].into_iter().zip(expected_mixed) {
        let actual = match mixed(source) {
            Ok(value) => format!("Ok({value})"),
            Err(err) => format!("Err({err})"),
        };
        println!("mixed({source}) = {actual}");
        println!("{expected}");
    }

    // 需要按类型分支处理时，可以向下转型（downcast）
    let boxed: Box<dyn Error> = Box::new(ConfigError::EmptyValue {
        key: String::from("token"),
    });
    if let Some(config_err) = boxed.downcast_ref::<ConfigError>() {
        println!("downcast 回自定义类型后可以匹配枚举 = {config_err:?}");
        println!(
            "// 预期输出：downcast 回自定义类型后可以匹配枚举 = EmptyValue {{ key: \"token\" }}"
        );
    }
    println!("Box<dyn Error> 的 Display = {boxed}");
    println!("// 预期输出：Box<dyn Error> 的 Display = 配置项 `token` 的值是空的");
    Ok(())
}

/// 示例 8（典型使用场景）：解析 `key=数字值` 配置文本，用自定义错误返回 Result
///
/// 要点：真实项目读取配置时，正确姿势是「返回 Result，让调用方决定怎么办」：
/// 出错信息里带上键名与原因，绝不用 panic 让整个程序无声退出。
/// 注意：本示例的配置值统一是数字（例如端口、超时、重试次数）；
/// 像主机名这种字符串值的配置项，值类型应改成 `String`，否则会被当成非法数字。
fn demo_8_config_parsing_scenario() -> AppResult<()> {
    println!("--- 示例 8：典型使用场景 —— 解析 key=value 配置 ---");

    let valid = "port = 8080\n\
                 timeout = 2.5\n\
                 \n\
                 retries = 3";
    match parse_config(valid) {
        Ok(config) => {
            // 说明：HashMap 的遍历顺序不固定，需要确定性输出时必须先对键排序
            let mut keys: Vec<&String> = config.keys().collect();
            keys.sort();
            let pairs: Vec<String> = keys
                .iter()
                .map(|key| format!("{key}={}", config[*key]))
                .collect();
            println!(
                "解析成功，共 {} 项（空行被忽略；注意：HashMap 顺序不固定，已按键排序）",
                config.len()
            );
            println!(
                "// 预期输出：解析成功，共 3 项（空行被忽略；注意：HashMap 顺序不固定，已按键排序）"
            );
            println!("排序后的配置 = {}", pairs.join("，"));
            println!("// 预期输出：排序后的配置 = port=8080，retries=3，timeout=2.5");
            println!(
                "直接按 key 查询 port = {:?}，timeout = {:?}",
                config.get("port"),
                config.get("timeout")
            );
            println!("// 预期输出：直接按 key 查询 port = Some(8080.0)，timeout = Some(2.5)");
            println!(
                "带默认值的查询 retries = {}，缺失键的默认值 = {}",
                config.get("retries").copied().unwrap_or(0.0),
                config.get("不存在").copied().unwrap_or(-1.0)
            );
            println!("// 预期输出：带默认值的查询 retries = 3，缺失键的默认值 = -1");
        }
        // valid 这段配置是合法的，Err 分支不会执行，期望值写成字面量
        Err(_) => println!("// 预期输出：解析失败：...（valid 配置合法，该分支不会执行）"),
    }

    // 每个错误分支都实际跑一遍，验证错误消息是否可定位问题
    let broken_cases = [
        "port = 8080\nhost 127.0.0.1", // 缺少 `=` 分隔符
        "port = 8080\nhost =",         // 值为空白
        "port = 80x0",                 // 值不是数字
    ];
    // 期望值逐条写成字面量（三种坏输入都会失败），与上一行的真实输出逐字节对照
    let expected_broken = [
        "// 预期输出：错误分支（输入 \"port = 8080\\nhost 127.0.0.1\"）-> 配置行缺少 `=` 分隔符：\"host 127.0.0.1\"",
        "// 预期输出：错误分支（输入 \"port = 8080\\nhost =\"）-> 配置项 `host` 的值是空的",
        "// 预期输出：错误分支（输入 \"port = 80x0\"）-> 配置项 `port` 不是合法数字：invalid float literal",
    ];
    for (case, expected) in broken_cases.into_iter().zip(expected_broken) {
        let actual = match parse_config(case) {
            Ok(config) => format!("意外解析成功，共 {} 项", config.len()),
            Err(err) => format!("错误分支（输入 {case:?}）-> {err}"),
        };
        println!("{actual}");
        println!("{expected}");
    }

    // 把自定义错误经 `?` 上转成 Box<dyn Error>，形成统一的错误出口
    fn load_port(text: &str) -> AppResult<f64> {
        let config = parse_config(text)?; // ConfigError -> Box<dyn Error> 由 `?` 自动完成
        let port = config.get("port").copied().ok_or_else(|| {
            boxed_error(ConfigError::MissingKey {
                key: String::from("port"),
            })
        })?; // Option -> Result -> Box<dyn Error>
        Ok(port)
    }
    println!(
        "load_port(\"port = 8080\") = {:?}",
        load_port("port = 8080")
    );
    println!("// 预期输出：load_port(\"port = 8080\") = Ok(8080.0)");
    println!(
        "load_port(缺少 port) 的错误消息 = {}",
        load_port("timeout = 2.5").unwrap_err()
    );
    println!("// 预期输出：load_port(缺少 port) 的错误消息 = 配置中缺少必需的键 `port`");
    Ok(())
}

/// 示例 9：常见错误示例（错误代码全部注释掉）
///
/// 要点：错误处理相关的编译错误大多集中在「`?` 用错位置」与「错误类型没实现需要的 trait」，
/// 记住 E0277（trait 约束不满足）与 E0308（类型不匹配）即可快速定位。
fn demo_9_common_mistakes() {
    println!("--- 示例 9：常见错误示例（代码已注释，仅作说明） ---");

    // 错误 1：在无返回值的函数里用 `?`
    // fn helper() { let value: i32 = "1".parse::<i32>()?; println!("{value}"); }
    // error[E0277]: the `?` operator can only be used in a function that returns `Result`
    //               or `Option` (or another type that implements `FromResidual`)
    // 修正：把返回类型改成 Result<(), Box<dyn Error>> 或显式 match
    println!("修正方式示例 —— 用 match 处理 = {:?}", "1".parse::<i32>());
    println!("// 预期输出：修正方式示例 —— 用 match 处理 = Ok(1)");

    // 错误 2：在返回 Result 的函数里对 Option 用 `?`
    // fn helper() -> Result<(), Box<dyn Error>> { let v = Some(1)?; Ok(()) }
    // error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s,
    //               in a function that returns `Result`
    // 修正：ok_or_else(...) 转成 Result；或让函数返回 Option
    let fixed_option: Result<i32, String> = Some(1).ok_or_else(|| String::from("没有值"));
    println!("修正后 = {fixed_option:?}");
    println!("// 预期输出：修正后 = Ok(1)");

    // 错误 3：`?` 的源错误类型无法转换成函数返回的错误类型
    // #[derive(Debug)] struct MyErr;
    // fn helper() -> Result<(), MyErr> { let n: i32 = "1".parse::<i32>()?; Ok(()) }
    // error[E0277]: `?` couldn't convert the error to `MyErr`
    //   note: the trait `From<ParseIntError>` is not implemented for `MyErr`
    //   note: this can't be annotated with `?` because it has type `Result<_, ParseIntError>`
    //   note: `MyErr` needs to implement `From<ParseIntError>`
    // 修正：为 MyErr 实现 `impl From<ParseIntError> for MyErr`
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("本文件的 ConfigError 已实现 From<ParseFloatError>，因此 `?` 才能自动上转");

    // 错误 4：自定义错误类型没有实现 Display / Error 就装箱
    // #[derive(Debug)] struct MyErr;
    // let boxed: Box<dyn std::error::Error> = Box::new(MyErr);
    // error[E0277]: the trait bound `MyErr: std::error::Error` is not satisfied
    //   note: required for the cast from `Box<MyErr>` to `Box<dyn std::error::Error>`
    // 紧接着（若继续对它调用 to_string()）还会报：
    // error[E0599]: `MyErr` doesn't implement `std::fmt::Display`
    //   note: the trait `std::fmt::Display` must be implemented
    // 修正：同时实现 Display 与 Error（本文件的 ConfigError 就是标准写法）
    println!(
        "实现 Display 后可以打印错误消息 = {}",
        ConfigError::MissingKey {
            key: String::from("host")
        }
    );
    println!("// 预期输出：实现 Display 后可以打印错误消息 = 配置中缺少必需的键 `host`");

    // 错误 5：未处理的 Result 被丢弃
    // "42".parse::<i32>();
    // warning: unused `Result` that must be used
    //   = note: this `Result` may be an `Err` variant, which should be handled
    //   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
    // 修正：用 `let _ = ...;` 显式忽略，或真正处理它
    let _ = "42".parse::<i32>(); // 显式忽略：这里只是演示，故用 let _ 标记
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("显式忽略未使用的 Result，编译不再产生 must_use 警告");

    // 错误 6：数字溢出 —— 要分清编译期常量和运行期数值两种情况
    // 情况 A：两边都是编译期可知的常量时，编译器直接报错（根本跑不到运行期）：
    // let big: u8 = 255;
    // let boom = big + 1;
    // error: this arithmetic operation will overflow
    //   = note: `#[deny(arithmetic_overflow)]` on by default
    //
    // 情况 B：值来自运行期（函数参数、解析结果等），编译期无法判断，
    //         此时 debug 构建会 panic：
    // fn add(a: u8, b: u8) -> u8 { a + b }
    // add(255, 1)  // thread 'main' panicked: attempt to add with overflow（debug）
    //              // release 构建则按二进制补码回绕，得到 0
    // 这里用 running_sum 演示情况 B 的实际行为（回绕由 wrapping_* 显式表达，便于移植）。
    // 修正：用 checked_add / wrapping_add / saturating_add 显式表达意图
    let overflow = 255u8.checked_add(1);
    let wrapped = 255u8.wrapping_add(1);
    let saturated = 255u8.saturating_add(1);
    println!("checked_add = {overflow:?}，wrapping_add = {wrapped}，saturating_add = {saturated}");
    println!("// 预期输出：checked_add = None，wrapping_add = 0，saturating_add = 255");

    // 错误 7：除零
    // let x = 1 / 0;
    // error: this operation will panic at runtime
    //   = note: `#[deny(unconditional_panic)]` on by default（编译期就能发现常量除零）
    // 浮点除零**不会** panic，而是得到 inf / NaN，问题更隐蔽：
    let float_div = 1.0f64 / 0.0;
    println!(
        "浮点除零 = {float_div}，0.0/0.0 = {}（需用 is_finite/is_nan 主动检查）",
        0.0f64 / 0.0
    );
    println!("// 预期输出：浮点除零 = inf，0.0/0.0 = NaN（需用 is_finite/is_nan 主动检查）");
    println!(
        "safe_divide(1, 0) 用 Result 显式拒绝非法输入 = {:?}",
        safe_divide(1.0, 0.0)
    );
    println!("// 预期输出：safe_divide(1, 0) 用 Result 显式拒绝非法输入 = Err(\"除数不能为 0\")");

    // 错误 8：想用 `?` 但函数返回 `()`，忘记写 Ok(()) 与错误类型的 From 转换
    // fn helper() -> Result<(), String> { "1".parse::<i32>()?; }
    // error[E0277]: `?` couldn't convert the error to `String`
    //   note: the trait `From<ParseIntError>` is not implemented for `String`
    //   note: this can't be annotated with `?` because it has type `Result<_, ParseIntError>`
    // error[E0308]: mismatched types
    //   expected `Result<(), String>`, found `()`（函数体结尾少了 Ok(())）
    // 修正两件事：末尾写 Ok(())；为错误类型实现 From，或让函数返回兼容的错误类型。
    // 本文件所有返回 Result 的 demo 都以 Ok(()) 结尾。

    // 错误 9：对「引用」匹配时又写 ref —— edition 2024 禁止这种双重借用
    // let opt = Some(String::from("x"));
    // match &opt {                       // 已经是 &Option<String>
    //     Some(ref s) => println!("{s}"), // 再写 ref 就多余且被禁止
    //     None => {}
    // }
    // error: cannot explicitly borrow within an implicitly-borrowing pattern（无 E 编号）
    // 修正：直接写 `Some(s)`（默认绑定模式已能正确处理），即下面这样。
    let opt = Some(String::from("x"));
    match &opt {
        Some(text) => println!("改用引用匹配后 text = {text}"),
        None => println!("None 分支"),
    }
    println!("// 预期输出：改用引用匹配后 text = x");

    // 错误 10：edition 2024 里把 `gen` 当标识符
    // let gen = 1;
    // error: expected identifier, found reserved keyword `gen`
    // 修正：改名（例如 generate / index），因为 2024 edition 起 `gen` 是保留字
    let generate = 1;
    println!("把 gen 改名后可以正常使用，generate = {generate}");
    println!("// 预期输出：把 gen 改名后可以正常使用，generate = 1");
}

/// 示例 10：什么时候 panic，什么时候返回 Result
///
/// 要点：判断标准是「这个失败是调用方的 bug，还是可预期的运行环境结果」：
///   - 调用方违反前置条件（下标越界、状态机非法转移）→ 适合 panic / assert!（bug，应尽早暴露）
///   - 外部世界的不可控结果（IO、网络、用户输入、解析）→ 适合返回 Result（必须被处理）
fn demo_10_when_to_panic_or_result() {
    println!("--- 示例 10：何时 panic，何时返回 Result ---");

    // 适合 panic 的场合：程序内部不变量被破坏，继续跑下去只会产生更糟的错误数据
    fn require_positive(value: i32) -> i32 {
        // 这是「调用方违反了契约」，属于 bug，assert! 会带着清晰信息 panic
        assert!(
            value > 0,
            "内部不变量被破坏：value 必须为正，实际为 {value}"
        );
        value
    }
    println!(
        "满足契约的调用 require_positive(3) = {}",
        require_positive(3)
    );
    println!("// 预期输出：满足契约的调用 require_positive(3) = 3");
    // require_positive(-1);
    // thread 'main' panicked at ...: 内部不变量被破坏：value 必须为正，实际为 -1
    // 这就是「该 panic 就 panic」的例子：它暴露的是代码 bug，而不是用户输入问题
    // 为了让演示输出干净（并保证退出码为 0），这里用 run_silently 包住这次 panic
    let unchecked = run_silently(|| require_positive(-1));
    println!("require_positive(-1) 会 panic，被捕获判定 = {unchecked}（本行用 catch_unwind 兜住）");
    println!(
        "// 预期输出：require_positive(-1) 会 panic，被捕获判定 = true（本行用 catch_unwind 兜住）"
    );

    // 适合返回 Result 的场合：外部输入 / IO，任何值都可能出现，必须让调用方决定
    fn parse_age(input: &str) -> Result<u8, ConfigError> {
        // parse 得到 ParseIntError，经 From 上转成 ConfigError；再用 ? 返回
        let age: u8 = input.parse::<u8>()?;
        Ok(age)
    }
    // 期望值逐条写成字面量（"18" 成功，"-1" 与 "abc" 都会失败），与上一行的真实输出对照
    let expected_ages = [
        "// 预期输出：parse_age(\"18\") = Ok(18)",
        "// 预期输出：parse_age(\"-1\") = Err(配置项 `<未知键>` 不是合法整数：invalid digit found in string)",
        "// 预期输出：parse_age(\"abc\") = Err(配置项 `<未知键>` 不是合法整数：invalid digit found in string)",
    ];
    for (input, expected) in ["18", "-1", "abc"].into_iter().zip(expected_ages) {
        let actual = match parse_age(input) {
            Ok(age) => format!("Ok({age})"),
            Err(err) => format!("Err({err})"),
        };
        println!("parse_age({input:?}) = {actual}");
        println!("{expected}");
    }
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("注意 -1 也是「错误」而不是 panic —— 用户输入问题必须走 Result");

    // 折中做法：内部用 expect 断言「已经论证过不会失败」，但把契约写进注释
    let definitely_parsed: i32 = "2024"
        .parse::<i32>()
        .expect("这里是源码里的字面量，不可能解析失败");
    println!("expect 的合法用法（字面量常量）= {definitely_parsed}");
    println!("// 预期输出：expect 的合法用法（字面量常量）= 2024");

    // main 返回 Result 的语义总结（本文件就是这个写法）
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!(
        "main 返回 Result<(), Box<dyn Error>> 时，成功退出码 0；失败则打印 `Error: ...` 并以 1 退出"
    );
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("因此 main 里只用 `?` 传播，不混用 unwrap，失败原因才能被清晰打印");
}
