//! lesson_03_functions.rs —— 主题：函数
//!
//! 学习目标：
//!   1. 掌握函数的定义、参数与返回值写法，明确每个参数都必须标注类型
//!   2. 分清"语句（statement）"与"表达式（expression）"，理解尾表达式的返回规则
//!   3. 会使用函数指针（fn 类型）与发散函数（返回类型 `!`）
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_03_functions.rs -o lesson_03 && ./lesson_03
//!   或在本项目根目录执行：cargo run --bin lesson_03_functions
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

/// 会员默认折扣率：用常量而不是局部变量，这样下面那个闭包不捕获任何环境，
/// 才能强制转换成函数指针 `fn(f64) -> f64`（捕获了外部变量的闭包不能转）。
const MEMBER_DISCOUNT_RATE: f64 = 0.9;

fn main() {
    // main 只负责按学习顺序调用各个 demo，本身不写业务逻辑。
    println!("========== lesson_03_functions：函数 ==========\n");

    demo_1_define_and_call();
    demo_2_multiple_params_and_return();
    demo_3_statement_vs_expression();
    demo_4_function_pointer();
    demo_5_diverging_function();
    demo_6_typical_use_case();
    demo_7_common_mistakes();
}

/// 示例 1：函数定义与调用
///
/// 要点：`fn 名字(参数: 类型) -> 返回类型 { 函数体 }`，调用时机与定义顺序无关。
fn demo_1_define_and_call() {
    println!("--- 示例 1：函数定义与调用 ---");

    // 调用一个定义在文件后面的函数：Rust 不要求"先定义后调用"。
    let result = add(3, 4);
    println!("add(3, 4) = {result}");
    println!("// 预期输出：add(3, 4) = 7");

    // 无返回值的函数返回单元类型 `()`，它的值可以被显式忽略，也可以赋给变量。
    let unit = print_separator();
    println!("无返回值函数返回单元类型，其值打印出来是 {unit:?}");
    println!("// 预期输出：无返回值函数返回单元类型，其值打印出来是 ()");

    println!();
}

/// 示例 2：多个参数与返回值
///
/// 要点：参数必须逐个标注类型；返回值用 `->` 标注，最后一行表达式即返回值。
fn demo_2_multiple_params_and_return() {
    println!("--- 示例 2：多个参数与返回值 ---");

    // 多参数：每个参数都要写"名字: 类型"，不能用 `a, b: i32` 这种省略写法。
    let area = rectangle_area(3.0, 4.5);
    println!("rectangle_area(3.0, 4.5) = {area}");
    println!("// 预期输出：rectangle_area(3.0, 4.5) = 13.5");

    // 多返回值：显式标注返回类型为元组，一次带回多个结果。
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {quotient} 余 {remainder}");
    println!("// 预期输出：17 ÷ 5 = 3 余 2");

    // 只带一个返回值的函数照样可以返回元组，二者并不冲突。
    let (min_value, max_value) = min_max(&[5, 2, 9, 1]);
    println!("最小 {min_value}，最大 {max_value}");
    println!("// 预期输出：最小 1，最大 9");

    println!();
}

/// 示例 3：语句与表达式
///
/// 要点：语句"做事"不产生值；表达式"算值"。函数体最后一个表达式就是返回值，
/// 一旦在它后面加上分号，它立刻变成语句，函数就返回 `()`。
fn demo_3_statement_vs_expression() {
    println!("--- 示例 3：语句与表达式 ---");

    // `let x = 5;` 整体是语句；而 `5` 本身是表达式。
    let x = 5;

    // 块 `{ ... }` 也是表达式：最后一个表达式的值就是块的值。
    let block_value = {
        let doubled = x * 2;
        doubled + 1 // 尾表达式：不能加分号
    };
    println!("块表达式的值 = {block_value}");
    println!("// 预期输出：块表达式的值 = 11");

    // if 是表达式，所以可以直接把它赋给变量（详见 lesson 04）。
    let sign_label = if x > 0 { "正数" } else { "非正数" };
    println!("x = {x} 是 {sign_label}");
    println!("// 预期输出：x = 5 是 正数");

    // 带尾表达式的函数与"提前 return 的函数"结果一致，但前者更符合 Rust 风格。
    println!("square_tail(6) = {}", square_tail(6));
    println!("// 预期输出：square_tail(6) = 36");
    println!("square_with_return(6) = {}", square_with_return(6));
    println!("// 预期输出：square_with_return(6) = 36");

    // 提前 return 的真实用途：先处理异常分支，再写主流程。
    println!("positive_only(9) = {}", positive_only(9));
    println!("// 预期输出：positive_only(9) = 9");
    println!("positive_only(-9) = {}", positive_only(-9));
    println!("// 预期输出：positive_only(-9) = 0");

    println!();
}

/// 示例 4：函数指针
///
/// 要点：`fn(参数类型) -> 返回类型` 是一种类型，可以把函数当值传递、存进数组。
fn demo_4_function_pointer() {
    println!("--- 示例 4：函数指针 ---");

    // 把函数名赋给变量：类型就是函数指针 `fn(i32, i32) -> i32`。
    let operation: fn(i32, i32) -> i32 = add;
    println!("通过函数指针调用 add(10, 5) = {}", operation(10, 5));
    println!("// 预期输出：通过函数指针调用 add(10, 5) = 15");

    // 闭包（不捕获外部变量时）也能强制转换成函数指针。
    let increment: fn(i32) -> i32 = |value| value + 1;
    println!(
        "不捕获环境的闭包作为函数指针：increment(41) = {}",
        increment(41)
    );
    println!("// 预期输出：不捕获环境的闭包作为函数指针：increment(41) = 42");

    // 最实用的形式：把函数指针放进数组，实现"策略表"。
    let strategies: [fn(i32, i32) -> i32; 3] = [add, subtract, max_of];
    let names = ["add", "subtract", "max_of"];
    for (name, strategy) in names.iter().zip(strategies.iter()) {
        // 每个元素都是一样的签名，因此可以统一调用。
        println!("{name}(8, 3) = {}", strategy(8, 3));
        // 三张策略表的期望输出写成硬编码字面量，便于逐字符核对。
        let expected = match *name {
            "add" => "// 预期输出：add(8, 3) = 11",
            "subtract" => "// 预期输出：subtract(8, 3) = 5",
            _ => "// 预期输出：max_of(8, 3) = 8",
        };
        println!("{expected}");
    }

    // 函数指针也可以作为参数传入，实现"可替换的行为"。
    println!("apply(add, 2, 3) = {}", apply(add, 2, 3));
    println!("// 预期输出：apply(add, 2, 3) = 5");
    println!("apply(max_of, 2, 3) = {}", apply(max_of, 2, 3));
    println!("// 预期输出：apply(max_of, 2, 3) = 3");

    println!();
}

/// 示例 5：发散函数（返回类型 `!`）
///
/// 要点：`!` 表示函数永不返回。它可以被强制转换成任意类型，
/// 所以能在需要某个类型的表达式位置上代替真正的值。
fn demo_5_diverging_function() {
    println!("--- 示例 5：发散函数（! 类型） ---");

    // 只查看发散函数的类型，不真正调用它，避免程序提前退出。
    let divergent: fn(&str) -> ! = exit_with_error;
    println!(
        "发散函数的类型是 fn(&str) -> !，地址可被获取：{:p}",
        divergent as *const ()
    );
    // 地址的具体数值由链接与 ASLR 决定，每次运行可能不同，所以这里不写死具体值。
    println!("// 预期输出：发散函数的类型是 fn(&str) -> !，地址可被获取：0x<十六进制地址>");

    // `!` 可以被强制转换成任何类型：下面 match 的两个分支一个给 i32、一个给 !，
    // 编译器会把 `!` 分支"当作" i32，于是整个 match 的类型就是 i32。
    let mode = "normal";
    let guaranteed: i32 = match mode {
        "normal" => 0,
        // exit_with_error 永不返回，类型是 !，因此它可以放在这里充当 i32 分支。
        _ => exit_with_error("未知模式，进程终止"),
    };
    println!("never 类型可被强转为任意类型，这里得到 guaranteed = {guaranteed}");
    println!("// 预期输出：never 类型可被强转为任意类型，这里得到 guaranteed = 0");

    // 反例（这样写拿不到"强转"的效果）：`loop { break 0; }` 的类型是 i32 而不是 !，
    // 因为带 break 值的 loop 会正常返回那个值，与 never 类型无关。
    let not_never: i32 = {
        loop {
            break 0; // 这里真的 break 了，所以整个块表达式类型是 i32
        }
    };
    println!("带 break 值的 loop 类型是 i32（不是 !），not_never = {not_never}");
    println!("// 预期输出：带 break 值的 loop 类型是 i32（不是 !），not_never = 0");

    // 用发散函数做参数校验：只有校验失败时才会走到检查。
    let port = 8080;
    let valid = check_port(port);
    println!("端口 {port} 校验通过 = {valid}");
    println!("// 预期输出：端口 8080 校验通过 = true");

    // 这里绝不会触发（port 合法），但代码保留了"出错就退出"的语义。
    if !valid {
        exit_with_error("端口非法，进程终止");
    }

    println!();
}

/// 示例 6：典型使用场景 —— 把一段计算逻辑拆成小函数
///
/// 要点：真实项目里一个函数只做一件事，靠组合小函数完成完整业务计算，
/// 这样每一段都能单独测试。
fn demo_6_typical_use_case() {
    println!("--- 示例 6：典型使用场景（拆分计算逻辑为函数） ---");

    // 订单：商品单价、数量、会员等级折扣。
    let unit_price = 19.9;
    let quantity = 3;
    let discount_rate = MEMBER_DISCOUNT_RATE; // 九折

    // 第一步：小计。
    let subtotal = subtotal(unit_price, quantity);
    println!("小计 = {subtotal}");
    println!("// 预期输出：小计 = 59.699999999999996");

    // 第二步：折扣后金额。
    let discounted = apply_discount(subtotal, discount_rate);
    println!("折后金额 = {discounted}");
    println!("// 预期输出：折后金额 = 53.73");

    // 第三步：加税并保留两位小数。
    let total = with_tax(discounted, 0.06);
    println!("含税总额 = {total:.2}");
    println!("// 预期输出：含税总额 = 56.95");

    // 用函数指针把三步串成一条流水线（这就是"组合小函数"的价值）。
    // 闭包内部只使用常量，不捕获局部变量，因此可以当作函数指针存放。
    let pipeline: [fn(f64) -> f64; 1] =
        [|value| with_tax(apply_discount(value, MEMBER_DISCOUNT_RATE), 0.06)];
    for step in pipeline.iter() {
        println!("流水线结果 = {:.2}", step(subtotal));
        println!("// 预期输出：流水线结果 = 56.95");
    }

    // 第二个场景：安全的除法函数，把"失败"编码进返回值而不是 panic。
    println!("safe_divide(10, 4) = {}", safe_divide(10, 4));
    println!("// 预期输出：safe_divide(10, 4) = 10 / 4 = 2.5");
    println!("safe_divide(10, 0) = {}", safe_divide(10, 0));
    println!("// 预期输出：safe_divide(10, 0) = 除数不能为 0");

    println!();
}

/// 示例 7：常见错误示例（错误代码全部注释掉）
///
/// 要点：漏写类型、漏写返回值、尾表达式多写分号、参数顺序写反，是函数章节的高频坑。
fn demo_7_common_mistakes() {
    println!("--- 示例 7：常见错误示例（错误代码已注释） ---");

    // ===== 错误 1：参数和返回值必须标注类型 =====
    // fn double(value) -> i32 { value * 2 }
    // error: expected one of `:`, `@`, or `|`, found `)`
    //   help: if this is a parameter name, give it a type
    // （这是解析阶段的无编号错误，不是 E0642；E0642 讲的是另一件事：
    //   没有函数体的函数里不允许写模式，真正触发它的是
    //   trait T { fn f((a, b): (i32, i32)); }
    //   error[E0642]: patterns aren't allowed in functions without bodies）
    // 修正：`fn double(value: i32) -> i32 { value * 2 }`
    println!("double(21) = {}", double(21));
    println!("// 预期输出：double(21) = 42");

    // ===== 错误 2：漏写返回类型，却用表达式返回了值 =====
    // fn half(value: i32) { value / 2 }
    // error[E0308]: mismatched types
    //   expected `()`, found `i32`
    //   help: try adding a return type: `-> i32`
    // （随后按整数使用 half(9) 时还会连带报
    //   error[E0277]: `()` doesn't implement `std::fmt::Display`；
    //   这里没有任何 "unused return value" 警告）
    // 修正：补上返回类型 `-> i32`。
    println!("half(9) = {}", half(9));
    println!("// 预期输出：half(9) = 4");

    // ===== 错误 3：尾表达式多写了分号 =====
    // fn square_tail(value: i32) -> i32 { value * value; }
    // error[E0308]: mismatched types
    //   expected `i32`, found `()`
    // 修正：删掉尾表达式的分号，或显式写 `return value * value;`。
    println!("square_tail(5) = {}", square_tail(5));
    println!("// 预期输出：square_tail(5) = 25");

    // ===== 错误 4：把 `let` 语句当成表达式使用 =====
    // let a = (let b = 1);
    // error: expected expression, found `let` statement
    // 修正：`let` 是语句，不能出现在表达式位置；改为直接写值或调用函数。
    let a = 1;
    println!("修正后 a = {a}");
    println!("// 预期输出：修正后 a = 1");

    // ===== 错误 5：实参个数或类型不匹配 =====
    // let v = add(1);
    // error[E0061]: this function takes 2 arguments but 1 argument was supplied
    // let v = add(1, 2.0);
    // error[E0308]: mismatched types（expected `i32`, found floating-point number）
    // 修正：按签名传够个数、且类型一致，比如 `add(1, 2)`。
    println!("修正后 add(1, 2) = {}", add(1, 2));
    println!("// 预期输出：修正后 add(1, 2) = 3");

    // ===== 错误 6：发散函数之后的代码不可达 =====
    // fn never_returns() -> i32 {
    //     return exit_with_error_number();
    //     println!("这行永远不会执行");
    // }
    // warning: unreachable statement
    //   = note: `#[warn(unreachable_code)]` (part of `#[warn(unused)]`) on by default
    // 修正：删除 return 之后的代码，或把这些代码移到 return 之前。
    println!("提示：发散函数（返回 `!`）之后的语句不可达，编译器会给出 unreachable_code 警告");
    println!(
        "// 预期输出：提示：发散函数（返回 `!`）之后的语句不可达，编译器会给出 unreachable_code 警告"
    );

    println!();
}

/// 两数相加，演示最基本的函数定义。
fn add(left: i32, right: i32) -> i32 {
    // 尾表达式（没有分号）：它的值就是函数返回值。
    left + right
}

/// 无返回值的函数：省略 `-> ()`，函数体最后是一个语句。
fn print_separator() {
    // 只做事、不产生有意义的值，因此返回单元类型 `()`。
    let _line = "-".repeat(4); // 下划线前缀表示"有意不使用这个值"
}

/// 矩形面积：演示 f64 参数与返回值。
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

/// 除法取商与余数：演示用元组一次返回两个结果。
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

/// 求切片中的最小值和最大值：演示返回元组 + 切片参数。
fn min_max(values: &[i32]) -> (i32, i32) {
    // 用首元素初始化，避免引入 Option（Option 见 lesson 07）。
    let mut min_value = values[0];
    let mut max_value = values[0];
    for &value in values {
        if value < min_value {
            min_value = value;
        }
        if value > max_value {
            max_value = value;
        }
    }
    (min_value, max_value)
}

/// 用尾表达式返回平方。
fn square_tail(value: i32) -> i32 {
    value * value
}

/// 用显式 `return` 返回平方：与 square_tail 等价，但更啰嗦。
fn square_with_return(value: i32) -> i32 {
    // `return` 是语句，所以它自己带分号。
    return value * value;
}

/// 大于 0 返回原值，否则提前返回 0：演示提前 return 的写法。
fn positive_only(value: i32) -> i32 {
    if value <= 0 {
        // 提前返回：后面的主流程就不会被执行。
        return 0;
    }
    value
}

/// 两数相减，用于函数指针策略表。
fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

/// 取较大值，用于函数指针策略表。
fn max_of(left: i32, right: i32) -> i32 {
    if left >= right { left } else { right }
}

/// 接收函数指针作为参数：调用方决定"怎么算"。
fn apply(operation: fn(i32, i32) -> i32, left: i32, right: i32) -> i32 {
    operation(left, right)
}

/// 发散函数：返回 `!`，表示调用后永远不会回到调用点。
fn exit_with_error(message: &str) -> ! {
    // 真实项目里这里常见于"配置错误直接终止进程"。
    eprintln!("致命错误：{message}");
    std::process::exit(2)
}

/// 端口校验：合法范围是 1..=65535。
fn check_port(port: u16) -> bool {
    port != 0
}

/// 小计：单价 × 数量。
fn subtotal(unit_price: f64, quantity: u32) -> f64 {
    // 数量是无符号整数，乘法前先转成 f64（转换见 lesson 02）。
    unit_price * f64::from(quantity)
}

/// 折扣计算。
fn apply_discount(amount: f64, discount_rate: f64) -> f64 {
    amount * discount_rate
}

/// 含税价计算。
fn with_tax(amount: f64, tax_rate: f64) -> f64 {
    amount * (1.0 + tax_rate)
}

/// 安全除法：除数为 0 时返回说明文本，绝不 panic。
fn safe_divide(dividend: i32, divisor: i32) -> String {
    if divisor == 0 {
        // 提前返回：把失败情况编码进返回值。
        return String::from("除数不能为 0");
    }
    // 格式化后返回拥有所有权的 String。
    format!(
        "{dividend} / {divisor} = {}",
        dividend as f64 / divisor as f64
    )
}

/// 简单翻倍函数，供常见错误示例中的"修正后"代码调用。
fn double(value: i32) -> i32 {
    value * 2
}

/// 整数折半（截断），供常见错误示例中的"修正后"代码调用。
fn half(value: i32) -> i32 {
    value / 2
}
