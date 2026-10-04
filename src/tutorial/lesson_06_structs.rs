//! lesson_06_structs.rs —— 主题：结构体（Struct）
//!
//! 学习目标：
//!   1. 会定义结构体、创建实例，并用点号访问字段。
//!   2. 掌握字段初始化简写与结构体更新语法（..other）的适用条件。
//!   3. 分清经典结构体、元组结构体、单元结构体各自的用途。
//!   4. 会写方法（&self / &mut self / self）与关联函数（如 new 构造函数）。
//!   5. 会用 `#[derive(Debug)]` 打印调试信息，并手动实现 `Display`。
//!   6. 理解方法与字段的可见性、所有权之间的关系，看懂常见结构体编译错误。
//!
//! 运行方式：
//!   rustc --edition 2024 lesson_06_structs.rs -o lesson_06 && ./lesson_06
//!   或在本项目根目录执行：cargo run --bin lesson_06_structs
//!
//! 说明：本文件所有示例均只依赖标准库，可直接编译运行。

use std::fmt;

fn main() {
    println!("========== lesson_06_structs：结构体 ==========\n");

    demo_1_define_and_instantiate();
    demo_2_field_init_shorthand();
    demo_3_struct_update_syntax();
    demo_4_tuple_struct();
    demo_5_unit_struct();
    demo_6_methods_and_receivers();
    demo_7_associated_functions();
    demo_8_debug_and_display();
    demo_9_typical_scenario_order();
    demo_10_common_mistakes();
}

/// 经典结构体：用命名字段描述一个平面矩形。
///
/// 这里先只定义数据，方法统一放在后面的 impl 块里，
/// 这是 Rust 的惯例：数据定义与行为实现分开书写。
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// 示例 1：定义与实例化
///
/// 要点：实例化时必须给出所有字段（除非用更新语法）；
/// 字段顺序可以和定义顺序不同，但必须写名字。
fn demo_1_define_and_instantiate() {
    println!("--- 示例 1：定义与实例化 ---");

    // 完整写法：字段名: 值，顺序无关。
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // 点号访问字段；u32 是 Copy，所以读取字段不会移动整个结构体。
    println!("rect.width = {}, rect.height = {}", rect.width, rect.height);
    println!("// 预期输出：rect.width = 30, rect.height = 50");

    // 结构体整体能否继续使用，取决于字段类型是否都是 Copy。
    // 这里 Rectangle 的所有字段都是 Copy，所以赋值就是按位复制。
    let copied = rect;
    println!("copied = {copied:?}（字段全是 Copy 类型，赋值即复制）");
    println!(
        "// 预期输出：copied = Rectangle {{ width: 30, height: 50 }}（字段全是 Copy 类型，赋值即复制）"
    );

    // 可变实例：给整个变量加 mut 才能改字段。
    let mut resizable = Rectangle {
        width: 10,
        height: 10,
    };
    resizable.width = 20; // 修改字段：变量必须是 mut
    println!("修改字段后 resizable = {resizable:?}");
    println!("// 预期输出：修改字段后 resizable = Rectangle {{ width: 20, height: 10 }}");

    // 花括号字面量以字段名开头时，需要给整个表达式加圆括号。
    let height = 8;
    let expression = (Rectangle { width: 4, height }).area();
    println!("花括号字面量作为表达式时的面积 = {expression}");
    println!("// 预期输出：花括号字面量作为表达式时的面积 = 32");

    // 创建辅助函数，避免到处重复写字段。
    let square = Rectangle::square(3);
    println!("关联函数创建的 square = {square:?}");
    println!("// 预期输出：关联函数创建的 square = Rectangle {{ width: 3, height: 3 }}");
}

/// 示例 2：字段初始化简写
///
/// 要点：当变量名与字段名相同时，可以只写一次名字。
fn demo_2_field_init_shorthand() {
    println!("--- 示例 2：字段初始化简写 ---");

    let width = 640;
    let height = 480;

    // 完整写法：width: width, height: height
    let verbose = Rectangle {
        width,
        height: height,
    };
    // 简写：字段名与变量名同名时省略冒号与值。
    let concise = Rectangle { width, height };
    println!("verbose = {verbose:?}, concise = {concise:?}");
    println!(
        "// 预期输出：verbose = Rectangle {{ width: 640, height: 480 }}, concise = Rectangle {{ width: 640, height: 480 }}"
    );

    // 简写不会移动变量：u32 是 Copy，这里再读一次仍然合法。
    println!("简写后 width 仍可用 = {width}, height = {height}");
    println!("// 预期输出：简写后 width 仍可用 = 640, height = 480");

    // 元组结构体同样支持简写风格的位置参数（见示例 4），这里再体验一次具名字段简写。
    let label = String::from("screen");
    let screen = NamedRect {
        label,
        width,
        height,
    };
    // label 是 String，已被移动进结构体，所以下面不能再打印它。
    println!("screen = {screen:?}");
    println!("// 预期输出：screen = NamedRect {{ label: \"screen\", width: 640, height: 480 }}");

    // 用方法把三个字段都真正读一遍（不借助 Debug），字段才不会被判定为"从未使用"。
    println!("屏幕说明：{}", screen.describe());
    println!("// 预期输出：屏幕说明：这是名为 screen 的 640x480 屏幕");
}

/// 另一个结构体：用来演示字段初始化简写与 String 字段的所有权。
#[derive(Debug)]
struct NamedRect {
    label: String,
    width: u32,
    height: u32,
}

impl NamedRect {
    /// &self 方法：把三个字段组织成一句可读的说明。
    fn describe(&self) -> String {
        // 三个字段都在这里被读取：label（借用）、width、height。
        format!(
            "这是名为 {} 的 {}x{} 屏幕",
            self.label, self.width, self.height
        )
    }
}

/// 示例 3：结构体更新语法（..other）
///
/// 要点：`..other` 会移动尚未显式赋值的字段；
/// 字段是 Copy 类型时只是复制，是 String 这类非 Copy 类型时就是 move。
fn demo_3_struct_update_syntax() {
    println!("--- 示例 3：结构体更新语法 ---");

    // 场景：把一份"主题配置"改几个值变成另一份配置。
    let default_theme = Theme {
        name: String::from("dark"),
        font_size: 14,
        contrast: 0.8,
    };
    // 只覆盖 font_size，其余字段（name / contrast）用更新语法从另一份配置补齐。
    // 注意：name 是 String，若来源不 clone，就会被移动走。
    let custom_theme = Theme {
        font_size: 16,
        ..default_theme.clone()
    };
    println!("custom_theme = {custom_theme:?}");
    println!(
        "// 预期输出：custom_theme = Theme {{ name: \"dark\", font_size: 16, contrast: 0.8 }}"
    );

    // 如果直接写 ..default_theme，default_theme.name 就会被移动，之后不能再用：
    // let custom_theme = Theme { font_size: 16, ..default_theme };
    // println!("{default_theme:?}"); // error[E0382]: borrow of moved value: `default_theme.name`
    // 修正方法一（上面用的）：给来源加 clone()，更新语法只搬克隆体的字段。
    // 修正方法二：把非 Copy 字段也显式写出来，这样 .. 只需要复制 Copy 字段。
    let explicit_theme = Theme {
        name: default_theme.name.clone(), // 显式给出 String 字段，避免被 .. 移动
        font_size: 12,
        contrast: 0.9, // 显式给出所有字段，于是完全不需要 .. 语法
    };
    println!("explicit_theme = {explicit_theme:?}, default_theme 仍可用 = {default_theme:?}");
    println!(
        "// 预期输出：explicit_theme = Theme {{ name: \"dark\", font_size: 12, contrast: 0.9 }}, default_theme 仍可用 = Theme {{ name: \"dark\", font_size: 14, contrast: 0.8 }}"
    );

    // 用方法读取 Theme 的三个字段（自定义对比度），让字段真正被使用。
    println!(
        "custom_theme 的对比度提示：{}",
        custom_theme.contrast_hint()
    );
    println!("// 预期输出：custom_theme 的对比度提示：dark 主题字号 16 对比度 0.80（偏低）");

    // 修正方法三：让被复制的字段全部是 Copy 类型，这样 ..other 只做复制。
    let base_point = Point { x: 1, y: 2 };
    let shifted_point = Point {
        x: 10,
        ..base_point
    };
    println!("shifted_point = {shifted_point:?}, base_point 仍可用 = {base_point:?}");
    println!(
        "// 预期输出：shifted_point = Point {{ x: 10, y: 2 }}, base_point 仍可用 = Point {{ x: 1, y: 2 }}"
    );

    // 读取 Point 的 x / y 字段：计算曼哈顿距离（|x| + |y|）。
    println!(
        "base_point 到原点的曼哈顿距离 = {}, shifted_point = {}",
        base_point.manhattan(),
        shifted_point.manhattan()
    );
    println!("// 预期输出：base_point 到原点的曼哈顿距离 = 3, shifted_point = 12");
}

/// 主题配置结构体：含一个 String 字段，用来演示更新语法中的"移动"。
#[derive(Debug, Clone)]
struct Theme {
    name: String,
    font_size: u32,
    contrast: f64,
}

impl Theme {
    /// &self 方法：把主题名、字号与对比度拼成一句提示。
    fn contrast_hint(&self) -> String {
        // 三个字段都被读取；对比度低于 0.85 时提示偏低。
        let level = if self.contrast < 0.85 {
            "偏低"
        } else {
            "正常"
        };
        format!(
            "{} 主题字号 {} 对比度 {:.2}（{}）",
            self.name, self.font_size, self.contrast, level
        )
    }
}

/// 二维点结构体：字段全是 Copy 类型，用于对比更新语法的行为差异。
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// &self 方法：曼哈顿距离，读取 x 与 y 两个字段。
    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

/// 示例 4：元组结构体
///
/// 要点：字段没有名字，靠位置访问 `.0` / `.1`，
/// 适合"语义明确、字段本身不需要名字"的包装类型。
fn demo_4_tuple_struct() {
    println!("--- 示例 4：元组结构体 ---");

    // Color 的三个分量语义由类型名说明，不需要 rgb_red 之类的字段名。
    let black = Color(0, 0, 0);
    let orange = Color(255, 165, 0);
    println!("black.0/1/2 = {}, {}, {}", black.0, black.1, black.2);
    println!("// 预期输出：black.0/1/2 = 0, 0, 0");
    println!("orange = {orange:?}");
    println!("// 预期输出：orange = Color(255, 165, 0)");

    // 解构元组结构体：和普通元组一样支持模式解构。
    let Color(r, g, b) = orange;
    println!("解构 orange 得到 r = {r}, g = {g}, b = {b}");
    println!("// 预期输出：解构 orange 得到 r = 255, g = 165, b = 0");

    // 单字段包装类型（newtype）：给"米"和"秒"各自一个类型，防止单位混用。
    let distance = Meters(1500);
    let duration = Seconds(300);
    // 注意：newtype 的防线在"函数签名"上，而不在 `.0` 上。
    // 下面这行其实能编译（两个 `.0` 都是 u32），但它把单位语义丢了 —— 这正是要避免的：
    // let wrong = distance.0 + duration.0; // 合法，但"米 + 秒"毫无意义
    // 真正的防线是把函数参数写成 Meters / Seconds 类型，让类型系统替你把关。
    println!(
        "distance = {distance:?}, 换算成千米 = {}",
        distance.to_kilometers()
    );
    println!("// 预期输出：distance = Meters(1500), 换算成千米 = 1.5");
    println!("duration = {duration:?}（Seconds 与 Meters 是不同类型，不能互相传参）");
    println!("// 预期输出：duration = Seconds(300)（Seconds 与 Meters 是不同类型，不能互相传参）");
    // Seconds 实现了 Display：用 {} 打印时读取内部的秒数并换算成"分 秒"。
    println!("duration 时长 = {duration}");
    println!("// 预期输出：duration 时长 = 5 分 0 秒");
}

/// 元组结构体：RGB 颜色。
#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

/// 元组结构体：米（单字段包装，避免单位混用）。
#[derive(Debug, Clone, Copy)]
struct Meters(u32);

/// 元组结构体：秒。
#[derive(Debug, Clone, Copy)]
struct Seconds(u32);

impl Meters {
    /// 把米换算成千米（返回 f64 便于演示小数）。
    fn to_kilometers(self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

impl fmt::Display for Seconds {
    /// 手动实现 Display：读取内部字段，输出"分 秒"。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} 分 {} 秒", self.0 / 60, self.0 % 60)
    }
}

/// 示例 5：单元结构体
///
/// 要点：没有任何字段，只占 0 字节，常用来做"标记"或实现 trait。
fn demo_5_unit_struct() {
    println!("--- 示例 5：单元结构体 ---");

    // 单元结构体的值就是类型名本身，不需要花括号。
    let marker = AlwaysEqual;
    println!(
        "marker = {marker:?}（size_of = {} 字节）",
        size_of_marker(&marker)
    );
    println!("// 预期输出：marker = AlwaysEqual（size_of = 0 字节）");

    // 单元结构体也可以有方法；这里演示它作为"策略标记"被传进函数。
    let described = describe_marker(AlwaysEqual);
    println!("describe_marker(AlwaysEqual) = {described}");
    println!(
        "// 预期输出：describe_marker(AlwaysEqual) = 这是一个零大小的标记结构体，不携带任何数据"
    );

    // 有一个容易混淆的点：`AlwaysEqual {}` 这种写法其实**能编译**（空字段花括号等价于单元值），
    // 真正会报错的是把它当函数调用——单元结构体不是元组结构体，没有构造函数：
    // let bad = AlwaysEqual(1); // error[E0618]: expected function, found `AlwaysEqual`
    // 对照记忆：`X` 是单元结构体的值，`X(...)` 只对元组结构体成立。
    let also_fine = AlwaysEqual {};
    println!(
        "AlwaysEqual {{}} 也可以构造出同样的零大小值，size = {}",
        size_of_marker(&also_fine)
    );
    println!("// 预期输出：AlwaysEqual {{}} 也可以构造出同样的零大小值，size = 0");
}

/// 单元结构体：零大小标记类型。
#[derive(Debug)]
struct AlwaysEqual;

/// 工具函数：读取单元结构体的大小（借用即可，不需要所有权）。
fn size_of_marker(marker: &AlwaysEqual) -> usize {
    // 直接用类型名构造一个实例来测量；单元结构体零成本。
    let _ = marker;
    std::mem::size_of::<AlwaysEqual>()
}

/// 工具函数：接受单元结构体并按值消费，返回一句说明。
fn describe_marker(_marker: AlwaysEqual) -> String {
    String::from("这是一个零大小的标记结构体，不携带任何数据")
}

/// 示例 6：方法与接收者（&self / &mut self / self）
///
/// 要点：方法第一个参数决定它能不能改数据、会不会消耗自身。
fn demo_6_methods_and_receivers() {
    println!("--- 示例 6：方法与接收者 ---");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // &self：只读借用。注意 owned 变量调用 &self 方法时，Rust 会自动加 &。
    println!("rect.area() = {}", rect.area());
    println!("// 预期输出：rect.area() = 1500");
    println!("rect.is_square() = {}", rect.is_square());
    println!("// 预期输出：rect.is_square() = false");
    // &self 方法不会移动 rect，所以后面还能继续用。
    println!("调用 &self 方法后 rect 仍可用 = {rect:?}");
    println!("// 预期输出：调用 &self 方法后 rect 仍可用 = Rectangle {{ width: 30, height: 50 }}");

    // &mut self：可写借用，用来修改自身。
    let mut growing = rect; // Rectangle 是 Copy，这里复制一份
    growing.scale(2);
    println!("growing.scale(2) 之后 = {growing:?}");
    println!("// 预期输出：growing.scale(2) 之后 = Rectangle {{ width: 60, height: 100 }}");

    // self：按值接收，会消耗（移动）自身；这里返回一个新值把所有权交回去。
    // growing 此刻是 60x100，先记下来，方便对照下面的最终尺寸。
    println!("进入 into_resized 之前的 growing = {growing:?}");
    println!(
        "// 预期输出：进入 into_resized 之前的 growing = Rectangle {{ width: 60, height: 100 }}"
    );
    let label = String::from("画布");
    let canvas = Canvas {
        label,
        rect: growing,
    };
    // into_resized(3) 会把画布内的矩形再放大 3 倍：60x100 -> 180x300。
    let resized_canvas = canvas.into_resized(3);
    println!("into_resized(3) 的结果 = {resized_canvas:?}");
    println!(
        "// 预期输出：into_resized(3) 的结果 = Canvas {{ label: \"画布\", rect: Rectangle {{ width: 180, height: 300 }} }}"
    );
    // 通过 Display 读取 label 字段（Debug 之外的"真实使用"，避免 dead_code 警告）。
    println!("画布说明：{resized_canvas}");
    println!("// 预期输出：画布说明：画布 180x300");
    // canvas 已经被 into_resized 消耗（move 进方法），下面这行会报错：
    // println!("{canvas:?}"); // error[E0382]: borrow of moved value: `canvas`
    println!("按值接收的 self 会移动调用者，这正是消耗型方法的设计意图");
    println!("// 预期输出：按值接收的 self 会移动调用者，这正是消耗型方法的设计意图");
}

/// 画布结构体：包含一个 String 字段，用来演示 self 接收者导致的移动。
#[derive(Debug)]
struct Canvas {
    label: String,
    rect: Rectangle,
}

impl Rectangle {
    /// 关联函数：用边长构造正方形。
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }

    /// &self 方法：只读计算面积。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// &self 方法：判断是否是正方形。
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// &mut self 方法：原地放大尺寸。
    fn scale(&mut self, factor: u32) {
        self.width *= factor; // 通过可变借用修改自己的字段
        self.height *= factor;
    }
}

impl Canvas {
    /// self 方法：消耗自身，返回放大后的新画布。
    fn into_resized(mut self, factor: u32) -> Canvas {
        // self 是本地绑定，加 mut 后可以原地修改；函数结束时把所有权交还调用者。
        self.rect.scale(factor);
        self
    }
}

impl fmt::Display for Canvas {
    /// 手动实现 Display：读取 label 字段，输出"画布尺寸"。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}x{}", self.label, self.rect.width, self.rect.height)
    }
}

/// 示例 7：关联函数与构造函数惯例
///
/// 要点：`impl` 块里不带 self 的函数是关联函数，用 `类型名::函数名` 调用；
/// `new` 只是社区惯例，不是语言关键字。
fn demo_7_associated_functions() {
    println!("--- 示例 7：关联函数 ---");

    // 关联函数：没有 self，因此不依附于某个实例。
    let basket = Basket::new(String::from("水果篮"), 3);
    println!("Basket::new(...) 得到 = {basket:?}");
    println!(
        "// 预期输出：Basket::new(...) 得到 = Basket {{ owner: \"水果篮\", items: 3, note: \"未备注\" }}"
    );

    // 另一个关联函数：给默认备注。
    let noted = Basket::with_note(String::from("菜篮"), 5, String::from("周末采购"));
    println!("Basket::with_note(...) 得到 = {noted:?}");
    println!(
        "// 预期输出：Basket::with_note(...) 得到 = Basket {{ owner: \"菜篮\", items: 5, note: \"周末采购\" }}"
    );

    // 关联函数通过 Self 复用：`Self` 就是当前 impl 的类型别名。
    let empty = Basket::empty(String::from("空篮"));
    println!(
        "Basket::empty(...) 得到 = {empty:?}，其中 note = {}",
        empty.note
    );
    println!(
        "// 预期输出：Basket::empty(...) 得到 = Basket {{ owner: \"空篮\", items: 0, note: \"未备注\" }}，其中 note = 未备注"
    );
    // Display 读取 owner 字段，把"谁的篮子"写给用户看。
    println!("给用户看的购物篮文案：{empty}");
    println!("// 预期输出：给用户看的购物篮文案：空篮的购物篮：0 件商品");
    // 方法与关联函数最终都作用在同一份数据上。
    println!("noted.items_after_add(2) = {}", noted.items_after_add(2));
    println!("// 预期输出：noted.items_after_add(2) = 7");
}

/// 购物篮结构体：演示关联函数作为构造函数。
#[derive(Debug)]
struct Basket {
    owner: String,
    items: u32,
    note: String,
}

impl Basket {
    /// 构造函数：给出必要的字段，其余字段填默认值。
    fn new(owner: String, items: u32) -> Basket {
        Basket {
            owner,
            items,
            note: String::from("未备注"),
        }
    }

    /// 用 Self 作为返回类型，等价于写 Basket。
    fn with_note(owner: String, items: u32, note: String) -> Self {
        Basket { owner, items, note }
    }

    /// 通过另一个关联函数复用逻辑，避免重复默认值。
    fn empty(owner: String) -> Self {
        Basket::new(owner, 0)
    }

    /// &self 方法：返回"再加 n 件"之后的数量，不修改自己。
    fn items_after_add(&self, extra: u32) -> u32 {
        self.items + extra
    }
}

impl fmt::Display for Basket {
    /// 手动实现 Display：读取 owner 字段，输出面向用户的描述。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}的购物篮：{} 件商品", self.owner, self.items)
    }
}

/// 示例 8：Debug 派生与 Display 手动实现
///
/// 要点：`#[derive(Debug)]` 给的是 `{:?}` / `{:#?}`；
/// 面向用户的输出要手动实现 `Display`，即 `{}`。
fn demo_8_debug_and_display() {
    println!("--- 示例 8：Debug 与 Display ---");

    let report = Report {
        title: String::from("月度销量"),
        amount: 128_500,
    };

    // Debug：开发期调试用，紧凑与美化两种格式。
    println!("Debug 紧凑格式：{report:?}");
    println!("// 预期输出：Debug 紧凑格式：Report {{ title: \"月度销量\", amount: 128500 }}");
    println!("Debug 美化格式（多行）：\n{report:#?}");
    println!("// 预期输出：Report {{");
    println!("// 预期输出：    title: \"月度销量\",");
    println!("// 预期输出：    amount: 128500,");
    println!("// 预期输出：}}");

    // Display：用户可见的格式化文本。
    println!("Display 格式：{report}");
    println!("// 预期输出：Display 格式：[月度销量] 金额 128500 元");

    // Display 让结构体可以直接参与 format! 拼接，也能指定宽度（右对齐 20 列）。
    let line = format!("{report:>20}");
    println!("右对齐 20 列：{line}");
    println!("// 预期输出：右对齐 20 列：  [月度销量] 金额 128500 元");

    // 手动 Display 的价值：把"内部数据"翻译成"用户语言"。
    println!("Debug 面向开发者，Display 面向用户，两者职责不同");
    println!("// 预期输出：Debug 面向开发者，Display 面向用户，两者职责不同");
}

/// 报表结构体：用于对比 Debug 与 Display 两种输出。
#[derive(Debug)]
struct Report {
    title: String,
    amount: u32,
}

impl fmt::Display for Report {
    /// 手动实现 Display：`{}` 会走到这里。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 复用宽度/对齐等格式参数：例如 `{report:>20}` 时 f.width() 为 20。
        match f.width() {
            Some(width) => write!(f, "{:>width$}", self.to_line(), width = width),
            None => write!(f, "{}", self.to_line()),
        }
    }
}

impl Report {
    /// 组装一行面向用户的文本（示范：把格式化逻辑抽成方法便于复用）。
    fn to_line(&self) -> String {
        format!("[{}] 金额 {} 元", self.title, self.amount)
    }
}

/// 示例 9：典型使用场景 —— 订单金额计算与优惠
///
/// 要点：用结构体把"一份订单"的所有相关数据绑在一起，
/// 方法与关联函数负责计算，调用方只关心结果。
fn demo_9_typical_scenario_order() {
    println!("--- 示例 9：典型使用场景 —— 订单计算 ---");

    // 用关联函数创建订单，避免字段散落各处。
    let mut order = Order::new(1001, 3, 2_990); // 单价单位：分
    println!("订单 {order:?}");
    println!(
        "// 预期输出：订单 Order {{ id: 1001, quantity: 3, unit_price_cents: 2990, discount_percent: 0 }}"
    );

    // 方法计算小计（只读借用）。
    println!("小计 = {} 分", order.subtotal_cents());
    println!("// 预期输出：小计 = 8970 分");

    // 可变方法设置折扣（可写借用）。
    order.apply_discount(10);
    println!("打 9 折后 order = {order:?}");
    println!(
        "// 预期输出：打 9 折后 order = Order {{ id: 1001, quantity: 3, unit_price_cents: 2990, discount_percent: 10 }}"
    );

    // 结算：返回一份"结算单"结构体，展示结构体作为返回值。
    let receipt = order.checkout();
    println!("结算单 = {receipt:?}");
    println!("// 预期输出：结算单 = Receipt {{ order_id: 1001, payable_cents: 8073 }}");

    // Display：把结算单打印成给用户看的文案。
    println!("给用户看的文案：{receipt}");
    println!("// 预期输出：给用户看的文案：订单 1001 应付 80.73 元");
}

/// 订单结构体：金额统一用"分"存储，避免浮点误差。
#[derive(Debug)]
struct Order {
    id: u32,
    quantity: u32,
    unit_price_cents: u32,
    discount_percent: u32,
}

/// 结算单结构体：只保留结算结果，作为 checkout 的返回值。
#[derive(Debug)]
struct Receipt {
    order_id: u32,
    payable_cents: u32,
}

impl Order {
    /// 关联函数：创建订单，默认无折扣。
    fn new(id: u32, quantity: u32, unit_price_cents: u32) -> Self {
        Order {
            id,
            quantity,
            unit_price_cents,
            discount_percent: 0,
        }
    }

    /// &self 方法：小计金额。
    fn subtotal_cents(&self) -> u32 {
        self.quantity * self.unit_price_cents
    }

    /// &mut self 方法：设置折扣百分比（大于 100 时按 100 处理，保证金额非负）。
    fn apply_discount(&mut self, percent: u32) {
        self.discount_percent = percent.min(100);
    }

    /// &self 方法：结算并生成结算单。
    fn checkout(&self) -> Receipt {
        let subtotal = self.subtotal_cents();
        // 用整数运算：先乘后除，减少精度损失。
        let payable = subtotal * (100 - self.discount_percent) / 100;
        Receipt {
            order_id: self.id,
            payable_cents: payable,
        }
    }
}

impl fmt::Display for Receipt {
    /// 手动 Display：把"分"换算成"元"，保留两位小数。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "订单 {} 应付 {}.{:02} 元",
            self.order_id,
            self.payable_cents / 100,
            self.payable_cents % 100
        )
    }
}

/// 示例 10：常见错误示例（错误代码全部注释掉，只保留修正后的写法）
///
/// 要点：结构体相关的报错几乎都集中在"字段没写全""想改非 mut 的字段"
/// 和"String 字段导致整体移动"这三类。
fn demo_10_common_mistakes() {
    println!("--- 示例 10：常见错误示例 ---");

    // 错误 1：实例化时漏写字段。
    // let bad = Rectangle { width: 10 };
    // error[E0063]: missing field `height` in initializer of `Rectangle`
    // 修正方法：补齐所有字段，或使用 ..other 更新语法补齐。
    let fixed = Rectangle {
        width: 10,
        ..Rectangle::square(1)
    };
    println!("修正 1：补齐字段 -> fixed = {fixed:?}");
    println!("// 预期输出：修正 1：补齐字段 -> fixed = Rectangle {{ width: 10, height: 1 }}");

    // 错误 2：字段名写错（拼写/多写）。
    // let bad = Rectangle { width: 3, hight: 4 };
    // error[E0560]: struct `Rectangle` has no field named `hight`
    // 修正方法：字段名必须与定义完全一致（可以用 IDE 补全或让编译器提示）。
    println!("修正 2：字段名必须与定义严格一致（width / height）");
    println!("// 预期输出：修正 2：字段名必须与定义严格一致（width / height）");

    // 错误 3：通过不可变引用修改字段。
    // let r = &rect;
    // r.width = 1;
    // error[E0594]: cannot assign to `r.width`, which is behind a `&` reference
    // （rustc 1.99 实测原文；旧版曾报 E0596，两者含义不同：E0594 是"给借来的数据赋值"，
    //   E0596 是"借一个不可变绑定来改"。这里触发的是 E0594。）
    // 修正方法：把绑定声明为 mut 并直接持有实例，或使用 &mut 借用。
    let mut a = Rectangle {
        width: 1,
        height: 1,
    };
    a.width = 2;
    println!("修正 3：需要修改字段时绑定要加 mut -> a = {a:?}");
    println!(
        "// 预期输出：修正 3：需要修改字段时绑定要加 mut -> a = Rectangle {{ width: 2, height: 1 }}"
    );

    // 错误 4：&self 方法里修改字段。
    // impl Rectangle {
    //     fn bad_scale(&self) { self.width *= 2; }
    //     // error[E0594]: cannot assign to `self.width`, which is behind a `&` reference
    // }
    // （rustc 1.99 实测原文；旧版曾报 "as `self` is not declared as mutable"，现措辞已改。）
    // 修正方法：把接收者改成 &mut self（见示例 6 的 scale）。
    println!("修正 4：要改字段，方法接收者必须是 &mut self 而不是 &self");
    println!("// 预期输出：修正 4：要改字段，方法接收者必须是 &mut self 而不是 &self");

    // 错误 5：想让结构体走 {:?} 却忘了派生 Debug。
    // struct NoDebug { n: u32 }
    // println!("{:?}", NoDebug { n: 1 });
    // error[E0277]: `NoDebug` doesn't implement `Debug`
    // 修正方法：在定义前加 #[derive(Debug)]。
    println!("修正 5：想用 {{:?}} 打印结构体必须 #[derive(Debug)]");
    println!("// 预期输出：修正 5：想用 {{:?}} 打印结构体必须 #[derive(Debug)]");

    // 错误 6：想要 {} 直接打印结构体却没有实现 Display。
    // println!("{}", Report { .. }); // error[E0277]: `Report` doesn't implement `Display`
    // 修正方法：手动实现 impl fmt::Display（见示例 8），或先用 {:?} 调试。
    println!("修正 6：{{}} 需要手动实现 Display，{{:?}} 只需要派生 Debug");
    println!("// 预期输出：修正 6：{{}} 需要手动实现 Display，{{:?}} 只需要派生 Debug");

    // 错误 7：结构体更新语法搬走 String 字段后仍使用旧变量。
    // let t1 = Theme { .. };
    // let t2 = Theme { font_size: 9, ..t1 };
    // println!("{t1:?}"); // error[E0382]: borrow of moved value: `t1.name`
    // 修正方法：克隆该字段（t1.name.clone()）或让字段类型实现 Copy。
    println!("修正 7：更新语法会移动非 Copy 字段，之后要用旧变量就得 clone");
    println!("// 预期输出：修正 7：更新语法会移动非 Copy 字段，之后要用旧变量就得 clone");
}
