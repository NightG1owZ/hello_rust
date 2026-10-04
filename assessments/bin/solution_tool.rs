//! assessments/bin/solution_tool.rs —— 参考答案工具箱
//!
//! 负责 `assessments/solutions/` 里参考答案的**校验、应用、还原与验证**，让「参考答案」既能被学员
//! 单独阅读，又能在需要时替换进考核文件跑一遍（证明答案确实能让 157 个知识点全绿）。
//!
//! - 参考答案文件与考核文件同名（`assessments/solutions/lesson_XX_<主题>.rs`），是**完整文件**：
//!   除了练习函数体之外，其余内容（`#[test]`、类型定义、注释、`assess(...)` 参数）必须与骨架
//!   **逐字节一致**——`--check` 会强制这一点，避免答案与考核标准脱节；
//! - 应用时只替换「练习函数的函数体」，`#[test]` 与其它代码绝不会被答案覆盖。
//!
//! 子命令：
//! ```text
//! --check                校验每对「考核文件 ↔ 参考答案」：函数体之外逐字节一致、练习一一对应
//! --list                 列出每课的练习函数、知识点与解答状态
//! --status               显示每个考核文件当前是「骨架」还是「已应用参考答案」
//! --apply   [--lesson NN] 把参考答案写入考核文件（先把骨架备份到 solutions/.backup/）
//! --restore [--lesson NN] 从备份还原骨架
//! --verify  [--lesson NN] 应用 → cargo test → 自动还原，报告每个目标的通过数
//! --help                 显示帮助
//! ```
//!
//! 退出码：`--check` / `--verify` 全部通过 → 0，否则 1；用法错误 → 2。

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ===========================================================================
// 一、路径与用法
// ===========================================================================

const SOLUTIONS_DIR: &str = "assessments/solutions";
const BACKUP_DIR: &str = "assessments/solutions/.backup";
const ASSESS_DIR: &str = "assessments";
/// `--verify` 用的隔离账本目录（不污染学员进度 `assessments/report/data/`）。
const VERIFY_DATA_DIR: &str = "target/solution-verify";

/// 项目根目录：优先用 `CARGO_MANIFEST_DIR`，否则从当前目录向上找 `Cargo.toml`。
fn repo_root() -> PathBuf {
    if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
        let path = PathBuf::from(dir);
        if path.join("Cargo.toml").is_file() {
            return path;
        }
    }
    let mut dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        if dir.join("Cargo.toml").is_file() {
            return dir;
        }
        if !dir.pop() {
            return PathBuf::from(".");
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    List,
    Status,
    Apply,
    Restore,
    Verify,
}

struct Options {
    mode: Mode,
    lessons: Vec<String>,
}

fn help_text() -> String {
    [
        "参考答案工具箱（assessments/solutions/）",
        "",
        "用法：cargo run --bin solution_tool -- <模式> [--lesson NN]",
        "",
        "模式（一次只能用一个）：",
        "  --check                校验「考核文件 ↔ 参考答案」：函数体之外逐字节一致、练习一一对应",
        "  --list                 列出每课的练习函数、知识点与解答状态",
        "  --status               显示每个考核文件当前是「骨架」还是「已应用参考答案」",
        "  --apply                把参考答案写入考核文件（骨架先备份到 solutions/.backup/）",
        "  --restore              从备份还原骨架",
        "  --verify               应用 → cargo test → 自动还原，并报告通过数（157/157 即全部可解）",
        "  --help                 显示本帮助",
        "",
        "选项：",
        "  --lesson NN            只处理指定课（可重复；也接受 lesson_05 或完整目标名）",
        "",
        "说明：参考答案只放在 assessments/solutions/ 下，cargo test 不会编译它们；",
        "      只有 --apply / --verify 才会（临时）写进考核文件，且 --verify 结束必然还原。",
    ]
    .join("\n")
}

fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut mode: Option<Mode> = None;
    let mut lessons: Vec<String> = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        let this = match arg {
            "--help" | "-h" => {
                print!("{}", help_text());
                std::process::exit(0);
            }
            "--check" => Mode::Check,
            "--list" => Mode::List,
            "--status" => Mode::Status,
            "--apply" => Mode::Apply,
            "--restore" => Mode::Restore,
            "--verify" => Mode::Verify,
            "--lesson" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "--lesson 后面要跟课号（如 --lesson 05）".to_string())?;
                lessons.push(value.clone());
                index += 1;
                continue;
            }
            other => return Err(format!("无法识别的参数：{other}（用 --help 查看用法）")),
        };
        if let Some(previous) = mode {
            if previous != this {
                return Err(
                    "一次只能用一种模式（--check/--list/--status/--apply/--restore/--verify）"
                        .to_string(),
                );
            }
        }
        mode = Some(this);
        index += 1;
    }
    let mode = mode.ok_or_else(|| "缺少模式参数（用 --help 查看用法）".to_string())?;
    Ok(Options { mode, lessons })
}

// ===========================================================================
// 二、极简 Rust 词法屏蔽：把字符串 / 字符字面量 / 注释替换成空格（字节长度不变）
// ===========================================================================

/// 把源码里「非代码」的字节替换成空格，长度与偏移保持不变。
///
/// 这样后续的花括号配对与 `todo_exercise` 识别都只看到真正的代码：字符串里的
/// `todo_exercise(...)`、注释里的 `fn foo()` 都不会被误判。
fn mask_non_code(src: &str) -> Vec<u8> {
    let bytes = src.as_bytes();
    let mut out = bytes.to_vec();
    let mut i = 0usize;
    while i < bytes.len() {
        // 原始字符串：r"..."、r#"..."#、br#"..."#
        if let Some(end) = raw_string_end(bytes, i) {
            for byte in out.iter_mut().take(end).skip(i) {
                *byte = b' ';
            }
            i = end;
            continue;
        }
        // 字节串 b"..."（按普通字符串处理，b 本身是代码）
        match bytes[i] {
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    out[i] = b' ';
                    i += 1;
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                let mut depth = 0usize;
                while i < bytes.len() {
                    if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
                        depth += 1;
                        out[i] = b' ';
                        out[i + 1] = b' ';
                        i += 2;
                    } else if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
                        depth -= 1;
                        out[i] = b' ';
                        out[i + 1] = b' ';
                        i += 2;
                        if depth == 0 {
                            break;
                        }
                    } else {
                        out[i] = b' ';
                        i += 1;
                    }
                }
            }
            b'"' => {
                let end = string_end(bytes, i);
                for byte in out.iter_mut().take(end).skip(i) {
                    *byte = b' ';
                }
                i = end;
            }
            b'\'' => {
                let end = char_or_lifetime_end(bytes, i);
                for byte in out.iter_mut().take(end).skip(i) {
                    *byte = b' ';
                }
                i = end;
            }
            _ => i += 1,
        }
    }
    out
}

/// 若 `start` 是原始字符串（`r"` / `r#"` / `br"` / `br#"`）的起点，返回其结束字节下标。
fn raw_string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    if bytes.get(i) == Some(&b'b') {
        i += 1;
    }
    if bytes.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let mut hashes = 0usize;
    while bytes.get(i) == Some(&b'#') {
        hashes += 1;
        i += 1;
    }
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let mut k = i + 1;
            let mut matched = 0usize;
            while matched < hashes && bytes.get(k) == Some(&b'#') {
                matched += 1;
                k += 1;
            }
            if matched == hashes {
                return Some(k);
            }
        }
        i += 1;
    }
    Some(bytes.len())
}

/// 普通字符串（起点是 `"`）的结束下标（不含结尾引号之后再算一位）。
fn string_end(bytes: &[u8], start: usize) -> usize {
    let mut i = start + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }
    bytes.len()
}

/// 字符字面量 `'x'` / `'\n'` / `'\u{1F600}'` 的结束下标；生命周期 `'a` 只吃掉撇号本身。
fn char_or_lifetime_end(bytes: &[u8], start: usize) -> usize {
    let mut i = start + 1;
    if bytes.get(i) == Some(&b'\\') {
        i += 1;
        while i < bytes.len() && bytes[i] != b'\'' {
            i += 1;
        }
        return (i + 1).min(bytes.len());
    }
    // 普通字符：按 UTF-8 宽度跨过一个字符，再看下一个是不是收尾引号
    let width = utf8_width(bytes.get(i).copied().unwrap_or(b' '));
    let after = i + width;
    if bytes.get(after) == Some(&b'\'') {
        return after + 1;
    }
    // 不是字符字面量 → 生命周期，只跳过撇号
    start + 1
}

fn utf8_width(first: u8) -> usize {
    match first {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1,
    }
}

// ===========================================================================
// 三、函数扫描：按源码顺序找出每个 `fn`，记录名字与函数体范围
// ===========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct FnSpan {
    /// 函数名（不含类型前缀）。
    name: String,
    /// 函数体左花括号的字节下标。
    body_start: usize,
    /// 与它配对的右花括号的字节下标。
    body_end: usize,
    /// 函数体（代码部分）里是否出现占位调用 `todo_exercise`。
    placeholder: bool,
}

/// 扫描源码里的所有 `fn`（含 `impl`/`trait` 内的方法；嵌套在函数体里的 fn 不单独登记）。
fn scan_functions(src: &str) -> Vec<FnSpan> {
    let masked = mask_non_code(src);
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < masked.len() {
        if !is_keyword_at(&masked, i, b"fn") {
            i += 1;
            continue;
        }
        // 读函数名
        let mut j = i + 2;
        while masked.get(j).is_some_and(|b| b.is_ascii_whitespace()) {
            j += 1;
        }
        let name_start = j;
        while masked.get(j).is_some_and(|b| is_ident_byte(*b)) {
            j += 1;
        }
        if j == name_start {
            // `fn(i32) -> i32` 这类函数指针类型：没有函数名，跳过
            i += 2;
            continue;
        }
        let name = src[name_start..j].to_string();
        // 找函数体的 `{`；签名里深度为 0 的 `;` 说明这是 trait 里的方法声明（没有函数体）。
        // 注意：`-> [&'static str; 3]` 这类返回类型里也有 `;`，所以要先看括号/方括号深度。
        let mut k = j;
        let mut declaration = false;
        let mut type_depth = 0i32;
        while k < masked.len() {
            match masked[k] {
                b'(' | b'[' => type_depth += 1,
                b')' | b']' => type_depth -= 1,
                b'{' if type_depth <= 0 => break,
                b';' if type_depth <= 0 => {
                    declaration = true;
                    break;
                }
                _ => {}
            }
            k += 1;
        }
        if declaration || k >= masked.len() {
            i = k + 1;
            continue;
        }
        let body_start = k;
        let mut depth = 0usize;
        let mut body_end = None;
        while k < masked.len() {
            match masked[k] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        body_end = Some(k);
                        break;
                    }
                }
                _ => {}
            }
            k += 1;
        }
        let Some(body_end) = body_end else {
            // 花括号不配对：交给编译器报错，这里只做保守处理
            i += 2;
            continue;
        };
        let placeholder = masked[body_start..body_end]
            .windows(b"todo_exercise".len())
            .any(|w| w == b"todo_exercise");
        spans.push(FnSpan {
            name,
            body_start,
            body_end,
            placeholder,
        });
        // 跳过整个函数体：嵌套 fn 属于函数体的一部分，不单独登记
        i = body_end + 1;
    }
    spans
}

fn is_keyword_at(masked: &[u8], index: usize, keyword: &[u8]) -> bool {
    if !masked[index..].starts_with(keyword) {
        return false;
    }
    if index > 0 && is_ident_byte(masked[index - 1]) {
        return false;
    }
    match masked.get(index + keyword.len()) {
        Some(byte) if is_ident_byte(*byte) => false,
        Some(byte) if byte.is_ascii_whitespace() => true,
        _ => false,
    }
}

fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// 把每个函数体替换成占位标记后的文本——用来比较「函数体之外」是否逐字节一致。
fn canonical_text(src: &str, spans: &[FnSpan]) -> String {
    let mut out = String::new();
    let mut pos = 0usize;
    for (index, span) in spans.iter().enumerate() {
        out.push_str(&src[pos..span.body_start]);
        out.push_str(&format!("{{#body{index}#}}"));
        pos = span.body_end + 1;
    }
    out.push_str(&src[pos..]);
    out
}

/// 用参考答案的函数体替换骨架的函数体（只替换函数体，签名与其它内容保持骨架原样）。
fn applied_text(
    skeleton: &str,
    skeleton_spans: &[FnSpan],
    solution: &str,
    solution_spans: &[FnSpan],
) -> String {
    let mut out = String::new();
    let mut pos = 0usize;
    for (skel, sol) in skeleton_spans.iter().zip(solution_spans.iter()) {
        out.push_str(&skeleton[pos..skel.body_start + 1]);
        out.push_str(&solution[sol.body_start + 1..sol.body_end]);
        pos = skel.body_end;
    }
    out.push_str(&skeleton[pos..]);
    out
}

// ===========================================================================
// 四、一对文件的检查
// ===========================================================================

struct Pair {
    /// 文件名，例如 `lesson_05_ownership_borrowing.rs`。
    name: String,
    /// 对应的测试目标名（= 文件去扩展名）。
    target: String,
    assess_path: PathBuf,
    solution_path: PathBuf,
}

/// 列出所有「考核文件 ↔ 参考答案」配对：以 `assessments/solutions/lesson_*.rs` 为准。
fn load_pairs(root: &Path, filters: &[String]) -> Result<Vec<Pair>, String> {
    let solutions_dir = root.join(SOLUTIONS_DIR);
    let mut pairs = Vec::new();
    let mut names: Vec<String> = Vec::new();
    if solutions_dir.is_dir() {
        for entry in
            fs::read_dir(&solutions_dir).map_err(|e| format!("读取 {SOLUTIONS_DIR} 失败：{e}"))?
        {
            let entry = entry.map_err(|e| format!("读取 {SOLUTIONS_DIR} 失败：{e}"))?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with("lesson_") && file_name.ends_with(".rs") {
                names.push(file_name);
            }
        }
    }
    names.sort();
    for name in names {
        let target = name.trim_end_matches(".rs").to_string();
        if !filters.is_empty() && !filters.iter().any(|f| lesson_matches(f, &target)) {
            continue;
        }
        pairs.push(Pair {
            assess_path: root.join(ASSESS_DIR).join(&name),
            solution_path: root.join(SOLUTIONS_DIR).join(&name),
            name,
            target,
        });
    }
    if pairs.is_empty() {
        return Err(format!(
            "没有在 {SOLUTIONS_DIR}/ 下找到参考答案文件（或 --lesson 过滤后为空）"
        ));
    }
    Ok(pairs)
}

/// `--lesson` 允许写 `05`、`lesson_05`、`lesson_05_ownership_borrowing` 或完整目标名。
fn lesson_matches(filter: &str, target: &str) -> bool {
    let trimmed = filter.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        let prefix = format!("lesson_{:0>2}", trimmed.parse::<u32>().unwrap_or(0));
        return target.starts_with(&prefix);
    }
    target == trimmed || target.starts_with(trimmed)
}

struct CheckReport {
    exercise_count: usize,
    /// `骨架` / `已应用` / `混合`
    state: &'static str,
    /// 空表示检查通过
    problems: Vec<String>,
}

fn check_pair(pair: &Pair) -> Result<CheckReport, String> {
    let skeleton = fs::read_to_string(&pair.assess_path)
        .map_err(|e| format!("读取 {} 失败：{e}", pair.assess_path.display()))?;
    let solution = fs::read_to_string(&pair.solution_path)
        .map_err(|e| format!("读取 {} 失败：{e}", pair.solution_path.display()))?;
    let skel_spans = scan_functions(&skeleton);
    let sol_spans = scan_functions(&solution);
    let mut problems = Vec::new();
    if skel_spans.len() != sol_spans.len() {
        problems.push(format!(
            "函数个数不一致：考核文件 {} 个，参考答案 {} 个",
            skel_spans.len(),
            sol_spans.len()
        ));
    }
    for (index, (skel, sol)) in skel_spans.iter().zip(sol_spans.iter()).enumerate() {
        if skel.name != sol.name {
            problems.push(format!(
                "第 {} 个函数名不一致：考核文件 `{}`，参考答案 `{}`",
                index + 1,
                skel.name,
                sol.name
            ));
        }
    }
    if problems.is_empty() {
        let skel_canonical = canonical_text(&skeleton, &skel_spans);
        let sol_canonical = canonical_text(&solution, &sol_spans);
        if skel_canonical != sol_canonical {
            problems.push(first_difference(
                "函数体之外的代码/注释不一致",
                &skel_canonical,
                &sol_canonical,
            ));
        }
    }
    let exercise_count = skel_spans.iter().filter(|s| s.placeholder).count();
    for (skel, sol) in skel_spans.iter().zip(sol_spans.iter()) {
        if skel.placeholder {
            if sol.placeholder {
                problems.push(format!(
                    "练习 `{}` 的参考答案里仍是 todo_exercise 占位",
                    skel.name
                ));
            } else if solution[sol.body_start + 1..sol.body_end].trim().is_empty() {
                problems.push(format!("练习 `{}` 的参考答案函数体是空的", skel.name));
            }
        } else if skeleton[skel.body_start + 1..skel.body_end]
            != solution[sol.body_start + 1..sol.body_end]
        {
            problems.push(format!(
                "非练习函数 `{}`（通常是 #[test]）的函数体被改动了；参考答案不得修改考核标准",
                skel.name
            ));
        }
    }
    let remaining = skel_spans.iter().filter(|s| s.placeholder).count();
    let state = if remaining == exercise_count && exercise_count > 0 {
        "骨架"
    } else if remaining == 0 {
        "已应用"
    } else {
        "混合"
    };
    Ok(CheckReport {
        exercise_count,
        state,
        problems,
    })
}

fn first_difference(label: &str, left: &str, right: &str) -> String {
    let left_bytes = left.as_bytes();
    let right_bytes = right.as_bytes();
    let mut index = 0usize;
    while index < left_bytes.len()
        && index < right_bytes.len()
        && left_bytes[index] == right_bytes[index]
    {
        index += 1;
    }
    let context = |bytes: &[u8]| {
        let start = index.saturating_sub(30);
        let end = (index + 30).min(bytes.len());
        String::from_utf8_lossy(&bytes[start..end]).replace('\n', "⏎")
    };
    format!(
        "{label}（首个差异在第 {} 字节）：考核文件 `…{}…` 参考答案 `…{}…`",
        index + 1,
        context(left_bytes),
        context(right_bytes)
    )
}

// ===========================================================================
// 五、应用 / 还原 / 验证
// ===========================================================================

fn backup_path(root: &Path, pair: &Pair) -> PathBuf {
    root.join(BACKUP_DIR).join(&pair.name)
}

fn apply_pair(root: &Path, pair: &Pair) -> Result<String, String> {
    let report = check_pair(pair)?;
    if !report.problems.is_empty() {
        return Err(format!(
            "{}：参考答案未通过校验，先修好再应用",
            report.problems[0]
        ));
    }
    if report.state == "已应用" {
        return Ok("已经是参考答案状态，跳过".to_string());
    }
    if report.state == "混合" {
        return Err(
            "考核文件处于「混合」状态（部分练习被实现过）：请先 --restore 或手工还原".to_string(),
        );
    }
    let skeleton = fs::read_to_string(&pair.assess_path).map_err(|e| e.to_string())?;
    let solution = fs::read_to_string(&pair.solution_path).map_err(|e| e.to_string())?;
    let backup = backup_path(root, pair);
    if let Some(parent) = backup.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建 {} 失败：{e}", parent.display()))?;
    }
    fs::write(&backup, &skeleton).map_err(|e| format!("写入备份失败：{e}"))?;
    let applied = applied_text(
        &skeleton,
        &scan_functions(&skeleton),
        &solution,
        &scan_functions(&solution),
    );
    fs::write(&pair.assess_path, &applied)
        .map_err(|e| format!("写入 {} 失败：{e}", pair.assess_path.display()))?;
    Ok(format!("已应用参考答案（骨架备份在 {}）", backup.display()))
}

fn restore_pair(root: &Path, pair: &Pair) -> Result<String, String> {
    let backup = backup_path(root, pair);
    if !backup.is_file() {
        return Err(format!(
            "找不到骨架备份 {}；若从未 --apply 过则无需还原",
            backup.display()
        ));
    }
    let skeleton = fs::read_to_string(&backup).map_err(|e| e.to_string())?;
    fs::write(&pair.assess_path, skeleton)
        .map_err(|e| format!("写入 {} 失败：{e}", pair.assess_path.display()))?;
    let report = check_pair(pair)?;
    if report.state != "骨架" {
        return Err(format!("还原后状态异常：{}", report.state));
    }
    Ok("已还原骨架".to_string())
}

/// 运行一个测试目标，返回（通过数, 失败数, 是否编译成功）。
///
/// 子进程的输出**重定向到日志文件**（而不是管道）：某些受限环境不允许给子进程接管道，
/// 写文件则没有这个限制；同时日志也方便失败时排查。
fn run_target(root: &Path, target: &str) -> Result<(usize, usize, bool), String> {
    let data_dir = root.join(VERIFY_DATA_DIR);
    fs::create_dir_all(&data_dir).map_err(|e| format!("创建 {} 失败：{e}", data_dir.display()))?;
    // 子进程的临时目录也指到工作区里：受限环境下 %TEMP% 可能不可写，而链接器（gcc/ld）
    // 一定要写临时文件；指到 target/ 下既安全，也不污染系统目录。
    let tmp_dir = data_dir.join("tmp");
    fs::create_dir_all(&tmp_dir).map_err(|e| format!("创建 {} 失败：{e}", tmp_dir.display()))?;
    let log_path = data_dir.join(format!("{target}.log"));
    let log = fs::File::create(&log_path)
        .map_err(|e| format!("创建日志 {} 失败：{e}", log_path.display()))?;
    let log_err = log
        .try_clone()
        .map_err(|e| format!("复制日志句柄失败：{e}"))?;
    let status = Command::new("cargo")
        .args(["test", "--test", target, "--no-fail-fast"])
        .current_dir(root)
        .env("ASSESSMENT_DATA_DIR", &data_dir)
        .env("ASSESSMENT_RUN_ID", "solution-verify")
        .env("TMP", &tmp_dir)
        .env("TEMP", &tmp_dir)
        .env("TMPDIR", &tmp_dir)
        .stdout(std::process::Stdio::from(log))
        .stderr(std::process::Stdio::from(log_err))
        .status()
        .map_err(|e| format!("运行 cargo test 失败：{e}"))?;
    let text = fs::read_to_string(&log_path).unwrap_or_default();
    let (passed, failed) = parse_test_result(&text);
    let compile_error = text.contains("error[") || text.contains("could not compile");
    // 以退出码为准：测试全绿才算通过
    let ok = status.success() && !compile_error && failed == 0;
    Ok((passed, failed, ok))
}

/// 从 cargo test 输出里累计 `test result: ok. N passed; M failed; …`。
fn parse_test_result(text: &str) -> (usize, usize) {
    let mut passed = 0usize;
    let mut failed = 0usize;
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("test result:") else {
            continue;
        };
        for part in rest.split(';') {
            let part = part.trim();
            // 形如 `ok. 8 passed` / `FAILED. 3 passed`：取 " passed" 之前最后一个数字词
            if let Some(index) = part.find(" passed") {
                let number = part[..index].split_whitespace().last().unwrap_or("");
                passed += number.parse::<usize>().unwrap_or(0);
            }
            if let Some(index) = part.find(" failed") {
                let number = part[..index].split_whitespace().last().unwrap_or("");
                failed += number.parse::<usize>().unwrap_or(0);
            }
        }
    }
    (passed, failed)
}

/// 从日志里挑出失败/编译错误的行，失败时打印给用户看。
fn failure_lines(root: &Path, target: &str) -> Vec<String> {
    let log_path = root.join(VERIFY_DATA_DIR).join(format!("{target}.log"));
    let Ok(text) = fs::read_to_string(&log_path) else {
        return Vec::new();
    };
    text.lines()
        .filter(|line| {
            let line = line.trim();
            line.starts_with("error")
                || line.contains("FAILED")
                || line.contains("panicked at")
                || line.starts_with("--> ")
        })
        .take(12)
        .map(|line| line.trim().to_string())
        .collect()
}

// ===========================================================================
// 六、主流程
// ===========================================================================

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let options = match parse_args(&args) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("[用法错误] {message}");
            std::process::exit(2);
        }
    };
    let root = repo_root();
    let pairs = match load_pairs(&root, &options.lessons) {
        Ok(pairs) => pairs,
        Err(message) => {
            eprintln!("[错误] {message}");
            std::process::exit(2);
        }
    };
    let code = match options.mode {
        Mode::Check => run_check(&pairs),
        Mode::List => run_list(&pairs),
        Mode::Status => run_status(&pairs),
        Mode::Apply => run_apply(&root, &pairs),
        Mode::Restore => run_restore(&root, &pairs),
        Mode::Verify => run_verify(&root, &pairs),
    };
    std::process::exit(code);
}

fn run_check(pairs: &[Pair]) -> i32 {
    let mut failures = 0usize;
    println!("=== 参考答案校验（{} 个文件）===", pairs.len());
    for pair in pairs {
        match check_pair(pair) {
            Ok(report) if report.problems.is_empty() => {
                println!(
                    "[通过] {:<42} 练习 {:<3} 状态：{}",
                    pair.name, report.exercise_count, report.state
                );
            }
            Ok(report) => {
                failures += 1;
                println!("[不通过] {}", pair.name);
                for problem in &report.problems {
                    println!("         - {problem}");
                }
            }
            Err(message) => {
                failures += 1;
                println!("[不通过] {}：{message}", pair.name);
            }
        }
    }
    if failures == 0 {
        println!("结论：全部通过（参考答案与考核标准一致）✅");
        0
    } else {
        println!("结论：{failures} 个文件不通过 ❌");
        1
    }
}

fn run_list(pairs: &[Pair]) -> i32 {
    let mut total = 0usize;
    println!("=== 参考答案清单 ===");
    for pair in pairs {
        let skeleton = fs::read_to_string(&pair.assess_path).unwrap_or_default();
        let solution = fs::read_to_string(&pair.solution_path).unwrap_or_default();
        let skel_spans = scan_functions(&skeleton);
        let sol_spans = scan_functions(&solution);
        println!("\n{}（{}）", pair.name, pair.target);
        let mut count = 0usize;
        for (index, skel) in skel_spans.iter().enumerate() {
            if !skel.placeholder {
                continue;
            }
            let Some(sol) = sol_spans.get(index) else {
                println!("  ??. {:<44} 参考答案缺少对应函数", skel.name);
                continue;
            };
            count += 1;
            let line = solution[..sol.body_start].matches('\n').count() + 1;
            println!("  {count:>2}. {:<44} 参考答案第 {line} 行", skel.name);
        }
        total += count;
        println!("  小计：{count} 个练习");
    }
    println!("\n合计：{total} 个练习（对应 157 个知识点）");
    0
}

fn run_status(pairs: &[Pair]) -> i32 {
    println!("=== 考核文件状态 ===");
    for pair in pairs {
        match check_pair(pair) {
            Ok(report) => println!(
                "{:<42} 练习 {:<3} 状态：{}",
                pair.name, report.exercise_count, report.state
            ),
            Err(message) => println!("{:<42} 读取失败：{message}", pair.name),
        }
    }
    0
}

fn run_apply(root: &Path, pairs: &[Pair]) -> i32 {
    let mut failures = 0usize;
    for pair in pairs {
        match apply_pair(root, pair) {
            Ok(message) => println!("[应用] {:<42} {message}", pair.name),
            Err(message) => {
                failures += 1;
                println!("[失败] {:<42} {message}", pair.name);
            }
        }
    }
    if failures == 0 { 0 } else { 1 }
}

fn run_restore(root: &Path, pairs: &[Pair]) -> i32 {
    let mut failures = 0usize;
    for pair in pairs {
        match restore_pair(root, pair) {
            Ok(message) => println!("[还原] {:<42} {message}", pair.name),
            Err(message) => {
                failures += 1;
                println!("[失败] {:<42} {message}", pair.name);
            }
        }
    }
    if failures == 0 { 0 } else { 1 }
}

fn run_verify(root: &Path, pairs: &[Pair]) -> i32 {
    let mut failures = 0usize;
    let mut total_passed = 0usize;
    let mut total_failed = 0usize;
    println!("=== 参考答案验证（应用 → cargo test → 自动还原）===");
    for pair in pairs {
        let applied = apply_pair(root, pair);
        if let Err(message) = &applied {
            failures += 1;
            println!("[失败] {:<42} {message}", pair.name);
            continue;
        }
        let outcome = run_target(root, &pair.target);
        let restored = restore_pair(root, pair);
        match (outcome, restored) {
            (Ok((passed, failed, ok)), Ok(_)) => {
                total_passed += passed;
                total_failed += failed;
                if ok && passed > 0 {
                    println!("[通过] {:<42} {passed} passed", pair.name);
                } else {
                    failures += 1;
                    println!(
                        "[不通过] {:<42} {passed} passed / {failed} failed",
                        pair.name
                    );
                    for line in failure_lines(root, &pair.target) {
                        println!("         {line}");
                    }
                }
            }
            (Err(message), _) => {
                failures += 1;
                println!("[失败] {:<42} {message}", pair.name);
            }
            (_, Err(message)) => {
                failures += 1;
                println!("[还原失败] {:<42} {message}（请手工检查）", pair.name);
            }
        }
    }
    println!("\n合计：{total_passed} passed / {total_failed} failed");
    if failures == 0 {
        println!("结论：参考答案下全部通过（157 个知识点全部可解）✅");
        0
    } else {
        println!("结论：{failures} 个文件有问题 ❌");
        1
    }
}

// ===========================================================================
// 七、单元测试（只测纯函数：屏蔽、扫描、替换）
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_result_sums_all_targets() {
        let text = "\
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
";
        assert_eq!(parse_test_result(text), (11, 2));
    }

    #[test]
    fn mask_hides_strings_and_comments() {
        let src = "fn a() { let s = \"todo_exercise(\"; // todo_exercise(\n }";
        let masked = String::from_utf8(mask_non_code(src)).unwrap();
        assert!(!masked.contains("todo_exercise"));
        assert!(masked.contains("fn a()"));
    }

    #[test]
    fn scan_finds_functions_and_placeholder() {
        let src = "\
fn outer() {
    fn inner() {}
}
impl Foo {
    fn method(&self) -> u32 {
        assessment_harness::todo_exercise(\"m\", \"d\", ())
    }
}
trait T { fn declared(&self) -> u32; }
";
        let spans = scan_functions(src);
        // 没有函数体的 trait 声明（`fn declared(&self) -> u32;`）不登记：
        // 它没有可替换的函数体，但它的文本仍参与「函数体之外」的逐字节比较。
        let names: Vec<&str> = spans.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["outer", "method"]);
        assert!(!spans[0].placeholder);
        assert!(spans[1].placeholder);
    }

    #[test]
    fn scan_ignores_function_pointer_types() {
        let src = "fn apply(f: fn(i32) -> i32, v: i32) -> i32 { f(v) }";
        let spans = scan_functions(src);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].name, "apply");
    }

    #[test]
    fn scan_handles_lifetimes_and_raw_strings() {
        let src = "\
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() > b.len() { a } else { b } }
fn raw() -> &'static str { r#\"has \" quote\"# }
fn ch() -> char { '}' }
";
        let spans = scan_functions(src);
        let names: Vec<&str> = spans.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["longest", "raw", "ch"]);
        for span in &spans {
            assert_eq!(
                mask_non_code(&src[span.body_start..=span.body_end])[0],
                b'{'
            );
        }
    }

    #[test]
    fn scan_handles_array_return_types() {
        // `-> [&'static str; 3]` 里的 `;` 不是「方法声明」的结束符
        let src = "fn codes() -> [&'static str; 3] { [\"a\", \"b\", \"c\"] }\nfn next() {}";
        let spans = scan_functions(src);
        let names: Vec<&str> = spans.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["codes", "next"]);
    }

    #[test]
    fn canonical_text_ignores_body_length_changes() {
        let skeleton = "fn a() { todo_exercise() }\nfn b() { 1 }";
        let solution = "fn a() {\n    1 + 2 + 3\n}\nfn b() { 1 }";
        assert_eq!(
            canonical_text(skeleton, &scan_functions(skeleton)),
            canonical_text(solution, &scan_functions(solution))
        );
    }

    #[test]
    fn canonical_text_replaces_bodies() {
        let src = "fn a() { 1 }\nfn b() { 2 }";
        let spans = scan_functions(src);
        assert_eq!(
            canonical_text(src, &spans),
            "fn a() {#body0#}\nfn b() {#body1#}"
        );
    }

    #[test]
    fn applied_text_keeps_skeleton_outside_bodies() {
        let skeleton = "fn a() -> u32 {\n    todo_exercise()\n}\n// 说明\nfn b() {}\n";
        let solution = "fn a() -> u32 {\n    42\n}\n// 说明\nfn b() {}\n";
        let applied = applied_text(
            skeleton,
            &scan_functions(skeleton),
            solution,
            &scan_functions(solution),
        );
        assert!(applied.contains("42"));
        assert!(applied.contains("// 说明"));
        assert!(!applied.contains("todo_exercise"));
        assert_eq!(applied.matches("fn a()").count(), 1);
    }

    #[test]
    fn lesson_filter_accepts_number_and_target() {
        assert!(lesson_matches("05", "lesson_05_ownership_borrowing"));
        assert!(lesson_matches("lesson_05", "lesson_05_ownership_borrowing"));
        assert!(lesson_matches(
            "lesson_05_ownership_borrowing",
            "lesson_05_ownership_borrowing"
        ));
        assert!(!lesson_matches("05", "lesson_15_iterators"));
        assert!(!lesson_matches("06", "lesson_05_ownership_borrowing"));
    }
}
