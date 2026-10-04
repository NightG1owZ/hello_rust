//! assessments/harness/lib.rs —— 零依赖考核框架（libtest 之上的一层「评估 harness」）
//!
//! 这是 `assessment_harness` 库 crate 的根文件：18 个考核目标（`assessments/lesson_*.rs`）
//! 与报告生成器（`assessments/bin/assessment_report.rs`）都复用它。
//!
//! 定位：
//!   本项目面向零基础新人，考核环节需要做三件 libtest 本身做不到的事：
//!     1. 区分「知识点尚未实现」与「实现了但结果错误」与「实现里 panic 了」；
//!     2. 给每一条断言附带**提示**与**复习指引**，失败时直接告诉新人去看哪一课；
//!     3. 把每次运行的结果落盘成**账本**（ledger.tsv），供 `assessment_report` 生成
//!        学习评估报告并跟踪学习进度。
//!
//! 设计要点（全部只用标准库，零第三方依赖）：
//!   - 执行框架仍是 Rust 官方测试框架 libtest：每个知识点就是一个 `#[test]` 函数，
//!     `cargo test` 统一调度、统一打印通过/失败；
//!   - 每个 `#[test]` 用 `assessment_harness::assess(...)` 包裹：它先登记知识点，再用
//!     `catch_unwind` 捕获断言失败，把原因写进账本，最后把 panic 原样抛回给 libtest
//!     （所以 libtest 的通过/失败判定依然权威、依然会返回非零退出码）；
//!   - 学员要实现的练习函数体里只有一行 `assessment_harness::todo_exercise(...)` 占位，
//!     它记录「未实现」并 panic；这样骨架态既能编译（零警告），又能明确报出「还没做」。
//!
//! 账本格式：TSV，每行一条记录，字段见 `write_record`。
//! 数据目录：`assessments/report/data/`，可用环境变量 `ASSESSMENT_DATA_DIR` 覆盖，
//!           运行 ID 用 `ASSESSMENT_RUN_ID` 指定（自动化脚本会设置）。

use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

// ===========================================================================
// 一、知识点分类
// ===========================================================================

/// 知识点在课程里的定位，决定失败时给出的建议口径，也用于报告的分类统计。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// 基础：该课最先要求掌握的概念。
    Basic,
    /// 核心：该课的主要知识点（README 表格里的核心知识点）。
    Core,
    /// 难点：新人最容易卡住的地方，允许对照示例反复尝试。
    Hard,
    /// 边界：极值、空输入、类型细节等容易被忽略的边界条件。
    Edge,
}

impl Kind {
    /// 中文标签（写进账本与报告）。
    pub fn label(self) -> &'static str {
        match self {
            Kind::Basic => "基础",
            Kind::Core => "核心",
            Kind::Hard => "难点",
            Kind::Edge => "边界",
        }
    }

    /// 英文标签（写进账本，便于脚本过滤）。
    pub fn code(self) -> &'static str {
        match self {
            Kind::Basic => "basic",
            Kind::Core => "core",
            Kind::Hard => "hard",
            Kind::Edge => "edge",
        }
    }

    /// 失败时给出的通用改进建议（建设性、可执行）。
    pub fn advice(self) -> &'static str {
        match self {
            Kind::Basic => {
                "基础概念不牢：先重跑本课 demo 1~4，把示例输出逐个猜对，再回来实现；不要靠试类型。"
            }
            Kind::Core => {
                "核心知识点：回到本课 README 对应条目精读一遍，先用注释写出思路（伪代码）再写代码；\
                 编译器报错时逐句读错误信息里的 help: 部分。"
            }
            Kind::Hard => {
                "难点：允许对照本课示例改写，但要能用自己的话说清「为什么必须这样写」；\
                 卡住超过 20 分钟就看示例注释，而不是乱改类型。"
            }
            Kind::Edge => {
                "边界条件：检查空输入、0/1 个元素、首尾元素、类型极值（如 u8::MAX）、\
                 整除与负数，以及 UTF-8 多字节字符。"
            }
        }
    }
}

// ===========================================================================
// 二、模块登记表（与 src/tutorial、src/tutorial 的 README 一一对应）
// ===========================================================================

/// 一个考核模块的元信息，全部会写进账本，报告直接引用，不需要另行维护映射关系。
#[derive(Clone, Copy)]
struct ModuleInfo {
    /// 模块 id，例如 `lesson_05`。
    id: &'static str,
    /// 中文主题名，与 README 表格中的主题一致。
    name: &'static str,
    /// 对应课程文件（复习入口）。
    course: &'static str,
    /// 运行该课的复习命令。
    command: &'static str,
    /// 该知识点在哪个 README 的哪一行有说明。
    readme: &'static str,
}

/// 未登记的模块（防呆：写错模块 id 时至少能跑完，而不是 panic）。
const UNKNOWN_MODULE: ModuleInfo = ModuleInfo {
    id: "unknown",
    name: "未登记模块",
    course: "（请检查 harness 中的模块登记表）",
    command: "cargo test",
    readme: "README.md",
};

/// 18 门课程的模块登记表：基础课程 01~13 + 补充实践课 14~18。
const MODULES: &[ModuleInfo] = &[
    ModuleInfo {
        id: "lesson_01",
        name: "变量与可变性",
        course: "src/tutorial/lesson_01_variables_mutability.rs",
        command: "cargo run --bin lesson_01_variables_mutability",
        readme: "src/tutorial/README.md 第一阶段「01 变量与可变性」",
    },
    ModuleInfo {
        id: "lesson_02",
        name: "数据类型",
        course: "src/tutorial/lesson_02_data_types.rs",
        command: "cargo run --bin lesson_02_data_types",
        readme: "src/tutorial/README.md 第一阶段「02 数据类型」",
    },
    ModuleInfo {
        id: "lesson_03",
        name: "函数",
        course: "src/tutorial/lesson_03_functions.rs",
        command: "cargo run --bin lesson_03_functions",
        readme: "src/tutorial/README.md 第一阶段「03 函数」",
    },
    ModuleInfo {
        id: "lesson_04",
        name: "流程控制",
        course: "src/tutorial/lesson_04_control_flow.rs",
        command: "cargo run --bin lesson_04_control_flow",
        readme: "src/tutorial/README.md 第一阶段「04 流程控制」",
    },
    ModuleInfo {
        id: "lesson_05",
        name: "所有权系统",
        course: "src/tutorial/lesson_05_ownership_borrowing.rs",
        command: "cargo run --bin lesson_05_ownership_borrowing",
        readme: "src/tutorial/README.md 第二阶段「05 所有权系统」",
    },
    ModuleInfo {
        id: "lesson_06",
        name: "结构体",
        course: "src/tutorial/lesson_06_structs.rs",
        command: "cargo run --bin lesson_06_structs",
        readme: "src/tutorial/README.md 第二阶段「06 结构体」",
    },
    ModuleInfo {
        id: "lesson_07",
        name: "枚举与模式匹配",
        course: "src/tutorial/lesson_07_enums_pattern_matching.rs",
        command: "cargo run --bin lesson_07_enums_pattern_matching",
        readme: "src/tutorial/README.md 第二阶段「07 枚举与模式匹配」",
    },
    ModuleInfo {
        id: "lesson_08",
        name: "常见集合",
        course: "src/tutorial/lesson_08_collections.rs",
        command: "cargo run --bin lesson_08_collections",
        readme: "src/tutorial/README.md 第三阶段「08 常见集合」",
    },
    ModuleInfo {
        id: "lesson_09",
        name: "包和模块",
        course: "src/tutorial/lesson_09_packages_modules.rs",
        command: "cargo run --bin lesson_09_packages_modules",
        readme: "src/tutorial/README.md 第三阶段「09 包和模块」",
    },
    ModuleInfo {
        id: "lesson_10",
        name: "错误处理",
        course: "src/tutorial/lesson_10_error_handling.rs",
        command: "cargo run --bin lesson_10_error_handling",
        readme: "src/tutorial/README.md 第三阶段「10 错误处理」",
    },
    ModuleInfo {
        id: "lesson_11",
        name: "泛型",
        course: "src/tutorial/lesson_11_generics.rs",
        command: "cargo run --bin lesson_11_generics",
        readme: "src/tutorial/README.md 第四阶段「11 泛型」",
    },
    ModuleInfo {
        id: "lesson_12",
        name: "Trait",
        course: "src/tutorial/lesson_12_traits.rs",
        command: "cargo run --bin lesson_12_traits",
        readme: "src/tutorial/README.md 第四阶段「12 Trait」",
    },
    ModuleInfo {
        id: "lesson_13",
        name: "生命周期",
        course: "src/tutorial/lesson_13_lifetimes.rs",
        command: "cargo run --bin lesson_13_lifetimes",
        readme: "src/tutorial/README.md 第四阶段「13 生命周期」",
    },
    ModuleInfo {
        id: "lesson_14",
        name: "闭包",
        course: "src/tutorial/lesson_14_closures.rs",
        command: "cargo run --bin lesson_14_closures",
        readme: "src/tutorial/README.md 第五阶段「14 闭包」",
    },
    ModuleInfo {
        id: "lesson_15",
        name: "迭代器",
        course: "src/tutorial/lesson_15_iterators.rs",
        command: "cargo run --bin lesson_15_iterators",
        readme: "src/tutorial/README.md 第五阶段「15 迭代器」",
    },
    ModuleInfo {
        id: "lesson_16",
        name: "智能指针",
        course: "src/tutorial/lesson_16_smart_pointers.rs",
        command: "cargo run --bin lesson_16_smart_pointers",
        readme: "src/tutorial/README.md 第五阶段「16 智能指针」",
    },
    ModuleInfo {
        id: "lesson_17",
        name: "线程与通道",
        course: "src/tutorial/lesson_17_threads_channels.rs",
        command: "cargo run --bin lesson_17_threads_channels",
        readme: "src/tutorial/README.md 第五阶段「17 线程与通道」",
    },
    ModuleInfo {
        id: "lesson_18",
        name: "声明宏",
        course: "src/tutorial/lesson_18_macros.rs",
        command: "cargo run --bin lesson_18_macros",
        readme: "src/tutorial/README.md 第五阶段「18 声明宏」",
    },
];

fn module_info(id: &str) -> &'static ModuleInfo {
    match MODULES.iter().find(|m| m.id == id) {
        Some(m) => m,
        None => &UNKNOWN_MODULE,
    }
}

// ===========================================================================
// 三、账本（ledger.tsv）：每次运行逐条落盘，供评估报告使用
// ===========================================================================

/// 账本字段（TSV，12 列，文本字段做转义）：
/// run_id, ts_ms, event, module_id, module_name, kp_id, kp_title, kind, location, detail, hint, guide
///
/// 写入与解析都以本函数为准；报告生成器复用 `escape` / `unescape`，避免格式漂移。
pub fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "")
}

/// `escape` 的逆运算：把 `\\t` / `\\n` / `\\\\` 还原成真实字符。
pub fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

/// 数据目录：默认 `assessments/report/data/`，可用 `ASSESSMENT_DATA_DIR` 覆盖。
fn data_dir() -> PathBuf {
    let override_dir = std::env::var("ASSESSMENT_DATA_DIR")
        .ok()
        .filter(|dir| !dir.trim().is_empty());
    match override_dir {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assessments")
            .join("report")
            .join("data"),
    }
}

fn now_ms() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_millis(),
        Err(_) => 0,
    }
}

fn sanitize_run_id(raw: &str) -> String {
    let cleaned: String = raw
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '-'
            }
        })
        .collect();
    if cleaned.is_empty() {
        "manual-run".to_string()
    } else {
        cleaned
    }
}

/// 会话窗口：同一个 10 分钟内连续运行的测试二进制视为**同一次考核**，
/// 这样学员直接敲 `cargo test`（不经过脚本）也能得到一份完整报告。
const SESSION_WINDOW_MS: u128 = 10 * 60 * 1000;

fn session_run_id() -> String {
    let dir = data_dir();
    let session = dir.join(".session");
    if let Some(id) = reuse_session(&session) {
        return id;
    }
    let id = format!("run-{}", now_ms());
    if fs::create_dir_all(&dir).is_ok() {
        let _ = fs::write(&session, format!("{}\t{}", id, now_ms()));
    }
    id
}

/// 读取会话文件：若其中记录的运行 ID 还在会话窗口内，就复用它。
fn reuse_session(session: &Path) -> Option<String> {
    let text = fs::read_to_string(session).ok()?;
    let mut parts = text.trim().split('\t');
    let id = parts.next()?;
    let started_at = parts.next()?.parse::<u128>().ok()?;
    if id.is_empty() || now_ms().saturating_sub(started_at) >= SESSION_WINDOW_MS {
        return None;
    }
    Some(sanitize_run_id(id))
}

fn run_id() -> &'static str {
    static RUN: OnceLock<String> = OnceLock::new();
    RUN.get_or_init(|| match std::env::var("ASSESSMENT_RUN_ID") {
        Ok(value) if !value.trim().is_empty() => sanitize_run_id(&value),
        _ => session_run_id(),
    })
}

/// 账本句柄：初始化失败（例如只读环境）时退化为 None，测试本身照常运行。
fn ledger() -> &'static Option<Mutex<File>> {
    static LEDGER: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    LEDGER.get_or_init(|| {
        let dir = data_dir();
        let path = dir.join("ledger.tsv");
        // 账本打不开时考核仍要照常跑（测试结果以 libtest 为准），但要**明确提示一次**：
        // 否则学员会遇到「测试跑了、报告却是空的」这种难以排查的情况。
        let fail = |reason: &str| {
            eprintln!(
                "【考核框架】无法写入账本 {path_display}（{reason}）：测试结果仍然有效，\
                 但 cargo run --bin assessment_report 生成的报告会缺少本次记录。\
                 请检查该目录是否可写，或用环境变量 ASSESSMENT_DATA_DIR 指到一个可写目录。",
                path_display = path.display(),
            );
            None
        };
        if let Err(error) = fs::create_dir_all(&dir) {
            return fail(&format!("创建目录失败：{error}"));
        }
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => Some(Mutex::new(file)),
            Err(error) => fail(&format!("打开文件失败：{error}")),
        }
    })
}

/// 写一条账本记录；任何 IO 失败都静默忽略（考核不能被日志问题拖垮）。
fn write_record(event: &str, view: &KpView, location: &str, detail: &str, hint: &str) {
    let Some(store) = ledger() else {
        return;
    };
    let Ok(mut file) = store.lock() else {
        return;
    };
    let line = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        escape(run_id()),
        now_ms(),
        escape(event),
        escape(view.module.id),
        escape(view.module.name),
        escape(&view.kp_id),
        escape(&view.title),
        escape(view.kind.code()),
        escape(location),
        escape(detail),
        escape(hint),
        escape(&view.guide),
    );
    let _ = file.write_all(line.as_bytes());
    let _ = file.flush();
}

// ===========================================================================
// 四、知识点上下文（线程局部：libtest 每个测试跑在独立线程里）
// ===========================================================================

/// 当前知识点的只读视图（供断言助手写账本、拼错误信息）。
struct KpView {
    module: &'static ModuleInfo,
    kp_id: String,
    title: String,
    kind: Kind,
    review: String,
    guide: String,
    /// 是否已经记录过失败原因（避免同一个失败被记两次）。
    recorded: bool,
}

thread_local! {
    // 显式写成 `const { ... }`：初始化在编译期完成（clippy 的 missing_const_for_thread_local
    // 在本写法下仍可能误报，属已知误报，见 docs/01_environment_setup.md 的说明）。
    static CTX: RefCell<Option<KpView>> = const { RefCell::new(None) };
}

fn set_ctx(view: KpView) {
    CTX.with(|cell| *cell.borrow_mut() = Some(view));
}

fn clear_ctx() {
    CTX.with(|cell| *cell.borrow_mut() = None);
}

/// 取当前知识点视图（克隆一份，避免在 panic 钩子里重入借用）。
fn snapshot() -> Option<KpView> {
    CTX.with(|cell| {
        cell.borrow().as_ref().map(|view| KpView {
            module: view.module,
            kp_id: view.kp_id.clone(),
            title: view.title.clone(),
            kind: view.kind,
            review: view.review.clone(),
            guide: view.guide.clone(),
            recorded: view.recorded,
        })
    })
}

/// 标记「失败原因已经记录过」，避免同一个失败被记两次。
fn mark_recorded() {
    CTX.with(|cell| {
        if let Some(view) = cell.borrow_mut().as_mut() {
            view.recorded = true;
        }
    });
}

/// 当前知识点是否已经记录过失败原因（无上下文时视为「已记录」，即无需补记）。
fn already_recorded() -> bool {
    CTX.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|view| view.recorded)
            .unwrap_or(true)
    })
}

/// 调用点位置（`file:line`），用于告诉新人「改哪一行」。
///
/// 必须带 `#[track_caller]`：`Location::caller()` 会把调用点一路上抛，
/// 这样记录下来的位置才是**考核文件里的那一行**，而不是本文件的行号。
#[track_caller]
fn caller_location() -> String {
    let location = panic::Location::caller();
    format!("{}:{}", location.file(), location.line())
}

// ===========================================================================
// 五、对外 API：知识点登记
// ===========================================================================

/// 登记并运行一个知识点考核。
///
/// 用法（每个 `#[test]` 函数体就是一个 `assess` 调用）：
/// ```ignore
/// #[test]
/// fn kp_05_01_move_semantics() {
///     assess(M, "kp_05_01", "所有权三规则：move 之后原变量失效", Kind::Core, "复习：…", || {
///         let (owned, len) = exercise_05_01_move_semantics(String::from("rust"));
///         eq(owned, String::from("rust"), "move 之后把所有权还回来");
///         eq(len, 4, "\"rust\" 的字节长度是 4");
///     });
/// }
/// ```
///
/// - 断言全部通过 → 账本记 `pass`；
/// - 断言失败 → 记 `fail`，并把 panic 抛回 libtest（libtest 依然判失败、退出码非零）；
/// - 练习函数还是占位 → 记 `missing`（未实现）；
/// - 其它 panic（越界、unwrap 等）→ 记 `panic`，并带上原始 panic 信息。
#[track_caller]
pub fn assess<F: FnOnce()>(
    module: &str,
    kp_id: &str,
    title: &str,
    kind: Kind,
    review: &str,
    body: F,
) {
    let info = module_info(module);
    let location = caller_location();
    let guide = format!(
        "复习入口：{}（{}）；说明见 {}",
        info.command, info.course, info.readme
    );
    let view = KpView {
        module: info,
        kp_id: kp_id.to_string(),
        title: title.to_string(),
        kind,
        review: review.to_string(),
        guide,
        recorded: false,
    };
    let guide_for_record = view.guide.clone();
    write_record("start", &view, &location, &guide_for_record, review);
    set_ctx(view);

    match panic::catch_unwind(AssertUnwindSafe(body)) {
        Ok(()) => {
            let Some(view) = snapshot() else {
                return;
            };
            write_record("pass", &view, &location, "", "");
            clear_ctx();
        }
        Err(payload) => {
            if !already_recorded() {
                let reason = panic_text(payload.as_ref());
                if let Some(view) = snapshot() {
                    write_record("panic", &view, &location, &reason, "");
                }
            }
            clear_ctx();
            // 关键：把 panic 原样抛回给 libtest，保持 libtest 的判定权与退出码语义。
            panic::resume_unwind(payload);
        }
    }
}

/// 从 panic 载荷里提取可读信息。
fn panic_text(payload: &(dyn Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else {
        "（非字符串 panic，可能是数组越界、unwrap 失败或除零）".to_string()
    }
}

// ===========================================================================
// 六、对外 API：断言助手（全部带提示语，失败即写入账本并 panic）
// ===========================================================================

/// 断言失败：记录账本 + 抛出带提示的 panic。
fn fail_now(summary: &str, detail: &str, hint: &str, location: &str) -> ! {
    let Some(view) = snapshot() else {
        panic!("【考核框架误用】{summary}\n  {detail}\n  位置：{location}");
    };
    mark_recorded();
    write_record("fail", &view, location, detail, hint);
    panic!(
        "【未通过·{}】{} · {}「{}」\n  {summary}\n  {detail}\n  提示：{hint}\n  改进建议：{}\n  \
         位置：{location}\n  复习：{}",
        view.kind.label(),
        view.module.name,
        view.kp_id,
        view.title,
        view.kind.advice(),
        view.review
    );
}

/// 断言成功：记一条 `ok`（报告里据此显示「这一条断言已掌握」）。
fn pass_now(detail: &str, hint: &str, location: &str) {
    if let Some(view) = snapshot() {
        write_record("ok", &view, location, detail, hint);
    }
}

/// 相等断言：`actual` 应等于 `expected`。
#[track_caller]
pub fn eq<T: PartialEq + Debug>(actual: T, expected: T, hint: &str) {
    let location = caller_location();
    if actual == expected {
        pass_now(&format!("值等于 {expected:?}"), hint, &location);
    } else {
        fail_now(
            &format!("值不相等（期望 {expected:?}，实际 {actual:?}）"),
            &format!("期望：{expected:?}\n  实际：{actual:?}"),
            hint,
            &location,
        );
    }
}

/// 切片/向量相等断言（错误信息按长度与内容列出，比 `Debug` 直观）。
#[track_caller]
pub fn eq_slice<T: PartialEq + Debug>(actual: &[T], expected: &[T], hint: &str) {
    let location = caller_location();
    if actual == expected {
        pass_now(&format!("切片等于 {expected:?}"), hint, &location);
    } else {
        fail_now(
            "切片内容不相等",
            &format!(
                "期望（长度 {}）：{expected:?}\n  实际（长度 {}）：{actual:?}",
                expected.len(),
                actual.len()
            ),
            hint,
            &location,
        );
    }
}

/// 布尔断言：条件应为真。
#[track_caller]
pub fn is_true(condition: bool, hint: &str) {
    let location = caller_location();
    if condition {
        pass_now("条件为真", hint, &location);
    } else {
        fail_now("条件为假（要求为真）", "实际：false", hint, &location);
    }
}

/// 布尔断言：条件应为假。
#[track_caller]
pub fn is_false(condition: bool, hint: &str) {
    let location = caller_location();
    if condition {
        fail_now("条件为真（要求为假）", "实际：true", hint, &location);
    } else {
        pass_now("条件为假", hint, &location);
    }
}

/// 断言 `Result` 为 `Ok`，并返回其中的值。
#[track_caller]
pub fn ok<T, E: Debug>(result: Result<T, E>, hint: &str) -> T {
    let location = caller_location();
    match result {
        Ok(value) => {
            pass_now("Result 为 Ok", hint, &location);
            value
        }
        Err(error) => fail_now(
            "本应成功（Ok），却返回了 Err",
            &format!("实际：Err({error:?})"),
            hint,
            &location,
        ),
    }
}

/// 断言 `Result` 为 `Err`，并返回其中的错误值。
#[track_caller]
pub fn err<T: Debug, E>(result: Result<T, E>, hint: &str) -> E {
    let location = caller_location();
    match result {
        Ok(value) => fail_now(
            "本应失败（Err），却返回了 Ok",
            &format!("实际：Ok({value:?})"),
            hint,
            &location,
        ),
        Err(error) => {
            pass_now("Result 为 Err", hint, &location);
            error
        }
    }
}

/// 断言 `Option` 为 `Some`，并返回其中的值。
#[track_caller]
pub fn some<T>(value: Option<T>, hint: &str) -> T {
    let location = caller_location();
    match value {
        Some(inner) => {
            pass_now("Option 为 Some", hint, &location);
            inner
        }
        None => fail_now(
            "本应有值（Some），却得到了 None",
            "实际：None",
            hint,
            &location,
        ),
    }
}

/// 断言 `Option` 为 `None`。
#[track_caller]
pub fn none<T: Debug>(value: Option<T>, hint: &str) {
    let location = caller_location();
    match value {
        None => pass_now("Option 为 None", hint, &location),
        Some(inner) => fail_now(
            "本应无值（None），却得到了 Some",
            &format!("实际：Some({inner:?})"),
            hint,
            &location,
        ),
    }
}

/// 浮点近似相等断言（浮点不能用 `==` 比较，这是本框架刻意提供的边界工具）。
#[track_caller]
pub fn approx(actual: f64, expected: f64, tolerance: f64, hint: &str) {
    let location = caller_location();
    if (actual - expected).abs() <= tolerance {
        pass_now(
            &format!("{actual} 与 {expected} 的误差在 {tolerance} 之内"),
            hint,
            &location,
        );
    } else {
        fail_now(
            "浮点值超出允许误差",
            &format!(
                "期望：{expected}（容差 {tolerance}）\n  实际：{actual}\n  差值：{}",
                (actual - expected).abs()
            ),
            hint,
            &location,
        );
    }
}

/// 断言字符串包含某个子串（用于「错误信息里应包含 …」这类考核）。
#[track_caller]
pub fn contains(haystack: &str, needle: &str, hint: &str) {
    let location = caller_location();
    if haystack.contains(needle) {
        pass_now(&format!("文本包含 {needle:?}"), hint, &location);
    } else {
        fail_now(
            "文本不包含期望的子串",
            &format!("应包含：{needle:?}\n  实际文本：{haystack:?}"),
            hint,
            &location,
        );
    }
}

/// 断言这段代码会 panic（例如学员实现的 `fn fail(...) -> !`）。
///
/// 做法：临时把 panic 钩子换成空实现并 `catch_unwind`，
/// 这样「预期内的 panic」不会污染测试输出。
#[track_caller]
pub fn panics<F: FnOnce()>(body: F, hint: &str) {
    let location = caller_location();
    let previous = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(AssertUnwindSafe(body));
    panic::set_hook(previous);
    match result {
        Err(_) => pass_now("代码按预期 panic", hint, &location),
        Ok(()) => fail_now(
            "本应 panic，却正常返回了",
            "提示：检查是否漏掉了 panic! / assert! / unreachable!",
            hint,
            &location,
        ),
    }
}

/// 断言这段代码会 panic，**且这次的 panic 不是因为练习还没实现**。
///
/// 为什么需要它：骨架态下练习函数自己会 panic（`todo_exercise` 占位），
/// 如果直接用 `panics(|| 练习函数(...))`，骨架态会被误判成「发散函数写对了」。
/// 本函数会检查 panic 载荷里是否含「未实现」，含则判为未通过。
///
/// 适用场景：练习本身就是「写出会 panic / 会触发运行期借用冲突的代码」时（例如
/// lesson_03 的发散函数、lesson_16 的 `RefCell` 双重借用边界）。
#[track_caller]
pub fn panics_real<F: FnOnce()>(body: F, hint: &str) {
    let location = caller_location();
    let previous = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(AssertUnwindSafe(body));
    panic::set_hook(previous);
    match result {
        Err(payload) => {
            let text = panic_text(payload.as_ref());
            if text.contains("未实现") {
                fail_now(
                    "本应因运行期冲突而 panic，但这次的 panic 来自「还没实现」的占位代码",
                    &format!("实际 panic：{}", text.lines().next().unwrap_or("")),
                    hint,
                    &location,
                );
            } else {
                pass_now(
                    &format!("代码按预期 panic：{}", text.lines().next().unwrap_or("")),
                    hint,
                    &location,
                );
            }
        }
        Ok(()) => fail_now(
            "本应 panic，却正常返回了",
            "提示：检查是否漏掉了会触发运行期冲突的语句（如同时持有 borrow 与 borrow_mut）",
            hint,
            &location,
        ),
    }
}

// ===========================================================================
// 七、对外 API：练习占位（学员实现前，练习函数体里只留这一行）
// ===========================================================================

/// 练习函数占位：记录「本知识点尚未实现」，并 panic 让 libtest 判为失败。
///
/// 参数：
///   - `name`：练习函数名（报告里会告诉新人去实现哪一个函数）；
///   - `requirement`：实现要求（一句话讲清输入与输出）；
///   - `inputs`：把函数入参原样传进来（这样骨架态不会出现「未使用参数」警告，
///     同时向编译器证明参数已被使用）。没有入参时传 `()`。
///
/// 学员实现练习时，**删除这一行**并写入自己的代码即可。
#[track_caller]
pub fn todo_exercise(name: &str, requirement: &str, inputs: impl Sized) -> ! {
    let location = caller_location();
    let _consumed = inputs;
    let Some(view) = snapshot() else {
        panic!(
            "【未实现】{name}\n  实现要求：{requirement}\n  位置：{location}\n  \
             （该练习函数应在 harness::assess(...) 的闭包中被调用）"
        );
    };
    mark_recorded();
    write_record("missing", &view, &location, name, requirement);
    panic!(
        "【未实现·{}】{} · {}「{}」\n  待实现函数：{name}\n  实现要求：{requirement}\n  \
         位置：{location}\n  复习：{}",
        view.kind.label(),
        view.module.name,
        view.kp_id,
        view.title,
        view.review
    );
}
