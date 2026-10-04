//! lesson_05_ownership_borrowing.rs —— 主题：所有权系统（Ownership）
//!
//! 学习目标：
//!   1. 记住所有权的三条规则，理解 move 之后原变量为什么不能再使用。
//!   2. 分清 Copy 与 Clone：哪些类型是"按位复制"，哪些必须显式克隆。
//!   3. 看懂栈与堆的差别，知道 String 这种"胖指针"到底把数据放在哪里。
//!   4. 掌握引用与借用规则（同一时刻：多个不可变引用 或 仅一个可变引用）。
//!   5. 会用 NLL（非词法生命周期）解释"借用结束即可再用"。
//!   6. 会写返回引用的切片函数，并理解 `&str` 与 `&[T]` 的本质。
//!   7. 知道值离开作用域时会发生 Drop，能读懂常见的所有权编译错误。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_05_ownership_borrowing.rs -o lesson_05 && ./lesson_05
//!   或在本项目根目录执行：cargo run --bin lesson_05_ownership_borrowing
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

fn main() {
    println!("========== lesson_05_ownership_borrowing：所有权系统 ==========\n");

    demo_1_three_rules();
    demo_2_move_vs_copy();
    demo_3_clone_explicit();
    demo_4_stack_and_heap();
    demo_5_borrow_rules();
    demo_6_mutable_borrow();
    demo_7_nll_non_lexical_lifetime();
    demo_8_slices();
    demo_9_typical_scenario_longest_word();
    demo_10_common_mistakes();
}

/// 示例 1：所有权的三条规则
///
/// 要点：每个值有且只有一个所有者；同一时刻只能有一个所有者；
/// 所有者离开作用域时值被丢弃（drop），这正是 Rust 不需要 GC 的原因。
fn demo_1_three_rules() {
    println!("--- 示例 1：所有权的三条规则 ---");

    // 规则一：每个值都有一个"所有者"变量。
    // 这里的 String 值由变量 owner 拥有。
    let owner = String::from("Rust");
    println!("owner = {owner}");
    println!("// 预期输出：owner = Rust");

    {
        // 规则二：同一时刻只能有一个所有者；进入新作用域后，新的变量接管所有权。
        // 把 owner 交给 inner：这是一次 move（移动），不是复制。
        let inner = owner;
        println!("inner = {inner}");
        println!("// 预期输出：inner = Rust");

        // 此时 owner 已经"失效"：值归 inner 所有，owner 不再可用。
        // 取消下面这行注释会报：error[E0382]: borrow of moved value: `owner`
        // println!("{owner}");
    }
    // 规则三：作用域结束时，inner 离开作用域，它拥有的 String 被 drop，
    // 底层堆内存被自动释放。drop 是确定性的（编译期就知道何时执行）。
    // 注意：这一步没有任何输出 —— 释放是"静默"发生的，正是这一点让 Rust 既安全又零开销。
    println!("（inner 已在上面那个块结束时被释放，此处没有任何输出）");
}

/// 示例 2：move 与 Copy —— 同样是赋值，行为完全不同
///
/// 要点：是否"交出所有权"取决于类型有没有实现 `Copy` trait。
fn demo_2_move_vs_copy() {
    println!("--- 示例 2：move 与 Copy ---");

    // ① String 没有实现 Copy：赋值是 move，源变量立即失效。
    let s1 = String::from("hello");
    let s2 = s1; // move：s1 的数据（栈上指针 + 堆上字节）整体移交
    println!("s2 = {s2}");
    println!("// 预期输出：s2 = hello");
    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`

    // ② 整数实现了 Copy：赋值是按位复制，两个变量各自有效。
    let n1 = 42;
    let n2 = n1; // copy：只是把 4 字节整数复制一份
    println!("n1 = {n1}, n2 = {n2}");
    println!("// 预期输出：n1 = 42, n2 = 42");

    // ③ 函数传参同样遵循"移动/复制"规则。
    takes_ownership(s2); // s2 被移进函数，函数返回时值在函数内被 drop
    // println!("{s2}");  // error[E0382]: borrow of moved value: `s2`
    makes_copy(n2); // n2 是 Copy 类型，传参只复制，n2 之后仍可用
    println!("调用 takes_ownership 之后 s2 已失效，而 makes_copy 之后 n2 仍然可用：n2 = {n2}");
    println!(
        "// 预期输出：调用 takes_ownership 之后 s2 已失效，而 makes_copy 之后 n2 仍然可用：n2 = 42"
    );

    // ④ 元组（以及数组、共享引用等）在"元素全是 Copy"时整体也就是 Copy。
    let pair = (1, true);
    let pair_copy = pair;
    println!("pair = {pair:?}, pair_copy = {pair_copy:?}");
    println!("// 预期输出：pair = (1, true), pair_copy = (1, true)");

    // ⑤ Copy 与 Clone 的关系：标准库里写的是 `trait Copy: Clone`，
    // 即 Copy 是 Clone 的"子 trait"（supertrait）——想实现 Copy 必须先实现 Clone。
    // 推论一：任何 Copy 类型都自动拥有 .clone()，对 n2、pair 这类值调用 .clone() 完全合法；
    // 推论二：派生时必须一起写 `#[derive(Copy, Clone)]`，只写 Copy 会因缺少 Clone 而编译失败。
}

/// 示例 3：Clone —— 想要"保留原变量"就得显式深拷贝
///
/// 要点：`Clone` 提供 `.clone()`，它是显式动作；Rust 不会偷偷帮你深拷贝。
fn demo_3_clone_explicit() {
    println!("--- 示例 3：Clone 显式深拷贝 ---");

    let original = String::from("data");
    // clone() 在堆上重新申请一块内存并复制内容，于是两个变量各自拥有独立数据。
    let duplicated = original.clone();
    println!("original = {original}, duplicated = {duplicated}");
    println!("// 预期输出：original = data, duplicated = data");

    // 两个变量互不影响：修改 duplicated 不会动到 original。
    let mut changed = duplicated.clone();
    changed.push_str("!!!");
    println!("changed = {changed}, duplicated = {duplicated}");
    println!("// 预期输出：changed = data!!!, duplicated = data");

    // Vec 也是 Clone 的：克隆一整条向量（元素为 String 时会逐元素克隆）。
    let words = vec![original.clone(), String::from("world")];
    let words_clone = words.clone();
    println!("words = {words:?}, words_clone = {words_clone:?}");
    println!("// 预期输出：words = [\"data\", \"world\"], words_clone = [\"data\", \"world\"]");

    // 提示：对 String / Vec 这类"自己拥有堆数据"的类型，clone 会真实地重新分配堆内存
    // 并逐字节复制内容，所以它不是零成本操作；至于 i32、bool 这些 Copy 类型，
    // clone 只是按位复制一个栈上的值，代价可以忽略（见示例 2 的 Copy 与示例 3 的对比）。
    println!(
        "提示：对 String/Vec 这类拥有堆数据的类型，clone 会真实分配并复制堆内存，不是零成本操作。"
    );
}

/// 示例 4：栈与堆 —— String 的"胖指针"到底存在哪
///
/// 要点：栈上的 String 只存 ptr/len/capacity 三个字段，字符字节放在堆上；
/// 借用（&String → &str）指向的正是那块堆内存。
fn demo_4_stack_and_heap() {
    println!("--- 示例 4：栈与堆 ---");

    // String::from 在堆上分配内存存放 UTF-8 字节，栈上只留一个"胖指针"结构。
    let heap_str = String::from("你好, Rust");
    println!(
        "字节数 len() = {}, 字符数 chars().count() = {}",
        heap_str.len(),
        heap_str.chars().count()
    );
    println!("// 预期输出：字节数 len() = 12, 字符数 chars().count() = 8");

    // 借用得到的 &str 指向堆上那块内存，本身不拥有数据。
    let borrowed: &str = heap_str.as_str();
    println!("borrowed（指向堆内存，不拥有数据）= {borrowed}");
    println!("// 预期输出：borrowed（指向堆内存，不拥有数据）= 你好, Rust");

    // 完全放在栈上的固定大小数组：整体随作用域自动回收，零堆分配。
    let stack_arr: [i32; 3] = [1, 2, 3];
    println!("stack_arr = {stack_arr:?}");
    println!("// 预期输出：stack_arr = [1, 2, 3]");

    // 作用域演示：Box/Vec 这类拥有堆内存的值在离开作用域时自动 drop。
    {
        let scoped = vec![String::from("临时"), String::from("数据")];
        println!("作用域内 scoped = {scoped:?}");
        println!("// 预期输出：作用域内 scoped = [\"临时\", \"数据\"]");
    } // 此处 scoped 被 drop：Vec 的每个 String 释放堆内存，Vec 自身释放缓冲区
    println!("离开花括号后 scoped 已被 drop，其堆内存自动释放");
    println!("// 预期输出：离开花括号后 scoped 已被 drop，其堆内存自动释放");
}

/// 示例 5：引用与借用规则（不可变借用）
///
/// 要点：`&T` 是借用，不取得所有权；同一时刻允许多个不可变引用共存。
fn demo_5_borrow_rules() {
    println!("--- 示例 5：引用与借用规则 ---");

    let text = String::from("borrow me");

    // 借用不移动所有权：text 依然是所有者，函数返回后还能继续用。
    let length = str_len(&text);
    println!("str_len(&text) = {length}, text 仍然可用 = {text}");
    println!("// 预期输出：str_len(&text) = 9, text 仍然可用 = borrow me");

    // 同一作用域内可以同时存在多个不可变引用（只读共享，安全）。
    let a: &String = &text;
    let b: &String = &text;
    println!("a = {a}, b = {b}（两个不可变借用同时存在是合法的）");
    println!("// 预期输出：a = borrow me, b = borrow me（两个不可变借用同时存在是合法的）");

    // 把引用交给函数，函数用完即归还借用，text 的所有权从未改变。
    let first = first_char(&text);
    println!("first_char(&text) = {first}");
    println!("// 预期输出：first_char(&text) = b");

    // 借用检查器只允许"读"：
    // text.push_str("!");  // error[E0596]: cannot borrow `text` as mutable, as it is not declared as mutable
    println!("借用期间所有者依然可以继续被共享读取，改值则必须走可变引用");
    println!("// 预期输出：借用期间所有者依然可以继续被共享读取，改值则必须走可变引用");
}

/// 示例 6：可变引用
///
/// 要点：`&mut T` 有两条硬规则 —— 同一时刻只能有一个可变引用；
/// 有可变引用时，不能再有任何不可变引用。
fn demo_6_mutable_borrow() {
    println!("--- 示例 6：可变引用 ---");

    let mut greeting = String::from("hello");
    // 把可变引用（可写的"钥匙"）交给函数，函数在借用期内可以直接修改。
    append_exclamation(&mut greeting);
    println!("append_exclamation 之后 greeting = {greeting}");
    println!("// 预期输出：append_exclamation 之后 greeting = hello!");

    // 通过可变引用原地修改：注意是 `*r = ...`，解引用后赋值。
    let mut value = 10;
    {
        let r: &mut i32 = &mut value;
        *r += 5;
    } // r 的借用在这里结束
    println!("通过 &mut 修改后 value = {value}");
    println!("// 预期输出：通过 &mut 修改后 value = 15");

    // 可变借用结束后，又可以自由地共享借用。
    let reading = &value;
    println!("借用结束后可再次共享借用：reading = {reading}");
    println!("// 预期输出：借用结束后可再次共享借用：reading = 15");

    // 取消注释下面两行会报：error[E0499]: cannot borrow `value` as mutable more than once
    // let r1 = &mut value;
    // let r2 = &mut value;
    println!("同一时刻两个可变引用是非法的：error[E0499]: cannot borrow as mutable more than once");
    println!(
        "// 预期输出：同一时刻两个可变引用是非法的：error[E0499]: cannot borrow as mutable more than once"
    );
}

/// 示例 7：NLL（非词法生命周期）
///
/// 要点：借用的"有效范围"由最后一次使用决定，而不是由花括号决定；
/// 所以只要旧引用不再被使用，新借用就可以立刻开始。
fn demo_7_nll_non_lexical_lifetime() {
    println!("--- 示例 7：NLL 非词法生命周期 ---");

    let mut nums = vec![1, 2, 3];

    // NLL 之前，可变借用会一直持续到块结束；有了 NLL，借用在最后一次使用处就结束。
    nums.push(4); // 第一次可变借用：在这一行用完就结束
    nums.push(5); // 第二次可变借用：因为上一次已结束，所以完全合法
    println!("经过两次 push 后 nums = {nums:?}");
    println!("// 预期输出：经过两次 push 后 nums = [1, 2, 3, 4, 5]");

    // 先可变借用，用完立刻不可变借用：NLL 让这两件事不再冲突。
    if let Some(last) = nums.last_mut() {
        // last 是 &mut i32：通过它原地修改最后一个元素。
        *last *= 10; // 这个可变借用在 if let 块结束时即告结束
    }
    let snapshot: &Vec<i32> = &nums; // 旧借用已结束，这里可以共享借用
    println!("snapshot = {snapshot:?}");
    println!("// 预期输出：snapshot = [1, 2, 3, 4, 50]");

    // 借用在一个 if 里用完即可释放，之后仍能移动所有权。
    if let Some(max) = nums.iter().max() {
        println!("最大值（借用期内只读）= {max}");
        println!("// 预期输出：最大值（借用期内只读）= 50");
    }
    let total: i32 = nums.iter().sum(); // 新的不可变借用，与前一个借出不重叠
    println!("总和 total = {total}");
    println!("// 预期输出：总和 total = 60");

    // 借用彻底结束之后，移动（move）也是允许的。
    let moved = nums;
    println!("借用结束，可以安全 move：moved = {moved:?}");
    println!("// 预期输出：借用结束，可以安全 move：moved = [1, 2, 3, 4, 50]");
}

/// 示例 8：切片 `&str` 与 `&[T]`
///
/// 要点：切片是"指向序列一部分的借用视图"，带长度、不带所有权。
fn demo_8_slices() {
    println!("--- 示例 8：切片 &str 与 &[T] ---");

    let sentence = String::from("hello world");

    // 字符串切片：&str 就是"某段 UTF-8 字节的借用视图"。
    let hello: &str = &sentence[0..5];
    let world: &str = &sentence[6..];
    println!("hello = {hello}, world = {world}");
    println!("// 预期输出：hello = hello, world = world");

    // 切片本身是借用：sentence 仍是所有者，切片不负责释放内存。
    println!("sentence 依然可用：{sentence}");
    println!("// 预期输出：sentence 依然可用：hello world");

    // 非 ASCII 字符串要注意：切片下标是"字节下标"，切到字符中间会 panic。
    let chinese = String::from("你好世界");
    let first_two = &chinese[0..6]; // 一个汉字 3 字节，取前两个汉字
    println!("chinese 的前两个汉字 = {first_two}");
    println!("// 预期输出：chinese 的前两个汉字 = 你好");

    // 数组/向量的切片 &[T]：只读视图，可再切片、可求和。
    let numbers = [10, 20, 30, 40, 50];
    let middle: &[i32] = &numbers[1..4];
    println!("numbers[1..4] = {middle:?}，其和 = {}", sum_slice(middle));
    println!("// 预期输出：numbers[1..4] = [20, 30, 40]，其和 = 90");

    // 切片的"胖指针"本质：包含数据地址 + 长度两部分。
    println!(
        "middle.len() = {}，middle 指向的是原数组的一部分（借用，不拷贝）",
        middle.len()
    );
    println!("// 预期输出：middle.len() = 3，middle 指向的是原数组的一部分（借用，不拷贝）");

    // &str 是 &[u8] 的"保证合法 UTF-8"版本，所以字符串切片比字节切片更安全。
    let bytes_view: &[u8] = hello.as_bytes();
    println!(
        "hello.as_bytes() 长度 = {}（&str 可以用 as_bytes 看到底层字节）",
        bytes_view.len()
    );
    println!("// 预期输出：hello.as_bytes() 长度 = 5（&str 可以用 as_bytes 看到底层字节）");
}

/// 示例 9：典型使用场景 —— 取句子中最长的单词
///
/// 要点：函数返回 `&str`（句子的切片）而不是 String，
/// 因为"最长单词"本来就是原句子的一部分，无需复制、无需转移所有权。
fn demo_9_typical_scenario_longest_word() {
    println!("--- 示例 9：典型使用场景 —— 取最长单词 ---");

    // 只要参数是 &str，String 和字符串字面量都能传进来（自动解引用强转）。
    let owned = String::from("Rust ownership makes memory safety possible");
    let longest_owned = longest_word(&owned);
    println!("owned 里的最长单词 = {longest_owned}");
    println!("// 预期输出：owned 里的最长单词 = ownership");

    let literal = "keep it simple and readable";
    let longest_literal = longest_word(literal);
    println!("literal 里的最长单词 = {longest_literal}");
    println!("// 预期输出：literal 里的最长单词 = readable");

    // 返回值是句子的切片：可以继续在原句子上做别的事，
    // 但注意此时 owned 处于不可变借用中，不能移动或可变借用它。
    let stats = word_count(&owned);
    println!("owned 的单词数 = {stats}，最长单词依然是 {longest_owned}");
    println!("// 预期输出：owned 的单词数 = 6，最长单词依然是 ownership");

    // 借用结束后，owned 依然可以被移动，说明切片从未取得所有权。
    let take_over = owned;
    println!("切片借用结束后 owned 仍可被移动：take_over = {take_over}");
    println!(
        "// 预期输出：切片借用结束后 owned 仍可被移动：take_over = Rust ownership makes memory safety possible"
    );
}

/// 示例 10：常见错误示例（错误代码全部注释掉，只保留修正后的写法）
///
/// 要点：读懂这三类所有权报错，就能解决 90% 的初学者编译失败。
fn demo_10_common_mistakes() {
    println!("--- 示例 10：常见错误示例 ---");

    // 错误 1：move 之后继续使用原变量。
    // let s = String::from("x");
    // let t = s;
    // println!("{s}"); // error[E0382]: borrow of moved value: `s`
    // 修正方法一：确实需要两份数据时显式 clone。
    // let t = s.clone();
    // 修正方法二：只是想读，就借出去而不是移动。
    // let t = &s;
    let s = String::from("x");
    let t = s.clone();
    println!("修正 1：用 clone 保留原变量 -> s = {s}, t = {t}");
    println!("// 预期输出：修正 1：用 clone 保留原变量 -> s = x, t = x");

    // 错误 2：同一时刻两个可变借用。
    // let mut v = vec![1];
    // let r1 = &mut v;
    // let r2 = &mut v;       // error[E0499]: cannot borrow `v` as mutable more than once at a time
    // println!("{r1:?} {r2:?}");
    // 修正方法：缩短第一个借用的生命周期，用完再借第二次。
    let mut v = vec![1];
    {
        let r1 = &mut v;
        r1.push(2);
    }
    let r2 = &mut v;
    r2.push(3);
    println!("修正 2：串行使用可变借用 -> v = {v:?}");
    println!("// 预期输出：修正 2：串行使用可变借用 -> v = [1, 2, 3]");

    // 错误 3：不可变借用与可变借用重叠。
    // let mut w = vec![1, 2, 3];
    // let first = &w[0];
    // w.push(4);                 // error[E0502]: cannot borrow `w` as mutable because it is also borrowed as immutable
    // println!("{first}");
    // 修正方法：先把借用得到的数据取出来（或复制成自有值），再修改原集合。
    let mut w = vec![1, 2, 3];
    let first_value = w[0]; // i32 是 Copy，直接取出值，借用立即结束
    w.push(4);
    println!("修正 3：先复制出值再修改 -> first_value = {first_value}, w = {w:?}");
    println!("// 预期输出：修正 3：先复制出值再修改 -> first_value = 1, w = [1, 2, 3, 4]");

    // 错误 4：把局部变量作为引用返回（引用的生命周期比数据长）。
    // 4a. 返回类型直接写 &String：连生命周期都无法确定，先报"缺少生命周期标注"。
    // fn dangling() -> &String {
    //     let local = String::from("oops");
    //     &local
    // }
    // error[E0106]: missing lifetime specifier
    //   --> src/main.rs:1:18
    //    | fn dangling() -> &String {
    //    |                  ^ expected named lifetime parameter
    // 4b. 硬把返回类型写成 &'static String，先过了 4a 这一关，才轮到"借用活得不够久"。
    // fn dangling() -> &'static String {
    //     let local = String::from("oops");
    //     &local
    // }
    // error[E0515]: cannot return reference to local variable `local`
    //   |     &local
    //   |     ^^^^^^ returns a reference to data owned by the current function
    // 两个错误是"先后两道关"：4a 解决（补上生命周期）后才会看到 4b。
    // 修正方法：返回所有权（String），或者返回由参数派生出的切片（如示例 9）。
    println!("修正 4：需要把数据带出函数时返回 String 所有权，而不是返回局部变量的引用");
    println!(
        "// 预期输出：修正 4：需要把数据带出函数时返回 String 所有权，而不是返回局部变量的引用"
    );

    // 错误 5：想通过不可变引用修改数据。
    // let r = &w;
    // r.push(5); // error[E0596]: cannot borrow `*r` as mutable, as it is behind a `&` reference
    // 修正方法：把绑定声明为可变并改用 &mut。
    let mut m = vec![1];
    let r = &mut m;
    r.push(5);
    println!("修正 5：改用 &mut 借用 -> m = {m:?}");
    println!("// 预期输出：修正 5：改用 &mut 借用 -> m = [1, 5]");
}

/// 工具函数：借用字符串并返回其字节长度（不取得所有权）。
fn str_len(s: &String) -> usize {
    s.len()
}

/// 工具函数：按值接收 String，取得其所有权（调用后实参失效）。
fn takes_ownership(s: String) {
    // 函数结束时 s 离开作用域，它拥有的堆内存被自动释放。
    println!("takes_ownership 收到并接管了：{s}");
}

/// 工具函数：按值接收 i32；因为 i32 是 Copy 类型，传参只是复制一份。
fn makes_copy(n: i32) {
    println!("makes_copy 收到了一份副本：{n}");
}

/// 工具函数：借用字符串并返回首个字符。
fn first_char(s: &String) -> char {
    // chars() 按 Unicode 标量值遍历，比按字节切片安全。
    s.chars().next().unwrap_or('\0')
}

/// 工具函数：可变借用字符串，在原值后面追加内容。
fn append_exclamation(s: &mut String) {
    s.push('!'); // 通过可变引用原地修改被借用的数据
}

/// 工具函数：接收切片并求和（&[i32] 能同时接受数组切片和 Vec 切片）。
fn sum_slice(values: &[i32]) -> i32 {
    values.iter().sum()
}

/// 典型场景核心函数：返回句子中最长的那个单词切片。
///
/// 返回 `&str` 而不是 `String`：结果本来就借用自入参，
/// 交给调用者一个"视图"即可，既不复制也不转移所有权。
fn longest_word(sentence: &str) -> &str {
    let mut longest = "";
    for word in sentence.split_whitespace() {
        // 严格大于才替换：长度相同时保留最先出现的单词，行为可预测。
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}

/// 工具函数：统计句子的单词数。
fn word_count(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}
