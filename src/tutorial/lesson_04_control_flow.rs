//! lesson_04_control_flow.rs —— 主题：流程控制
//!
//! 学习目标：
//!   1. 理解 `if` 是表达式，可以把分支结果直接赋给变量，并掌握 if / else if 链
//!   2. 分清 `loop`、`while`、`for` 三种循环的适用场景与 `break` / `continue` 的用法
//!   3. 会用循环标签（`'outer:`）精确控制多层嵌套循环，并实现"状态机 + 跳转表"
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_04_control_flow.rs -o lesson_04 && ./lesson_04
//!   或在本项目根目录执行：cargo run --bin lesson_04_control_flow
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

fn main() {
    // main 只负责按学习顺序调用各个 demo，本身不写业务逻辑。
    println!("========== lesson_04_control_flow：流程控制 ==========\n");

    demo_1_if_expression();
    demo_2_if_else_chain();
    demo_3_loop_while_for();
    demo_4_break_value_and_continue();
    demo_5_loop_labels();
    demo_6_typical_use_case();
    demo_7_common_mistakes();
}

/// 示例 1：if 是表达式
///
/// 要点：`if` 有值，两个分支必须返回同一类型，因此它可以直接参与赋值与运算。
fn demo_1_if_expression() {
    println!("--- 示例 1：if 是表达式 ---");

    let temperature = 28;

    // if/else 作为表达式使用：分支的值就是整个表达式的值。
    let suggestion = if temperature > 30 {
        "开空调"
    } else if temperature > 20 {
        "开窗通风"
    } else {
        "注意保暖"
    };
    println!("{temperature}℃ 的建议：{suggestion}");
    println!("// 预期输出：28℃ 的建议：开窗通风");

    // 两个分支类型必须一致：下面这种一边整数一边字符串的写法会报 E0308。
    // let bad = if temperature > 20 { 1 } else { "冷" };
    // error[E0308]: `if` and `else` have incompatible types

    // 条件本身就是 bool 表达式，不需要像 C 那样写 `!= 0`。
    let is_weekend = true;
    println!(
        "周末 = {is_weekend}，出门吗：{}",
        !is_weekend || temperature > 25
    );
    println!("// 预期输出：周末 = true，出门吗：true");

    // if 也能当表达式参与算术：这里把布尔分支折算成 0/1 计数。
    let mut coupon_count = 0;
    coupon_count += if is_weekend { 2 } else { 0 };
    println!("可用优惠券 = {coupon_count}");
    println!("// 预期输出：可用优惠券 = 2");

    // 没有 else 的 if 只能当语句用，它的值恒为 `()`。
    let mut log_level = "info";
    if temperature > 35 {
        log_level = "warn";
    }
    println!("日志级别 = {log_level}");
    println!("// 预期输出：日志级别 = info");

    println!();
}

/// 示例 2：if / else if 链与区间判断
///
/// 要点：多分支用 `else if` 串联，条件自上而下依次判断，命中第一个为真就结束。
fn demo_2_if_else_chain() {
    println!("--- 示例 2：if / else if 链 ---");

    // 成绩等级：从高到低判断，顺序非常关键。
    let score = 86;
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    };
    println!("分数 {score} 的等级是 {grade}");
    println!("// 预期输出：分数 86 的等级是 B");

    // 逻辑运算符用于组合条件：&& 与、|| 或、! 非。
    let is_weekday = true;
    let has_meeting = false;
    let need_commute = is_weekday && !has_meeting;
    println!("需要通勤 = {need_commute}");
    println!("// 预期输出：需要通勤 = true");

    // 区间判断用范围模式会更清晰，这里先用比较运算符展示等价写法。
    let hour = 14;
    let period = if (0..6).contains(&hour) {
        "凌晨"
    } else if (6..12).contains(&hour) {
        "上午"
    } else if (12..18).contains(&hour) {
        "下午"
    } else {
        "晚上"
    };
    println!("{hour} 点属于：{period}");
    println!("// 预期输出：14 点属于：下午");

    println!();
}

/// 示例 3：loop / while / for 三种循环
///
/// 要点：`loop` 用于"不确定次数、靠 break 退出"；`while` 用于条件驱动；
/// `for` 用于遍历序列，是 Rust 里最推荐、最不容易写错循环变量的写法。
fn demo_3_loop_while_for() {
    println!("--- 示例 3：loop / while / for ---");

    // ===== loop：无限循环 + break 退出 =====
    let mut attempts = 0;
    loop {
        attempts += 1;
        // 模拟"第三次尝试成功"，成功后立即 break 跳出。
        if attempts == 3 {
            break;
        }
    }
    println!("loop 共尝试 {attempts} 次后退出");
    println!("// 预期输出：loop 共尝试 3 次后退出");

    // ===== while：先判断条件，再决定是否执行循环体 =====
    let mut countdown = 3;
    let mut ticks = 0;
    while countdown > 0 {
        ticks += 1;
        countdown -= 1; // 必须让条件趋向结束，否则会死循环
    }
    println!(
        "while 循环执行了 {ticks} 次，倒计时归零 = {}",
        countdown == 0
    );
    println!("// 预期输出：while 循环执行了 3 次，倒计时归零 = true");

    // while 的经典用途：不断从序列末尾取出元素。
    let mut queue = vec!["任务A", "任务B", "任务C"];
    let mut handled = String::new();
    while let Some(task) = queue.pop() {
        // pop 从尾部弹出，所以处理顺序是反的。
        handled.push_str(task);
        handled.push(' ');
    }
    println!("处理顺序：{handled}");
    println!("// 预期输出：处理顺序：任务C 任务B 任务A ");

    // ===== for：遍历范围或集合，由迭代器管理边界 =====
    let mut sum = 0;
    for value in 1..=5 {
        // 1..=5 是包含 5 的闭区间；1..5 则是左闭右开区间。
        sum += value;
    }
    println!("1..=5 求和 = {sum}");
    println!("// 预期输出：1..=5 求和 = 15");

    let fruits = ["苹果", "香蕉", "橘子"];
    for (index, fruit) in fruits.iter().enumerate() {
        // enumerate 同时给出下标和元素，比手写计数器安全。
        println!("第 {} 个水果：{fruit}", index + 1);
        // 三个水果的期望输出写成硬编码字面量，便于逐字符核对。
        let expected = match index {
            0 => "// 预期输出：第 1 个水果：苹果",
            1 => "// 预期输出：第 2 个水果：香蕉",
            _ => "// 预期输出：第 3 个水果：橘子",
        };
        println!("{expected}");
    }

    // 倒序与步长：用 rev 和 step_by 组合。
    let mut descending = String::new();
    for value in (1..=5).rev() {
        descending.push_str(&value.to_string());
    }
    println!("倒序拼接 = {descending}");
    println!("// 预期输出：倒序拼接 = 54321");

    println!();
}

/// 示例 4：break 带值 与 continue
///
/// 要点：`loop` 的 `break` 可以带一个值作为整个循环表达式的值；
/// `continue` 立即结束本轮、进入下一轮。
fn demo_4_break_value_and_continue() {
    println!("--- 示例 4：break 带值 与 continue ---");

    // loop 是表达式，break value 就是它的值：这就是"带返回值的循环"。
    let mut number = 1;
    let first_multiple_of_7 = loop {
        if number % 7 == 0 {
            // break 把找到的值交回给外层变量。
            break number;
        }
        number += 1;
    };
    println!("第一个 7 的倍数是 {first_multiple_of_7}");
    println!("// 预期输出：第一个 7 的倍数是 7");

    // break 也可以带元组，一次返回多个结果。
    let (found_index, found_value) = {
        let data = [3, 8, 15, 22, 9];
        let mut index = 0;
        loop {
            if data[index] > 20 {
                break (index, data[index]);
            }
            index += 1;
        }
    };
    println!("第一个大于 20 的元素：下标 {found_index}，值 {found_value}");
    println!("// 预期输出：第一个大于 20 的元素：下标 3，值 22");

    // continue：跳过偶数，只累加奇数。
    let mut odd_sum = 0;
    for value in 1..=10 {
        if value % 2 == 0 {
            // 立即进入下一轮，后面的累加不会执行。
            continue;
        }
        odd_sum += value;
    }
    println!("1..=10 中所有奇数之和 = {odd_sum}");
    println!("// 预期输出：1..=10 中所有奇数之和 = 25");

    // while 中同样可以用 break 带值吗？不能——只有 loop 支持 break 返回值。
    // let value = while false { break 1; };
    // error[E0571]: `break` with value from a `while` loop

    println!();
}

/// 示例 5：循环标签（loop label）
///
/// 要点：给循环加 `'name:` 标签后，`break 'name` / `continue 'name` 就能
/// 精确作用于外层循环，避免用布尔标志位"绕圈"退出。
fn demo_5_loop_labels() {
    println!("--- 示例 5：循环标签 ---");

    // 在二维网格里找第一对和为目标值的坐标：找到就一次性跳出两层循环。
    // 行、列都只有 0..4，两数之和最大是 3 + 3 = 6，所以这个目标值取不到，
    // 结果一定是 None —— 这正好演示"循环正常跑完"的分支。
    let target_sum = 11;
    let mut found = None;

    'outer: for row in 0..4 {
        for column in 0..4 {
            if row + column == target_sum {
                // 直接跳出外层循环，label 让"一次跳两层"变得干净。
                found = Some((row, column));
                break 'outer;
            }
        }
    }
    println!("和为 {target_sum} 的坐标 = {found:?}");
    println!("// 预期输出：和为 11 的坐标 = None");

    // continue 带标签：跳过外层循环的当前轮，而不是只跳过内层。
    let mut collected = String::new();
    'rows: for row in 1..=3 {
        for column in 1..=3 {
            if row == column {
                // 遇到对角线就放弃整行剩余元素（这是带标签 continue 的语义）。
                continue 'rows;
            }
            collected.push_str(&format!("{row}{column} "));
        }
    }
    println!("非对角线元素：{collected}");
    println!("// 预期输出：非对角线元素：21 31 32 ");

    // 带标签的 while 循环同样有效。
    let mut outer_count = 0;
    'counting: while outer_count < 3 {
        outer_count += 1;
        if outer_count == 2 {
            continue 'counting;
        }
    }
    println!("带标签的 while 执行完毕，outer_count = {outer_count}");
    println!("// 预期输出：带标签的 while 执行完毕，outer_count = 3");

    println!();
}

/// 示例 6：典型使用场景 —— 循环 + 条件分支实现任务状态机
///
/// 要点：真实业务里状态流转就是"遍历事件表 + match/if 决定下一个状态"，
/// 循环负责驱动，`match` 负责分派（`match` 的完整用法见 lesson 07）。
fn demo_6_typical_use_case() {
    println!("--- 示例 6：典型使用场景（任务状态机 + 表格遍历） ---");

    // 事件序列：合法事件推动状态前进，非法事件被拒绝并保持原状态。
    let events = [
        "start", "process", "pause", "process", "resume", "process", "finish", "boom",
    ];

    // 状态变量必须可变：每一轮循环都可能被改写。
    // 初值是字符串字面量（`&'static str`），与 next_state 的返回类型一致。
    let mut state: &'static str = "idle";
    let mut trace = String::new();

    // 每个事件对应的期望输出（硬编码字面量，便于逐行核对状态流转）。
    let expected_states = [
        "// 预期输出：事件 start    后状态变为 running",
        "// 预期输出：事件 process  后状态变为 running",
        "// 预期输出：事件 pause    后状态变为 paused",
        "// 预期输出：事件 process  后状态变为 paused",
        "// 预期输出：事件 resume   后状态变为 running",
        "// 预期输出：事件 process  后状态变为 running",
        "// 预期输出：事件 finish   后状态变为 finished",
        "// 预期输出：事件 boom     后状态变为 finished",
    ];

    for (index, event) in events.iter().enumerate() {
        // 状态 + 事件 -> 下一个状态，这就是一条"跳转表"。
        state = next_state(state, event);
        trace.push_str(state);
        trace.push_str(" -> ");
        println!("事件 {event:<8} 后状态变为 {state}");
        println!("{}", expected_states[index]);
    }

    // 去掉末尾多余的 " -> "：链式方法调用这里只用到最基础的 pop/truncate。
    let trace_len = trace.len();
    trace.truncate(trace_len - 4);
    println!("完整状态轨迹：{trace}");
    println!(
        "// 预期输出：完整状态轨迹：running -> running -> paused -> paused -> running -> running -> finished -> finished"
    );

    // 用 if 链把最终状态翻译成人能读懂的结论。
    let conclusion = if state == "finished" {
        "任务已完成"
    } else if state == "running" {
        "任务进行中"
    } else {
        "任务未开始或被暂停"
    };
    println!("结论：{conclusion}");
    println!("// 预期输出：结论：任务已完成");

    // 第二个场景：用嵌套循环 + 条件生成乘法表（控制流驱动输出格式）。
    let mut table = String::new();
    for row in 1..=3 {
        for column in 1..=3 {
            // 只在每行最后一个元素后换行，否则补空格分隔。
            if column == 3 {
                table.push_str(&format!("{:>4}\n", row * column));
            } else {
                table.push_str(&format!("{:>4}", row * column));
            }
        }
    }
    println!("3×3 乘法表：");
    println!("// 预期输出：3×3 乘法表：");
    print!("{table}");
    println!("// 预期输出：   1   2   3");
    println!("// 预期输出：   2   4   6");
    println!("// 预期输出：   3   6   9");

    println!();
}

/// 示例 7：常见错误示例（错误代码全部注释掉）
///
/// 要点：break 带值只能用在 `loop`、忘记改变循环条件会死循环、
/// 嵌套循环里误用无标签 break，是流程控制章节的高频坑。
fn demo_7_common_mistakes() {
    println!("--- 示例 7：常见错误示例（错误代码已注释） ---");

    // ===== 错误 1：把 loop 的值丢掉又忘记 break 带值 =====
    // let value = loop { 1 };
    // error[E0308]: mismatched types
    //   expected `()`, found integer（loop 体最后是表达式但没有 break 带值）
    //   help: you might have meant to break the loop with this value
    // warning: unreachable statement（loop 永不 break，所以它之后的语句不可达）
    // 修正：写成 `let value = loop { break 1; };`
    let value = loop {
        // break 带值才是 loop 表达式的值。
        break 1;
    };
    println!("修正后 loop 的值 = {value}");
    println!("// 预期输出：修正后 loop 的值 = 1");

    // ===== 错误 2：while 循环条件永远不变（死循环） =====
    // let mut i = 0;
    // while i < 3 {
    //     println!("{i}");
    // }
    // 编译通过但没有输出结尾：程序会一直循环（死循环，不是编译错误）
    // 修正：在循环体内改变条件变量，例如 `i += 1;`。
    let mut i = 0;
    while i < 3 {
        i += 1; // 修正：让循环趋向终止
    }
    println!("修正后 i = {i}");
    println!("// 预期输出：修正后 i = 3");

    // ===== 错误 3：while 循环里用 break 带值 =====
    // let n = while i < 5 { break 1; };
    // error[E0571]: `break` with value from a `while` loop
    //   详情文本：can only break with a value inside `loop` or breakable block
    //             you can't `break` with a value in a `while` loop
    //   help: use `break` on its own without a value inside this `while` loop
    // 修正：改用 loop + break value，或用普通变量在循环外接收结果。
    let mut n = 0;
    while i < 5 {
        n = 1;
        break;
    }
    println!("修正后 n = {n}");
    println!("// 预期输出：修正后 n = 1");

    // ===== 错误 4：嵌套循环里无标签 break 只跳出内层 =====
    // for row in 0..3 {
    //     for column in 0..3 {
    //         if row + column == 2 { break; } // 只跳出内层，外层继续跑
    //     }
    // }
    // 这是逻辑错误：想一次跳出两层必须给外层加标签并写 `break 'outer;`。
    let mut inner_breaks = 0;
    'outer_check: for row in 0..3 {
        for column in 0..3 {
            if row + column == 2 {
                inner_breaks += 1;
                break 'outer_check; // 修正：带标签才跳出外层
            }
        }
    }
    println!("带标签 break 之后只跳出了一次：inner_breaks = {inner_breaks}");
    println!("// 预期输出：带标签 break 之后只跳出了一次：inner_breaks = 1");

    // ===== 错误 5：忘记给 if 条件提供 bool =====
    // let x = 1;
    // if x { }
    // error[E0308]: mismatched types
    //   expected `bool`, found integer
    // 修正：写成 `if x != 0 { }`，Rust 不会把整数当作真值。
    let x = 1;
    println!("修正后判断 x != 0 的结果 = {}", x != 0);
    println!("// 预期输出：修正后判断 x != 0 的结果 = true");

    // ===== 错误 6：循环变量在循环外被误用 =====
    // for index in 0..3 { }
    // println!("{index}");
    // error[E0425]: cannot find value `index` in this scope
    // 修正：把结果存到循环外声明的变量里，例如 `let mut last_index = 0;`。
    let mut last_index = 0;
    for index in 0..3 {
        last_index = index;
    }
    println!("修正后拿到循环结束时的 last_index = {last_index}");
    println!("// 预期输出：修正后拿到循环结束时的 last_index = 2");

    // ===== 错误 7：break 之后的语句不可达 =====
    // loop {
    //     break;
    //     println!("永远不会执行");
    // }
    // warning: unreachable statement
    //   = note: `#[warn(unreachable_code)]` (part of `#[warn(unused)]`) on by default
    println!("提示：break 之后的同级语句不可达，编译器会警告 unreachable_code");
    println!("// 预期输出：提示：break 之后的同级语句不可达，编译器会警告 unreachable_code");

    println!();
}

/// 状态机跳转表：根据"当前状态 + 事件"返回下一个状态。
///
/// 所有状态都是字符串字面量（类型 `&'static str`），因此这次改编
/// 不需要写生命周期参数（生命周期是 lesson 13 的内容）。
/// 非法事件不会改变状态，这样调用方无需额外判断就能安全遍历事件流。
fn next_state(current: &'static str, event: &str) -> &'static str {
    match (current, event) {
        // 空闲状态只接受 start。
        ("idle", "start") => "running",
        // 运行中可以暂停或结束。
        ("running", "pause") => "paused",
        ("running", "finish") => "finished",
        // 暂停后可以恢复。
        ("paused", "resume") => "running",
        // 其余组合（例如 process、boom 等不改变状态的事件）保持原状态。
        _ => current,
    }
}
