//! lesson_02_data_types.rs —— 主题：数据类型
//!
//! 学习目标：
//!   1. 掌握整数/浮点/布尔/字符四类标量类型，理解整数溢出与字面量后缀
//!   2. 会使用元组与数组这两种复合类型，并掌握解构、索引与安全访问
//!   3. 分清 `as`、`From`、`TryFrom` 三种类型转换方式的适用场景
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_02_data_types.rs -o lesson_02 && ./lesson_02
//!   或在本项目根目录执行：cargo run --bin lesson_02_data_types
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

use std::convert::TryFrom;

fn main() {
    // main 只负责按学习顺序调用各个 demo，本身不写业务逻辑。
    println!("========== lesson_02_data_types：数据类型 ==========\n");

    demo_1_integer_types();
    demo_2_float_types();
    demo_3_bool_and_char_literals();
    demo_4_tuples();
    demo_5_arrays();
    demo_6_type_conversion();
    demo_7_typical_use_case();
    demo_8_common_mistakes();
}

/// 示例 1：整数类型与字面量写法
///
/// 要点：整数类型有明确位宽，字面量可以带后缀、下划线和进制前缀，默认类型是 i32。
fn demo_1_integer_types() {
    println!("--- 示例 1：整数类型与字面量 ---");

    // 类型推断的默认值：没有后缀、也没有上下文时，整数字面量就是 i32。
    let default_int = 10;
    println!("默认整数 default_int = {default_int}");
    println!("// 预期输出：默认整数 default_int = 10");

    // 需要明确位宽时，用后缀最简洁：i8/i16/i32/i64/i128/isize 与对应的 u* 无符号类型。
    let small: i8 = -128; // i8 的取值范围是 -128..=127
    let unsigned: u8 = 255; // u8 的取值范围是 0..=255
    let big: u64 = 18_446_744_073_709_551_615; // 下划线只是可读性分隔符
    println!("i8 最小值 = {small}，u8 最大值 = {unsigned}");
    println!("// 预期输出：i8 最小值 = -128，u8 最大值 = 255");
    println!("u64 最大值 = {big}");
    println!("// 预期输出：u64 最大值 = 18446744073709551615");

    // 不同进制字面量与字节字面量：它们最终都是整数。
    let decimal = 1_000; // 十进制
    let hex = 0xFF; // 十六进制
    let octal = 0o755; // 八进制
    let binary = 0b1010_1010; // 二进制
    let byte = b'A'; // 字节字面量，类型是 u8
    println!("十进制 {decimal}，十六进制 {hex}，八进制 {octal}，二进制 {binary}，字节 {byte}");
    println!("// 预期输出：十进制 1000，十六进制 255，八进制 493，二进制 170，字节 65");

    // usize / isize 的宽度跟随平台指针宽度，专门用于索引和长度。
    let count: usize = 7;
    println!("usize 长度值 count = {count}");
    println!("// 预期输出：usize 长度值 count = 7");

    println!();
}

/// 示例 2：浮点类型
///
/// 要点：浮点分 f32/f64，默认是 f64；浮点运算存在精度误差，比较时要小心。
fn demo_2_float_types() {
    println!("--- 示例 2：浮点类型 ---");

    // 默认推断为 f64（双精度），f32 需要显式标注或加后缀。
    let pi = 3.14159;
    let ratio: f32 = 1.5;
    println!("f64 值 pi = {pi}，f32 值 ratio = {ratio}");
    println!("// 预期输出：f64 值 pi = 3.14159，f32 值 ratio = 1.5");

    // 整数形式的字面量加后缀就能变成浮点。
    let rate = 2.0f64 * 3.0;
    println!("rate = {rate}");
    println!("// 预期输出：rate = 6");

    // 整数除法与浮点除法的差别：这一行是整数除法，会截断小数。
    let int_division = 7 / 2;
    println!("整数除法 7 / 2 = {int_division}");
    println!("// 预期输出：整数除法 7 / 2 = 3");

    // 只要有一个操作数是浮点，结果就是浮点。
    let float_division = 7.0 / 2.0;
    println!("浮点除法 7.0 / 2.0 = {float_division}");
    println!("// 预期输出：浮点除法 7.0 / 2.0 = 3.5");

    // 精度陷阱：0.1 + 0.2 在二进制浮点里不等于 0.3。
    // 显式标注 f64：否则编译器无法确定浮点字面量到底该是 f32 还是 f64，
    // 后续调用 `abs()` 就会报 error[E0689]: can't call method `abs` on ambiguous numeric type。
    let sum: f64 = 0.1 + 0.2;
    println!("0.1 + 0.2 = {sum}");
    println!("// 预期输出：0.1 + 0.2 = 0.30000000000000004");
    // 正确做法是判断"差值足够小"，而不是直接比较相等。
    println!("(0.1 + 0.2) == 0.3 的结果 = {}", sum == 0.3);
    println!("// 预期输出：(0.1 + 0.2) == 0.3 的结果 = false");
    println!("差值是否小于 1e-9：{}", (sum - 0.3).abs() < 1e-9);
    println!("// 预期输出：差值是否小于 1e-9：true");

    // 取整与绝对值等常用方法。
    println!(
        "3.7 向下取整 = {}, -3.7 的绝对值 = {}",
        3.7_f64.floor(),
        (-3.7_f64).abs()
    );
    println!("// 预期输出：3.7 向下取整 = 3, -3.7 的绝对值 = 3.7");

    println!();
}

/// 示例 3：布尔、字符与字面量后缀
///
/// 要点：bool 只有 true/false；char 是 4 字节的 Unicode 标量值，用单引号书写。
fn demo_3_bool_and_char_literals() {
    println!("--- 示例 3：布尔、字符与字面量后缀 ---");

    // 布尔值来自比较或逻辑运算，不需要从 0/1 转换。
    let enabled = true;
    let finished = false;
    println!("enabled = {enabled}，finished = {finished}");
    println!("// 预期输出：enabled = true，finished = false");
    println!("逻辑运算：enabled && !finished = {}", enabled && !finished);
    println!("// 预期输出：逻辑运算：enabled && !finished = true");

    // 字符用单引号，字符串用双引号，两者不能混用。
    let letter = 'R';
    let chinese = '中';
    let emoji = '🚀';
    println!("字母 {letter}，汉字 {chinese}，表情 {emoji}");
    println!("// 预期输出：字母 R，汉字 中，表情 🚀");

    // char 占 4 字节（Unicode 标量值），而字符串里的字符可能占多个字节。
    println!("char 占 {} 字节", std::mem::size_of::<char>());
    println!("// 预期输出：char 占 4 字节");

    // 转义字符也是合法的 char 字面量。
    let newline = '\n';
    let quote = '\'';
    println!("换行字符转义成功 = {}", newline == '\n');
    println!("// 预期输出：换行字符转义成功 = true");
    println!("单引号字符 = {quote}");
    println!("// 预期输出：单引号字符 = '");

    // 各类型的大小：bool 占 1 字节，i32 占 4 字节，f64 占 8 字节。
    println!(
        "大小：bool {} 字节，i32 {} 字节，f64 {} 字节",
        std::mem::size_of::<bool>(),
        std::mem::size_of::<i32>(),
        std::mem::size_of::<f64>()
    );
    println!("// 预期输出：大小：bool 1 字节，i32 4 字节，f64 8 字节");

    println!();
}

/// 示例 4：元组（tuple）
///
/// 要点：元组把不同类型的值打包成一个整体，长度固定，适合"临时返回多个值"。
fn demo_4_tuples() {
    println!("--- 示例 4：元组 ---");

    // 元组可以混合类型，并用 .0/.1 这样的下标访问。
    let point = (3, 4.5, "坐标");
    println!(
        "point.0 = {}，point.1 = {}，point.2 = {}",
        point.0, point.1, point.2
    );
    println!("// 预期输出：point.0 = 3，point.1 = 4.5，point.2 = 坐标");

    // 解构（destructuring）：一次把元组拆成多个变量，是最常用的写法。
    let (x, y, label) = point;
    println!("解构结果：{label} = ({x}, {y})");
    println!("// 预期输出：解构结果：坐标 = (3, 4.5)");

    // 显式标注元组类型：类型写在括号里，用逗号分隔。
    let http_status: (u16, &str) = (200, "OK");
    println!("HTTP 状态：{} {}", http_status.0, http_status.1);
    println!("// 预期输出：HTTP 状态：200 OK");

    // 单元类型 `()` 就是"空元组"，函数没有返回值时返回的正是它。
    let unit: () = ();
    println!("空元组的长度 = {}", std::mem::size_of_val(&unit));
    println!("// 预期输出：空元组的长度 = 0");

    // 元组大小受内存对齐影响：i32(4) + f64(8) + &str(16) 还要补齐到 8 字节对齐，共 32 字节。
    println!("point 占 {} 字节", std::mem::size_of_val(&point));
    println!("// 预期输出：point 占 32 字节");

    println!();
}

/// 示例 5：数组（array）
///
/// 要点：数组长度固定、元素类型相同、存放在栈上；越界索引会 panic。
fn demo_5_arrays() {
    println!("--- 示例 5：数组 ---");

    // 数组类型写作 [元素类型; 长度]。
    let scores: [i32; 5] = [90, 85, 78, 92, 88];
    println!("数组长度 = {}，第一个元素 = {}", scores.len(), scores[0]);
    println!("// 预期输出：数组长度 = 5，第一个元素 = 90");
    println!("完整数组 = {scores:?}");
    println!("// 预期输出：完整数组 = [90, 85, 78, 92, 88]");

    // [值; 数量] 语法用于快速创建重复元素的数组。
    let zeros = [0u8; 4];
    println!("重复元素数组 = {zeros:?}");
    println!("// 预期输出：重复元素数组 = [0, 0, 0, 0]");

    // 数组可以整体迭代；`iter()` 产生元素引用。
    let mut total = 0;
    for score in scores.iter() {
        total += score;
    }
    println!("总分 = {total}，平均分 = {}", total / scores.len() as i32);
    println!("// 预期输出：总分 = 433，平均分 = 86");

    // 修改数组元素需要数组本身可变。
    let mut grades = [1, 2, 3];
    grades[1] = 20;
    println!("修改后的数组 = {grades:?}");
    println!("// 预期输出：修改后的数组 = [1, 20, 3]");

    // 安全访问：get 返回 Option，越界不会 panic，而是返回 None。
    println!(
        "get(0) = {:?}，get(99) = {:?}",
        scores.get(0),
        scores.get(99)
    );
    println!("// 预期输出：get(0) = Some(90)，get(99) = None");

    // 数组在栈上占用的字节数 = 元素大小 × 长度。
    println!("scores 占 {} 字节", std::mem::size_of_val(&scores));
    println!("// 预期输出：scores 占 20 字节");

    println!();
}

/// 示例 6：类型转换（as / From / TryFrom）
///
/// 要点：`as` 是无检查的强制转换（可能截断），`From` 表示一定成功，
/// `TryFrom` 表示可能失败、必须处理失败情况。
fn demo_6_type_conversion() {
    println!("--- 示例 6：类型转换 as / From / TryFrom ---");

    // ===== 1) as：数值之间的强制转换，不做溢出检查 =====
    let big_number: i32 = 300;
    // 300 超出 u8 范围（0..=255），`as` 会按二进制截断，留下低 8 位 = 44。
    let truncated = big_number as u8;
    println!("300i32 as u8 = {truncated}（二进制截断，不是 300）");
    println!("// 预期输出：300i32 as u8 = 44（二进制截断，不是 300）");

    // 负数转无符号同样会发生环绕。
    let negative: i8 = -1;
    println!("-1i8 as u8 = {}", negative as u8);
    println!("// 预期输出：-1i8 as u8 = 255");

    // 浮点转整数是"向零截断"，会直接丢弃小数部分。
    println!("3.99f64 as i32 = {}", 3.99_f64 as i32);
    println!("// 预期输出：3.99f64 as i32 = 3");

    // 整数转浮点不会失败也不会 panic，但超过浮点精度时会被舍入。
    let as_float = 42u8 as f64;
    println!("42u8 as f64 = {as_float}");
    println!("// 预期输出：42u8 as f64 = 42");

    // ===== 2) From / Into：保证成功的转换 =====
    // u16 -> u32 一定不会溢出，所以标准库为它实现了 From。
    let widened: u32 = u32::from(1000u16);
    println!("u32::from(1000u16) = {widened}");
    println!("// 预期输出：u32::from(1000u16) = 1000");

    // 有了 From 就自动获得 Into，两种写法等价。
    let widened_again: u32 = 1000u16.into();
    println!("1000u16.into() = {widened_again}（与 From 等价）");
    println!("// 预期输出：1000u16.into() = 1000（与 From 等价）");

    // 整数 -> 浮点也有 From。
    let from_int: f64 = f64::from(7i32);
    println!("f64::from(7i32) = {from_int}");
    println!("// 预期输出：f64::from(7i32) = 7");

    // ===== 3) TryFrom：可能失败的转换，必须处理 Result =====
    let ok_value: Result<u8, _> = u8::try_from(200i32);
    let bad_value: Result<u8, _> = u8::try_from(300i32);
    println!("u8::try_from(200) = {ok_value:?}");
    println!("// 预期输出：u8::try_from(200) = Ok(200)");
    println!("u8::try_from(300) = {bad_value:?}");
    println!("// 预期输出：u8::try_from(300) = Err(TryFromIntError(PosOverflow))");

    // 用 match 明确区分成功与失败（Result 的完整用法见 lesson 07/10）。
    match u8::try_from(300i32) {
        Ok(value) => println!("转换成功：{value}"),
        Err(_) => {
            println!("转换失败：300 超出 u8 的取值范围，需要用更大的类型或先校验");
            println!("// 预期输出：转换失败：300 超出 u8 的取值范围，需要用更大的类型或先校验");
        }
    }

    // 字符串到数字属于"可能失败"的转换，用 parse。
    let parsed = "3.5".parse::<f64>();
    println!("\"3.5\".parse::<f64>() = {parsed:?}");
    println!("// 预期输出：\"3.5\".parse::<f64>() = Ok(3.5)");

    println!();
}

/// 示例 7：典型使用场景 —— 解析外部数据并做安全的数值转换
///
/// 要点：真实项目里数字常以文本形式到来，必须"解析 + 校验 + 转换"三步走。
fn demo_7_typical_use_case() {
    println!("--- 示例 7：典型使用场景（解析并转换数值） ---");

    // 模拟从配置文件/接口读到的原始文本记录：(字段名, 文本值)。
    let raw_records = [("端口", "8080"), ("最大连接数", "1024"), ("超时", "30")];

    // 逐条解析：文本 -> i64 -> u32，任何一步失败都给出提示而不是 panic。
    for (field, text) in raw_records {
        match text.parse::<i64>() {
            // 第一步成功：再尝试收窄成 u32（业务上端口/连接数都是非负的）。
            Ok(number) => match u32::try_from(number) {
                Ok(value) => {
                    println!("{field}：解析并转换成功 -> {value}");
                    // 三条记录的期望输出写成硬编码字面量，便于逐字符核对。
                    let expected = match field {
                        "端口" => "// 预期输出：端口：解析并转换成功 -> 8080",
                        "最大连接数" => "// 预期输出：最大连接数：解析并转换成功 -> 1024",
                        _ => "// 预期输出：超时：解析并转换成功 -> 30",
                    };
                    println!("{expected}");
                }
                Err(_) => println!("{field}：数值 {number} 超出 u32 范围，已拒绝"),
            },
            // 第一步失败：文本根本不是数字。
            Err(err) => println!("{field}：无法解析为数字（{err}）"),
        }
    }

    // 汇总：把一组文本分数解析成整数并求平均，统计失败条数。
    let score_texts = ["88", "92", "not-a-number", "75"];
    let mut sum = 0i32;
    let mut parsed_count = 0;
    let mut failed_count = 0;

    for text in score_texts {
        match text.parse::<i32>() {
            Ok(score) => {
                // 解析成功后累加，这里用 saturating_add 防止极端情况下的溢出。
                sum = sum.saturating_add(score);
                parsed_count += 1;
            }
            Err(_) => failed_count += 1,
        }
    }

    println!("成功解析 {parsed_count} 条，失败 {failed_count} 条，总分 {sum}");
    println!("// 预期输出：成功解析 3 条，失败 1 条，总分 255");
    // 注意：整数除法会截断，255 / 3 = 85。
    println!("平均分（整数除法）= {}", sum / parsed_count);
    println!("// 预期输出：平均分（整数除法）= 85");
    println!(
        "平均分（浮点除法）= {}",
        f64::from(sum) / f64::from(parsed_count)
    );
    println!("// 预期输出：平均分（浮点除法）= 85");

    println!();
}

/// 示例 8：常见错误示例（错误代码全部注释掉）
///
/// 要点：整数溢出、数组越界、类型不匹配、直接比较浮点数，是数据类型章节的高频坑。
fn demo_8_common_mistakes() {
    println!("--- 示例 8：常见错误示例（错误代码已注释） ---");

    // ===== 错误 1：debug 模式下整数溢出会 panic =====
    // let x: u8 = 255;
    // let y = x + 1;
    // error: this arithmetic operation will overflow
    //   = note: `#[deny(arithmetic_overflow)]` on by default（编译期常量折叠时直接报错）
    //   debug 构建下运行时溢出会 panic: attempt to add with overflow
    // 修正：改用 checked_/wrapping_/saturating_ 系列方法，或换成更大的类型。
    let x: u8 = 255;
    let checked = x.checked_add(1); // 返回 Option，溢出时是 None
    let wrapped = x.wrapping_add(1); // 环绕：255 + 1 -> 0
    let saturated = x.saturating_add(1); // 饱和：停在最大值 255
    println!(
        "255u8：checked_add(1) = {checked:?}，wrapping_add(1) = {wrapped}，saturating_add(1) = {saturated}"
    );
    println!(
        "// 预期输出：255u8：checked_add(1) = None，wrapping_add(1) = 0，saturating_add(1) = 255"
    );

    // ===== 错误 2：数组越界索引 =====
    // let arr = [1, 2, 3];
    // println!("{}", arr[3]);
    // 索引是编译期常量时直接编译失败（实测 rustc 1.99，没有 E 编号）：
    // error: this operation will panic at runtime
    //   | index out of bounds: the length is 3 but the index is 3
    //   = note: `#[deny(unconditional_panic)]` on by default
    // 索引来自运行时变量时编译通过，执行到该行 panic（退出码 101）：
    //   index out of bounds: the len is 3 but the index is 3
    // 修正：用 `arr.get(3)` 得到 Option，或先判断 `index < arr.len()`。
    let arr = [1, 2, 3];
    println!("安全访问 arr.get(3) = {:?}", arr.get(3));
    println!("// 预期输出：安全访问 arr.get(3) = None");

    // ===== 错误 3：类型不匹配，i32 不能直接赋给 u32 变量 =====
    // let port: u32 = 8080i32;
    // error[E0308]: mismatched types
    //   expected `u32`, found `i32`
    // 修正：用 `as`（需确认无溢出）或 `u32::try_from(8080i32)` 显式转换。
    let port_number: u32 = u32::try_from(8080i32).expect("8080 一定在 u32 范围内");
    println!("转换后的端口 = {port_number}");
    println!("// 预期输出：转换后的端口 = 8080");

    // ===== 错误 4：混用 char 与 &str 字面量 =====
    // let c: char = "A";
    // error[E0308]: mismatched types
    //   expected `char`, found `&str`
    // 修正：单个字符写单引号 `'A'`；字符串写双引号，需要取字符时用 `.chars()`。
    let c: char = 'A';
    let first_char = "A".chars().next();
    println!("char 字面量 = {c}，字符串取首字符 = {first_char:?}");
    println!("// 预期输出：char 字面量 = A，字符串取首字符 = Some('A')");

    // ===== 错误 5：直接用 == 比较浮点数 =====
    // if 0.1 + 0.2 == 0.3 { println!("相等"); }
    // 编译通过但结果永远为 false（逻辑错误，不是编译错误）
    // 修正：比较差值 `(a - b).abs() < EPS`。
    // 这里同样要标注 f64，原因和示例 2 里的 E0689 一样。
    let a: f64 = 0.1 + 0.2;
    println!(
        "(0.1 + 0.2 - 0.3).abs() < 1e-9 = {}",
        (a - 0.3).abs() < 1e-9
    );
    println!("// 预期输出：(0.1 + 0.2 - 0.3).abs() < 1e-9 = true");

    // ===== 错误 6：无后缀的浮点字面量赋给整数类型 =====
    // let n: i32 = 1.0;
    // error[E0308]: mismatched types
    //   expected `i32`, found floating-point number
    // 修正：整数就写 `1`，确实要截断则写 `1.0 as i32`。
    let n: i32 = 1;
    let m = 1.0_f64 as i32;
    println!("整数 {n}，由浮点截断得到 {m}");
    println!("// 预期输出：整数 1，由浮点截断得到 1");

    // ===== 错误 7：元组下标越界或写错下标格式 =====
    // let t = (1, 2);
    // println!("{}", t.2);
    // error[E0609]: no field `2` on type `({integer}, {integer})`
    // 修正：下标从 0 开始，只有 .0 和 .1 合法；元素多时改用结构体（lesson 06）。
    let t = (1, 2);
    println!("元组元素 t.0 = {}, t.1 = {}", t.0, t.1);
    println!("// 预期输出：元组元素 t.0 = 1, t.1 = 2");

    println!();
}
