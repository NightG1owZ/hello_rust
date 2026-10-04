//! lesson_15_iterators.rs —— 主题：迭代器（Iterator）
//!
//! 学习目标：
//!   1. 理解 Iterator trait 与 for 循环的关系，分清 `iter()` / `iter_mut()` / `into_iter()` 三种迭代；
//!   2. 会用适配器（map / filter / enumerate / zip / take / skip / rev / chain）组合数据处理流水线；
//!   3. 会用消费器（sum / max / min / count / any / all / find / fold / collect）取出结果，
//!      并理解「适配器惰性、消费器触发执行」的机制；
//!   4. 会为自定义类型实现 Iterator，理解迭代器的零成本抽象；
//!   5. 会阅读迭代器相关的高频错误（E0502 / E0382 / E0282）。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_15_iterators.rs -o lesson_15_iterators && ./lesson_15_iterators
//!   或在本项目根目录执行：cargo run --bin lesson_15_iterators
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。
//!       迭代器大量以闭包为参数（见 lesson_14_closures），其「零开销」依赖
//!       泛型单态化（见 lesson_11_generics）；demoweb 的业务代码几乎处处
//!       是迭代器链，本课示例 9 的词频统计是它的最小版。

use std::collections::HashMap;

fn main() {
    println!("========== lesson_15_iterators：迭代器 ==========\n");

    // main 只负责按顺序调用各知识点示例，不写业务逻辑
    demo_1_iterator_and_for();
    demo_2_three_iteration_modes();
    demo_3_adapters();
    demo_4_consumers();
    demo_5_fold_and_collect();
    demo_6_laziness_proof();
    demo_7_zero_cost();
    demo_8_custom_iterator();
    demo_9_scenario_word_freq();
    demo_10_common_mistakes();
}

// ---------------------------------------------------------------------------
// 示例 1：Iterator trait 与 for 循环的关系
// ---------------------------------------------------------------------------

fn demo_1_iterator_and_for() {
    println!("--- 示例 1：迭代器与 for 循环 ---");

    let nums = vec![10, 20, 30];

    // for 循环只是语法糖：`for n in &nums` 实际是 `(&nums).into_iter()`，
    // 它拿到一个迭代器，反复调用 next() 直到返回 None
    let mut joined = String::new();
    for n in &nums {
        joined.push_str(&n.to_string());
        joined.push(' ');
    }
    println!("for 循环遍历：{}", joined.trim_end());
    println!("// 预期输出：for 循环遍历：10 20 30");

    // 证据：循环结束后 nums 仍然可用 —— 因为 &nums 只是被借用
    println!("循环后 nums = {nums:?}");
    println!("// 预期输出：循环后 nums = [10, 20, 30]");

    // 手动驱动迭代器：next() 返回 Option<Item>，None 表示耗尽
    let mut it = nums.iter();
    println!("next() 三次：{:?} {:?} {:?}", it.next(), it.next(), it.next());
    println!("// 预期输出：next() 三次：Some(10) Some(20) Some(30)");
    println!("next() 第四次：{:?}", it.next());
    println!("// 预期输出：next() 第四次：None");

    println!("迭代器 = 一个反复调用 next() 直到 None 的惰性序列");
}

// ---------------------------------------------------------------------------
// 示例 2：三种迭代方式 —— iter / iter_mut / into_iter
// ---------------------------------------------------------------------------

fn demo_2_three_iteration_modes() {
    println!("--- 示例 2：三种迭代方式 ---");

    // 方式一：iter() —— 借用，产出 &T，集合保留
    let words = vec![String::from("ab"), String::from("cd"), String::from("ef")];
    let lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    println!("iter() 借用，产出 &T：长度 = {lengths:?}");
    println!("// 预期输出：iter() 借用，产出 &T：长度 = [2, 2, 2]");
    println!("iter() 后 words 仍在：{:?}", words.len());
    println!("// 预期输出：iter() 后 words 仍在：3");

    // 方式二：into_iter() —— 消费，产出 T（所有权逐个交出），集合被移走
    let mut concatenated = String::new();
    for w in words.into_iter() {
        concatenated.push_str(&w);
    }
    println!("into_iter() 消费：拼接结果 = {concatenated}");
    println!("// 预期输出：into_iter() 消费：拼接结果 = abcdef");
    // words 在这里已经不可用（被 into_iter 消费）—— 需要保留集合就用 iter()

    // 方式三：iter_mut() —— 可变借用，产出 &mut T，可原地修改
    let mut nums = vec![1, 2, 3];
    for n in nums.iter_mut() {
        *n *= 10;
    }
    println!("iter_mut() 原地修改：{nums:?}");
    println!("// 预期输出：iter_mut() 原地修改：[10, 20, 30]");

    println!("借用用 iter，修改用 iter_mut，拿走用 into_iter");
}

// ---------------------------------------------------------------------------
// 示例 3：适配器 —— 组合数据处理流水线（全部惰性）
// ---------------------------------------------------------------------------

fn demo_3_adapters() {
    println!("--- 示例 3：常用适配器 ---");

    let nums = vec![1, 2, 3, 4, 5, 6];

    // map + filter：翻倍后只留 4 的倍数
    let result: Vec<i32> = nums.iter().map(|x| x * 2).filter(|x| x % 4 == 0).collect();
    println!("map+filter：{result:?}");
    println!("// 预期输出：map+filter：[4, 8, 12]");

    // enumerate：带上下标
    let letters = vec!['a', 'b', 'c'];
    let indexed: Vec<String> = letters
        .iter()
        .enumerate()
        .map(|(i, ch)| format!("{i}:{ch}"))
        .collect();
    println!("enumerate：{indexed:?}");
    println!("// 预期输出：enumerate：[\"0:a\", \"1:b\", \"2:c\"]");

    // zip：把两个序列按位置配对
    let names = vec!["甲", "乙"];
    let scores = vec![95, 87];
    let pairs: Vec<String> = names
        .iter()
        .zip(scores.iter())
        .map(|(n, s)| format!("{n}={s}"))
        .collect();
    println!("zip：{pairs:?}");
    println!("// 预期输出：zip：[\"甲=95\", \"乙=87\"]");

    // skip + take：跳过 1 个取 3 个
    let base = [1, 2, 3, 4, 5];
    let window: Vec<i32> = base.into_iter().skip(1).take(3).collect();
    println!("skip(1).take(3)：{window:?}");
    println!("// 预期输出：skip(1).take(3)：[2, 3, 4]");

    // rev + chain：反转、拼接
    let reversed: Vec<i32> = base.into_iter().rev().collect();
    println!("rev：{reversed:?}");
    println!("// 预期输出：rev：[5, 4, 3, 2, 1]");
    let chained: Vec<i32> = [1, 2].into_iter().chain([3, 4]).collect();
    println!("chain：{chained:?}");
    println!("// 预期输出：chain：[1, 2, 3, 4]");

    println!("适配器像流水线传送带：接上才转，不接不动（见示例 6）");
}

// ---------------------------------------------------------------------------
// 示例 4：消费器 —— 取出最终结果（触发执行）
// ---------------------------------------------------------------------------

fn demo_4_consumers() {
    println!("--- 示例 4：常用消费器 ---");

    let nums = vec![4, 9, 1, 7];

    println!("sum = {}", nums.iter().sum::<i32>());
    println!("// 预期输出：sum = 21");
    println!("max = {:?}", nums.iter().max());
    println!("// 预期输出：max = Some(9)");
    println!("min = {:?}", nums.iter().min());
    println!("// 预期输出：min = Some(1)");
    println!("count = {}", nums.iter().count());
    println!("// 预期输出：count = 4");

    // 谓词类消费器：any / all / find / position
    println!("存在偶数？any(even) = {}", nums.iter().any(|x| x % 2 == 0));
    println!("// 预期输出：存在偶数？any(even) = true");
    println!("全为正数？all(pos) = {}", nums.iter().all(|x| *x > 0));
    println!("// 预期输出：全为正数？all(pos) = true");
    println!("第一个 >5 的：{:?}", nums.iter().find(|x| **x > 5));
    println!("// 预期输出：第一个 >5 的：Some(9)");
    println!("值为 1 的下标：{:?}", nums.iter().position(|x| *x == 1));
    println!("// 预期输出：值为 1 的下标：Some(2)");

    // 谓词短路：find 从前往后找第一个，找到即停 —— 迭代器是「按需驱动」的
    println!("find 只消费到命中即停（短路求值）");
}

// ---------------------------------------------------------------------------
// 示例 5：fold 折叠 与 collect 收集
// ---------------------------------------------------------------------------

fn demo_5_fold_and_collect() {
    println!("--- 示例 5：fold 与 collect ---");

    // fold：带累积值的通用折叠，sum/max 都可以用它实现
    let product = [1, 2, 3, 4].into_iter().fold(1, |acc, x| acc * x);
    println!("fold 连乘 = {product}");
    println!("// 预期输出：fold 连乘 = 24");

    let csv = ["a", "b", "c"]
        .into_iter()
        .fold(String::new(), |acc, x| {
            if acc.is_empty() {
                x.to_string()
            } else {
                format!("{acc},{x}")
            }
        });
    println!("fold 拼 CSV = {csv}");
    println!("// 预期输出：fold 拼 CSV = a,b,c");

    // collect 的目标类型由「接收方」决定：字符串、Vec、BTreeMap 都行
    let word: String = vec!['r', 'u', 's', 't'].into_iter().collect();
    println!("collect 成 String = {word}");
    println!("// 预期输出：collect 成 String = rust");

    // collect 成 Result：任何一个元素出错，整体变成 Err（? 链路的好搭档）
    let ok: Result<Vec<i32>, _> = ["1", "2", "3"]
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("collect<Result> 全成功 = {:?}", ok);
    println!("// 预期输出：collect<Result> 全成功 = Ok([1, 2, 3])");

    let bad: Result<Vec<i32>, _> = ["1", "x"]
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("collect<Result> 有失败 = {}", bad.err().unwrap());
    println!("// 预期输出：collect<Result> 有失败 = invalid digit found in string");

    println!("fold 是「自定义聚合」，collect 是「换容器」");
}

// ---------------------------------------------------------------------------
// 示例 6：惰性求值证明 —— 没有消费器就一行代码都不执行
// ---------------------------------------------------------------------------

fn demo_6_laziness_proof() {
    println!("--- 示例 6：惰性求值 ---");

    let nums = vec![1, 2, 3];
    let mut calls = 0;

    // 此刻 map 的闭包还没被创建，计数自然是 0
    println!("流水线尚未搭建：calls = {calls}");
    println!("// 预期输出：流水线尚未搭建：calls = 0");

    // map 的闭包里顺手计数：每次真正执行 map 的逻辑，calls 就加 1；
    // 中间没有任何 println —— 只有 collect 这一个消费器在驱动整条链子
    let values: Vec<i32> = nums
        .iter()
        .map(|x| {
            calls += 1;
            x * 2
        })
        .collect();

    println!("collect 驱动后：calls = {calls}，结果 = {values:?}");
    println!("// 预期输出：collect 驱动后：calls = 3，结果 = [2, 4, 6]");

    // 这就是「惰性」：适配器只搭流水线，消费器才按需逐件加工。
    // 好处：take(2).map(...) 只加工 2 件，绝不会先把整个序列算完。
    println!("适配器惰性，消费器驱动");
}

// ---------------------------------------------------------------------------
// 示例 7：零成本抽象 —— 迭代器链与手写循环编译结果等价
// ---------------------------------------------------------------------------

fn demo_7_zero_cost() {
    println!("--- 示例 7：零成本抽象 ---");

    let nums: Vec<i32> = (1..=6).collect();

    // 迭代器链写法：取偶数 → 乘 10 → 求和
    let via_iterators: i32 = nums.iter().filter(|x| *x % 2 == 0).map(|x| x * 10).sum();

    // 手写循环写法：逻辑完全相同
    let mut via_loop = 0;
    for x in &nums {
        if x % 2 == 0 {
            via_loop += x * 10;
        }
    }

    println!("迭代器链求和 = {via_iterators}");
    println!("// 预期输出：迭代器链求和 = 120");
    println!("手写循环求和 = {via_loop}");
    println!("// 预期输出：手写循环求和 = 120");

    // 为什么一样快？filter/map 是泛型适配器，编译期单态化（见 lesson_11）后
    // 与手写循环生成相同的机器码：没有装箱、没有虚调用、没有额外堆分配。
    println!("抽象不花运行期成本：单态化让迭代器链 ≈ 手写循环");
}

// ---------------------------------------------------------------------------
// 示例 8：自定义迭代器 —— 实现 Iterator trait
// ---------------------------------------------------------------------------

/// 斐波那契数列：只需一个 next() 方法，就能接入整个迭代器生态
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.curr;
        let following = self.curr + self.next; // u64 溢出会 panic，教学场景数值足够小
        self.curr = self.next;
        self.next = following;
        Some(current) // 数列无限长，永远返回 Some；配合 take 截断
    }
}

fn demo_8_custom_iterator() {
    println!("--- 示例 8：自定义迭代器 ---");

    // 无限序列 + take 截断：这就是惰性带来的表达力
    let first_ten: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("斐波那契前 10 项：{first_ten:?}");
    println!("// 预期输出：斐波那契前 10 项：[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]");

    // 适配器/消费器全部可用：filter 取偶数项、sum 求和
    let even_sum: u64 = Fibonacci::new().take(10).filter(|x| x % 2 == 0).sum();
    println!("前 10 项中偶数之和 = {even_sum}");
    println!("// 预期输出：前 10 项中偶数之和 = 44");

    println!("实现一个 next()，免费获得全部适配器与消费器");
}

// ---------------------------------------------------------------------------
// 示例 9：典型场景 —— 词频统计与配置行解析
// ---------------------------------------------------------------------------

fn demo_9_scenario_word_freq() {
    println!("--- 示例 9：场景：词频统计 + 配置解析 ---");

    let text = "the quick brown fox the lazy dog the fox";

    // 切词 → 计数（entry API 见 lesson_08）
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }

    // 收集成 Vec 后按「次数降序、单词升序」排序 —— 保证输出确定
    let mut pairs: Vec<(&str, usize)> = freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (word, count) in &pairs {
        println!("「{word}」出现 {count} 次");
    }
    println!("// 预期输出：「the」出现 3 次");
    println!("// 预期输出：「fox」出现 2 次");
    println!("// 预期输出：「brown」出现 1 次");
    println!("// 预期输出：「dog」出现 1 次");
    println!("// 预期输出：「lazy」出现 1 次");
    println!("// 预期输出：「quick」出现 1 次");

    // 配置行解析：split_once 一分为二，filter_map 顺手丢掉格式不对的行
    let raw = "host=127.0.0.1\nport=8080\n这一行没有等号\nname=demo";
    let pairs: Vec<(&str, &str)> = raw
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect();
    for (key, value) in &pairs {
        println!("{key} => {value}");
    }
    println!("// 预期输出：host => 127.0.0.1");
    println!("// 预期输出：port => 8080");
    println!("// 预期输出：name => demo");

    println!("split / filter_map / map / collect 一条流水线完成解析");
}

// ---------------------------------------------------------------------------
// 示例 10：常见错误对照（全部注释掉，取消注释即可看到真实编译错误）
// ---------------------------------------------------------------------------

fn demo_10_common_mistakes() {
    println!("--- 示例 10：常见错误 ---");

    // 错误 1：迭代期间修改集合 —— 遍历的借用还没结束就去 push
    // let mut v = vec![1, 2, 3];
    // for x in &v {
    //     v.push(*x);        // error[E0502]: cannot borrow `v` as mutable
    // }                      //   because it is also borrowed as immutable
    // 修正：先把要追加的值收集到新 Vec，循环结束后再 extend；
    //       或改用 into_iter 消费旧集合并新建集合。

    // 错误 2：into_iter 消费后又使用原集合
    // let v = vec![1, 2, 3];
    // let it = v.into_iter();
    // println!("{v:?}");     // error[E0382]: borrow of moved value: `v`
    // 修正：需要保留集合用 iter()；确定消费就用 into_iter 且之后不再碰 v。

    // 错误 3：collect 目标类型不明
    // let xs = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
    // error[E0282]: type annotations needed
    // 修正：写明接收类型 let xs: Vec<i32> = ...; 或 turbofish collect::<Vec<i32>>()

    // 错误 4：只有适配器没有消费器 —— 流水线从未运转
    // vec![1, 2, 3].iter().map(|x| println!("看到 {x}"));
    // 编译通过但什么都不打印：map 惰性，没有消费器驱动它。
    // 修正：接 for 循环、collect、sum 等消费器；或用 for_each 消费器。

    // 错误 5：find 返回的是引用，误当作值继续链式消费
    // let nums = vec![4, 9, 1];
    // let found = nums.iter().find(|x| **x > 5);
    // let doubled = found.map(|x| x * 2);  // x 是 &i32，*x * 2 才对
    // error[E0369]: cannot multiply `&i32` by `{integer}`（写法上需要解引用）
    // 修正：find(...).map(|&x| x * 2) 用模式解构掉引用，或 copied() 后再算。
    println!("迭代器高频错误：E0502 / E0382 / E0282 / 忘接消费器");

    // 对照 lesson_08 的集合所有权陷阱：迭代器把同一套借用规则
    // 以更清晰的「三种迭代方式」呈现 —— 记住口诀就不会踩坑。
    println!("口诀：借用 iter，修改 iter_mut，拿走 into_iter");
}

// 本文件示例按 lesson-conventions.md 约定编写：
// 「// 预期输出：」为字面量断言，可用 .dsh/check_expected_output.ps1 一键核对。
