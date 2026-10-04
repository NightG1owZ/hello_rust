//! lesson_01_variables_mutability.rs —— 主题：变量与可变性
//!
//! 学习目标：
//!   1. 理解 Rust 变量默认不可变（immutable）以及 `mut` 如何显式开启可变性
//!   2. 掌握变量遮蔽（shadowing）的写法与它和 `mut` 的本质区别
//!   3. 会用类型推断与显式类型标注，并区分 `const` 与 `static` 的差异
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_01_variables_mutability.rs -o lesson_01 && ./lesson_01
//!   或在本项目根目录执行：cargo run --bin lesson_01_variables_mutability
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

/// 全局常量：编译期就要确定的值，习惯用 SCREAMING_SNAKE_CASE 命名。
/// 常量与 `let` 的最大区别是：它可以定义在函数外部，并且必须标注类型。
const APP_NAME: &str = "Rust 学习工程";
const MAX_RETRY: u32 = 3;
const BASE_TIMEOUT_SECS: u64 = 5;

/// 全局静态变量：同样要求显式类型，但与常量不同的是它拥有固定内存地址，
/// 因此可以被引用（`&PAGE_SIZE`）。这里只读取它，不涉及可变静态变量。
static PAGE_SIZE: u32 = 4096;

fn main() {
    // main 只负责按学习顺序调用各个 demo，本身不写业务逻辑。
    println!("========== lesson_01_variables_mutability：变量与可变性 ==========\n");

    demo_1_immutable_by_default();
    demo_2_mut();
    demo_3_shadowing();
    demo_4_type_inference_and_annotation();
    demo_5_const_and_static();
    demo_6_typical_use_case();
    demo_7_common_mistakes();
}

/// 示例 1：默认不可变（immutable）
///
/// 要点：Rust 的变量默认是只读的，这能从编译期就挡掉"意外修改"这类 bug。
fn demo_1_immutable_by_default() {
    println!("--- 示例 1：默认不可变 ---");

    // 没有 `mut` 的变量只能被读取，不能被重新赋值。
    let max_retry = 3;
    println!("默认不可变变量 max_retry = {max_retry}");
    println!("// 预期输出：默认不可变变量 max_retry = 3");

    // 不可变指的是"不能改这个绑定"，并不影响把一个新值交给另一个绑定。
    let retry_limit = max_retry + 1;
    println!("retry_limit = {retry_limit}");
    println!("// 预期输出：retry_limit = 4");

    println!();
}

/// 示例 2：`mut` 显式声明可变
///
/// 要点：只有加了 `mut` 的绑定才允许被重新赋值，可变性是要花钱（代价）买的，
/// 所以 Rust 让你主动写出来，而不是默认允许。
fn demo_2_mut() {
    println!("--- 示例 2：mut 显式可变 ---");

    // `mut` 表示这个绑定可以指向新的值。
    let mut counter = 0;
    println!("初始 counter = {counter}");
    println!("// 预期输出：初始 counter = 0");

    // 累加 1：这里用 `+=` 而不是 `counter = counter + 1`，两者等价。
    counter += 1;
    println!("第一次累加后 counter = {counter}");
    println!("// 预期输出：第一次累加后 counter = 1");

    // 同一个可变变量可以反复改。
    counter += 2;
    println!("再加 2 后 counter = {counter}");
    println!("// 预期输出：再加 2 后 counter = 3");

    // 在循环里累加是可变变量最常见的用法之一。
    let mut retry_total = 0u32;
    for _ in 0..MAX_RETRY {
        // 每次循环把可变变量 +1，循环结束后它保存了"重试次数总和"。
        retry_total += 1;
    }
    println!("循环累加后的 retry_total = {retry_total}");
    println!("// 预期输出：循环累加后的 retry_total = 3");

    println!();
}

/// 示例 3：遮蔽（shadowing）
///
/// 要点：用新的 `let` 重新声明同名变量会"遮蔽"旧变量：旧绑定并没有被修改，
/// 只是被挡住了，而且新变量可以换成完全不同的类型。
fn demo_3_shadowing() {
    println!("--- 示例 3：变量遮蔽 shadowing ---");

    // 基础用法：后声明的同名变量挡住前面的值。
    let input = 5;
    let input = input + 1;
    // 此处 input 已经是 6：第一行定义的 input 依然存在，只是不再可见。
    println!("遮蔽一次后 input = {input}");
    println!("// 预期输出：遮蔽一次后 input = 6");

    let input = input * 3;
    println!("再遮蔽一次后 input = {input}");
    println!("// 预期输出：再遮蔽一次后 input = 18");

    // 关键差异：遮蔽可以改变类型，而 `mut` 绝对不能改变类型。
    let budget = "1200"; // 此时是 &str
    let budget = budget.len(); // 遮蔽后变成 usize（字符串长度）
    println!("字符串长度 budget = {budget}");
    println!("// 预期输出：字符串长度 budget = 4");

    // 遮蔽也常用于"逐步加工"同一个业务对象的名字。
    let host = "  api.example.com  ";
    let host = host.trim(); // 去掉首尾空格，仍是 &str
    let host = host.to_uppercase(); // 变成 String（新类型），旧绑定再次被遮蔽
    println!("规范化后的 host = {host}");
    println!("// 预期输出：规范化后的 host = API.EXAMPLE.COM");

    println!();
}

/// 示例 4：类型推断与显式类型标注
///
/// 要点：编译器通常能推断出变量类型；当推断不出来或想表明意图时，必须显式标注。
fn demo_4_type_inference_and_annotation() {
    println!("--- 示例 4：类型推断与类型标注 ---");

    // 推断：整数字面量在没有任何线索时默认推断为 i32。
    let inferred_int = 42;
    println!("inferred_int = {inferred_int}");
    println!("// 预期输出：inferred_int = 42");

    // 推断：小数字面量默认推断为 f64。
    let inferred_float = 3.5;
    println!("inferred_float = {inferred_float}");
    println!("// 预期输出：inferred_float = 3.5");

    // 推断也可以来自后面的使用方式：字面量先"待定"，由第一次真正用到的语义决定类型。
    let total = 10.0; // 小数字面量：默认 f64
    let ratio = 2; // 无后缀整数字面量：这里被推断为 i32（整数文字永远不可能变成 f64）
    // 整数与浮点不能直接运算，所以必须显式转换（as 的用法见 lesson 02）。
    // 正因为 ratio 被推断成了整数，这行 `as f64` 才必不可少。
    let quotient = total / ratio as f64;
    println!("quotient = {quotient}");
    println!("// 预期输出：quotient = 5");
    // 反过来，如果类型真由上下文决定，可以看这个例子：元素被 push 成 i64。
    let mut counters = Vec::new(); // 此刻元素类型还是"待定"
    counters.push(1_i64); // 由这一行敲定：Vec<i64>
    println!("counters 的元素类型由使用方式推断，值 = {:?}", counters);
    println!("// 预期输出：counters 的元素类型由使用方式推断，值 = [1]");

    // 显式标注：写在变量名后面，用冒号加类型。
    let timeout_secs: u64 = 30;
    println!("timeout_secs = {timeout_secs}");
    println!("// 预期输出：timeout_secs = 30");

    // 显式标注最有价值的场景：把 String 真正转成数字。
    let port_text = "8080";
    // 若写 `let port = port_text.parse()?;`，编译器报的是（实测 rustc 1.99）：
    // error[E0277]: the `?` operator can only be used in a function that returns
    //                `Result` or `Option` (or another type that implements `FromResidual`)
    //   （细节行：cannot use the `?` operator in a function that returns `()`）
    // 原因是 `?` 只能用在返回 Result/Option 的函数里，而 main 返回 `()`；
    // 因此这里既显式标注 u16，也用 expect 而不是 `?`。
    let port: u16 = port_text.parse().expect("端口必须是数字");
    println!("port = {port}");
    println!("// 预期输出：port = 8080");

    // 也可以直接给字面量加后缀，效果等价且更紧凑。
    let max_conn: u32 = 1_024;
    let max_conn_suffix = 1_024u32;
    println!("max_conn = {max_conn}，max_conn_suffix = {max_conn_suffix}");
    println!("// 预期输出：max_conn = 1024，max_conn_suffix = 1024");

    println!();
}

/// 示例 5：常量 const 与静态变量 static
///
/// 要点：`const` 在编译期被内联到使用处，`static` 则有固定的内存地址可被引用。
fn demo_5_const_and_static() {
    println!("--- 示例 5：const 与 static ---");

    // 常量定义在函数外（见文件顶部），这里直接使用。
    println!("APP_NAME = {APP_NAME}");
    println!("// 预期输出：APP_NAME = Rust 学习工程");

    // 常量可以参与编译期计算，下面这个值在编译时就已经算好了。
    let total_timeout = BASE_TIMEOUT_SECS * MAX_RETRY as u64;
    println!("总超时时间 = {total_timeout} 秒");
    println!("// 预期输出：总超时时间 = 15 秒");

    // static 可以被取地址：取引用用 `&`，它证明变量有稳定的内存位置。
    let page_size_ref: &u32 = &PAGE_SIZE;
    println!("页大小 = {page_size_ref}，地址可被引用");
    println!("// 预期输出：页大小 = 4096，地址可被引用");

    // static 的名字属于模块作用域，因此可以被局部同名变量遮蔽。
    let page_size = PAGE_SIZE / 2;
    println!("局部遮蔽后的 page_size = {page_size}");
    println!("// 预期输出：局部遮蔽后的 page_size = 2048");

    // 说明：`static mut` 的赋值需要 unsafe，属于进阶内容，本课不演示。
    println!("注意：可变的全局状态请优先用 const 组合或 Cell/Atomic（后续课程介绍）");
    println!("// 预期输出：注意：可变的全局状态请优先用 const 组合或 Cell/Atomic（后续课程介绍）");

    println!();
}

/// 示例 6：典型使用场景 —— 通过"常量配置 + 可变计数器"统计一次带重试的任务
///
/// 要点：真实代码里，配置项是不可变的常量，而运行过程中的统计量必须是可变变量。
fn demo_6_typical_use_case() {
    println!("--- 示例 6：典型使用场景（配置常量 + 可变计数器） ---");

    // 一次数据同步任务：名字和每条记录大小都来自常量配置。
    let task_name = "用户数据同步";
    let record_size_kb = 8;
    let total_records = 5;

    // 可变计数器：记录这次任务实际发出的请求数、传输的总字节数。
    let mut requests = 0;
    let mut total_kb = 0;

    for _index in 0..total_records {
        // 每处理一条记录就 +1；可变变量的自增在这里是"真实业务动作"。
        requests += 1;
        total_kb += record_size_kb;
    }

    println!("{task_name}：共 {total_records} 条记录，发出请求 {requests} 次");
    println!("// 预期输出：用户数据同步：共 5 条记录，发出请求 5 次");
    println!("累计传输 {total_kb} KB（均摊每条 {record_size_kb} KB）");
    println!("// 预期输出：累计传输 40 KB（均摊每条 8 KB）");

    // 常量决定重试策略：常量值不会被运行时修改，统计值则随运行变化。
    let mut attempt = 0;
    let mut success = false;
    // 每一轮的期望输出都写成硬编码字面量，读者可以逐个字符核对（不再由变量算出来）。
    let expected_attempts = [
        "// 预期输出：第 1 次尝试，成功 = false",
        "// 预期输出：第 2 次尝试，成功 = false",
        "// 预期输出：第 3 次尝试，成功 = true",
    ];
    while attempt < MAX_RETRY && !success {
        // 真实项目里这里会调用一次网络请求，本示例用"第三次才成功"模拟。
        attempt += 1;
        success = attempt == 3;
        println!("第 {attempt} 次尝试，成功 = {success}");
        println!("{}", expected_attempts[(attempt - 1) as usize]);
    }
    println!("最终在第 {attempt} 次尝试成功：{success}");
    println!("// 预期输出：最终在第 3 次尝试成功：true");

    println!();
}

/// 示例 7：常见错误示例（错误代码全部注释掉）
///
/// 要点：把初学者最容易踩的坑集中起来，逐条标注编译器错误编号与修正方法。
fn demo_7_common_mistakes() {
    println!("--- 示例 7：常见错误示例（错误代码已注释） ---");

    // ===== 错误 1：对不可变变量重新赋值 =====
    // let lock = 10;
    // lock = 20;
    // error[E0384]: cannot assign twice to immutable variable `lock`
    // help: consider making this binding mutable: `mut lock`
    // 修正：需要改值就声明成 `let mut lock = 10;`，
    //       若只是换一个值，用遮蔽 `let lock = 20;` 也可以。
    let mut lock = 10;
    // 先读出一次初始值，证明这个绑定确实被使用了（否则编译器会警告 unused_assignments）。
    println!("lock 的初始值 = {lock}");
    println!("// 预期输出：lock 的初始值 = 10");
    lock = 20;
    println!("修正后 lock = {lock}");
    println!("// 预期输出：修正后 lock = 20");

    // ===== 错误 2：用 `mut` 改变变量类型 =====
    // let mut value = 5;
    // value = "hello";
    // error[E0308]: mismatched types
    //   expected integer, found `&str`
    // 修正：`mut` 只允许改值、不允许改类型；想换类型必须用遮蔽重新 `let`。
    let value = 5;
    // 先使用一次，避免"赋值后从未读取"的告警，也让读者看到遮蔽前后的差异。
    println!("遮蔽前的 value = {value}（i32）");
    println!("// 预期输出：遮蔽前的 value = 5（i32）");
    let value = "hello"; // 遮蔽：新绑定可以拥有新类型
    println!("遮蔽后 value = {value}");
    println!("// 预期输出：遮蔽后 value = hello");

    // ===== 错误 3：变量未初始化就使用 =====
    // let timeout: u64;
    // println!("{timeout}");
    // error[E0381]: used binding `timeout` isn't initialized
    // 修正：声明时直接给初值，或保证所有分支都赋值后再使用。
    let timeout: u64;
    timeout = 30; // 使用前完成初始化
    println!("初始化后 timeout = {timeout}");
    println!("// 预期输出：初始化后 timeout = 30");

    // ===== 错误 4：误用赋值运算符 `=` 做比较 =====
    // let a = 1;
    // let b = 2;
    // if a = b {}
    // error[E0308]: mismatched types（`if` 的条件必须是 `bool`，赋值返回 `()`）
    // 修正：比较要用 `==`，即 `if a == b {}`。
    let a = 1;
    let b = 2;
    println!("a == b 的结果 = {}", a == b);
    println!("// 预期输出：a == b 的结果 = false");

    // ===== 错误 5：类型推断失败时忘了标注 =====
    // let port = "8080".parse().unwrap();
    // error[E0284]: type annotations needed
    //   = note: cannot satisfy `<_ as FromStr>::Err == _`
    // 另一个真正报 E0282 的推断失败例子：
    // let values = Vec::new();
    // println!("{}", values.len());
    // error[E0282]: type annotations needed for `Vec<_>`
    // 修正：写成 `let port: u16 = "8080".parse().unwrap();`
    //       或 `let port = "8080".parse::<u16>().unwrap();`
    println!("提示：`parse()` 的目标类型无法推断时，必须显式标注或使用 turbofish 语法");
    println!(
        "// 预期输出：提示：`parse()` 的目标类型无法推断时，必须显式标注或使用 turbofish 语法"
    );

    // ===== edition 2024 注意点 =====
    // let gen = 1;
    // error: expected identifier, found reserved keyword `gen`
    // 修正：`gen` 在 edition 2024 是保留关键字，请改用 `generation` 之类的名字。
    let generation = 1;
    println!("保留字规避示例：generation = {generation}");
    println!("// 预期输出：保留字规避示例：generation = 1");

    println!();
}
