//! assessments/lesson_13_lifetimes.rs —— 考核：生命周期（对应 lesson_13）
//!
//! - 对应课程：`src/tutorial/lesson_13_lifetimes.rs`
//! - 知识点出处：`src/tutorial/README.md` 第四阶段「13 生命周期」
//! - 运行方式：
//!   ```text
//!   cargo test --test lesson_13_lifetimes     # 只考这一课
//!   cargo test                                 # 考全部 18 课
//!   cargo run --bin assessment_report          # 生成学习评估报告
//!   ```
//!
//! # 怎么用（三分钟上手）
//!
//! 1. 先读下面的 `#[test]` 函数：它写明了**这一题会怎么检查你**（期望值是什么、
//!    边界情况有哪些）；
//! 2. 再往下找到对应的 `exercise_13_xx_xxx` 练习函数（kp_13_04 / kp_13_09 是 `impl` 块里的
//!    方法），它的函数体里只有一行 `todo_exercise(...)` 占位；
//! 3. **删掉那一行**，按「实现要求」写出你的实现；
//! 4. 运行 `cargo test --test lesson_13_lifetimes`，直到全部用例变绿；
//! 5. 运行 `cargo run --bin assessment_report` 看评估报告与改进建议。
//!
//! # 规则
//!
//! - `#[test]` 函数是考核标准的一部分，**不要修改**（改了就无法验证你的掌握程度）；
//! - 上面的「提供给你的类型」小节**不要修改**：`Excerpt` / `Config` 的字段已经定义好，
//!   你只需要补 `impl` 块里的方法（kp_13_04、kp_13_09）；
//! - 只修改 `exercise_*` 练习函数与对应 `impl` 块里方法的函数体，可以按需增加局部变量与辅助函数；
//! - 本文件只依赖标准库 + 本项目自带的考核框架 `assessment_harness`；
//! - 骨架态（一行 `todo_exercise` 都没删）也能编译通过，测试会明确报「未实现」。
//!
//! # 最重要的一句话
//!
//! **生命周期注解只描述「引用之间的存活关系」，它不会延长任何数据的存活时间。**
//! 它不会让一个局部变量活得更久，也不会把悬垂引用变成合法；编译器借它来检查你的代码，
//! 真正的修复手段永远是「调整作用域 / 改用拥有所有权的类型 / 由调用方把数据传进来」。
//!
//! # 本课常见错误速查（对应课程示例 11）
//!
//! | 代码 | 报错 | 修正方法 |
//! | --- | --- | --- |
//! | 两个引用参数 + 一个输出引用却省略注解 | `error[E0106]` missing lifetime specifier | 手写 `<'a>`：`fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` |
//! | 结构体字段是引用却没写生命周期参数 | `error[E0106]` missing lifetime specifier | `struct Excerpt<'a> { part: &'a str }` |
//! | 返回局部变量的引用（`-> &String`） | `error[E0106]` missing lifetime specifier（补上注解后变成 `error[E0515]` cannot return reference to local variable） | 返回拥有所有权的类型 `String`，或让数据由调用方传入 |
//! | `fn first_word<'static>(...)` 把 `'static` 当参数名 | `error[E0262]` invalid lifetime parameter name | `'static` 只能写在**类型**位置（`&'static str`） |
//! | 只给一个参数标注生命周期却想返回另一个 | `error[E0621]` explicit lifetime required in the type of `y` | 给 `y` 单独的 `'b` 并且只返回 `x`，或把 `y` 也写成 `&'a str` |
//! | 结构体实例活得比被借用的数据更久 | `error[E0597]` `temporary` does not live long enough | 把被借用的数据提升到外层作用域 |

use assessment_harness::{Kind, assess, eq, eq_slice, is_true};

/// 本模块 id：与 harness 里的模块登记表、`docs/04_knowledge_map.md` 一一对应。
const M: &str = "lesson_13";

// ===========================================================================
// ===== 提供给你的类型（不要修改）=====
// ===========================================================================
//
// 这两个结构体都**持有引用**，所以必须带一个生命周期参数：
//   * `Excerpt<'a>`：由 kp_13_04 补上 `new` / `part` / `announce`；
//   * `Config<'a>` ：由 kp_13_09 补上 `new` / `endpoint` / `host`。
// 字段名与字段类型不要改动（多一个或少一个 `'a`，测试与练习都无法编译）。

/// 从文本中摘出来的一段（方法由学员实现）。
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

/// 配置（方法由学员实现）。
#[derive(Debug)]
struct Config<'a> {
    host: &'a str,
    port: u16,
}

// ===========================================================================
// kp_13_01 函数生命周期注解：多个输入引用 + 一个输出引用
// ===========================================================================

/// 知识点考核：手写 `<'a>` 把返回值与两个参数关联起来。
#[test]
fn kp_13_01_longest() {
    assess(
        M,
        "kp_13_01",
        "函数签名里的生命周期注解：返回较长的那个字符串切片",
        Kind::Core,
        "复习 lesson_13 示例 2（函数签名中的生命周期注解）：\
         `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` 的意思是\
         「返回值活得和这两个参数一样久」——`'a` 会被推断为两个实参中**较短**的那个存活区间；\
         注解只描述关系，不延长任何数据的寿命。",
        || {
            eq(
                exercise_13_01_longest("hello", "hi"),
                "hello",
                "按字节长度比较：\"hello\"(5) 比 \"hi\"(2) 长，所以返回它",
            );
            eq(
                exercise_13_01_longest("hi", "hello"),
                "hello",
                "参数顺序不影响结果：较长的那个（\"hello\"）才该被返回",
            );
            // 边界：长度相等时取第一个
            eq(
                exercise_13_01_longest("ab", "cd"),
                "ab",
                "边界：两个切片长度相等时返回**第一个**（对应课程里的 first.len() >= second.len()）",
            );
            // 边界：空串
            eq(
                exercise_13_01_longest("", ""),
                "",
                "边界：两个都是空串时返回空串，不能 panic、也不能越界切片",
            );
            eq(
                exercise_13_01_longest("", "x"),
                "x",
                "边界：一个空串一个非空，返回非空的那个",
            );
            // 边界：UTF-8 多字节 —— 本题按**字节数**比较（与课程示例一致）
            eq(
                exercise_13_01_longest("中文", "abc"),
                "中文",
                "边界：\"中文\" 是 6 字节、\"abc\" 是 3 字节，按字节数比较应返回 \"中文\"",
            );
            eq(
                exercise_13_01_longest("中文", "abcdef"),
                "中文",
                "边界：\"中文\" 与 \"abcdef\" 都是 6 字节，长度相等 → 取第一个 \"中文\"（按字符数比较会得到相反结果）",
            );
        },
    );
}

/// 【待实现】返回两个字符串切片中较长的那个。
///
/// 实现要求：
///   - 签名固定为 `fn exercise_13_01_longest<'a>(a: &'a str, b: &'a str) -> &'a str`；
///   - 必须手写 `<'a>`：两个输入引用 + 一个输出引用是省略规则**无法**推断的情况，
///     不写就报 `error[E0106]: missing lifetime specifier`；
///   - 长度按**字节数**比较（`a.len()` / `b.len()`，与课程示例 2 的 `longest` 一致）；
///   - 长度相等（含两个都是空串）时返回第一个参数 `a`；
///   - 只返回这两个引用之一，不要构造新的字符串；
///   - 注解只说明「返回值借的是这两个参数之一」，它既不延长 `a` / `b` 的寿命，
///     也不会让悬垂引用变得合法。
///
/// 示例输入：
/// ```text
/// a = "hello", b = "hi"
/// ```
/// 示例输出：
/// ```text
/// "hello"
/// ```
fn exercise_13_01_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // 按**字节数**比较；长度相等（含两个都是空串）时返回第一个参数 a
    if a.len() >= b.len() { a } else { b }
}

// ===========================================================================
// kp_13_02 生命周期省略规则：只有一个输入引用时不必写注解
// ===========================================================================

/// 知识点考核：写一个不需要生命周期注解的切片函数。
#[test]
fn kp_13_02_first_word() {
    assess(
        M,
        "kp_13_02",
        "生命周期省略规则 1 + 2：只有一个输入引用时，输出引用就取它",
        Kind::Core,
        "复习 lesson_13 示例 3（生命周期省略的三条规则）：规则 1 给每个引用参数各一个生命周期参数，\
         规则 2 在「只有一个输入引用」时把输出引用的生命周期取成它；\
         所以 `fn first_word(text: &str) -> &str` 等价于 `fn first_word<'a>(text: &'a str) -> &'a str`。",
        || {
            eq(
                exercise_13_02_first_word("hello world"),
                "hello",
                "取第一个空格之前的部分：\"hello world\" → \"hello\"（不含空格）",
            );
            // 边界：没有空格时返回整个字符串
            eq(
                exercise_13_02_first_word("rust"),
                "rust",
                "边界：完全没有空格时返回整个切片，而不是空串",
            );
            // 边界：空串
            eq(
                exercise_13_02_first_word(""),
                "",
                "边界：空串返回空串，不能 panic、也不能越界",
            );
            // 边界：前导空格 —— 第一个字符就是空格，第一个单词是空串
            eq(
                exercise_13_02_first_word("  leading"),
                "",
                "边界：文本以空格开头时，第一个空格之前是空串（不要先 trim 再找空格）",
            );
            eq(
                exercise_13_02_first_word("a  b"),
                "a",
                "边界：连续多个空格只截到**第一个**空格，返回值里不能带空格",
            );
            // 边界：多字节字符
            eq(
                exercise_13_02_first_word("中文 空格"),
                "中文",
                "边界：按字节切片也必须落在字符边界上（\"中文\" 是完整的一个词）",
            );
        },
    );
}

/// 【待实现】返回文本的第一个单词（第一个空格之前的切片）。
///
/// 实现要求：
///   - 签名固定为 `fn exercise_13_02_first_word(text: &str) -> &str`（**不要**手写 `<'a>`）；
///   - 用 `text.find(' ')` 找第一个空格：找到就返回 `&text[..index]`，没找到就返回 `text`；
///   - 不要 trim、不要分配新的 String；
///   - 这里能省掉注解靠的是省略规则 1 + 2（只有一个输入引用，输出引用就取它的生命周期）；
///     手写 `fn exercise_13_02_first_word<'a>(text: &'a str) -> &'a str` 也能编译，
///     但那不是本题要考的「省略」。
///
/// 示例输入：
/// ```text
/// text = "hello world"
/// ```
/// 示例输出：
/// ```text
/// "hello"
/// ```
fn exercise_13_02_first_word(text: &str) -> &str {
    // 省略规则 1 + 2：只有一个输入引用，输出引用就取它的生命周期
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

// ===========================================================================
// kp_13_03 两个不同的生命周期参数：返回值只和第一个绑定
// ===========================================================================

/// 知识点考核：给两个参数各自的 `'a` / `'b`，返回值只与第一个绑定。
#[test]
fn kp_13_03_pick_first() {
    assess(
        M,
        "kp_13_03",
        "两个独立生命周期参数：'a 与 'b 互不影响，返回值只借用 'a",
        Kind::Core,
        "复习 lesson_13 示例 4（多个生命周期参数）：\
         `fn pick_first<'a, 'b>(first: &'a str, second: &'b str) -> &'a str` 明确写出\
         「只与 first 有关」；如果两个参数共用同一个 `'a`，\
         `'a` 会被压缩成较短的那个，返回值就带不出内层作用域了。",
        || {
            let long_lived = String::from("长期存在的数据");
            // result 在外层声明、在内层赋值：它只能借用 long_lived（比内层块活得久）
            let result;
            {
                let short_lived = String::from("临时");
                // 'a 是 long_lived 的区间，'b 是 short_lived 的区间，两者互不影响
                result = exercise_13_03_pick_first(long_lived.as_str(), short_lived.as_str());
            }
            // short_lived 已经释放；这一行能编译，正是因为返回值只与 'a（first）绑定
            eq(
                result,
                "长期存在的数据",
                "返回值必须是 first；它能活到内层块之外，说明返回值只借用 'a 而不是 'b",
            );
            // 边界：first 是空串
            eq(
                exercise_13_03_pick_first("", "非空"),
                "",
                "边界：first 是空串时原样返回空串（不要「聪明地」改返回 second）",
            );
            // 边界：两者相同
            eq(
                exercise_13_03_pick_first("相同", "相同"),
                "相同",
                "边界：两个参数内容相同也要返回 first（返回值与 second 完全无关）",
            );
        },
    );
}

/// 【待实现】只返回第一个参数（两个参数各有自己的生命周期）。
///
/// 实现要求：
///   - 签名固定为 `fn exercise_13_03_pick_first<'a, 'b>(first: &'a str, second: &'b str) -> &'a str`；
///   - 函数体只返回 `first`，绝对不要返回 `second`；
///   - 返回值一定是 `first`，所以只能写成 `&'a str`：写成 `&'b str` 编译不过；若把两个参数
///     合并成一个 `'a`，`'a` 会被推断成较短的那个区间，返回值就带不出内层作用域
///     （`error[E0597]: `short_lived` does not live long enough`）；
///   - 实现里没用 `second` 时写一行 `let _ = second.len();`，以免 `unused variable` 警告
///     （课程示例 4 正是顺手读了它的长度）；
///   - 注解只描述关系、不延长存活时间，「谁活多久」必须由作用域决定。
///
/// 示例输入：
/// ```text
/// first = "长期存在的数据", second = "临时"
/// ```
/// 示例输出：
/// ```text
/// "长期存在的数据"
/// ```
fn exercise_13_03_pick_first<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    // second 只用于对比：读一下长度，避免 unused variable 警告
    let _ = second.len();
    first
}

// ===========================================================================
// kp_13_04 结构体持有引用 + impl 块与方法上的生命周期
// ===========================================================================

/// 知识点考核：结构体持有引用，方法用省略规则 / 返回字段里的引用。
#[test]
fn kp_13_04_excerpt_methods() {
    assess(
        M,
        "kp_13_04",
        "结构体持有引用：Excerpt<'a> 的 new / part / announce",
        Kind::Core,
        "复习 lesson_13 示例 5 与示例 6（结构体持有引用、impl 块与方法中的生命周期）：\
         字段是引用就必须写 `struct Excerpt<'a> { part: &'a str }`；\
         `fn part(&self) -> &str` 用省略规则 3（输出引用取 `&self` 的生命周期）；\
         `announce` 返回 `self.part`，绝不能返回 `msg` —— 因为返回类型借的是 self，不是 msg。",
        || {
            // 直接用结构体字面量读一次字段：既确认字段语义，也让 part 在骨架态不触发 dead_code
            let probe = Excerpt { part: "骨架态" };
            eq(
                probe.part,
                "骨架态",
                "字段 part 就是「从文本里摘出来的那一段」，类型是 &'a str",
            );
            // 正常用例：new 把借来的切片存进结构体
            let excerpt = Excerpt::new("hello");
            eq(
                excerpt.part(),
                "hello",
                "part() 返回结构体字段里的那一段（省略规则 3：返回值借用 self，只能在这个引用有效期内使用）",
            );
            // 正常用例：announce 必须返回 part，而不是 msg
            eq(
                excerpt.announce("完全不同的消息"),
                "hello",
                "announce() 必须返回 self.part：msg 只是随行参数，它的生命周期与返回值无关",
            );
            // 边界：msg 比 part 长得多，返回值仍然只能是 part
            let long_msg = "很长的消息".repeat(20);
            eq(
                excerpt.announce(&long_msg),
                "hello",
                "边界：msg 再长也不影响返回值 —— 返回类型借的是 self，不是 msg",
            );
            // 边界：part 是空切片
            let empty = Excerpt::new("");
            eq(
                empty.part(),
                "",
                "边界：part 为空串时 part() 返回空串（不能 panic，也不能改返回 msg 那样的占位物）",
            );
        },
    );
}

impl<'a> Excerpt<'a> {
    /// 【待实现】构造 `Excerpt`（kp_13_04 的第一个方法）。
    ///
    /// 实现要求：
    ///   - 签名固定为 `fn new(part: &'a str) -> Self`；
    ///   - 返回 `Self { part }`：把借来的切片直接存进结构体字段（引用是 Copy，不需要 clone）；
    ///   - 结构体实例的存活期不能超过它借用的数据，这条约束由 `'a` 记录在类型里；
    ///     实例活得比数据更久会报 `error[E0597]: `xxx` does not live long enough`。
    ///
    /// 示例输入：
    /// ```text
    /// part = "hello"
    /// ```
    /// 示例输出：
    /// ```text
    /// Excerpt { part: "hello" }
    /// ```
    fn new(part: &'a str) -> Self {
        // 引用是 Copy：直接存进字段，不需要 clone
        Self { part }
    }

    /// 【待实现】返回字段里的那一段（kp_13_04 的第二个方法）。
    ///
    /// 实现要求：
    ///   - 签名固定为 `fn part(&self) -> &str`，**不要**手写生命周期注解；
    ///   - 直接返回 `self.part`；
    ///   - 这里用的是省略规则 3：参数里有 `&self` 时，输出引用的生命周期取 `self` 的，
    ///     所以 `fn part(&self) -> &str` 等价于 `fn part<'s>(&'s self) -> &'s str`；
    ///     课程的 `fn part(&self) -> &'a str` 返回值可以比实例活得更久，本题按省略规则写即可。
    ///
    /// 示例输入：
    /// ```text
    /// self = Excerpt { part: "hello" }
    /// ```
    /// 示例输出：
    /// ```text
    /// "hello"
    /// ```
    fn part(&self) -> &str {
        // 省略规则 3：输出引用的生命周期取 &self 的
        self.part
    }

    /// 【待实现】打印/返回摘要（kp_13_04 的第三个方法）。
    ///
    /// 实现要求：
    ///   - 签名固定为 `fn announce(&self, msg: &str) -> &str`；
    ///   - **必须**返回 `self.part`；不许返回 `msg`；
    ///   - 返回值类型里的生命周期来自 `&self`（省略规则 3），与 `msg` 无关：返回 `msg` 会报
    ///     `error[E0515]`（或 `error[E0621]: explicit lifetime required in the type of `msg``），
    ///     因为 `msg` 的存活区间与 `self` 没有关系；
    ///   - 实现里没有用到 `msg` 时写一行 `let _ = msg.len();`，以免 `unused variable` 警告。
    ///
    /// 示例输入：
    /// ```text
    /// self = Excerpt { part: "hello" }, msg = "完全不同的消息"
    /// ```
    /// 示例输出：
    /// ```text
    /// "hello"
    /// ```
    fn announce(&self, msg: &str) -> &str {
        // 返回值借的是 self，不是 msg：msg 只读一下长度，避免 unused variable
        let _ = msg.len();
        self.part
    }
}

// ===========================================================================
// kp_13_05 省略规则不需要注解也能编译
// ===========================================================================

/// 知识点考核：靠省略规则写一个不出现 `'a` 的切片函数。
#[test]
fn kp_13_05_elision() {
    assess(
        M,
        "kp_13_05",
        "生命周期省略：签名里一个 'a 都不写，也能安全返回切片",
        Kind::Core,
        "复习 lesson_13 示例 3（省略规则）与示例 9（注解不会延长存活时间）：\
         只有一个输入引用时，输出引用的生命周期自动取它；\
         返回的切片仍然借用 `text`，所以调用方不能让 `text` 先失效。",
        || {
            eq(
                exercise_13_05_elision("  rust  "),
                "rust",
                "去掉首尾空白后返回中间的切片：\"  rust  \" → \"rust\"",
            );
            eq(
                exercise_13_05_elision("没有空白"),
                "没有空白",
                "没有空白时原样返回（多字节字符也安全：trim 只会落在字符边界上）",
            );
            // 边界：空串
            eq(
                exercise_13_05_elision(""),
                "",
                "边界：空串 trim 之后还是空串，不能 panic",
            );
            // 边界：全是空白字符
            eq(
                exercise_13_05_elision(" \t\n "),
                "",
                "边界：全部是空白字符时返回空串（trim 会把这些字符全部去掉）",
            );
            // 边界：只有首部空白 / 只有尾部空白
            eq(
                exercise_13_05_elision("  a"),
                "a",
                "边界：只有前导空白时返回 \"a\"",
            );
            eq(
                exercise_13_05_elision("a  "),
                "a",
                "边界：只有尾部空白时返回 \"a\"",
            );
        },
    );
}

/// 【待实现】用省略规则写一个返回切片的函数（签名里不出现生命周期参数）。
///
/// 实现要求：
///   - 签名固定为 `fn exercise_13_05_elision(text: &str) -> &str`（**不要**加 `<'a>`）；
///   - 返回 `text.trim()`（首尾空白都去掉，返回值仍然借用 `text`）；
///   - `text.trim()` 返回的切片借用 `text`，数据的所有者还在调用方手里，所以能安全返回；
///     这也再次说明：真正保证安全的是「被借用的数据活得更久」，注解本身不延长任何东西。
///
/// 示例输入：
/// ```text
/// text = "  rust  "
/// ```
/// 示例输出：
/// ```text
/// "rust"
/// ```
fn exercise_13_05_elision(text: &str) -> &str {
    // 返回的切片仍然借用 text，数据的所有者在调用方手里
    text.trim()
}

// ===========================================================================
// kp_13_06 'static 生命周期：引用活得够久
// ===========================================================================

/// 知识点考核：返回 `&'static str` 与它的字节长度。
#[test]
fn kp_13_06_static_lifetime() {
    assess(
        M,
        "kp_13_06",
        "'static：字符串字面量活在只读区，带出任何作用域都有效",
        Kind::Core,
        "复习 lesson_13 示例 7（'static 生命周期）：字符串字面量的类型就是 `&'static str`，\
         它的数据嵌在可执行文件里，程序全程有效；\
         要分清 `&'static T`（引用活得够久）与 `T: 'static`（类型内部不含短生命周期借用，\
         拥有所有权的 String、i32 都满足）这两种完全不同的含义。",
        || {
            let (text, len) = exercise_13_06_static_lifetime();
            eq(
                text,
                "hello-rust-lessons",
                "返回的必须是字符串字面量 \"hello-rust-lessons\"（类型是 &'static str）",
            );
            eq(
                len,
                18usize,
                "\"hello-rust-lessons\" 的字节长度是 18（len() 数的是字节，不是字符）",
            );
            // 边界：'static 引用不借用任何局部数据，所以可以带出内层块继续使用
            let kept;
            {
                kept = text;
            }
            eq(
                kept,
                "hello-rust-lessons",
                "边界：`&'static str` 的存活期与任何局部作用域无关，可以带出内层块继续用",
            );
        },
    );
}

/// 【待实现】返回一个 `'static` 引用与它的字节长度。
///
/// 实现要求：
///   - 签名固定为 `fn exercise_13_06_static_lifetime() -> (&'static str, usize)`；
///   - 返回 `("hello-rust-lessons", 18)`，其中长度用 `"...".len()` 算出来（不要写死数字 18，
///     否则就失去了「量一遍」的意义；两者都要对得上）；
///   - 字符串字面量天生就是 `&'static str`；局部变量的引用**永远**借不出 `'static`
///     （`error[E0515]: cannot return reference to local variable`），想拿到 `'static` 的
///     `String` 引用只能主动泄漏（`String::from("x").leak()`）；
///   - 注意区分两种含义：`&'static str` 说的是「这个引用活得够久」；`String: 'static` 说的是
///     「String 类型里没有短命的借用」，并不代表某个具体的 String 值会活到程序结束。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ("hello-rust-lessons", 18)
/// ```
fn exercise_13_06_static_lifetime() -> (&'static str, usize) {
    // 字符串字面量天生是 &'static str；长度用 len() 量出来，不写死数字
    let text: &'static str = "hello-rust-lessons";
    (text, text.len())
}

// ===========================================================================
// kp_13_07 生命周期参数与 trait bound 组合
// ===========================================================================

/// 知识点考核：`<'a, T: Display + 'a>` 同时使用生命周期参数与 trait 约束。
#[test]
fn kp_13_07_lifetime_bound() {
    assess(
        M,
        "kp_13_07",
        "生命周期参数 + trait bound：T: Display + 'a，返回拥有所有权的 String",
        Kind::Hard,
        "复习 lesson_13 示例 8（生命周期与 trait bound）：`<'a, T: Display + 'a>` 里\
         `'a` 是生命周期参数、`T` 是类型参数，`T: 'a` 约束表示「T 里不含比 'a 更短的借用」；\
         返回 `String` 与 `'a` 无关，所以调用方拿到的东西不受借用区间限制。",
        || {
            let number = 42i32;
            eq(
                exercise_13_07_lifetime_bound(&number),
                String::from("42"),
                "i32 满足 Display + 'a：格式化 42 得 \"42\"",
            );
            let word = String::from("rust");
            eq(
                exercise_13_07_lifetime_bound(&word),
                String::from("rust"),
                "String 也满足 Display + 'a：格式化后就是它本身的内容",
            );
            // 边界：0 与空串
            let zero = 0i32;
            eq(
                exercise_13_07_lifetime_bound(&zero),
                String::from("0"),
                "边界：整数 0 也要打印成 \"0\"（不能变成空串）",
            );
            let empty = String::new();
            eq(
                exercise_13_07_lifetime_bound(&empty),
                String::new(),
                "边界：空 String 格式化后仍是空串",
            );
            // 边界：借用只活在内层块里也没问题 —— 返回的是拥有所有权的 String
            let owned = {
                let temporary = String::from("临时");
                exercise_13_07_lifetime_bound(&temporary)
            };
            eq(
                owned,
                String::from("临时"),
                "边界：返回值是 String（拥有所有权），所以临时变量离开作用域后它依然有效",
            );
        },
    );
}

/// 【待实现】把任意可打印的值格式化成 `String`。
///
/// 实现要求：
///   - 签名固定为
///     `fn exercise_13_07_lifetime_bound<'a, T: std::fmt::Display + 'a>(value: &'a T) -> String`；
///   - 返回 `format!("{value}")`；
///   - `'a` 管「引用活多久」，`T: 'a` 管「T 内部不含更短的借用」；因为返回的是拥有所有权的
///     `String`，调用方可以把结果带出借用区间（这与返回 `&'a str` 完全不同）。
///
/// 示例输入：
/// ```text
/// value = &42
/// ```
/// 示例输出：
/// ```text
/// "42"
/// ```
fn exercise_13_07_lifetime_bound<'a, T: std::fmt::Display + 'a>(value: &'a T) -> String {
    // 返回拥有所有权的 String：与 'a 无关，可以带出借用区间
    format!("{value}")
}

// ===========================================================================
// kp_13_08 常见错误诊断：读得懂编译器报错，才改得动代码
// ===========================================================================

/// 知识点考核：按课程示例 11 的注释顺序写出前三个错误编号。
#[test]
fn kp_13_08_diagnose_errors() {
    assess(
        M,
        "kp_13_08",
        "常见错误诊断：E0106 缺生命周期注解（课程前三个错误都是它）",
        Kind::Hard,
        "复习 lesson_13 示例 11（common_mistakes）：前三个坑分别是「两个引用参数省略了返回值注解」\
         「结构体字段是引用却没写生命周期参数」「返回局部变量的引用」——\
         这三处编译器的诊断都是 missing lifetime specifier（E0106）；\
         再往后才是 E0515（返回局部变量）、E0621（漏标一个参数）、E0597（被借用数据活得太短）。",
        || {
            let codes = exercise_13_08_diagnose_errors();
            eq_slice(
                &codes,
                &["E0106", "E0106", "E0106"],
                "顺序必须是课程示例 11 的前三个错误：缺返回值的生命周期注解、结构体缺生命周期参数、\
                 返回局部变量引用 —— 它们的错误编号都是 E0106",
            );
            // 边界：错误编号的书写格式必须规范（E + 4 位数字），例如 "E0106" 而不是 "106"
            is_true(
                codes
                    .iter()
                    .all(|code| code.starts_with('E') && code.len() == 5),
                "边界：每一项都要写成 `E` 加 4 位数字（如 \"E0106\"），不能省略前缀或前导零",
            );
        },
    );
}

/// 【待实现】写出三个场景对应的编译器错误编号。
///
/// 场景（与课程示例 11 的前三个错误一致）：
///   1. `fn longest(x: &str, y: &str) -> &str` —— 两个引用参数却省略了返回值的生命周期；
///   2. `struct Excerpt { part: &str }` —— 结构体字段是引用却没有生命周期参数；
///   3. `fn dangling() -> &String { let s = String::from("临时"); &s }` —— 想返回局部变量的引用。
///
/// 实现要求：
///   - 返回 3 个错误编号字符串（形如 `"E0106"`），顺序与上面一致；
///   - 这三个场景给出的都是同一个编号：只要「有引用却没有说明它从哪来、活多久」，编译器第一步
///     都先报 missing lifetime specifier；补注解时才会暴露更深层的问题
///     （例如场景 3 补完注解就变成 E0515：不能返回局部变量的引用）。
///
/// 示例输入：
/// ```text
/// （无参数）
/// ```
/// 示例输出：
/// ```text
/// ["E0106", "E0106", "E0106"]
/// ```
fn exercise_13_08_diagnose_errors() -> [&'static str; 3] {
    // 课程示例 11 的前三个坑：缺返回值注解 / 结构体缺参数 / 返回局部变量引用
    // 编译器第一步都先报 missing lifetime specifier（E0106）
    ["E0106", "E0106", "E0106"]
}

// ===========================================================================
// kp_13_09 结构体持有引用：Config<'a> 的 impl
// ===========================================================================

/// 知识点考核：结构体借用外部字符串，`endpoint` 返回 String，`host` 用省略规则。
#[test]
fn kp_13_09_config_methods() {
    assess(
        M,
        "kp_13_09",
        "结构体持有引用：Config<'a> 的 new / endpoint / host",
        Kind::Hard,
        "复习 lesson_13 示例 10（典型场景：借用式配置）：配置项常常来自命令行参数或环境变量，\
         本来就是长命的 String，用 `&'a str` 借用可以省掉一次分配；\
         `endpoint()` 返回拥有所有权的 `String`，`host()` 用省略规则返回借用 `self` 的 `&str`。",
        || {
            // 直接构造一次并读字段：既确认字段语义，也让 host / port 在骨架态不触发 dead_code
            let probe = Config {
                host: "127.0.0.1",
                port: 8080,
            };
            is_true(
                probe.host == "127.0.0.1" && probe.port == 8080,
                "字段 host / port 就是拼端点用的两段：本例是 \"127.0.0.1\" 与 8080",
            );
            // 正常用例
            let host = String::from("10.0.0.7");
            let config = Config::new(host.as_str(), 8080);
            eq(
                config.endpoint(),
                String::from("10.0.0.7:8080"),
                "endpoint() 返回 `host:port`（中间一个冒号，端口用十进制）",
            );
            eq(
                config.host(),
                "10.0.0.7",
                "host() 返回字段里的引用（省略规则 3：返回值借用 self，不借用任何局部数据）",
            );
            // 边界：端口 0 与空 host 也要原样拼接
            let edge = Config::new("", 0);
            eq(
                edge.endpoint(),
                String::from(":0"),
                "边界：host 为空串、端口为 0 时必须原样拼成 \":0\"（不能省略、补零或加默认值）",
            );
        },
    );
}

impl<'a> Config<'a> {
    /// 【待实现】构造 `Config`（kp_13_09 的第一个方法）。
    ///
    /// 实现要求：
    ///   - 签名固定为 `fn new(host: &'a str, port: u16) -> Self`；
    ///   - 返回 `Self { host, port }`（字段名与参数名同名，可以直接用字段简写）；
    ///   - `host` 是借用进来的，`port` 是 Copy 的值；类型里的 `'a` 记录的是
    ///     「实例不会比借来的 host 活得更久」，它不会延长 host 的寿命。
    ///
    /// 示例输入：
    /// ```text
    /// host = "10.0.0.7", port = 8080
    /// ```
    /// 示例输出：
    /// ```text
    /// Config { host: "10.0.0.7", port: 8080 }
    /// ```
    fn new(host: &'a str, port: u16) -> Self {
        // host 是借来的 &'a str，port 是 Copy 的值：字段简写
        Self { host, port }
    }

    /// 【待实现】拼出 `host:port`（kp_13_09 的第二个方法）。
    ///
    /// 实现要求：
    ///   - 签名固定为 `fn endpoint(&self) -> String`；
    ///   - 返回 `format!("{}:{}", self.host, self.port)`：`{}:{}` 中间那个冒号是字面量，
    ///     两边的花括号分别对应 `self.host` 与 `self.port`；
    ///   - 返回的是拥有所有权的 String，所以它不借用 self，可以比 Config 实例活得更久。
    ///
    /// 示例输入：
    /// ```text
    /// self = Config { host: "10.0.0.7", port: 8080 }
    /// ```
    /// 示例输出：
    /// ```text
    /// "10.0.0.7:8080"
    /// ```
    fn endpoint(&self) -> String {
        // 返回拥有所有权的 String：不借用 self
        format!("{}:{}", self.host, self.port)
    }

    /// 【待实现】返回主机名（kp_13_09 的第三个方法）。
    ///
    /// 实现要求：
    ///   - 签名固定为 `fn host(&self) -> &str`，**不要**手写生命周期注解；
    ///   - 直接返回 `self.host`；
    ///   - 省略规则 3（有 `&self` 时输出引用取 self 的生命周期）让它不需要任何 `'a`；
    ///     如果要写成「返回 `&'a str`、可以比实例活得更久」，就必须显式写
    ///     `fn host(&self) -> &'a str`（课程示例 6 的 `part()` 就是这种写法）。
    ///
    /// 示例输入：
    /// ```text
    /// self = Config { host: "10.0.0.7", port: 8080 }
    /// ```
    /// 示例输出：
    /// ```text
    /// "10.0.0.7"
    /// ```
    fn host(&self) -> &str {
        // 省略规则 3：返回值借用 self
        self.host
    }
}
