# 04 · 知识点覆盖矩阵（考核 ↔ 课程 README 对应表）

> 本文件由 `cargo run --bin assessment_report -- --emit-map` 自动生成，请勿手工编辑；
> 修改考核用例后重新运行该命令即可。

本矩阵说明考核体系「考什么、在哪考、对应课程哪一段」：

- 每一行是一个知识点，`考核代码位置` 是 `assess(...)` 的调用点（`文件:行号`），打开该文件即可看到考核标准与对应的练习函数；
- `对应 README 出处` 取自考核文件里的复习指引，指向课程中讲这个知识点的段落，卡住时按这一列去读原文；
- 类型含义：**基础**（本课最先要求掌握的概念）/ **核心**（本课主要知识点）/ **难点**（新人最容易卡住处，允许对照示例反复尝试）/ **边界**（空输入、极值、多字节字符等容易被忽略的边界条件）；
- 数据来源：账本 `assessments/report/data/ledger.tsv` 的**全部运行并集**，因此新增知识点后要先跑一次对应考核，再重新生成本文件。

## 一、知识点总览

| 模块 | 模块名 | 知识点 | 知识点标题 | 类型 | 考核代码位置 | 对应 README 出处 |
| --- | --- | --- | --- | --- | --- | --- |
| lesson_01 | 变量与可变性 | kp_01_01 | let 绑定与 mut：默认不可变，需要改值就写 mut | 基础 | assessments\lesson_01_variables_mutability.rs:99 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_02 | 变量遮蔽（shadowing）：同名 let 覆盖旧绑定 | 基础 | assessments\lesson_01_variables_mutability.rs:166 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_03 | 遮蔽可以改变类型，mut 不能（这一条是两者的本质区别） | 核心 | assessments\lesson_01_variables_mutability.rs:229 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_04 | const 与 static：编译期常量 vs 静态变量（命名用 SCREAMING_SNAKE_CASE） | 核心 | assessments\lesson_01_variables_mutability.rs:279 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_05 | 类型推断与显式标注：能推断就推断，推断不了必须标注 | 核心 | assessments\lesson_01_variables_mutability.rs:334 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_06 | 作用域与遮蔽：内层 let 遮蔽不影响外层绑定的值 | 边界 | assessments\lesson_01_variables_mutability.rs:381 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_07 | const 表达式：单位换算在编译期完成，零运行期开销 | 边界 | assessments\lesson_01_variables_mutability.rs:431 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_01 | 变量与可变性 | kp_01_08 | 常见错误诊断：E0384 / E0308 / E0381 分别对应哪类错误 | 难点 | assessments\lesson_01_variables_mutability.rs:481 | src/tutorial/README.md 第一阶段「01 变量与可变性」 |
| lesson_02 | 数据类型 | kp_02_01 | 整数类型与字面量：位宽决定取值范围，字面量用后缀指定类型 | 基础 | assessments\lesson_02_data_types.rs:112 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_02 | 整数溢出：wrapping_add / saturating_add / overflowing_add / checked_add 的区别 | 边界 | assessments\lesson_02_data_types.rs:186 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_03 | 浮点基础：默认 f64、0.1 + 0.2 != 0.3、NaN 不等于任何值（包括自身） | 基础 | assessments\lesson_02_data_types.rs:255 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_04 | 布尔与字符：bool 只有 true/false，char 是 4 字节的 Unicode 标量值 | 基础 | assessments\lesson_02_data_types.rs:323 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_05 | 元组解构：一次把元组拆成多个变量，再参与运算 | 核心 | assessments\lesson_02_data_types.rs:387 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_06 | 数组安全访问：get 返回 Option，比 [] 索引更安全（越界不 panic） | 核心 | assessments\lesson_02_data_types.rs:460 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_07 | 类型转换：`as` 静默截断，`u8::try_from` 做范围检查并返回 Result | 核心 | assessments\lesson_02_data_types.rs:557 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_02 | 数据类型 | kp_02_08 | 元组比较与数组长度：元组按字典序逐元素比较，数组长度是类型的一部分 | 边界 | assessments\lesson_02_data_types.rs:624 | src/tutorial/README.md 第一阶段「02 数据类型」 |
| lesson_03 | 函数 | kp_03_01 | 函数定义：参数逐个标注类型，尾表达式就是返回值 | 基础 | assessments\lesson_03_functions.rs:114 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_02 | 多返回值：用一个元组同时带回面积与周长 | 基础 | assessments\lesson_03_functions.rs:184 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_03 | 语句与表达式：`if` 是表达式，放在函数体末尾就是返回值 | 核心 | assessments\lesson_03_functions.rs:253 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_04 | 提前 return + 尾表达式：先挡掉非法输入，再走正常路径 | 核心 | assessments\lesson_03_functions.rs:322 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_05 | 函数指针：`fn(i32) -> i32` 是类型，函数名与不捕获环境的闭包都能当值传 | 核心 | assessments\lesson_03_functions.rs:389 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_06 | 发散函数（`!`）：永不返回的函数可被强制转换成任意类型 | 难点 | assessments\lesson_03_functions.rs:50 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_07 | 函数指针数组：按顺序把每个函数作用到 value 上（空数组原样返回） | 边界 | assessments\lesson_03_functions.rs:565 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_03 | 函数 | kp_03_08 | 常见错误诊断：函数课前三个坑分别报什么（两个无编号解析错误 + E0308） | 难点 | assessments\lesson_03_functions.rs:624 | src/tutorial/README.md 第一阶段「03 函数」 |
| lesson_04 | 流程控制 | kp_04_01 | if 是表达式：分支的值可以直接做函数尾表达式 | 基础 | assessments\lesson_04_control_flow.rs:114 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_02 | if / else if 链：条件自上而下判断，命中第一个为真的分支 | 核心 | assessments\lesson_04_control_flow.rs:206 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_03 | loop 与 break 带值：把循环当成有返回值的表达式 | 核心 | assessments\lesson_04_control_flow.rs:276 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_04 | while 循环与 continue：跳过奇数，只累加偶数 | 核心 | assessments\lesson_04_control_flow.rs:352 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_05 | for 遍历切片：元素个数由迭代器决定，不需要手写下标 | 基础 | assessments\lesson_04_control_flow.rs:428 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_06 | 循环标签（'outer:）：break 'outer 一次跳出两层嵌套循环 | 难点 | assessments\lesson_04_control_flow.rs:505 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_07 | 循环实现 Collatz 步数：偶数减半、奇数 3n+1，数到 1 为止 | 边界 | assessments\lesson_04_control_flow.rs:576 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_04 | 流程控制 | kp_04_08 | 常见错误诊断：E0308（loop 的值）/ E0571（while 里 break 带值）/ E0425（循环外用了循环变量） | 难点 | assessments\lesson_04_control_flow.rs:632 | src/tutorial/README.md 第一阶段「04 流程控制」 |
| lesson_05 | 所有权系统 | kp_05_01 | move 语义：String 传参即移交所有权，函数通过返回值把所有权交还 | 核心 | assessments\lesson_05_ownership_borrowing.rs:153 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_02 | Copy 与 Clone：i32 赋值是复制、String 必须显式 .clone() | 核心 | assessments\lesson_05_ownership_borrowing.rs:263 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_03 | 共享借用：同一时刻可以存在任意多个 &T（只读共享是安全的） | 核心 | assessments\lesson_05_ownership_borrowing.rs:335 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_04 | 可变借用：两次 &mut 只要不重叠就合法（每次借用用完即结束） | 难点 | assessments\lesson_05_ownership_borrowing.rs:417 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_05 | 返回切片：最长单词是原句的一部分，返回 &str 而不是 String | 核心 | assessments\lesson_05_ownership_borrowing.rs:514 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_06 | len 与 capacity：len 是已有元素个数，capacity 是已分配的坑位数 | 基础 | assessments\lesson_05_ownership_borrowing.rs:566 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_07 | NLL 边界：先把值取出来（借用结束），再可变借用修改 | 边界 | assessments\lesson_05_ownership_borrowing.rs:661 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_08 | 常见错误诊断：E0382（move 后使用）/ E0499（两个可变借用）/ E0502（不可变与可变重叠） | 难点 | assessments\lesson_05_ownership_borrowing.rs:715 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_05 | 所有权系统 | kp_05_09 | Drop 顺序：作用域结束时后声明的先 drop，内层作用域先于外层 | 难点 | assessments\lesson_05_ownership_borrowing.rs:770 | src/tutorial/README.md 第二阶段「05 所有权系统」 |
| lesson_06 | 结构体 | kp_06_01 | 结构体定义与实例化：字段写全、用点号访问 | 基础 | assessments\lesson_06_structs.rs:151 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_02 | 字段初始化简写与结构体更新语法（..other） | 核心 | assessments\lesson_06_structs.rs:249 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_03 | 方法接收者：&self 读、&mut self 改、self 消费 | 核心 | assessments\lesson_06_structs.rs:345 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_04 | 关联函数与 Self：用 `类型名::函数名` 调用，不依赖实例 | 核心 | assessments\lesson_06_structs.rs:457 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_05 | 元组结构体（.0 位置访问）与单元结构体（零大小标记） | 基础 | assessments\lesson_06_structs.rs:540 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_06 | 手动实现 Display（{}）与派生 Debug（{:?}）的分工 | 核心 | assessments\lesson_06_structs.rs:610 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_07 | 链式调用：构造器方法消费 self 并返回 Self | 难点 | assessments\lesson_06_structs.rs:681 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_06 | 结构体 | kp_06_08 | 部分移动：move 出 String 字段的同时仍可读取 Copy 字段 | 边界 | assessments\lesson_06_structs.rs:799 | src/tutorial/README.md 第二阶段「06 结构体」 |
| lesson_07 | 枚举与模式匹配 | kp_07_01 | match 的穷尽性：每个变体都要有分支 | 基础 | assessments\lesson_07_enums_pattern_matching.rs:156 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_02 | Option<T>：把「可能没有值」写进类型里 | 基础 | assessments\lesson_07_enums_pattern_matching.rs:231 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_03 | Result<T, E>：成功取 Ok，失败给出可读原因 | 核心 | assessments\lesson_07_enums_pattern_matching.rs:319 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_04 | 或模式 `\|` 与通配 `_`：合并同结果分支、兜住其余取值 | 核心 | assessments\lesson_07_enums_pattern_matching.rs:401 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_05 | 匹配守卫（if）与 @ 绑定：范围判断 + 取到原值 | 难点 | assessments\lesson_07_enums_pattern_matching.rs:486 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_06 | while let：不断取出直到 None，None 用 continue 跳过 | 核心 | assessments\lesson_07_enums_pattern_matching.rs:559 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_07 | let else：失败分支发散，成功路径无需再嵌套 | 难点 | assessments\lesson_07_enums_pattern_matching.rs:631 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_08 | 枚举 + match 的典型用法：文本 → 结构化命令 | 难点 | assessments\lesson_07_enums_pattern_matching.rs:726 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_07 | 枚举与模式匹配 | kp_07_09 | 常见错误诊断：E0004 / E0308 / E0005 分别对应哪类错误 | 难点 | assessments\lesson_07_enums_pattern_matching.rs:786 | src/tutorial/README.md 第二阶段「07 枚举与模式匹配」 |
| lesson_08 | 常见集合 | kp_08_01 | Vec 的创建与 push：`to_vec()` 复制出拥有所有权的向量，`push` 在尾部追加 | 基础 | assessments\lesson_08_collections.rs:120 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_02 | Vec 的索引与安全访问：`get` / `first` 返回 `Option`，`[]` 越界会 panic | 核心 | assessments\lesson_08_collections.rs:191 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_03 | Vec 的原地修改：`retain` 按条件删除、`iter_mut` 解引用修改元素 | 核心 | assessments\lesson_08_collections.rs:261 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_04 | String 与 UTF-8：`len()` 是字节数，`chars().count()` 是字符数 | 核心 | assessments\lesson_08_collections.rs:337 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_05 | 字符串拼接三种方式：`push_str` 原地追加、`+` 消耗左侧、`format!` 不消耗任何操作数 | 核心 | assessments\lesson_08_collections.rs:434 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_06 | HashMap 基础：`insert` 覆盖旧值并返回旧值；遍历顺序不固定，输出前必须排序 | 核心 | assessments\lesson_08_collections.rs:508 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_07 | entry API 统计词频：`entry(word).or_insert(0)` 一次查找完成「查 + 改」，输出前排序 | 难点 | assessments\lesson_08_collections.rs:595 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_08 | 借用陷阱：`get` 拿到引用期间不能 `insert`（E0502），先把值拷贝出来即可 | 难点 | assessments\lesson_08_collections.rs:677 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_08 | 常见集合 | kp_08_09 | 常见错误诊断：E0596 / E0282 / E0277 分别对应哪类集合错误 | 难点 | assessments\lesson_08_collections.rs:732 | src/tutorial/README.md 第三阶段「08 常见集合」 |
| lesson_09 | 包和模块 | kp_09_01 | 模块基础：`mod math { pub fn ... }` + 路径 `math::add` 调用 | 基础 | assessments\lesson_09_packages_modules.rs:107 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_09 | 包和模块 | kp_09_02 | 可见性层级：`pub(crate)`（本 crate 可见）/ `pub(super)`（父模块可见）/ 私有（仅本模块可见） | 核心 | assessments\lesson_09_packages_modules.rs:179 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_09 | 包和模块 | kp_09_03 | `use` 引入与 `as` 重命名：路径别名不改变可见性，只改调用时写的名字 | 核心 | assessments\lesson_09_packages_modules.rs:239 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_09 | 包和模块 | kp_09_04 | `pub use` 重导出：门面模块让调用方只记住一条短路径 | 核心 | assessments\lesson_09_packages_modules.rs:310 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_09 | 包和模块 | kp_09_05 | 模块路径：`super::` 父模块、`self::` 当前模块、`crate::` 绝对路径（crate 根） | 核心 | assessments\lesson_09_packages_modules.rs:372 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_09 | 包和模块 | kp_09_06 | 迷你库：`pub mod` 暴露 API、私有辅助函数隐藏实现、`pub use` 汇总成一条门面 | 难点 | assessments\lesson_09_packages_modules.rs:453 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_09 | 包和模块 | kp_09_07 | 常见错误诊断：E0603（私有项）/ E0425（路径写错）/ E0255（use 与本地定义同名） | 难点 | assessments\lesson_09_packages_modules.rs:512 | src/tutorial/README.md 第三阶段「09 包和模块」 |
| lesson_10 | 错误处理 | kp_10_01 | 可恢复错误：用 Result 表达「除数为 0」，把决定权交给调用方 | 基础 | assessments\lesson_10_error_handling.rs:204 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_02 | unwrap / expect：成功时取值，失败时 panic（风险就在这里） | 基础 | assessments\lesson_10_error_handling.rs:268 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_03 | 用 match 处理 Result：两个分支都必须处理，编译器强制你面对错误 | 核心 | assessments\lesson_10_error_handling.rs:339 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_04 | `?` 运算符：Ok 时取值继续执行，Err 时立刻 return Err(...) | 核心 | assessments\lesson_10_error_handling.rs:426 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_05 | 自定义错误类型：用枚举区分失败原因，用 Display 说人话 | 核心 | assessments\lesson_10_error_handling.rs:527 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_06 | From 上转：`?` 自动把 ParseError 转换成 AppError | 难点 | assessments\lesson_10_error_handling.rs:606 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_07 | Box<dyn Error> 统一出口：`?` 把 ParseIntError 自动装箱 | 核心 | assessments\lesson_10_error_handling.rs:686 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_08 | 典型场景：解析 key=value 配置，跳过空行与 # 注释行，坏行错误信息保留原文 | 难点 | assessments\lesson_10_error_handling.rs:780 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_10 | 错误处理 | kp_10_09 | 常见错误诊断：`?` 用错位置、对 Option 用 `?`、缺 From 分别报什么错 | 难点 | assessments\lesson_10_error_handling.rs:836 | src/tutorial/README.md 第三阶段「10 错误处理」 |
| lesson_11 | 泛型 | kp_11_01 | 泛型函数：`fn largest<T: PartialOrd>(list: &[T]) -> Option<&T>` 一份代码适配多种类型 | 基础 | assessments\lesson_11_generics.rs:133 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_02 | 泛型结构体：`struct Point<T>` 能生成 Point<i32>、Point<f64> 等互相独立的具体类型 | 核心 | assessments\lesson_11_generics.rs:189 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_03 | 泛型枚举：标准库的 Option<T> / Result<T, E> 就是泛型枚举，自己定义的也能互相转换 | 核心 | assessments\lesson_11_generics.rs:251 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_04 | 泛型方法：impl<T: Copy + Add<Output = T>> Pair<T> 让 sum() 对两种数值类型都可用 | 核心 | assessments\lesson_11_generics.rs:308 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_05 | trait bound 用起来：`<T: Display, U: Display>` 让两种不同类型的值拼成一句话 | 核心 | assessments\lesson_11_generics.rs:373 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_06 | where 子句：约束放在函数签名下方，比挤在尖括号里更好读 | 核心 | assessments\lesson_11_generics.rs:452 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_07 | 多个泛型参数：`<K: Display, V: Display>` 让键与值各自是不同类型 | 核心 | assessments\lesson_11_generics.rs:518 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_08 | const 泛型：`<const N: usize>` 把编译期常量也作为参数，数组长度是类型的一部分 | 边界 | assessments\lesson_11_generics.rs:589 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_09 | 单态化（monomorphization）：泛型在编译期展开，运行期没有类型判断开销 | 边界 | assessments\lesson_11_generics.rs:656 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_11 | 泛型 | kp_11_10 | 常见错误诊断：泛型里直接相加、用 {} 打印未约束 Display、类型参数没被使用 | 难点 | assessments\lesson_11_generics.rs:710 | src/tutorial/README.md 第四阶段「11 泛型」 |
| lesson_12 | Trait | kp_12_01 | 为自定义类型实现 trait：impl Area for Circle 与 impl Area for Square | 核心 | assessments\lesson_12_traits.rs:159 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_02 | 默认方法：只实现必需方法，就能得到 shout() 的默认行为 | 核心 | assessments\lesson_12_traits.rs:251 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_03 | trait 作为参数（写法一）：item: &impl Area 接收任意实现者 | 核心 | assessments\lesson_12_traits.rs:316 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_04 | trait 作为参数（写法二）：泛型参数 + trait bound，可 turbofish | 核心 | assessments\lesson_12_traits.rs:382 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_05 | 返回 impl Trait：调用方只依赖 Area，看不到具体类型 | 核心 | assessments\lesson_12_traits.rs:448 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_06 | 返回 Box<dyn Area>：分支返回不同类型时必须用 trait 对象 | 难点 | assessments\lesson_12_traits.rs:519 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_07 | 多个 trait bound：T: Area + Debug，既能算面积又能 {:?} 打印 | 核心 | assessments\lesson_12_traits.rs:582 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_08 | 关联类型：type Item = u32，调用处不需要写类型标注 | 难点 | assessments\lesson_12_traits.rs:685 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_09 | dyn trait 对象集合：Vec<Box<dyn Area>> 排序后返回面积列表 | 难点 | assessments\lesson_12_traits.rs:768 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_10 | 派生标准库 trait：Default 置零、PartialEq 逐字段比较、Debug 的固定格式 | 核心 | assessments\lesson_12_traits.rs:838 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_12 | Trait | kp_12_11 | 常见错误诊断：trait bound 未满足 / impl 漏写必需方法 / 重复实现 | 难点 | assessments\lesson_12_traits.rs:899 | src/tutorial/README.md 第四阶段「12 Trait」 |
| lesson_13 | 生命周期 | kp_13_01 | 函数签名里的生命周期注解：返回较长的那个字符串切片 | 核心 | assessments\lesson_13_lifetimes.rs:155 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_02 | 生命周期省略规则 1 + 2：只有一个输入引用时，输出引用就取它 | 核心 | assessments\lesson_13_lifetimes.rs:235 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_03 | 两个独立生命周期参数：'a 与 'b 互不影响，返回值只借用 'a | 核心 | assessments\lesson_13_lifetimes.rs:310 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_04 | 结构体持有引用：Excerpt<'a> 的 new / part / announce | 核心 | assessments\lesson_13_lifetimes.rs:390 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_05 | 生命周期省略：签名里一个 'a 都不写，也能安全返回切片 | 核心 | assessments\lesson_13_lifetimes.rs:519 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_06 | 'static：字符串字面量活在只读区，带出任何作用域都有效 | 核心 | assessments\lesson_13_lifetimes.rs:589 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_07 | 生命周期参数 + trait bound：T: Display + 'a，返回拥有所有权的 String | 难点 | assessments\lesson_13_lifetimes.rs:669 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_08 | 常见错误诊断：E0106 缺生命周期注解（课程前三个错误都是它） | 难点 | assessments\lesson_13_lifetimes.rs:733 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_13 | 生命周期 | kp_13_09 | 结构体持有引用：Config<'a> 的 new / endpoint / host | 难点 | assessments\lesson_13_lifetimes.rs:807 | src/tutorial/README.md 第四阶段「13 生命周期」 |
| lesson_14 | 闭包 | kp_14_01 | 闭包基础：竖线参数、可省花括号、闭包能捕获定义处的变量 | 基础 | assessments\lesson_14_closures.rs:100 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_02 | Fn trait：只读捕获的闭包可被调用任意多次（泛型参数 + Fn bound） | 核心 | assessments\lesson_14_closures.rs:164 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_03 | FnMut trait：可变借用捕获 + 函数参数用 mut 绑定，两次调用共享同一份状态 | 核心 | assessments\lesson_14_closures.rs:242 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_04 | move 捕获与 FnOnce：所有权搬进闭包，调用后由返回值交还 | 核心 | assessments\lesson_14_closures.rs:305 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_05 | 闭包作为参数（impl Trait）：静态分发，写法最简 | 核心 | assessments\lesson_14_closures.rs:368 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_06 | 返回闭包（impl Fn）：必须 move 把 n 搬进闭包，否则借用随函数结束失效 | 难点 | assessments\lesson_14_closures.rs:432 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_07 | Box<dyn Fn>：异构闭包必须装箱才能放进同一个 Vec（动态分发） | 难点 | assessments\lesson_14_closures.rs:496 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_08 | 任务队列：Vec<Box<dyn FnOnce() -> String>> 按顺序执行消费型任务 | 难点 | assessments\lesson_14_closures.rs:557 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_14 | 闭包 | kp_14_09 | 常见错误诊断：闭包参数类型锁定 E0308 / 可变借用冲突 E0502 / move 后使用 E0382 | 难点 | assessments\lesson_14_closures.rs:618 | src/tutorial/README.md 第五阶段「14 闭包」 |
| lesson_15 | 迭代器 | kp_15_01 | for 循环与 Iterator 的关系：for 只是「反复调用 next()」的语法糖 | 基础 | assessments\lesson_15_iterators.rs:175 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_02 | iter / iter_mut / into_iter 的所有权差别：iter_mut() 产出 &mut T，可原地改写 | 核心 | assessments\lesson_15_iterators.rs:246 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_03 | 适配器流水线：enumerate + filter + map（惰性组合，不打乱原始下标） | 核心 | assessments\lesson_15_iterators.rs:309 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_04 | 消费器：sum() / max() / count() / any()（只有消费器才驱动流水线） | 核心 | assessments\lesson_15_iterators.rs:372 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_05 | fold 与 collect：fold 是自定义聚合，collect 是「换容器」 | 核心 | assessments\lesson_15_iterators.rs:442 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_06 | 惰性与短路：适配器只搭流水线，消费器才按需驱动（take(3) 只加工 3 个元素） | 边界 | assessments\lesson_15_iterators.rs:504 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_07 | 自定义迭代器：实现 Counter::new 与 Iterator::next，免费获得全部适配器与消费器 | 难点 | assessments\lesson_15_iterators.rs:78 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_08 | 词频统计：split_whitespace + 统一小写 + 计数 + 确定性排序（次数降序、字典序升序） | 难点 | assessments\lesson_15_iterators.rs:621 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_15 | 迭代器 | kp_15_09 | 常见错误诊断：迭代期间修改 E0502 / into_iter 后使用 E0382 / collect 类型不明 E0282 | 难点 | assessments\lesson_15_iterators.rs:682 | src/tutorial/README.md 第五阶段「15 迭代器」 |
| lesson_16 | 智能指针 | kp_16_01 | Box 与递归类型：用 Box 把「无限大小」变成固定大小，再递归处理它 | 核心 | assessments\lesson_16_smart_pointers.rs:175 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_02 | Box<T>：唯一所有权 + 堆分配，`*b` 解引用取出里面的值 | 基础 | assessments\lesson_16_smart_pointers.rs:272 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_03 | Rc 共享所有权：`Rc::clone` 只让计数 +1，不复制堆上的数据 | 核心 | assessments\lesson_16_smart_pointers.rs:338 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_04 | RefCell 内部可变性：把借用检查从编译期挪到运行期 | 核心 | assessments\lesson_16_smart_pointers.rs:397 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_05 | RefCell 的运行期边界：可变借用没释放就再借用 → panic（不是编译错误） | 边界 | assessments\lesson_16_smart_pointers.rs:433 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_06 | Rc<RefCell<T>>：Rc 管「几个所有者」，RefCell 管「能不能改」 | 核心 | assessments\lesson_16_smart_pointers.rs:531 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_07 | Weak 打破循环引用：`Rc::downgrade` 不加强计数，`upgrade()` 拿临时强引用 | 难点 | assessments\lesson_16_smart_pointers.rs:614 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_08 | 典型场景：`Rc<RefCell<HashMap>>` 做共享可变缓存，并统计命中次数 | 难点 | assessments\lesson_16_smart_pointers.rs:711 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_16 | 智能指针 | kp_16_09 | 常见错误诊断：改 Rc 内部值 / Rc 跨线程 / 递归类型没加间接层，分别报什么错 | 难点 | assessments\lesson_16_smart_pointers.rs:768 | src/tutorial/README.md 第五阶段「16 智能指针」 |
| lesson_17 | 线程与通道 | kp_17_01 | thread::spawn + join：等子线程结束，并把它的返回值收回主线程 | 基础 | assessments\lesson_17_threads_channels.rs:105 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_02 | move 闭包与所有权转移：编译器用「独占」保证没有数据竞争 | 核心 | assessments\lesson_17_threads_channels.rs:169 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_03 | mpsc 通道：一个生产者线程发送，主线程收集求和 | 核心 | assessments\lesson_17_threads_channels.rs:235 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_04 | 多生产者单消费者：用 `tx.clone()` 把发送端分给多个线程 | 核心 | assessments\lesson_17_threads_channels.rs:297 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_05 | Arc<Mutex<T>>：Arc 管「多线程共享」，Mutex 管「同一时刻只有一个能改」 | 核心 | assessments\lesson_17_threads_channels.rs:360 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_06 | Send / Sync：`Arc<Mutex<i32>>` 两者都满足（换成 `Rc` 根本编译不过） | 核心 | assessments\lesson_17_threads_channels.rs:430 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_07 | thread::scope：子线程借用局部数据，不需要 move 所有权 | 核心 | assessments\lesson_17_threads_channels.rs:509 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_08 | 并行求和：按 chunks 分块、并发执行、收集合并（切分 → 并发 → 合并） | 难点 | assessments\lesson_17_threads_channels.rs:595 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_17 | 线程与通道 | kp_17_09 | 常见错误诊断：忘记 join / 闭包缺 move / Rc 跨线程，各是什么性质的问题 | 难点 | assessments\lesson_17_threads_channels.rs:652 | src/tutorial/README.md 第五阶段「17 线程与通道」 |
| lesson_18 | 声明宏 | kp_18_01 | 可变参数宏：macro_rules! 用重复模式 $(...)* 接受任意个参数（含 0 个） | 基础 | assessments\lesson_18_macros.rs:99 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_02 | 片段说明符：$i:ident 要名字、$e:expr 要表达式、$t:ty 要类型 | 核心 | assessments\lesson_18_macros.rs:181 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_03 | 重复模式：$(...),+ 起「逗号分隔」的展开，$(,)? 吃掉可选尾逗号 | 核心 | assessments\lesson_18_macros.rs:268 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_04 | stringify! 把源码文本变成字符串；concat! 在编译期拼接字面量 | 基础 | assessments\lesson_18_macros.rs:385 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_05 | 提前返回宏 ensure!：展开成 if !cond { return Err(...) }，校验代码收敛成一行 | 难点 | assessments\lesson_18_macros.rs:482 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_06 | 宏写小型 DSL：`"备份" => 10` 这种箭头语法只有宏能表达，展开成 Vec 后顺序与书写一致 | 难点 | assessments\lesson_18_macros.rs:551 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_07 | 宏卫生性（hygiene）：宏内部定义的名字不会污染调用方的命名空间 | 难点 | assessments\lesson_18_macros.rs:656 | src/tutorial/README.md 第五阶段「18 声明宏」 |
| lesson_18 | 声明宏 | kp_18_08 | 宏的典型报错：先用后定义 / 形状不匹配 / 分隔符不符（取自课程示例 7 的注释原文） | 难点 | assessments\lesson_18_macros.rs:747 | src/tutorial/README.md 第五阶段「18 声明宏」 |

## 二、课程 ↔ 考核文件 ↔ 运行命令对应表

| 课程文件 | 复习命令 | 考核目标（单独运行） |
| --- | --- | --- |
| src/tutorial/lesson_01_variables_mutability.rs | cargo run --bin lesson_01_variables_mutability | cargo test --test lesson_01_variables_mutability |
| src/tutorial/lesson_02_data_types.rs | cargo run --bin lesson_02_data_types | cargo test --test lesson_02_data_types |
| src/tutorial/lesson_03_functions.rs | cargo run --bin lesson_03_functions | cargo test --test lesson_03_functions |
| src/tutorial/lesson_04_control_flow.rs | cargo run --bin lesson_04_control_flow | cargo test --test lesson_04_control_flow |
| src/tutorial/lesson_05_ownership_borrowing.rs | cargo run --bin lesson_05_ownership_borrowing | cargo test --test lesson_05_ownership_borrowing |
| src/tutorial/lesson_06_structs.rs | cargo run --bin lesson_06_structs | cargo test --test lesson_06_structs |
| src/tutorial/lesson_07_enums_pattern_matching.rs | cargo run --bin lesson_07_enums_pattern_matching | cargo test --test lesson_07_enums_pattern_matching |
| src/tutorial/lesson_08_collections.rs | cargo run --bin lesson_08_collections | cargo test --test lesson_08_collections |
| src/tutorial/lesson_09_packages_modules.rs | cargo run --bin lesson_09_packages_modules | cargo test --test lesson_09_packages_modules |
| src/tutorial/lesson_10_error_handling.rs | cargo run --bin lesson_10_error_handling | cargo test --test lesson_10_error_handling |
| src/tutorial/lesson_11_generics.rs | cargo run --bin lesson_11_generics | cargo test --test lesson_11_generics |
| src/tutorial/lesson_12_traits.rs | cargo run --bin lesson_12_traits | cargo test --test lesson_12_traits |
| src/tutorial/lesson_13_lifetimes.rs | cargo run --bin lesson_13_lifetimes | cargo test --test lesson_13_lifetimes |
| src/tutorial/lesson_14_closures.rs | cargo run --bin lesson_14_closures | cargo test --test lesson_14_closures |
| src/tutorial/lesson_15_iterators.rs | cargo run --bin lesson_15_iterators | cargo test --test lesson_15_iterators |
| src/tutorial/lesson_16_smart_pointers.rs | cargo run --bin lesson_16_smart_pointers | cargo test --test lesson_16_smart_pointers |
| src/tutorial/lesson_17_threads_channels.rs | cargo run --bin lesson_17_threads_channels | cargo test --test lesson_17_threads_channels |
| src/tutorial/lesson_18_macros.rs | cargo run --bin lesson_18_macros | cargo test --test lesson_18_macros |

## 三、统计

- 模块数：18；知识点数：157
- 类型分布：基础 25 / 核心 75 / 难点 45 / 边界 12
