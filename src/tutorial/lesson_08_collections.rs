//! lesson_08_collections.rs —— 主题：常见集合及操作（Vec / String / HashMap）
//!
//! 学习目标：
//!   1. 掌握 `Vec<T>` 的创建、增删、索引、遍历，以及容量与重新分配的基本概念
//!   2. 分清 `String` 与 `&str` 的关系，理解 UTF-8 编码下「字节」与「字符」的差异
//!   3. 掌握字符串拼接（`push_str` / `+` / `format!`）各自的所有权语义
//!   4. 掌握 `HashMap` 的插入、查询、更新、遍历，以及 `entry` API 避免重复查询的价值
//!   5. 能识别集合与所有权/借用交互时的典型编译错误并写出修正版本
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_08_collections.rs -o lesson_08 && ./lesson_08
//!   或在本项目根目录执行：cargo run --bin lesson_08_collections
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

use std::collections::HashMap;

fn main() {
    println!("========== lesson_08_collections：常见集合及操作 ==========\n");

    demo_1_vec_create_and_push();
    demo_2_vec_access_and_modify();
    demo_3_vec_iterate_and_capacity();
    demo_4_string_basics_and_utf8();
    demo_5_string_concat_and_format();
    demo_6_hashmap_basics();
    demo_7_hashmap_entry_api();
    demo_8_word_frequency_scenario();
    demo_9_ownership_and_borrow_traps();
    demo_10_common_mistakes();
}

/// 示例 1：Vec 的创建与增删
///
/// 要点：`Vec::new`、`vec![]`、`Vec::with_capacity` 三种创建方式，
/// 以及 `push` / `pop` / `insert` / `remove` / `clear` 的语义差异。
fn demo_1_vec_create_and_push() {
    println!("--- 示例 1：Vec 的创建与增删 ---");

    // 方式一：Vec::new 创建空向量，类型由后续 push 或类型标注推断
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("push 之后 numbers = {numbers:?}");
    println!("// 预期输出：push 之后 numbers = [10, 20, 30]");

    // 方式二：vec! 宏直接给出初值，最常用
    let mut letters = vec!['a', 'b', 'c'];
    println!("vec! 宏创建 letters = {letters:?}");
    println!("// 预期输出：vec! 宏创建 letters = ['a', 'b', 'c']");

    // 方式三：with_capacity 预留容量，避免连续 push 时反复重新分配
    let mut with_cap: Vec<&str> = Vec::with_capacity(3);
    with_cap.push("第一");
    with_cap.push("第二");
    println!(
        "with_cap = {with_cap:?}，len = {}，capacity 至少为 3（实际 {}）",
        with_cap.len(),
        with_cap.capacity()
    );
    println!("// 预期输出：with_cap = [\"第一\", \"第二\"]，len = 2，capacity 至少为 3（实际 3）");

    // pop 从尾部取出元素并返回 Option，空向量时得到 None
    let last = numbers.pop();
    println!("pop() 得到 Some(30)，numbers 变为 {numbers:?}，返回值 = {last:?}");
    println!("// 预期输出：pop() 得到 Some(30)，numbers 变为 [10, 20]，返回值 = Some(30)");

    // insert 在指定下标插入，后续元素整体后移，代价 O(n)
    numbers.insert(0, 5);
    println!("在下标 0 插入 5 后 numbers = {numbers:?}");
    println!("// 预期输出：在下标 0 插入 5 后 numbers = [5, 10, 20]");

    // remove 按下标删除并返回被删元素，同样 O(n)
    let removed = numbers.remove(1);
    println!("remove(1) 删掉 10 并返回 {removed}，numbers = {numbers:?}");
    println!("// 预期输出：remove(1) 删掉 10 并返回 10，numbers = [5, 20]");

    // retain 按条件就地保留元素，比「倒序 remove」更直观
    numbers.retain(|value| *value >= 10);
    println!("retain 只保留 >= 10 的元素，numbers = {numbers:?}");
    println!("// 预期输出：retain 只保留 >= 10 的元素，numbers = [20]");

    // clear 清空元素但保留已分配的堆内存，capacity 不变
    letters.clear();
    println!(
        "clear 后 letters = {letters:?}，len = {}，capacity 仍是 {}",
        letters.len(),
        letters.capacity()
    );
    println!("// 预期输出：clear 后 letters = []，len = 0，capacity 仍是 3");
}

/// 示例 2：Vec 的索引与安全访问
///
/// 要点：`v[i]` 越界会 panic；`v.get(i)` 返回 `Option<&T>`，越界得到 `None`。
/// 真实程序里「下标可能越界」时优先用 `get`。
fn demo_2_vec_access_and_modify() {
    println!("--- 示例 2：Vec 的索引与安全访问 ---");

    let mut scores = vec![88, 92, 76];

    // 下标访问返回元素的引用（因为 i32 实现了 Copy，读取时会自动复制出值）
    let first = scores[0];
    println!("scores[0] = {first}");
    println!("// 预期输出：scores[0] = 88");

    // 通过下标可变访问就地修改元素
    scores[1] = 100;
    println!("scores[1] = 100 之后 scores = {scores:?}");
    println!("// 预期输出：scores[1] = 100 之后 scores = [88, 100, 76]");

    // get 返回 Option，越界是可控的 None 而不是进程崩溃
    match scores.get(2) {
        Some(value) => {
            println!("scores.get(2) = Some({value})");
            println!("// 预期输出：scores.get(2) = Some(76)")
        }
        None => println!("// 预期输出：scores.get(2) = None"),
    }
    match scores.get(99) {
        // 下标 99 越界，只会走 None 分支；Some 分支是死代码，期望值同样写成字面量
        Some(_) => println!("// 预期输出：scores.get(99) = Some(...)（越界时该分支不会执行）"),
        None => {
            // 补充说明（不是某个具体值，故不使用"预期输出"标记）
            println!("scores.get(99) = None（越界被安全地表达为 None）");
        }
    }

    // 最后一个元素：last 等价于 get(len - 1)
    println!("scores.last() = {:?}", scores.last());
    println!("// 预期输出：scores.last() = Some(76)");

    // 遍历时要用 &mut 才能修改元素；下面的写法不会移动整个 Vec
    for value in &mut scores {
        *value += 1; // 解引用后自增
    }
    println!("每个元素 +1 后 scores = {scores:?}");
    println!("// 预期输出：每个元素 +1 后 scores = [89, 101, 77]");
}

/// 示例 3：Vec 的遍历与容量变化
///
/// 要点：`iter()` / `iter_mut()` / `into_iter()` 三种遍历的所有权差别，
/// 以及 `capacity` 与重新分配的关系（用数据而非指针演示，保证输出确定）。
fn demo_3_vec_iterate_and_capacity() {
    println!("--- 示例 3：Vec 的遍历与容量变化 ---");

    let values = vec![1, 2, 3, 4];

    // iter() 借用遍历：values 仍然可用
    let mut sum = 0;
    for value in values.iter() {
        sum += value; // value 是 &i32：`+=` 走的是 AddAssign（i32 实现了 AddAssign<&i32>）
    }
    println!("iter() 求和 = {sum}，values 仍可用 = {values:?}");
    println!("// 预期输出：iter() 求和 = 10，values 仍可用 = [1, 2, 3, 4]");

    // into_iter() 按值遍历：values 被移动，此后不能再使用
    let doubled: Vec<i32> = values.into_iter().map(|value| value * 2).collect();
    println!("into_iter 消费后生成 doubled = {doubled:?}");
    println!("// 预期输出：into_iter 消费后生成 doubled = [2, 4, 6, 8]");
    // println!("{values:?}"); // error[E0382]: borrow of moved value: `values`

    // iter_mut() 借用可变遍历
    let mut words = vec![String::from("a"), String::from("b")];
    for word in words.iter_mut() {
        word.push('!'); // 元素是 &mut String，可以直接调用方法修改
    }
    println!("iter_mut 后 words = {words:?}");
    println!("// 预期输出：iter_mut 后 words = [\"a!\", \"b!\"]");

    // 容量演示：预分配足够容量，则本次 push 不会触发重新分配
    let mut planned: Vec<i32> = Vec::with_capacity(8);
    let cap_before = planned.capacity();
    planned.push(1);
    let cap_after = planned.capacity();
    println!("预留容量为 {cap_before}，push 后仍为 {cap_after}（二者相等，未重新分配）");
    println!("// 预期输出：预留容量为 8，push 后仍为 8（二者相等，未重新分配）");

    // shrink_to_fit 把容量收缩到与长度一致，释放多余的堆内存
    let mut slack: Vec<i32> = Vec::with_capacity(100);
    slack.push(7);
    println!(
        "slack 收缩前 len = {}，capacity = {}",
        slack.len(),
        slack.capacity()
    );
    println!("// 预期输出：slack 收缩前 len = 1，capacity = 100");
    slack.shrink_to_fit();
    println!(
        "shrink_to_fit 后 capacity = {}（等于 len，说明多余内存已归还分配器）",
        slack.capacity()
    );
    println!("// 预期输出：shrink_to_fit 后 capacity = 1（等于 len，说明多余内存已归还分配器）");

    // 从切片构造 Vec：to_vec 会复制数据，不共享底层缓冲区
    let source = [5, 6, 7];
    let copied = source.to_vec();
    println!("由数组切片复制得到 copied = {copied:?}");
    println!("// 预期输出：由数组切片复制得到 copied = [5, 6, 7]");
}

/// 示例 4：String 与 &str，以及 UTF-8 的字节 / 字符差异
///
/// 要点：`String` 是可增长、拥有所有权的 UTF-8 字节序列；`&str` 是借用来的
/// UTF-8 切片。中文一个字符占 3 个字节，所以 `len()`（字节数）常大于
/// `chars().count()`（字符数）。字节下标切分若不在字符边界上会 panic。
fn demo_4_string_basics_and_utf8() {
    println!("--- 示例 4：String 与 &str、UTF-8 字节与字符 ---");

    // String::from / to_string / String::new + push_str 三种常见构造
    let owned: String = String::from("你好世界");
    let borrowed: &str = "hello"; // 字符串字面量本身就是 &'static str
    let mut built = String::new();
    built.push_str("拼");
    built.push('接');
    println!("owned = {owned}，borrowed = {borrowed}，built = {built}");
    println!("// 预期输出：owned = 你好世界，borrowed = hello，built = 拼接");

    // 字节长度 vs 字符数量：中文各占 3 字节
    println!(
        "owned.len() = {} 字节，owned.chars().count() = {} 字符",
        owned.len(),
        owned.chars().count()
    );
    println!("// 预期输出：owned.len() = 12 字节，owned.chars().count() = 4 字符");
    println!(
        "borrowed.len() = {} 字节，borrowed.chars().count() = {} 字符（ASCII 二者相等）",
        borrowed.len(),
        borrowed.chars().count()
    );
    println!(
        "// 预期输出：borrowed.len() = 5 字节，borrowed.chars().count() = 5 字符（ASCII 二者相等）"
    );

    // bytes() 得到 UTF-8 原始字节；chars() 得到 Unicode 标量值
    let first_bytes: Vec<u8> = owned.bytes().take(3).collect();
    println!("owned 前 3 个字节 = {first_bytes:?}（即「你」的 UTF-8 编码）");
    println!("// 预期输出：owned 前 3 个字节 = [228, 189, 160]（即「你」的 UTF-8 编码）");

    // char_indices 给出「每个字符的起始字节下标 + 字符本身」，是安全切分的依据
    let index_info: Vec<String> = owned
        .char_indices()
        .map(|(index, ch)| format!("字节 {index} -> {ch}"))
        .collect();
    println!("owned 的字符边界 = {}", index_info.join("，"));
    println!(
        "// 预期输出：owned 的字符边界 = 字节 0 -> 你，字节 3 -> 好，字节 6 -> 世，字节 9 -> 界"
    );
    println!(
        "也可以写成「字节下标分别为 {:?}」，因此合法切分点是 0/3/6/9/12",
        owned
            .char_indices()
            .map(|(index, _)| index)
            .collect::<Vec<usize>>()
    );
    println!("// 预期输出：也可以写成「字节下标分别为 [0, 3, 6, 9]」，因此合法切分点是 0/3/6/9/12");

    // 按字符边界切分是合法的
    let world: &str = &owned[6..12];
    println!("&owned[6..12] = {world}（正好覆盖「世界」两个字，合法）");
    println!("// 预期输出：&owned[6..12] = 世界（正好覆盖「世界」两个字，合法）");

    // 按字符遍历并判断是否 CJK 统一表意文字
    let cjk_count = owned
        .chars()
        .filter(|ch| ('\u{4e00}'..='\u{9fff}').contains(ch))
        .count();
    println!("owned 中的 CJK 字符个数 = {cjk_count}");
    println!("// 预期输出：owned 中的 CJK 字符个数 = 4");

    // 反向操作：从 Vec<u8> 还原 String
    let restored = String::from_utf8(first_bytes);
    match restored {
        Ok(text) => {
            println!("String::from_utf8 还原出「{text}」");
            println!("// 预期输出：String::from_utf8 还原出「你」");
        }
        // 取出的 3 个字节正是「你」的合法 UTF-8 编码，Err 分支不会执行，期望值写成字面量
        Err(_) => println!("// 预期输出：还原失败：...（本输入是合法 UTF-8，该分支不会执行）"),
    }

    // 常见查询方法
    println!(
        "owned.contains(\"世界\") = {}，owned.starts_with('你') = {}",
        owned.contains("世界"),
        owned.starts_with('你')
    );
    println!("// 预期输出：owned.contains(\"世界\") = true，owned.starts_with('你') = true");
}

/// 示例 5：字符串拼接与格式化
///
/// 要点：`push_str` 原地追加（不转移所有权）；`+` 会消耗左侧 `String`
/// 的所有权（右侧必须是 `&str`）；`format!` 不消耗任何操作数，返回新 `String`。
fn demo_5_string_concat_and_format() {
    println!("--- 示例 5：字符串拼接与格式化 ---");

    // push_str 原地追加，s1 的所有权没有移动
    let mut s1 = String::from("Rust");
    s1.push_str(" 语言");
    println!("push_str 后 s1 = {s1}");
    println!("// 预期输出：push_str 后 s1 = Rust 语言");

    // + 运算符：左侧被移动，右侧按引用借用
    let s2 = String::from("Hello, ");
    let s3 = String::from("world!");
    let s4 = s2 + &s3; // s2 的所有权在这里被消耗
    println!("s2 + &s3 = {s4}，s3 仍然可用 = {s3}");
    println!("// 预期输出：s2 + &s3 = Hello, world!，s3 仍然可用 = world!");
    // println!("{s2}"); // error[E0382]: borrow of moved value: `s2`

    // format! 最通用：不消耗任何参数，可拼接任意多段
    let name = "小明";
    let score = 95;
    let report = format!("{name} 的成绩是 {score} 分");
    println!("format! 结果 = {report}");
    println!("// 预期输出：format! 结果 = 小明 的成绩是 95 分");

    // 位置参数与命名参数两种风格
    println!("位置参数 = {} 与命名参数 = {city}", "北京", city = "上海");
    println!("// 预期输出：位置参数 = 北京 与命名参数 = 上海");

    // 常用格式化：宽度、对齐、补零、进制、浮点精度、Debug
    // 重点：Rust 的宽度按「字符数（Unicode 标量值）」计算，**不是**按终端显示宽度算。
    // "右对齐" 是 3 个字符，宽度 6 时补 6 - 3 = 3 个空格（中文不会被当成 2 列宽）。
    println!("[{:>6}]", "右对齐");
    println!("// 预期输出：[   右对齐]");
    println!("[{:<6}]", "左对齐");
    println!("// 预期输出：[左对齐   ]");
    println!("补零 = {:05}，十六进制 = {:#x}，二进制 = {:#b}", 42, 255, 5);
    println!("// 预期输出：补零 = 00042，十六进制 = 0xff，二进制 = 0b101");
    println!(
        "浮点保留两位 = {:.2}，Debug 打印 = {:?}",
        3.14159,
        vec![1, 2, 3]
    );
    println!("// 预期输出：浮点保留两位 = 3.14，Debug 打印 = [1, 2, 3]");

    // join 适合「片段 -> 一个字符串」的场景，比手写循环更清晰
    let parts = ["a", "b", "c"];
    println!("parts.join(\"-\") = {}", parts.join("-"));
    println!("// 预期输出：parts.join(\"-\") = a-b-c");

    // 拆分与修剪：split_whitespace 对连续空白更健壮
    let line = "  姓名=小明   年龄=18  ";
    let tokens: Vec<&str> = line.split_whitespace().collect();
    println!(
        "split_whitespace 得到 {tokens:?}，trim 后 = \"{}\"",
        line.trim()
    );
    println!(
        "// 预期输出：split_whitespace 得到 [\"姓名=小明\", \"年龄=18\"]，trim 后 = \"姓名=小明   年龄=18\""
    );

    // 按 key=value 拆分：split_once 只切一次，比 split 再 collect 更省
    if let Some((key, value)) = tokens[0].split_once('=') {
        println!("split_once 解析出 key = {key}，value = {value}");
        println!("// 预期输出：split_once 解析出 key = 姓名，value = 小明");
    }
}

/// 示例 6：HashMap 的插入、查询、更新与遍历
///
/// 要点：`HashMap` 不保证遍历顺序；`insert` 覆盖旧值并返回旧值；
/// `get` 返回 `Option<&V>`，用 `copied()`/`cloned()` 可拿到值本身。
fn demo_6_hashmap_basics() {
    println!("--- 示例 6：HashMap 的插入、查询、更新与遍历 ---");

    // HashMap::new + 逐个插入（必须 use std::collections::HashMap）
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("小明"), 18);
    ages.insert(String::from("小红"), 20);

    // 键值类型可以由 turbo fish 指定，也可以像上面这样让编译器推断
    let from_pairs: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
    // 注意：不要直接打印 HashMap 的 Debug 值来断言顺序——它的遍历顺序不固定，
    // 因此下面所有需要「确定输出」的地方都先取键并排序。
    println!(
        "由数组 pairs 收集得到 {} 个键值对（键为 a、b），顺序不保证",
        from_pairs.len()
    );
    println!("// 预期输出：由数组 pairs 收集得到 2 个键值对（键为 a、b），顺序不保证");

    // insert 覆盖旧值，并返回被覆盖的旧值（第一次插入返回 None）
    let old = ages.insert(String::from("小明"), 19);
    println!(
        "再次 insert(\"小明\") 返回旧值 {old:?}，现在 ages[\"小明\"] = {:?}",
        ages.get("小明")
    );
    println!(
        "// 预期输出：再次 insert(\"小明\") 返回旧值 Some(18)，现在 ages[\"小明\"] = Some(19)"
    );
    println!(
        "此时 ages 中共 {} 个键（不能直接打印 HashMap 的 Debug 值来断言顺序）",
        ages.len()
    );
    println!("// 预期输出：此时 ages 中共 2 个键（不能直接打印 HashMap 的 Debug 值来断言顺序）");

    // get 返回 Option<&u32>，copied() 把 &u32 变成 u32
    println!(
        "ages.get(\"小红\") = {:?}，copied 后 = {:?}",
        ages.get("小红"),
        ages.get("小红").copied()
    );
    println!("// 预期输出：ages.get(\"小红\") = Some(20)，copied 后 = Some(20)");
    println!(
        "ages.get(\"不存在\") = {:?}（用 get 不会 panic）",
        ages.get("不存在")
    );
    println!("// 预期输出：ages.get(\"不存在\") = None（用 get 不会 panic）");

    // contains_key 只判断存在性；remove 删除并返回被删值
    println!("contains_key(\"小明\") = {}", ages.contains_key("小明"));
    println!("// 预期输出：contains_key(\"小明\") = true");
    let removed = ages.remove("小明");
    println!(
        "remove(\"小明\") 返回 {removed:?}，剩余键的个数 = {}",
        ages.len()
    );
    println!("// 预期输出：remove(\"小明\") 返回 Some(19)，剩余键的个数 = 1");

    // len / is_empty / 遍历：注意顺序不固定，需要确定输出时必须排序
    println!(
        "当前键值对个数 = {}，is_empty = {}",
        ages.len(),
        ages.is_empty()
    );
    println!("// 预期输出：当前键值对个数 = 1，is_empty = false");

    // 遍历顺序说明：HashMap 默认不保证顺序，即使这里只插入两个键也不能依赖输出次序，
    // 所以下面先在循环里按键排序再遍历，保证每次运行输出一致。
    let mut sorted_keys: Vec<&str> = from_pairs.keys().copied().collect();
    sorted_keys.sort_unstable();
    println!("按键排序后 -> {sorted_keys:?}");
    println!("// 预期输出：按键排序后 -> [\"a\", \"b\"]");
    // 期望值逐轮写成字面量（按键排序后的顺序固定为 a、b），与上一行的真实输出逐字节对照
    let expected_rows = [
        "// 预期输出：排序遍历 -> a = 1",
        "// 预期输出：排序遍历 -> b = 2",
    ];
    for (key, expected) in sorted_keys.iter().zip(expected_rows) {
        // get 一定成功（键来自 keys()），这里用 unwrap 是为了让示例简短，且已注释说明
        let value = from_pairs.get(key).unwrap(); // 教学示例：键必然存在，故此处 unwrap 安全
        println!("排序遍历 -> {key} = {value}");
        println!("{expected}");
    }
}

/// 示例 7：entry API —— 避免重复查询
///
/// 要点：`entry(key).or_insert(default)` 只在键不存在时插入，并返回
/// `&mut V`。它一次哈希查找就完成「查询 + 可能插入」，而
/// `contains_key` 后再 `insert` 需要两次哈希查找。
fn demo_7_hashmap_entry_api() {
    println!("--- 示例 7：entry API 避免重复查询 ---");

    let text = "苹果 香蕉 苹果 苹果 香蕉 橙子";
    let mut counts: HashMap<&str, u32> = HashMap::new();

    // or_insert 返回 &mut u32，直接对它做加法即可完成「计数 +1」
    for word in text.split_whitespace() {
        let counter = counts.entry(word).or_insert(0);
        *counter += 1;
    }
    let mut keys: Vec<&str> = counts.keys().copied().collect();
    keys.sort_unstable();
    let summary: Vec<String> = keys
        .iter()
        .map(|key| format!("{key} 出现 {} 次", counts[key]))
        .collect();
    println!(
        "entry + or_insert 统计结果（键排序后）= {}",
        summary.join("，")
    );
    println!(
        "// 预期输出：entry + or_insert 统计结果（键排序后）= 橙子 出现 1 次，苹果 出现 3 次，香蕉 出现 2 次"
    );
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("说明：HashMap 遍历顺序不固定，以上结果已按键排序以保证确定性");

    // or_insert_with 用于「默认值构造代价较高」的场合，只有真的缺失时才构造
    let mut groups: HashMap<&str, Vec<i32>> = HashMap::new();
    for (name, value) in [("偶数", 2), ("奇数", 3), ("偶数", 4), ("奇数", 5)] {
        groups.entry(name).or_insert_with(Vec::new).push(value);
    }
    let mut group_keys: Vec<&str> = groups.keys().copied().collect();
    group_keys.sort_unstable();
    // 期望值逐轮写成字面量（按键排序后的顺序固定为 偶数、奇数），与上一行的真实输出对照
    let expected_groups = [
        "// 预期输出：分组 偶数 -> [2, 4]",
        "// 预期输出：分组 奇数 -> [3, 5]",
    ];
    for (key, expected) in group_keys.iter().zip(expected_groups) {
        println!("分组 {key} -> {:?}", groups[key]);
        println!("{expected}");
    }

    // and_modify 只在键已存在时改值，链式写法可读性好
    let mut logs: HashMap<&str, u32> = HashMap::new();
    for level in ["INFO", "WARN", "INFO", "ERROR", "INFO"] {
        logs.entry(level)
            .and_modify(|count| *count += 1) // 已存在：计数 +1
            .or_insert(1); // 不存在：置为 1
    }
    let mut log_keys: Vec<&str> = logs.keys().copied().collect();
    log_keys.sort_unstable();
    let log_summary: Vec<String> = log_keys
        .iter()
        .map(|key| format!("{key}={}", logs[key]))
        .collect();
    println!(
        "and_modify + or_insert 统计日志级别 = {}",
        log_summary.join("，")
    );
    println!("// 预期输出：and_modify + or_insert 统计日志级别 = ERROR=1，INFO=3，WARN=1");
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("对比——insert 会无条件覆盖，若用它统计会永远写成 1；entry 只在缺失时写入默认值");

    // 统计单个键的查询：entry().or_default() 依赖 Default trait
    let mut visits: HashMap<&str, i32> = HashMap::new();
    visits.entry("首页").or_default();
    visits.entry("首页").and_modify(|count| *count += 5);
    println!("or_default 后访问计数 = {:?}", visits["首页"]);
    println!("// 预期输出：or_default 后访问计数 = 5");
}

/// 示例 8（典型使用场景）：统计一段文本的词频并排序输出
///
/// 要点：真实场景里 HashMap 几乎总和排序配合使用——先统计，再把
/// `(词, 次数)` 收集成 `Vec`，按「次数降序 + 词升序」排序后打印，
/// 这样输出才是确定的、可测试的。
fn demo_8_word_frequency_scenario() {
    println!("--- 示例 8：典型使用场景 —— 文本词频统计 ---");

    let article = "Rust makes memory safety easy. \
                   Rust is fast. Rust is reliable. \
                   Safety first, then speed.";

    // 步骤 1：规范化（转小写）并切分成单词，过滤掉标点等非字母数字字符
    let words: Vec<String> = article
        .to_lowercase() // 统一小写，避免 "Rust" 与 "rust" 被算成两个词
        .split(|ch: char| !ch.is_alphanumeric()) // 用「非字母数字」作为分隔符
        .filter(|piece| !piece.is_empty()) // 去掉连续分隔符产生的空片段
        .map(String::from)
        .collect();
    println!("规范化后共 {} 个词：{words:?}", words.len());
    println!(
        "// 预期输出：规范化后共 15 个词：[\"rust\", \"makes\", \"memory\", \"safety\", \"easy\", \"rust\", \"is\", \"fast\", \"rust\", \"is\", \"reliable\", \"safety\", \"first\", \"then\", \"speed\"]"
    );

    // 步骤 2：用 HashMap 统计词频；entry 一次查找完成计数
    let mut frequency: HashMap<String, usize> = HashMap::new();
    for word in &words {
        *frequency.entry(word.clone()).or_insert(0) += 1;
    }
    println!("去重后共有 {} 个不同的词（HashMap 长度）", frequency.len());
    println!("// 预期输出：去重后共有 11 个不同的词（HashMap 长度）");

    // 步骤 3：收集成 Vec 后排序。顺序规则：次数多的在前；次数相同按字母序
    let mut ranked: Vec<(String, usize)> = frequency
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // 步骤 4：只取前 5 名，格式化成表格行
    let top5: Vec<String> = ranked
        .iter()
        .take(5)
        .map(|(word, count)| format!("{word}={count}"))
        .collect();
    println!(
        "词频 Top5（次数降序，同次数按字母升序）= {}",
        top5.join("  ")
    );
    println!(
        "// 预期输出：词频 Top5（次数降序，同次数按字母升序）= rust=3  is=2  safety=2  easy=1  fast=1"
    );
    println!(
        "出现次数最多的词是「{}」，共 {} 次",
        ranked[0].0, ranked[0].1
    );
    println!("// 预期输出：出现次数最多的词是「rust」，共 3 次");

    // 步骤 5：顺手统计一下总字母数，演示 entry 之外的聚合写法
    let total_letters: usize = words.iter().map(|word| word.len()).sum();
    println!("所有单词的字节数之和 = {total_letters}");
    println!("// 预期输出：所有单词的字节数之和 = 69");
}

/// 示例 9：集合与所有权 / 借用的陷阱
///
/// 要点：`&v[i]` 借用了 `v`，在借用仍然存活期间不能对 `v` 做可变操作；
/// 遍历 `HashMap` 引用时也不能插入新键。下面每个陷阱都给出「错误代码（注释）
/// + 可编译的修正版本」。
fn demo_9_ownership_and_borrow_traps() {
    println!("--- 示例 9：集合与所有权 / 借用陷阱 ---");

    let v = vec![1, 2, 3, 4, 5];
    let first = v[0]; // i32 是 Copy，这里是复制值，不产生借用
    println!("先复制出 v[0] = {first}，之后 v 依然可用 = {v:?}");
    println!("// 预期输出：先复制出 v[0] = 1，之后 v 依然可用 = [1, 2, 3, 4, 5]");

    // ——陷阱一：不可变借用与可变借用不能同时存在——
    // let mut v = vec![1, 2, 3];
    // let a = &v[0];      // 不可变借用开始，a 指向 v 的堆缓冲区
    // v.push(1);          // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("{a}");    // 这里仍然使用 a，借用一直存活到这里
    // 修正方法一：先把值复制出来，借用在下一行之前就结束
    let mut fixed = vec![1, 2, 3];
    let a = fixed[0]; // Copy 类型取值即复制，借用立即结束
    fixed.push(1);
    println!("修正一（复制出值）后 a = {a}，fixed = {fixed:?}");
    println!("// 预期输出：修正一（复制出值）后 a = 1，fixed = [1, 2, 3, 1]");
    // 修正方法二：用下标重新读取，而不是长期持有引用
    let mut fixed2 = vec![1, 2, 3];
    fixed2.push(1);
    println!("修正二（push 之后再读）fixed2[0] = {}", fixed2[0]);
    println!("// 预期输出：修正二（push 之后再读）fixed2[0] = 1");

    // ——陷阱二：遍历集合时修改集合——
    let mut table: HashMap<&str, i32> = HashMap::new();
    table.insert("a", 1);
    table.insert("b", 2);
    // for (key, _) in &table {
    //     table.insert(key, 9); // error[E0502]: cannot borrow `table` as mutable because it is also borrowed as immutable
    // }
    // 修正：先收集要改的键（借用结束），再逐个修改
    let keys_to_bump: Vec<&str> = table.keys().copied().collect();
    for key in keys_to_bump {
        table.insert(key, 9);
    }
    // 打印前先排序，避免依赖 HashMap 的随机遍历顺序
    let mut table_keys: Vec<&str> = table.keys().copied().collect();
    table_keys.sort_unstable();
    let table_line: Vec<String> = table_keys
        .iter()
        .map(|key| format!("{key}={}", table[key]))
        .collect();
    println!(
        "先收集键再修改后 table = {{{}}}（值全部变为 9，已按键排序输出）",
        table_line.join(", ")
    );
    println!("// 预期输出：先收集键再修改后 table = {{a=9, b=9}}（值全部变为 9，已按键排序输出）");

    // ——陷阱三：用一个键取出的值引用不能与对同一 map 的写操作共存——
    let mut counter: HashMap<&str, i32> = HashMap::new();
    counter.insert("hits", 10);
    // let value = counter.get("hits").unwrap();
    // counter.insert("misses", 1); // error[E0502]: cannot borrow `counter` as mutable because it is also borrowed as immutable
    // println!("{value}");
    // 修正：用 entry 拿到 &mut，或先复制出值
    let copied = counter.get("hits").copied().unwrap_or(0);
    counter.insert("misses", 1);
    let mut counter_keys: Vec<&str> = counter.keys().copied().collect();
    counter_keys.sort_unstable();
    let counter_line: Vec<String> = counter_keys
        .iter()
        .map(|key| format!("{key}={}", counter[key]))
        .collect();
    println!(
        "先复制值 {copied}，再插入新键后 counter = {{{}}}（已按键排序输出）",
        counter_line.join(", ")
    );
    println!(
        "// 预期输出：先复制值 10，再插入新键后 counter = {{hits=10, misses=1}}（已按键排序输出）"
    );

    // ——陷阱四：通过 entry 拿到的 &mut 不能跨迭代保存——
    let mut stock: HashMap<String, i32> = HashMap::new();
    for name in ["苹果", "香蕉"] {
        // entry 返回的 &mut 只在本次循环体内有效，写回后立即释放
        let slot = stock.entry(name.to_string()).or_insert(0);
        *slot += 3;
    }
    let mut stock_keys: Vec<String> = stock.keys().cloned().collect();
    stock_keys.sort();
    let stock_line: Vec<String> = stock_keys
        .iter()
        .map(|key| format!("{key}={}", stock[key]))
        .collect();
    println!(
        "entry 在循环中使用（每次写回）stock = {}",
        stock_line.join("，")
    );
    println!("// 预期输出：entry 在循环中使用（每次写回）stock = 苹果=3，香蕉=3");

    // ——陷阱五：String 作为键时注意「移动」——
    let mut index: HashMap<String, usize> = HashMap::new();
    let owned_key = String::from("chapter1");
    index.insert(owned_key.clone(), 12); // clone 保留原变量，若直接传 owned_key 则它被移动
    println!(
        "插入后还能使用原键 = {owned_key}，map 中查到 = {:?}",
        index.get("chapter1")
    );
    println!("// 预期输出：插入后还能使用原键 = chapter1，map 中查到 = Some(12)");
}

/// 示例 10：常见错误示例（错误代码全部注释掉）
///
/// 要点：把集合相关的典型编译错误集中展示，并给出对应的修正方法。
/// 本函数自身不包含非法代码，因此可以正常编译运行。
fn demo_10_common_mistakes() {
    println!("--- 示例 10：常见错误示例（代码已注释，仅作说明） ---");

    // 错误 1：忘了 mut（E0596），以及「元素类型完全没有来源」时的 E0282
    // let values = Vec::new();   // 少写了 mut
    // values.push("hi");
    // error[E0596]: cannot borrow `values` as mutable, as it is not declared as mutable
    //   help: consider changing this to be mutable
    // 补上 mut 后可以直接编译通过：push("hi") 已经把元素类型定成 &str
    // let mut values = Vec::new();
    // values.push("hi");
    // 真正推断不出元素类型时才会报 E0282（元素类型没有任何来源）：
    // let values = Vec::new();
    // println!("{}", values.len());
    // error[E0282]: type annotations needed for `Vec<_>`
    // 修正：写清类型或给出初值
    let values: Vec<&str> = Vec::new();
    println!("修正后 values = {values:?}（空向量，类型标注为 Vec<&str>）");
    println!("// 预期输出：修正后 values = []（空向量，类型标注为 Vec<&str>）");

    // 错误 2：越界下标访问
    // let v = vec![1, 2, 3];
    // let x = v[3];
    // println!("{x}");
    // 编译通过但运行期 panic：index out of bounds: the len is 3 but the index is 3
    // 修正：用 get 返回 Option，或先判断下标范围
    let v = vec![1, 2, 3];
    println!(
        "改用 get 后 v.get(3) = {:?}（得到 None 而不是崩溃）",
        v.get(3)
    );
    println!("// 预期输出：改用 get 后 v.get(3) = None（得到 None 而不是崩溃）");

    // 错误 3：Vec 越界之外的另一类「索引」错误 —— 字符串不能按整数下标取字符
    // let s = String::from("你好");
    // let ch = s[0];
    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    //   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
    //   = note: you can use `.chars().nth()` or `.bytes().nth()`
    //   = note: required for `String` to implement `Index<{integer}>`
    // 修正：用 chars().nth(0) 或按字符边界切片
    let s = String::from("你好");
    println!(
        "s.chars().nth(0) = {:?}（字符串用字符迭代而非整数下标）",
        s.chars().nth(0)
    );
    println!("// 预期输出：s.chars().nth(0) = Some('你')（字符串用字符迭代而非整数下标）");

    // 错误 4：字符串按非字符边界切片
    // let s = String::from("你好");
    // let part = &s[0..1];
    // 编译期无法发现，运行期 panic：
    //   end byte index 1 is not a char boundary; it is inside '你' (bytes 0..3 of string)
    // 修正：用 char_indices 得到合法边界（0 与 3）
    let part = &s[0..3];
    println!("按合法边界切片 &s[0..3] = {part}");
    println!("// 预期输出：按合法边界切片 &s[0..3] = 你");

    // 错误 5：String 与 &str 类型不匹配
    // let text: &str = String::from("abc");
    // error[E0308]: mismatched types
    //   expected `&str`, found `String`
    // 修正：借用 .as_str() / &text，或显式声明 String
    let text: &str = &String::from("abc");
    println!("借用后 text = {text}");
    println!("// 预期输出：借用后 text = abc");

    // 错误 6：拼接时误用 String + String
    // let a = String::from("a");
    // let b = String::from("b");
    // let c = a + b;
    // error[E0308]: mismatched types
    //   expected `&str`, found `String`
    // 修正：右侧传引用 a + &b，或直接用 format!（format! 不消耗任何一方）
    let a = String::from("a");
    let b = String::from("b");
    let c = format!("{a}{b}");
    println!("用 format! 拼接 = {c}，a 与 b 仍可用 = {a} {b}");
    println!("// 预期输出：用 format! 拼接 = ab，a 与 b 仍可用 = a b");

    // 错误 7：把借用遍历出来的元素当成拥有所有权的值
    // let names = vec![String::from("x")];
    // for name in &names {
    //     let owned: String = name;
    // }
    // error[E0308]: mismatched types
    //   expected `String`, found `&String`
    // 修正：name.clone() 或 String::from(name.as_str())
    let names = vec![String::from("x")];
    for name in &names {
        let owned: String = name.clone();
        println!("clone 之后得到拥有所有权的 owned = {owned}");
        // names 只有一个元素，所以期望值直接写成字面量，并与上一行逐字节对照
        println!("// 预期输出：clone 之后得到拥有所有权的 owned = x");
    }

    // 错误 8：HashMap 的键类型必须实现 Eq + Hash
    // #[derive(Debug)]
    // struct Point { x: i32 }
    // let mut m: HashMap<Point, i32> = HashMap::new();
    // m.insert(Point { x: 1 }, 1);
    // error[E0599]: the method `insert` exists for struct `HashMap<Point, i32>`, but its trait bounds were not satisfied
    //   note: the following trait bounds were not satisfied: `Point: Eq`、`Point: Hash`
    //   help: consider annotating `Point` with `#[derive(Eq, Hash, PartialEq)]`
    // 修正：为结构体派生 #[derive(PartialEq, Eq, Hash)]（Debug 可选，便于打印）
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
    }
    let mut point_map: HashMap<Point, &str> = HashMap::new();
    point_map.insert(Point { x: 1 }, "原点右侧");
    println!(
        "派生 Eq + Hash 后可作键，point_map.get(&Point {{ x: 1 }}) = {:?}",
        point_map.get(&Point { x: 1 })
    );
    println!(
        "// 预期输出：派生 Eq + Hash 后可作键，point_map.get(&Point {{ x: 1 }}) = Some(\"原点右侧\")"
    );

    // 错误 9：HashMap 默认遍历顺序不确定，却被当成有序输出
    // let mut m = HashMap::new();
    // m.insert("b", 2);
    // m.insert("a", 1);
    // println!("{:?}", m); // 输出顺序与插入顺序无关
    // 修正：需要确定性输出时把键排序
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("b", 2);
    m.insert("a", 1);
    let mut ordered: Vec<(&str, i32)> = m.into_iter().collect();
    ordered.sort_by_key(|(key, _)| *key);
    println!("排序后的确定性输出 = {ordered:?}");
    println!("// 预期输出：排序后的确定性输出 = [(\"a\", 1), (\"b\", 2)]");

    // 错误 10：把 get 的返回值当成立即拿到的值参与算术，忘记它是 &T 且外层还有 Option
    // let mut m: HashMap<&str, i32> = HashMap::new();
    // m.insert("k", 3);
    // m.get("k") += 1;
    // error[E0368]: binary assignment operation `+=` cannot be applied to type `Option<&i32>`
    //   note: `Option<&i32>` does not implement `AddAssign<{integer}>`
    // error[E0067]: invalid left-hand side of assignment（右值表达式不能作为赋值目标）
    // 修正 A：比较用 copied() 先把 &i32 复制成 i32
    let mut m2: HashMap<&str, i32> = HashMap::new();
    m2.insert("k", 3);
    println!(
        "先 copied 再比较，m2.get(\"k\").copied() == Some(3) 成立 = {}",
        m2.get("k").copied() == Some(3)
    );
    println!("// 预期输出：先 copied 再比较，m2.get(\"k\").copied() == Some(3) 成立 = true");
    // 修正 B：想就地修改就用 entry，显式解引用后自增，语义最清晰
    *m2.entry("k").or_insert(0) += 1;
    // 补充说明（不是某个具体值，故不使用"预期输出"标记）
    println!("显式解引用累加后 m2 = {{\"k\": 4}}（由 3 自增为 4）");
}
