# Rust 课程文件统一编写规范（所有 lesson_XX_主题.rs 必须遵守）

目标环境：Rust 1.99 stable，edition 2024。禁止 nightly / 实验性特性 / 未稳定 API。

## 1. 文件骨架（严格照抄此结构）

```rust
//! lesson_XX_主题.rs —— 主题：<中文主题名>（例如：变量与可变性）
//!
//! 学习目标：
//!   1. <目标一>
//!   2. <目标二>
//!   3. <目标三>
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_XX_主题.rs -o lesson_XX_主题 && ./lesson_XX_主题
//!   或在本项目根目录执行：cargo run --bin lesson_XX_主题
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

fn main() {
    println!("========== lesson_XX_主题：<主题> ==========\n");

    demo_1_xxx();
    demo_2_xxx();
    // ... 依次调用
}

/// 示例 1：<示例名>
///
/// 要点：<一句话说明这个示例想证明什么>
fn demo_1_xxx() {
    println!("--- 示例 1：<示例名> ---");
    // 详细行内注释，解释每一行关键代码
    println!("// 预期输出：...");
}
```

要求：
- 文件名统一为 `lesson_<两位序号>_<英文主题>.rs`（例如 `lesson_01_variables_mutability.rs`），全部放在 `src/tutorial/` 目录下；
  **二进制名与文件名（去掉 .rs）保持一致**，在根 `Cargo.toml` 中显式注册 `[[bin]]`（本项目设置了 `autobins = false`）。
  序号 01~18 连续（01~13 语言基础，14~18 补充进阶），学习指南类 Markdown 放在 `src/tutorial/guides/`。
- 每个文件必须以 `//!` 内部文档注释开头（文件头注释）。
- 必须有且仅有一个 `fn main()`，从 main 调用所有 demo 函数，main 中不写业务逻辑。
- 每个知识点拆成独立的 `fn demo_N_xxx()`，函数名用蛇形命名，禁止 `fn demo1()` 这类无意义名字。
- 每个 demo 开头打印 `--- 示例 N：<标题> ---`，便于阅读运行输出。

## 2. 注释与预期输出

- 关键行必须有行内注释 `//`，解释"为什么"而不只是"是什么"。
- 每个有输出价值的语句块后，写明预期输出，格式统一为：
  `println!("// 预期输出：<内容>");`
  或对多行结果用紧邻的 `// 预期输出：` 注释块。
- **"预期输出"必须是字面量断言**（硬性约定）：
  1. 期望值全部写死，**禁止插值**（不能写 `println!("// 预期输出：x = {x}")`，那等于让程序自己证明自己）；
  2. 期望值必须与**紧邻上方的真实输出行逐字节一致**（多行输出就逐行各写一条）；
  3. 补充说明文字（"说明：…""注意：…"）**不得**使用 `// 预期输出：` 前缀，直接作为普通输出行打印并在上方注释里说明；
  4. 处理字符串时用**普通字符串**并正确转义 `{{` / `}}`；不要用 `r#"..."#` 原始字符串写期望值（原始字符串里的 `{{` 会原样打印成两个花括号）。
- 每个文件至少包含一个"典型使用场景"小节（真实场景：解析配置、统计词频、状态机等）。
- 每个文件包含"常见错误示例"部分：把错误代码放在 `//` 注释里，或者放在
  `fn demo_N_common_mistakes()` 中用注释逐条说明；**错误代码必须注释掉**，
  并在注释中写明编译错误信息（例如 `// error[E0382]: borrow of moved value: 's'`）与修正方法。
- 不要使用 `#[allow(dead_code)]` 之类的技巧掩盖问题；所有定义的项都必须被用到，
  保证 `cargo build` 无 warning。

## 3. 编码规范（rustfmt 兼容）

- 4 空格缩进，无 tab；行尾不留空格；文件末尾保留一个换行。
- 函数/变量：`snake_case`；类型/trait/枚举：`UpperCamelCase`；常量/静态：`SCREAMING_SNAKE_CASE`。
- 使用 `println!` 宏时优先用内联参数捕获：`println!("x = {x}")`。
- 避免 `unwrap()` 出现在正常路径上（教学示例展示 `Result` 时可用，但需注释说明）。
- 保持代码与 `rustfmt` 默认输出一致（可本地执行 `rustfmt --edition 2024 <file>` 校验）。

## 4. edition 2024 注意事项（重要）

- `gen` 是保留关键字，不能作为标识符。
- match 默认绑定模式下不允许 `ref` / `ref mut`，改用 `&` / `&mut` 模式或直接按值匹配。
- `unsafe` 属性需写成 `#[unsafe(no_mangle)]` 形式（本课程尽量不涉及 unsafe）。
- `impl Trait` 返回类型、async 等高级用法保持最简，避免边界行为差异。
- 结构体字段初始化简写、`let ... else`、`if let` 链式等稳定特性可放心使用。

## 5. 教学顺序与主题分工（不要越界抢内容）

| 文件 | 主题 |
| --- | --- |
| lesson_01_variables_mutability.rs | 变量与可变性：let、mut、shadowing、const、static、类型推断与标注 |
| lesson_02_data_types.rs | 数据类型：整数/浮点/布尔/字符、元组与数组、类型转换 as / From / TryFrom、字面量后缀 |
| lesson_03_functions.rs | 函数：定义、参数、返回值、表达式与语句区别、函数指针、发散函数 `!` |
| lesson_04_control_flow.rs | 流程控制：if 表达式、loop/while/for、break 带值、continue、循环标签 |
| lesson_05_ownership_borrowing.rs | 所有权：三条规则、move/Copy/Clone、栈与堆、引用与借用规则、可变引用、切片、Drop |
| lesson_06_structs.rs | 结构体：定义、实例化、字段初始化简写、更新语法、元组结构体、方法、关联函数、Debug/Display |
| lesson_07_enums_pattern_matching.rs | 枚举与模式匹配：枚举带数据、Option、Result、match、通配、守卫、绑定、if let、while let、let else |
| lesson_08_collections.rs | 常见集合：Vec、String、HashMap 的创建与常用方法、迭代、entry API、所有权与借用陷阱 |
| lesson_09_packages_modules.rs | 包与模块：mod、pub、可见性、use、路径、as 重命名、pub use、嵌套模块、文件与目录模块 |
| lesson_10_error_handling.rs | 错误处理：panic!/unwrap/expect、Result、? 与 From 转换、自定义错误类型、Box<dyn Error>、main 返回 Result |
| lesson_11_generics.rs | 泛型：泛型函数、结构体、枚举、方法、单态化、trait bounds、where 子句、const 泛型 |
| lesson_12_traits.rs | Trait：定义、实现、默认方法、trait 作为参数（impl Trait / 泛型）、返回 impl Trait、关联类型、trait 对象 dyn |
| lesson_13_lifetimes.rs | 生命周期：为什么需要、函数签名注解、结构体持有引用、省略规则、'static、生命周期与 trait bound |

内容边界：只讲本文件主题，可调用此前文件已讲过的基础知识（不一定重复解释）。
不要提前讲后面文件的核心内容（例如 01 里不要讲所有权细节）。
