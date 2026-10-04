//! assessments/bin/assessment_report.rs —— 学习评估报告生成器
//!
//! 职责：读取考核框架 `assessment_harness` 落盘的账本（TSV），汇总每个模块、
//! 每个知识点的掌握情况，输出：
//!
//! - Markdown 学习评估报告（人读）：`<报告目录>/assessment_report.md`
//! - 机器可读摘要：`<报告目录>/summary.json`
//! - 学习进度历史（每次运行追加一行）：`<数据目录>/history.jsonl`
//! - 知识点覆盖矩阵：`<项目>/assessments/docs/04_knowledge_map.md`（`--emit-map`）
//!
//! 设计要点（教学项目，读者是零基础新人）：
//!
//! 1. **只用标准库**：参数解析、时间换算、JSON 拼装全部手写，不引入任何第三方 crate；
//! 2. **账本转义复用 harness**：`escape` / `unescape` 是账本格式的唯一权威实现。
//!    本工具**写账本的是 harness，读账本的是本文件**，所以解析一律调用
//!    `assessment_harness::unescape`，绝不自己另写一套，避免格式漂移
//!    （`escape` 在同一模块的单元测试里用来验证这一对函数确实互逆）；
//! 3. **函数职责单一**：解析 → 判定 → 统计 → 渲染 → 写盘 分层，主流程一眼能读完；
//! 4. 任何 IO / 用法错误都收拢成 `AppError`，由 `main` 统一打印并设置退出码。
//!
//! 退出码约定：本次运行全部通过 → 0；否则 → 1；用法错误 → 2；
//! `--list` / `--emit-map` 正常完成 → 0；`--check-map` 不一致 → 1。

use assessment_harness::unescape;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};

/// 账本列数。尾部的 detail / hint / guide 里可能有真实换行，所以只按前 11 个制表符切分。
const LEDGER_COLUMNS: usize = 12;

/// 知识点类型的固定顺序：基础 / 核心 / 难点 / 边界。
const KIND_ORDER: [&str; 4] = ["basic", "core", "hard", "edge"];

/// 模块 id → 中文主题名。
///
/// 账本里本来就带 `module_name`，正常情况下不需要这张表；它只用于「某个模块从来没跑过、
/// 账本里一条记录都没有」的场景（否则报告里的模块名只能写成占位符）。
/// 维护者新增课程时，请同步 `assessments/harness/lib.rs` 的 `MODULES` 表与本表
/// （见 `assessments/docs/05_maintainer_guide.md` 第四节）。
const MODULE_LABELS: &[(&str, &str)] = &[
    ("lesson_01", "变量与可变性"),
    ("lesson_02", "数据类型"),
    ("lesson_03", "函数"),
    ("lesson_04", "流程控制"),
    ("lesson_05", "所有权系统"),
    ("lesson_06", "结构体"),
    ("lesson_07", "枚举与模式匹配"),
    ("lesson_08", "常见集合"),
    ("lesson_09", "包和模块"),
    ("lesson_10", "错误处理"),
    ("lesson_11", "泛型"),
    ("lesson_12", "Trait"),
    ("lesson_13", "生命周期"),
    ("lesson_14", "闭包"),
    ("lesson_15", "迭代器"),
    ("lesson_16", "智能指针"),
    ("lesson_17", "线程与通道"),
    ("lesson_18", "声明宏"),
];

/// 查模块中文名（账本里没有记录时的兜底）。
fn module_label(module_id: &str) -> &'static str {
    MODULE_LABELS
        .iter()
        .find(|(id, _)| *id == module_id)
        .map(|(_, name)| *name)
        .unwrap_or("（未登记模块）")
}

/// `--list` / `--emit-map` / `--check-map` 以这个长度截断长列表。
const PREVIEW_LIMIT: usize = 6;

// ===========================================================================
// 一、错误类型与文件写入
// ===========================================================================

/// 统一错误类型：`main` 负责打印它并决定退出码。
enum AppError {
    /// 命令行用法错误（参数缺失、模式互斥等）。
    Usage(String),
    /// 文件与目录操作失败。
    Io(io::Error),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

/// 把「动作 + 路径 + 原因」拼成一句新人看得懂的报错。
fn io_error(action: &str, path: &Path, error: &io::Error) -> AppError {
    AppError::Io(io::Error::new(
        error.kind(),
        format!("{action} {} 失败：{error}", path.display()),
    ))
}

/// 写文件（自动创建父目录）。
fn write_text(path: &Path, text: &str) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|error| io_error("创建目录", parent, &error))?;
        }
    }
    fs::write(path, text).map_err(|error| io_error("写入文件", path, &error))
}

/// 追加一行文本（文件不存在就创建），用于 `history.jsonl`。
fn append_line(path: &Path, line: &str) -> Result<(), AppError> {
    let mut text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(io_error("读取文件", path, &error)),
    };
    text.push_str(line);
    text.push('\n');
    write_text(path, &text)
}

// ===========================================================================
// 二、命令行参数（手写解析，不引入 clap）
// ===========================================================================

/// 运行模式：三者互斥；都不指定时走默认的「分析最新一次运行」。
enum Mode {
    /// 默认：分析一次运行，写报告 + 摘要 + 历史。
    Report,
    /// `--list`：列出账本里所有运行后退出。
    List,
    /// `--emit-map`：重新生成知识点覆盖矩阵后退出。
    EmitMap,
    /// `--check-map`：校验矩阵与账本并集是否一致后退出。
    CheckMap,
}

/// 解析后的命令行选项。
struct Options {
    mode: Mode,
    /// `--run <run_id>`：指定要分析的运行。
    run_id: Option<String>,
    /// `--data-dir <path>`：覆盖数据目录。
    data_dir: Option<PathBuf>,
    /// `--report-dir <path>`：覆盖报告目录。
    report_dir: Option<PathBuf>,
    /// `--no-history`：不追加 history.jsonl。
    no_history: bool,
}

/// `--help` 打印的用法说明。
fn help_text() -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "assessment_report —— 学习评估报告生成器（零第三方依赖）"
    );
    let _ = writeln!(out, "\n用法：");
    let _ = writeln!(out, "  assessment_report [选项]");
    let _ = writeln!(out, "\n选项：");
    let _ = writeln!(
        out,
        "  （无选项）            分析最新一次运行：写报告 + 追加历史 + 打印控制台摘要"
    );
    let _ = writeln!(
        out,
        "  --run <run_id>        分析指定运行（run_id 见 --list）"
    );
    let _ = writeln!(out, "  --list                列出账本里所有运行后退出");
    let _ = writeln!(
        out,
        "  --emit-map            重新生成知识点覆盖矩阵 docs/04_knowledge_map.md 后退出"
    );
    let _ = writeln!(
        out,
        "  --check-map           校验矩阵与账本并集是否一致后退出（不一致时退出码 1）"
    );
    let _ = writeln!(out, "  --no-history          不追加 history.jsonl");
    let _ = writeln!(
        out,
        "  --data-dir <path>     覆盖数据目录（默认取环境变量 ASSESSMENT_DATA_DIR）"
    );
    let _ = writeln!(
        out,
        "  --report-dir <path>   覆盖报告目录（默认取环境变量 ASSESSMENT_REPORT_DIR）"
    );
    let _ = writeln!(out, "  --help                显示本说明");
    let _ = writeln!(out, "\n产物：");
    let _ = writeln!(
        out,
        "  <报告目录>/assessment_report.md   Markdown 学习评估报告"
    );
    let _ = writeln!(
        out,
        "  <报告目录>/summary.json           本次运行的机器可读摘要"
    );
    let _ = writeln!(
        out,
        "  <数据目录>/history.jsonl          每次运行追加一行，用于进度对比"
    );
    let _ = writeln!(
        out,
        "  assessments/docs/04_knowledge_map.md  知识点覆盖矩阵（--emit-map）"
    );
    let _ = writeln!(
        out,
        "\n退出码：本次运行全部通过 → 0；否则 → 1；参数错误 → 2。"
    );
    out
}

/// 取下一个参数的值（`--run xxx` 这种形式）。
fn next_value(args: &[String], index: &mut usize, flag: &str) -> Result<String, AppError> {
    *index += 1;
    match args.get(*index) {
        Some(value) => Ok(value.clone()),
        None => Err(AppError::Usage(format!("{flag} 缺少参数值"))),
    }
}

/// 解析命令行参数；不认识的位置参数会直接报用法错误。
fn parse_args(args: &[String]) -> Result<Options, AppError> {
    let mut options = Options {
        mode: Mode::Report,
        run_id: None,
        data_dir: None,
        report_dir: None,
        no_history: false,
    };
    let mut help = false;
    let mut list = false;
    let mut emit_map = false;
    let mut check_map = false;
    let mut index = 0usize;
    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "--help" | "-h" => help = true,
            "--list" => list = true,
            "--emit-map" => emit_map = true,
            "--check-map" => check_map = true,
            "--no-history" => options.no_history = true,
            "--run" => options.run_id = Some(next_value(args, &mut index, "--run")?),
            "--data-dir" => {
                options.data_dir = Some(PathBuf::from(next_value(args, &mut index, "--data-dir")?));
            }
            "--report-dir" => {
                options.report_dir =
                    Some(PathBuf::from(next_value(args, &mut index, "--report-dir")?));
            }
            other => {
                return Err(AppError::Usage(format!(
                    "无法识别的参数：{other}（用 --help 查看用法）"
                )));
            }
        }
        index += 1;
    }
    if help {
        println!("{}", help_text());
        std::process::exit(0);
    }
    if [list, emit_map, check_map]
        .iter()
        .filter(|flag| **flag)
        .count()
        > 1
    {
        return Err(AppError::Usage(
            "--list / --emit-map / --check-map 一次只能用一个".to_string(),
        ));
    }
    options.mode = if list {
        Mode::List
    } else if emit_map {
        Mode::EmitMap
    } else if check_map {
        Mode::CheckMap
    } else {
        Mode::Report
    };
    Ok(options)
}

// ===========================================================================
// 三、账本（ledger.tsv）：解析、分组、判定
// ===========================================================================

/// 账本里的一条记录（12 列，文本字段已由 harness 转义）。
struct LedgerRecord {
    run_id: String,
    ts_ms: u128,
    event: String,
    module_id: String,
    module_name: String,
    kp_id: String,
    kp_title: String,
    kind: String,
    location: String,
    detail: String,
    hint: String,
    guide: String,
}

impl LedgerRecord {
    /// 按字段名取文本值（判定与合并逻辑只关心这四个字段）。
    fn field(&self, name: &str) -> &str {
        match name {
            "location" => &self.location,
            "detail" => &self.detail,
            "hint" => &self.hint,
            "guide" => &self.guide,
            _ => "",
        }
    }
}

/// 解析账本的一行；列数不足（格式不对）时返回 None，由调用方计入警告。
fn parse_ledger_line(line: &str) -> Option<LedgerRecord> {
    let parts: Vec<&str> = line.splitn(LEDGER_COLUMNS, '\t').collect();
    if parts.len() < LEDGER_COLUMNS {
        return None;
    }
    let ts_ms = parts[1].trim().parse::<u128>().unwrap_or(0);
    Some(LedgerRecord {
        run_id: unescape(parts[0]),
        ts_ms,
        event: unescape(parts[2]),
        module_id: unescape(parts[3]),
        module_name: unescape(parts[4]),
        kp_id: unescape(parts[5]),
        kp_title: unescape(parts[6]),
        kind: unescape(parts[7]),
        location: unescape(parts[8]),
        detail: unescape(parts[9]),
        hint: unescape(parts[10]),
        guide: unescape(parts[11]),
    })
}

/// 读取整个账本；文件不存在时视为「还没有数据」而不是错误。
fn load_ledger(path: &Path) -> Result<Vec<LedgerRecord>, AppError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(io_error("读取账本", path, &error)),
    };
    let mut records = Vec::new();
    let mut broken = 0usize;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match parse_ledger_line(line) {
            Some(record) => records.push(record),
            None => broken += 1,
        }
    }
    if broken > 0 {
        eprintln!("提示：账本里有 {broken} 行格式不完整，已跳过。");
    }
    Ok(records)
}

/// 一个知识点在**某一次运行**里的判定结果。
#[derive(Clone, Copy)]
enum KpOutcome {
    /// 最后一段事件里出现 `pass`：全部断言通过。
    Passed,
    /// 出现 `missing`：练习函数还是 `todo_exercise(...)` 占位。
    Unimplemented,
    /// 出现 `fail`：实现了但断言不成立。
    Failed,
    /// 出现 `panic`：实现里发生了意外 panic。
    Panicked,
    /// 只有 `start`：测试没跑完（中断、或者只登记没执行）。
    Incomplete,
    /// 本次运行里**完全没有记录**（模块没跑或编译失败）。
    NotRun,
}

impl KpOutcome {
    /// 报告里的中文状态名。
    fn label(self) -> &'static str {
        match self {
            KpOutcome::Passed => "通过",
            KpOutcome::Unimplemented => "未实现",
            KpOutcome::Failed => "未通过",
            KpOutcome::Panicked => "运行期错误",
            KpOutcome::Incomplete => "未完成或中断",
            KpOutcome::NotRun => "未运行或编译失败",
        }
    }

    /// 是否算「掌握」（只有 Passed 计分）。
    fn is_passed(self) -> bool {
        matches!(self, KpOutcome::Passed)
    }

    /// 是否算「需要加强」（未运行不算：先解决能不能跑起来的问题）。
    fn needs_work(self) -> bool {
        !self.is_passed() && !matches!(self, KpOutcome::NotRun)
    }
}

/// 判定最后一段事件的结论：有 `pass` 即通过，否则按 missing / fail / panic 依次判断。
fn resolve_outcome(segment: &[&LedgerRecord]) -> KpOutcome {
    let has = |event: &str| segment.iter().any(|record| record.event == event);
    if has("pass") {
        KpOutcome::Passed
    } else if has("missing") {
        KpOutcome::Unimplemented
    } else if has("fail") {
        KpOutcome::Failed
    } else if has("panic") {
        KpOutcome::Panicked
    } else {
        KpOutcome::Incomplete
    }
}

/// 取最后一条匹配事件的记录里、某个字段的值。
fn last_field(segment: &[&LedgerRecord], event: &str, field: &str) -> String {
    segment
        .iter()
        .rev()
        .find(|record| record.event == event)
        .map(|record| record.field(field).to_string())
        .unwrap_or_default()
}

/// 一个知识点的完整判定结果（报告渲染只依赖它）。
struct KpResult {
    outcome: KpOutcome,
    module_id: String,
    module_name: String,
    kp_id: String,
    title: String,
    kind: String,
    location: String,
    /// fail 的 detail（期望 / 实际）。
    detail: String,
    /// 最近一条 fail 的 hint。
    hint: Option<String>,
    /// missing 的 detail（待实现的练习函数名）。
    missing_fn: String,
    /// missing 的 hint（实现要求）。
    missing_hint: String,
    /// panic 的原始信息。
    panic_text: String,
    /// 作者写的复习指引（`assess` 的第 5 个参数）。
    review: String,
    /// 复习入口：`复习入口：命令（课程文件）；说明见 README`。
    guide: String,
}

/// 把某知识点在本次运行里的全部记录合成判定结果。
///
/// 关键规则：只取**最后一段**（最后一个 `start` 及其之后的事件）——这样同一次运行里
/// 重复执行同一测试目标时，只有最后一次算数，不会重复计数。
fn merge_kp(entries: &[&LedgerRecord]) -> KpResult {
    let last_start = entries.iter().rposition(|record| record.event == "start");
    let segment: &[&LedgerRecord] = match last_start {
        Some(index) => &entries[index..],
        None => entries,
    };
    let lead = entries.last().copied();
    let text = |value: Option<&str>| value.unwrap_or_default().to_string();
    KpResult {
        outcome: resolve_outcome(segment),
        module_id: text(lead.map(|record| record.module_id.as_str())),
        module_name: text(lead.map(|record| record.module_name.as_str())),
        kp_id: text(lead.map(|record| record.kp_id.as_str())),
        title: text(lead.map(|record| record.kp_title.as_str())),
        kind: text(lead.map(|record| record.kind.as_str())),
        location: last_field(segment, "start", "location"),
        detail: last_field(segment, "fail", "detail"),
        hint: {
            let value = last_field(segment, "fail", "hint");
            if value.is_empty() { None } else { Some(value) }
        },
        missing_fn: last_field(segment, "missing", "detail"),
        missing_hint: last_field(segment, "missing", "hint"),
        panic_text: last_field(segment, "panic", "detail"),
        review: last_field(segment, "start", "hint"),
        guide: text(lead.map(|record| record.guide.as_str())),
    }
}

/// 按 `run_id` 过滤并按 `ts_ms` 排序（同一毫秒保持账本顺序）。
fn records_for_run<'a>(records: &'a [LedgerRecord], run_id: &str) -> Vec<&'a LedgerRecord> {
    let mut subset: Vec<&LedgerRecord> = records
        .iter()
        .filter(|record| record.run_id == run_id)
        .collect();
    subset.sort_by_key(|record| record.ts_ms);
    subset
}

/// 账本里出现过的全部 run_id，按开始时间升序。
fn list_runs(records: &[LedgerRecord]) -> Vec<(String, u128)> {
    let mut first_ts: BTreeMap<String, u128> = BTreeMap::new();
    for record in records {
        let entry = first_ts
            .entry(record.run_id.clone())
            .or_insert(record.ts_ms);
        if record.ts_ms < *entry {
            *entry = record.ts_ms;
        }
    }
    let mut runs: Vec<(String, u128)> = first_ts.into_iter().collect();
    runs.sort_by_key(|(_, ts)| *ts);
    runs
}

/// 全部运行出现过的知识点并集：kp_id → (module_id, module_name, kind)。
fn ledger_union(records: &[LedgerRecord]) -> BTreeMap<String, (String, String, String)> {
    let mut union: BTreeMap<String, (String, String, String)> = BTreeMap::new();
    for record in records {
        if record.kp_id.is_empty() {
            continue;
        }
        union.entry(record.kp_id.clone()).or_insert_with(|| {
            (
                record.module_id.clone(),
                record.module_name.clone(),
                record.kind.clone(),
            )
        });
    }
    union
}

/// 最近一次运行的 run_id（开始时间最大的那个）。
fn latest_run_id(records: &[LedgerRecord]) -> Option<String> {
    list_runs(records).pop().map(|(run_id, _)| run_id)
}

// ===========================================================================
// 四、预期知识点集合：账本并集 与 知识矩阵
// ===========================================================================

/// 解析知识矩阵：所有以 `| lesson_` 开头的表格行，第 3 列是 kp_id。
fn parse_map_kp_ids(map_text: &str) -> BTreeSet<String> {
    let mut kp_ids = BTreeSet::new();
    for line in map_text.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("| lesson_") {
            continue;
        }
        let columns: Vec<&str> = trimmed.split('|').map(|cell| cell.trim()).collect();
        if columns.len() > 3 && columns[1].starts_with("lesson_") && !columns[3].is_empty() {
            kp_ids.insert(columns[3].to_string());
        }
    }
    kp_ids
}

/// 集合差集（`left` 有而 `right` 没有），保持字典序。
fn difference(left: &BTreeSet<String>, right: &BTreeSet<String>) -> Vec<String> {
    left.difference(right).cloned().collect()
}

/// 生成预期知识点集合并说明来源：矩阵存在则以矩阵为准，同时报告与账本并集的差异。
fn expected_kp_ids(
    union: &BTreeMap<String, (String, String, String)>,
    map: Option<&BTreeSet<String>>,
) -> (BTreeSet<String>, String, String) {
    let union_ids: BTreeSet<String> = union.keys().cloned().collect();
    match map {
        Some(ids) => {
            let missing = difference(&union_ids, ids);
            let extra = difference(ids, &union_ids);
            let warning = if missing.is_empty() && extra.is_empty() {
                "知识点矩阵与账本并集一致。".to_string()
            } else {
                let mut text = String::new();
                if !missing.is_empty() {
                    let _ = write!(
                        text,
                        "账本里有 {} 个知识点不在矩阵中（矩阵可能过期，请重新运行 `--emit-map`）：{}；",
                        missing.len(),
                        preview_list(&missing)
                    );
                }
                if !extra.is_empty() {
                    let _ = write!(
                        text,
                        "矩阵里有 {} 个知识点从未出现在账本中（考核没运行过或已改名）：{}；",
                        extra.len(),
                        preview_list(&extra)
                    );
                }
                text
            };
            (ids.clone(), warning, "知识点矩阵".to_string())
        }
        None => (
            union_ids,
            "未找到知识点矩阵文件，本次以「账本中全部运行出现过的知识点并集」为准。".to_string(),
            "账本并集".to_string(),
        ),
    }
}

/// 把长列表压成一行预览（最多 `PREVIEW_LIMIT` 项，多了用「等」收尾）。
fn preview_list(items: &[String]) -> String {
    let head: Vec<&str> = items
        .iter()
        .take(PREVIEW_LIMIT)
        .map(String::as_str)
        .collect();
    if items.len() > head.len() {
        format!("{} 等", head.join("、"))
    } else if head.is_empty() {
        "（无）".to_string()
    } else {
        head.join("、")
    }
}

// ===========================================================================
// 五、从 Cargo.toml 解析「预期模块集合」
// ===========================================================================

/// 解析 `Cargo.toml` 里所有 `[[test]]` 段的 `name`。
///
/// 返回两样东西：
///   - 模块 id 列表（取名字前两段：`lesson_05_ownership_borrowing` → `lesson_05`）；
///   - 「模块 id → 测试目标名」映射，这样报告里能直接写出可照抄的命令
///     （`cargo test --test lesson_05_ownership_borrowing`），而不是 `--test lesson_05`。
fn parse_expected_modules(cargo_toml: &str) -> (Vec<String>, BTreeMap<String, String>) {
    let mut modules = Vec::new();
    let mut targets = BTreeMap::new();
    let mut in_test = false;
    for raw in cargo_toml.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            in_test = line == "[[test]]";
            continue;
        }
        if !in_test || !line.starts_with("name") {
            continue;
        }
        let Some(value) = line.split('=').nth(1) else {
            continue;
        };
        let name = value.trim().trim_matches('"').trim();
        let mut parts = name.split('_');
        if let (Some(prefix), Some(number)) = (parts.next(), parts.next()) {
            if prefix == "lesson" && !number.is_empty() {
                let module_id = format!("lesson_{number}");
                if !modules.contains(&module_id) {
                    modules.push(module_id.clone());
                }
                targets.insert(module_id, name.to_string());
            }
        }
    }
    (modules, targets)
}

/// 课号（`lesson_05` → 5），用于按数字排序而不是字符串排序。
fn lesson_number(module_id: &str) -> Option<u32> {
    module_id
        .strip_prefix("lesson_")
        .and_then(|rest| rest.parse::<u32>().ok())
}

/// 按课号数字排序（`lesson_02` 排在 `lesson_10` 前面）。
fn sort_modules_by_number(modules: &mut [String]) {
    modules.sort_by_key(|module| (lesson_number(module).unwrap_or(u32::MAX), module.clone()));
}

// ===========================================================================
// 六、时间：UNIX 毫秒 → `YYYY-MM-DD HH:MM:SS`
// ===========================================================================

/// 当前 UNIX 毫秒。
fn now_ms() -> u128 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_millis(),
        Err(_) => 0,
    }
}

/// 本地时区：偏移（分钟，UTC 以东为正）与用于标注的名称。
struct TimeZone {
    offset_minutes: i64,
    /// `本地时间` 或 `UTC`——报告里必须写清楚用的是哪一个。
    label: &'static str,
}

/// 时区来源：环境变量 `ASSESSMENT_LOCAL_OFFSET_MINUTES`（整数、单位分钟，UTC 以东为正）。
///
/// 标准库**没有**时区数据库（这正是本任务禁止引入 chrono 的原因），所以：
/// - 设置了这个变量 → 时间 = UTC + 偏移，标注「本地时间」；
/// - 没有设置 → 直接按 UTC 打印，并**明确标注「UTC」**；
/// - 绝不允许出现「标着本地时间、实际却是 UTC」的情况，因此这里不做任何猜测。
fn resolve_time_zone() -> TimeZone {
    match non_empty_env("ASSESSMENT_LOCAL_OFFSET_MINUTES") {
        Some(raw) => match raw.trim().parse::<i64>() {
            Ok(minutes) if (-24 * 60..=24 * 60).contains(&minutes) => TimeZone {
                offset_minutes: minutes,
                label: "本地时间",
            },
            _ => {
                eprintln!(
                    "提示：ASSESSMENT_LOCAL_OFFSET_MINUTES=`{raw}` 不是合法的分钟偏移，本次按 UTC 输出。"
                );
                TimeZone {
                    offset_minutes: 0,
                    label: "UTC",
                }
            }
        },
        None => TimeZone {
            offset_minutes: 0,
            label: "UTC",
        },
    }
}

/// 把「1970-01-01 起的天数」换算成公历年月日（Howard Hinnant 的 civil_from_days）。
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    if month <= 2 {
        year += 1;
    }
    (year, month, day)
}

/// UNIX 毫秒 → `YYYY-MM-DD HH:MM:SS`（先加时区偏移，再做公历换算）。
fn format_timestamp(ms: u128, offset_minutes: i64) -> String {
    let seconds = (ms / 1000) as i64 + offset_minutes * 60;
    let days = seconds.div_euclid(86_400);
    let second_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = second_of_day / 3_600;
    let minute = second_of_day % 3_600 / 60;
    let second = second_of_day % 60;
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}")
}

// ===========================================================================
// 七、JSON：手写转义与拼装（不引入 serde）
// ===========================================================================

/// JSON 字符串转义：`"` `\` 以及所有控制字符。
fn json_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 8);
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", control as u32);
            }
            other => out.push(other),
        }
    }
    out
}

/// 拼一个模块对象的 JSON 片段。
fn json_module_fragment(module: &ModuleStat) -> String {
    let score = percentage(module.passed, module.total);
    let mut out = String::new();
    let _ = write!(
        out,
        "{{\"module_id\":\"{}\",\"name\":\"{}\",\"passed\":{},\"total\":{},\"score\":{}}}",
        json_escape(&module.module_id),
        json_escape(&module.name),
        module.passed,
        module.total,
        score
    );
    out
}

/// 拼一整份摘要 JSON（`history.jsonl` 的一行 / `summary.json` 的内容）。
///
/// `all_modules = true` 时包含「本次没有记录」的模块（`summary.json` 更完整），
/// 否则只写真正跑过的模块（`history.jsonl` 更干净）。
fn build_summary_json(analysis: &RunAnalysis, modules: &[ModuleStat], all_modules: bool) -> String {
    let shown: Vec<&ModuleStat> = if all_modules {
        modules.iter().collect()
    } else {
        modules.iter().filter(|module| module.total > 0).collect()
    };
    let fragments: Vec<String> = shown
        .iter()
        .map(|module| json_module_fragment(module))
        .collect();
    let mut out = String::new();
    let _ = write!(
        out,
        "{{\"run_id\":\"{}\",\"finished_at\":\"{}\",\"score\":{},\"passed\":{},\"total\":{},\
         \"unimplemented\":{},\"failed\":{},\"panicked\":{},\"modules\":[{}]}}",
        json_escape(&analysis.run_id),
        json_escape(&analysis.finished_at),
        analysis.score,
        analysis.passed,
        analysis.total,
        analysis.unimplemented,
        analysis.failed,
        analysis.panicked,
        fragments.join(",")
    );
    out
}

// ===========================================================================
// 八、统计：一次运行的判定结果
// ===========================================================================

/// 一次运行的分析结果（总数、分数、等级等）。
struct RunAnalysis {
    run_id: String,
    /// 本次结束时间（该运行最后一条记录的时间，格式化成文本）。
    finished_at: String,
    /// 通过的知识点数。
    passed: usize,
    /// 预期知识点总数。
    total: usize,
    unimplemented: usize,
    failed: usize,
    panicked: usize,
    incomplete: usize,
    /// 本次运行里完全没有记录的知识点数。
    not_run: usize,
    /// 总分（0~100，四舍五入）。
    score: u32,
    grade: &'static str,
}

/// 模块级得分行。
#[derive(Clone)]
struct ModuleStat {
    module_id: String,
    name: String,
    total: usize,
    passed: usize,
    unimplemented: usize,
    failed: usize,
    panicked: usize,
    incomplete: usize,
    not_run: usize,
    /// 是否算「需要加强」。
    ran: bool,
}

impl ModuleStat {
    /// 需要加强的知识点数（未实现 + 未通过 + 运行期错误 + 中断；不含「本轮未运行」）。
    fn work_items(&self) -> usize {
        self.unimplemented + self.failed + self.panicked + self.incomplete
    }

    /// 得分（0~100）。
    fn score(&self) -> u32 {
        percentage(self.passed, self.total)
    }

    /// 是否值得在「薄弱模块」里占一个名额：本轮跑过、且确实有需要加强的知识点。
    fn is_weak(&self) -> bool {
        self.ran && self.work_items() > 0
    }
}

/// 分类（基础/核心/难点/边界）掌握情况。
#[derive(Default, Clone, Copy)]
struct KindStat {
    total: usize,
    passed: usize,
}

/// 计算「通过率 × 100」的四舍五入整数分。
fn percentage(passed: usize, total: usize) -> u32 {
    if total == 0 {
        return 0;
    }
    ((passed * 200 + total) / (total * 2)) as u32
}

/// 等级：≥90 优秀 / ≥75 良好 / ≥60 合格 / <60 待加强。
fn grade_label(score: u32) -> &'static str {
    if score >= 90 {
        "优秀"
    } else if score >= 75 {
        "良好"
    } else if score >= 60 {
        "合格"
    } else {
        "待加强"
    }
}

/// 从 kp_id（`kp_01_08`）里取出模块 id（`lesson_01`）。
fn kp_module_id(kp_id: &str) -> String {
    let mut parts = kp_id.split('_');
    match (parts.next(), parts.next()) {
        (Some("kp"), Some(number)) => format!("lesson_{number}"),
        _ => String::new(),
    }
}

/// 从 kp_id 里取「第几号知识点」，用于稳定排序。
fn kp_ordinal(kp_id: &str) -> u32 {
    match kp_id.split('_').next_back() {
        Some(number) => number.parse::<u32>().unwrap_or(0),
        None => 0,
    }
}

/// 知识点的类型中文名（basic/core/hard/edge → 基础/核心/难点/边界）。
fn kind_label(kind: &str) -> &'static str {
    match kind {
        "basic" => "基础",
        "core" => "核心",
        "hard" => "难点",
        "edge" => "边界",
        _ => "未分类",
    }
}

/// 报告渲染需要的一切（由 `analyze` 一次算好）。
struct Report {
    analysis: RunAnalysis,
    /// 按模块、知识点排序的明细（只含预期集合里的知识点）。
    results: Vec<KpResult>,
    /// 按课号排序的模块表（含未运行的模块）。
    modules: Vec<ModuleStat>,
    /// 基础/核心/难点/边界 的掌握率。
    kinds: [(String, KindStat); 4],
    /// 知识点集合的来源与一致性提示。
    source_note: String,
    /// 计分口径（「账本并集」或「知识点矩阵」）。
    source_label: String,
    /// 预期模块里本次一条记录都没有的模块（多半是没跑或编译失败）。
    unrun_modules: Vec<String>,
    /// 模块 id → 测试目标名（用于报告里写出可照抄的命令）。
    test_targets: BTreeMap<String, String>,
}

impl Report {
    /// 该模块对应的测试目标名；查不到（例如 Cargo.toml 读不到）时退回模块 id。
    fn test_target<'a>(&'a self, module_id: &'a str) -> &'a str {
        self.test_targets
            .get(module_id)
            .map(String::as_str)
            .unwrap_or(module_id)
    }
}

/// 主分析：账本 + 预期集合 → 完整报告数据。
///
/// `union` 是账本**全部运行**的知识点并集：用来给「本次没跑到的模块」补上中文名。
fn analyze(
    records: &[LedgerRecord],
    run_id: &str,
    expected: &BTreeSet<String>,
    expected_modules: &[String],
    union: &BTreeMap<String, (String, String, String)>,
    test_targets: BTreeMap<String, String>,
    source_note: String,
    source_label: String,
    zone: &TimeZone,
) -> Report {
    let run_records = records_for_run(records, run_id);

    // 0) 模块名兜底表：本次没跑到的模块也能显示中文名（取自历史账本记录）。
    let mut known_names: BTreeMap<String, String> = BTreeMap::new();
    for (module_id, module_name, _) in union.values() {
        if !module_id.is_empty() && !module_name.is_empty() {
            known_names
                .entry(module_id.clone())
                .or_insert_with(|| module_name.clone());
        }
    }

    // 1) 按知识点分组，并对每个知识点做「最后一段」判定。
    let mut groups: BTreeMap<String, Vec<&LedgerRecord>> = BTreeMap::new();
    for record in &run_records {
        if record.kp_id.is_empty() {
            continue;
        }
        groups.entry(record.kp_id.clone()).or_default().push(record);
    }
    let mut outcomes: BTreeMap<String, KpResult> = BTreeMap::new();
    for (kp_id, entries) in &groups {
        outcomes.insert(kp_id.clone(), merge_kp(entries));
    }

    // 2) 本次运行的判定结果：预期集合里的**每一个**知识点都算进分母
    //    （包括本轮完全没跑的模块，这样跨次比较才有意义）。
    //    本次没有记录的 → NotRun（只在「本轮未运行」计数与未运行模块行里体现，
    //    不算「未完成或中断」，也不会在逐条明细里展开）。
    let mut results: Vec<KpResult> = Vec::new();
    let mut not_run = 0usize;
    for kp_id in expected {
        match outcomes.remove(kp_id) {
            Some(result) => results.push(result),
            None => {
                not_run += 1;
                let module_id = kp_module_id(kp_id);
                results.push(KpResult {
                    outcome: KpOutcome::NotRun,
                    module_name: known_names.get(&module_id).cloned().unwrap_or_default(),
                    module_id,
                    kp_id: kp_id.clone(),
                    title: String::new(),
                    kind: String::new(),
                    location: String::new(),
                    detail: String::new(),
                    hint: None,
                    missing_fn: String::new(),
                    missing_hint: String::new(),
                    panic_text: String::new(),
                    review: String::new(),
                    guide: String::new(),
                });
            }
        }
    }
    results.sort_by_key(|result| (result.module_id.clone(), kp_ordinal(&result.kp_id)));

    // 3) 顶层计数。「未完成或中断」只统计本次确实有记录、却停在 start 的知识点。
    let mut passed = 0usize;
    let mut unimplemented = 0usize;
    let mut failed = 0usize;
    let mut panicked = 0usize;
    let mut incomplete = 0usize;
    for result in &results {
        match result.outcome {
            KpOutcome::Passed => passed += 1,
            KpOutcome::Unimplemented => unimplemented += 1,
            KpOutcome::Failed => failed += 1,
            KpOutcome::Panicked => panicked += 1,
            KpOutcome::Incomplete => incomplete += 1,
            KpOutcome::NotRun => {}
        }
    }
    let total = results.len();
    let score = percentage(passed, total);

    // 4) 模块表：预期模块（Cargo.toml）+ 账本里出现过的模块，按课号排序。
    let mut module_ids: Vec<String> = expected_modules.to_vec();
    for result in &results {
        if !result.module_id.is_empty() && !module_ids.contains(&result.module_id) {
            module_ids.push(result.module_id.clone());
        }
    }
    sort_modules_by_number(&mut module_ids);
    let mut modules: Vec<ModuleStat> = Vec::new();
    for module_id in &module_ids {
        let subset: Vec<&KpResult> = results
            .iter()
            .filter(|result| &result.module_id == module_id)
            .collect();
        let name = subset
            .iter()
            .map(|result| result.module_name.as_str())
            .find(|name| !name.is_empty())
            .map(|name| name.to_string())
            .or_else(|| known_names.get(module_id).cloned())
            .unwrap_or_else(|| module_label(module_id).to_string());
        let mut stat = ModuleStat {
            module_id: module_id.clone(),
            name,
            total: subset.len(),
            passed: 0,
            unimplemented: 0,
            failed: 0,
            panicked: 0,
            incomplete: 0,
            not_run: 0,
            ran: !subset.is_empty(),
        };
        for result in &subset {
            match result.outcome {
                KpOutcome::Passed => stat.passed += 1,
                KpOutcome::Unimplemented => stat.unimplemented += 1,
                KpOutcome::Failed => stat.failed += 1,
                KpOutcome::Panicked => stat.panicked += 1,
                KpOutcome::Incomplete => stat.incomplete += 1,
                KpOutcome::NotRun => stat.not_run += 1,
            }
        }
        modules.push(stat);
    }

    // 5) 分类掌握率（基础/核心/难点/边界）。
    let mut kinds: [(String, KindStat); 4] = [
        (KIND_ORDER[0].to_string(), KindStat::default()),
        (KIND_ORDER[1].to_string(), KindStat::default()),
        (KIND_ORDER[2].to_string(), KindStat::default()),
        (KIND_ORDER[3].to_string(), KindStat::default()),
    ];
    for result in &results {
        if let Some(entry) = kinds.iter_mut().find(|entry| entry.0 == result.kind) {
            entry.1.total += 1;
            if result.outcome.is_passed() {
                entry.1.passed += 1;
            }
        }
    }

    // 6) 未运行模块：预期模块里本次一条记录都没有的（标「未运行或编译失败」）。
    let unrun_modules: Vec<String> = modules
        .iter()
        .filter(|module| !module.ran)
        .map(|module| module.module_id.clone())
        .collect();

    // 7) 本次结束时间 = 该运行最后一条记录的时间。
    let highest_ts = run_records
        .iter()
        .map(|record| record.ts_ms)
        .max()
        .unwrap_or(0);
    let analysis = RunAnalysis {
        run_id: run_id.to_string(),
        finished_at: format_timestamp(highest_ts, zone.offset_minutes),
        passed,
        total,
        unimplemented,
        failed,
        panicked,
        incomplete,
        not_run,
        score,
        grade: grade_label(score),
    };
    Report {
        analysis,
        results,
        modules,
        kinds,
        source_note,
        source_label,
        unrun_modules,
        test_targets,
    }
}

/// 需要加强的模块：只考虑**本轮跑过、且确实有未实现/未通过/运行期错误**的模块，
/// 按需要加强的知识点数降序，同数量时按得分升序。
/// 本轮完全没运行的模块不参与（那是「先修编译错误」的问题，不是知识薄弱）。
fn weak_modules(report: &Report, limit: usize) -> Vec<ModuleStat> {
    let mut weak: Vec<ModuleStat> = report
        .modules
        .iter()
        .filter(|module| module.is_weak())
        .cloned()
        .collect();
    weak.sort_by_key(|module| {
        (
            std::cmp::Reverse(module.work_items()),
            module.score(),
            module.module_id.clone(),
        )
    });
    weak.truncate(limit);
    weak
}

// ===========================================================================
// 九、Markdown 报告渲染
// ===========================================================================

/// 渲染报告需要的额外上下文。
struct RenderContext {
    ledger_path: PathBuf,
    report_path: PathBuf,
    map_path: PathBuf,
    /// 时间标注：`本地时间` 或 `UTC`（不允许含糊）。
    time_label: &'static str,
    generated_at: String,
    /// 与上一次运行的历史对比。
    comparison: HistoryComparison,
    /// 本次是否追加了 history.jsonl。
    history_written: bool,
    history_path: PathBuf,
}

/// 生成完整 Markdown 报告（中文，UTF-8）。
fn render_markdown(report: &Report, context: &RenderContext) -> String {
    let analysis = &report.analysis;
    let mut out = String::new();

    // ---- 1) 标题与计分口径 ----
    let _ = writeln!(out, "# 学习评估报告（assessment_report）\n");
    let _ = writeln!(
        out,
        "- 生成时间：{}（{}）",
        context.generated_at, context.time_label
    );
    let _ = writeln!(out, "- 运行 id：`{}`", analysis.run_id);
    let _ = writeln!(
        out,
        "- 本次结束时间：{}（{}）",
        analysis.finished_at, context.time_label
    );
    let _ = writeln!(
        out,
        "- 数据来源（账本）：`{}`",
        context.ledger_path.display()
    );
    let _ = writeln!(out, "- 报告文件：`{}`", context.report_path.display());
    let _ = writeln!(out, "- 知识点矩阵：`{}`\n", context.map_path.display());
    let _ = writeln!(out, "**计分口径**");
    let _ = writeln!(
        out,
        "- 预期知识点集合来源：**{}**；{}",
        report.source_label, report.source_note
    );
    let _ = writeln!(
        out,
        "- 预期知识点总数：**{}**（账本中所有运行出现过的知识点并集；矩阵存在时以矩阵为准）",
        analysis.total
    );
    let _ = writeln!(
        out,
        "- **分母口径**：总分 = 通过知识点数 ÷ 预期知识点总数 × 100（四舍五入）= {} ÷ {} → **{} 分**；\
         某个模块本轮**完全没跑**时，它的知识点**仍然计入分母**（本轮记作「未运行或编译失败」），\
         这样跨次运行比较分数才有意义。",
        analysis.passed, analysis.total, analysis.score
    );
    let _ = writeln!(out, "- 等级线：≥90 优秀 / ≥75 良好 / ≥60 合格 / <60 待加强");
    let _ = writeln!(
        out,
        "- 判定口径：同一次运行内同一知识点只取**最后一组**（最后一个 `start` 及其之后的事件），\
         这样重复运行同一测试目标不会重复计数；\n  有 `pass` → 通过，否则依次看 \
         `missing`（未实现）/ `fail`（未通过）/ `panic`（运行期错误），有 `start` 但没有终态事件的算\
         「未完成或中断」；本轮一条记录都没有的算「未运行或编译失败」。\n"
    );

    // ---- 2) 总览 ----
    let _ = writeln!(out, "## 一、总览\n");
    let _ = writeln!(out, "| 指标 | 数值 |");
    let _ = writeln!(out, "| --- | --- |");
    let _ = writeln!(out, "| 总分 | {} 分 |", analysis.score);
    let _ = writeln!(out, "| 等级 | {} |", analysis.grade);
    let _ = writeln!(out, "| 通过 | {} |", analysis.passed);
    let _ = writeln!(out, "| 未实现 | {} |", analysis.unimplemented);
    let _ = writeln!(out, "| 未通过 | {} |", analysis.failed);
    let _ = writeln!(out, "| 运行期错误 | {} |", analysis.panicked);
    let _ = writeln!(
        out,
        "| 未完成或中断（本次有记录但没有终态事件） | {} |",
        analysis.incomplete
    );
    let _ = writeln!(
        out,
        "| 本轮未运行或编译失败（计入分母） | {} |",
        analysis.not_run
    );
    let _ = writeln!(out, "| 预期知识点总数 | {} |", analysis.total);
    let _ = writeln!(out, "\n**一句话诊断**：{}\n", diagnosis(report));

    // ---- 3) 模块得分表 ----
    let _ = writeln!(out, "## 二、模块得分表（按课号顺序）\n");
    let _ = writeln!(
        out,
        "| 模块 | 模块名 | 知识点数 | 通过 | 未实现 | 未通过 | 得分 | 等级 | 本次是否运行 |"
    );
    let _ = writeln!(
        out,
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- |"
    );
    for module in &report.modules {
        // 本轮有没有该模块的记录：8 个知识点全都没有记录，就算「未运行或编译失败」。
        let seen = module.total - module.not_run;
        let (score_text, grade_text) = if seen > 0 {
            (
                module.score().to_string(),
                grade_label(module.score()).to_string(),
            )
        } else {
            ("—".to_string(), "未评".to_string())
        };
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            cell(&module.module_id),
            cell(&module.name),
            if seen > 0 {
                module.total.to_string()
            } else {
                "—".to_string()
            },
            if seen > 0 {
                module.passed.to_string()
            } else {
                "—".to_string()
            },
            if seen > 0 {
                module.unimplemented.to_string()
            } else {
                "—".to_string()
            },
            if seen > 0 {
                module.failed.to_string()
            } else {
                "—".to_string()
            },
            score_text,
            grade_text,
            if seen > 0 {
                "已运行"
            } else {
                "未运行或编译失败"
            }
        );
    }
    if !report.unrun_modules.is_empty() {
        let _ = writeln!(
            out,
            "\n> ⚠️ 本轮**未运行或编译失败**的模块（没有任何记录，但知识点仍计入分母）：{}。",
            preview_list(&report.unrun_modules)
        );
        let _ = writeln!(
            out,
            "> 先修编译错误：运行 `cargo test --no-run` 看报错并改掉，\
             再执行 `cargo test --test <上表任一模块对应的目标名>` 重新考核。\n"
        );
    }

    // ---- 4) 分类掌握率 ----
    let _ = writeln!(out, "## 三、分类掌握率\n");
    let _ = writeln!(out, "| 类型 | 通过 | 总数 | 掌握率 |");
    let _ = writeln!(out, "| --- | --- | --- | --- |");
    for (kind, stat) in &report.kinds {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            kind_label(kind),
            stat.passed,
            stat.total,
            kind_rate(stat)
        );
    }
    let _ = writeln!(out, "\n{}\n", kind_advice(report));

    // ---- 5) 逐条明细 ----
    let _ = writeln!(out, "## 四、逐条明细（按模块分组）\n");
    for module in &report.modules {
        // 本轮没有记录的模块：只写一行提示，绝不伪造逐条明细。
        if !module.ran {
            let _ = writeln!(
                out,
                "### {} · {} —— 本轮没有该模块的任何记录：**未运行或编译失败**\
                 （先跑 `cargo test --test {}` 看编译错误）\n",
                module.module_id,
                module.name,
                report.test_target(&module.module_id)
            );
            continue;
        }
        let _ = writeln!(
            out,
            "### {} · {}（通过 {}/{}，得分 {} · {}）\n",
            module.module_id,
            module.name,
            module.passed,
            module.total,
            module.score(),
            grade_label(module.score())
        );
        for result in report
            .results
            .iter()
            .filter(|result| result.module_id == module.module_id)
        {
            let _ = writeln!(
                out,
                "- `{}` **{}**｜类型：{}｜状态：**{}**",
                result.kp_id,
                cell(&result.title),
                kind_label(&result.kind),
                result.outcome.label()
            );
            let _ = writeln!(
                out,
                "  - 位置：{}",
                if result.location.is_empty() {
                    "—".to_string()
                } else {
                    format!("`{}`", result.location)
                }
            );
            if !result.detail.is_empty() {
                let _ = writeln!(out, "  - 失败原因：{}", cell(&result.detail));
            }
            if let Some(hint) = &result.hint {
                let _ = writeln!(out, "  - 提示：{}", cell(hint));
            }
            if !result.missing_fn.is_empty() {
                let _ = writeln!(out, "  - 待实现函数：`{}`", result.missing_fn);
            }
            if !result.missing_hint.is_empty() {
                let _ = writeln!(out, "  - 实现要求：{}", cell(&result.missing_hint));
            }
            if !result.panic_text.is_empty() {
                let _ = writeln!(out, "  - 运行期错误信息：{}", cell(&result.panic_text));
            }
            if !result.review.is_empty() {
                let _ = writeln!(out, "  - 作者复习指引：{}", cell(&result.review));
            }
            let _ = writeln!(out, "  - 复习入口：{}", guide_text(&result.guide));
        }
        let _ = writeln!(out);
    }

    // ---- 6) 需要加强的领域 ----
    let _ = writeln!(out, "## 五、需要加强的领域（Top 5）\n");
    let weak = weak_modules(report, 5);
    if weak.is_empty() {
        let _ = writeln!(
            out,
            "本次运行没有「未通过 / 未实现 / 运行期错误」的知识点，保持这个节奏即可。\n"
        );
    } else {
        for (index, module) in weak.iter().enumerate() {
            let _ = writeln!(
                out,
                "### {}. {} · {}（未通过 {}，未实现 {}，运行期错误/中断 {}）\n",
                index + 1,
                module.module_id,
                module.name,
                module.failed,
                module.unimplemented,
                module.panicked + module.incomplete
            );
            let _ = writeln!(out, "{}", module_suggestion(report, &module.module_id));
        }
    }
    // 本轮没跑的模块单独成段：它们不是「知识薄弱」，而是「先要能跑起来」。
    if !report.unrun_modules.is_empty() {
        let _ = writeln!(
            out,
            "### 另外：本轮未运行或编译失败的模块（不计入上面的 Top 5）\n"
        );
        let _ = writeln!(
            out,
            "{} 个模块本轮一条记录都没有：{}。\
             先运行 `cargo test --no-run` 修掉编译错误，再逐个执行 `cargo test --test <目标名>`；\
             这些模块的知识点仍然计入总分分母。\n",
            report.unrun_modules.len(),
            preview_list(&report.unrun_modules)
        );
    }

    // ---- 7) 学习进度对比 ----
    let _ = writeln!(out, "## 六、学习进度对比\n");
    let _ = writeln!(out, "{}", render_comparison(&context.comparison));
    let _ = writeln!(
        out,
        "\n> 对比口径：`history.jsonl` 记录的是每次运行的**模块级聚合**（模块通过数），\
         因此这里给出总分变化与各模块通过数增减；单条知识点的增减见上表明细。\
         历史文件：`{}`（本次{}）。",
        context.history_path.display(),
        if context.history_written {
            "已追加"
        } else {
            "未追加（--no-history）"
        }
    );

    // ---- 8) 下一步行动清单 ----
    let _ = writeln!(out, "\n## 七、下一步行动清单\n");
    for (index, item) in next_actions(report).iter().enumerate() {
        let _ = writeln!(out, "{}. {}", index + 1, item);
    }
    let _ = writeln!(out);
    out
}

/// Markdown 表格单元格：竖线与换行必须转义，否则会撑破表格。
fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', "<br>")
}

/// 复习入口文本转成行内代码；没有记录时写 `—`。
fn guide_text(guide: &str) -> String {
    let trimmed = guide.trim();
    if trimmed.is_empty() {
        return "—".to_string();
    }
    format!("`{}`", trimmed.replace('`', "'"))
}

/// 「一句话诊断」：先点名最严重的问题，再给出下一步方向。
fn diagnosis(report: &Report) -> String {
    let analysis = &report.analysis;
    if analysis.total == 0 {
        return "账本里还没有可统计的知识点：先运行 `cargo test --test <目标名>` 生成一次考核记录。"
            .to_string();
    }
    if analysis.passed == analysis.total {
        return format!(
            "本次运行 {} 个知识点全部通过（{} 分 · {}）：可以进入下一课，\
             并重新运行 `--emit-map` 更新知识点矩阵。",
            analysis.total, analysis.score, analysis.grade
        );
    }
    let mut parts = Vec::new();
    if analysis.unimplemented > 0 {
        parts.push(format!("{} 个知识点还没动手实现", analysis.unimplemented));
    }
    if analysis.failed > 0 {
        parts.push(format!("{} 个知识点断言未通过", analysis.failed));
    }
    if analysis.panicked > 0 {
        parts.push(format!("{} 个知识点发生运行期 panic", analysis.panicked));
    }
    if !report.unrun_modules.is_empty() {
        parts.push(format!(
            "{} 个模块本轮没有记录（未运行或编译失败），涉及 {} 个知识点仍计入分母",
            report.unrun_modules.len(),
            analysis.not_run
        ));
    }
    let head = if parts.is_empty() {
        "本次运行还有知识点没跑完".to_string()
    } else {
        parts.join("，")
    };
    let focus = match weak_modules(report, 1).first() {
        Some(module) => format!("建议先集中攻 `{}`（{}）", module.module_id, module.name),
        None => "建议按明细逐条推进".to_string(),
    };
    format!(
        "{head}；当前 {} 分 · {}。{focus}——把「未实现」变成「通过」是最快的提分方式。",
        analysis.score, analysis.grade
    )
}

/// 分类掌握率文本：`3/8（38%）`，没有该类型知识点时写说明。
fn kind_rate(stat: &KindStat) -> String {
    if stat.total == 0 {
        "—（本次没有该类型知识点）".to_string()
    } else {
        format!(
            "{}/{}（{}%）",
            stat.passed,
            stat.total,
            percentage(stat.passed, stat.total)
        )
    }
}

/// 分类诊断：专门指出「难点」「边界」是否薄弱。
fn kind_advice(report: &Report) -> String {
    let rate = |kind: &str| -> Option<u32> {
        report
            .kinds
            .iter()
            .find(|entry| entry.0 == kind)
            .and_then(|entry| {
                if entry.1.total == 0 {
                    None
                } else {
                    Some(percentage(entry.1.passed, entry.1.total))
                }
            })
    };
    let mut parts = Vec::new();
    if let Some(value) = rate("core") {
        if value < 60 {
            parts.push(
                "**核心**掌握率偏低：核心知识点是后续课程的地基，建议回到对应 README 条目精读，\
                 先用注释写出思路（伪代码）再写代码。"
                    .to_string(),
            );
        }
    }
    if let Some(value) = rate("hard") {
        if value < 60 {
            parts.push(
                "**难点**薄弱：难点允许对照本课示例改写，但每改一次都要能用自己的话说清\
                 「为什么必须这样写」；卡住超过 20 分钟就去看示例注释，不要乱改类型。"
                    .to_string(),
            );
        }
    }
    if let Some(value) = rate("edge") {
        if value < 60 {
            parts.push(
                "**边界**薄弱：边界题考的是空输入、0/1 个元素、首尾元素、类型极值（如 `u8::MAX`）、\
                 整除与负数、以及中文等多字节字符——写完先自己举三个极端输入试一遍。"
                    .to_string(),
            );
        }
    }
    if parts.is_empty() {
        "各类型掌握率都比较均衡，继续保持「先跑通、再优化」的节奏。".to_string()
    } else {
        parts.join("\n\n")
    }
}

/// 单个模块的可执行建议：引用该模块失败/未实现知识点的 hint 与复习指引。
fn module_suggestion(report: &Report, module_id: &str) -> String {
    let entries: Vec<&KpResult> = report
        .results
        .iter()
        .filter(|result| result.module_id == module_id && result.outcome.needs_work())
        .collect();
    let mut out = String::new();
    let unimplemented: Vec<&KpResult> = entries
        .iter()
        .copied()
        .filter(|result| matches!(result.outcome, KpOutcome::Unimplemented))
        .collect();
    if !unimplemented.is_empty() {
        let names: Vec<String> = unimplemented
            .iter()
            .map(|result| format!("`{}`", result.missing_fn))
            .collect();
        let _ = writeln!(
            out,
            "- 待实现函数 {} 个：{}；这些练习函数的函数体里现在只有一行 `todo_exercise(...)` 占位，\
             删掉那一行、按函数上方「实现要求」写实现即可（考核文件里的注释已写清输入输出）。",
            names.len(),
            preview_list(&names)
        );
    }
    for result in entries.iter().take(4) {
        if let Some(hint) = &result.hint {
            let _ = writeln!(
                out,
                "- `{}`（{}）未通过：{} → 提示：{}",
                result.kp_id,
                cell(&result.title),
                cell(&result.detail),
                cell(hint)
            );
        } else if !result.missing_hint.is_empty() {
            let _ = writeln!(
                out,
                "- `{}`（{}）未实现：{}",
                result.kp_id,
                cell(&result.title),
                cell(&result.missing_hint)
            );
        } else if !result.panic_text.is_empty() {
            let _ = writeln!(
                out,
                "- `{}`（{}）运行期错误：{}",
                result.kp_id,
                cell(&result.title),
                cell(&result.panic_text)
            );
        }
    }
    if let Some(guide) = entries.first().map(|result| result.guide.as_str()) {
        if !guide.trim().is_empty() {
            let _ = writeln!(out, "- 复习入口：{}", guide_text(guide));
        }
    }
    out
}

/// 渲染「学习进度对比」小节。
fn render_comparison(comparison: &HistoryComparison) -> String {
    let mut out = String::new();
    match &comparison.previous_run_id {
        None => {
            let _ = writeln!(
                out,
                "这是本工具记录的**首次记录**：`history.jsonl` 里还没有更早的运行，暂时无可对比的历史。\
                 继续完成练习后再次运行本命令，就能看到分数变化与各模块进步。"
            );
            let _ = writeln!(
                out,
                "\n本次基线：{} 分，通过 {} 个知识点。",
                comparison.score, comparison.passed
            );
        }
        Some(previous) => {
            let delta = comparison.score as i64 - comparison.previous_score as i64;
            let _ = writeln!(
                out,
                "上一次运行 `{previous}` 得 {} 分，本次 `{}` 得 {} 分（{}{}）。",
                comparison.previous_score,
                comparison.run_id,
                comparison.score,
                sign(delta),
                delta
            );
            if comparison.improved.is_empty() && comparison.regressed.is_empty() {
                let _ = writeln!(
                    out,
                    "- 各模块通过数没有变化（本次通过 {} 个知识点，上次 {} 个）。",
                    comparison.passed, comparison.previous_passed
                );
            } else {
                if !comparison.improved.is_empty() {
                    let items: Vec<String> = comparison
                        .improved
                        .iter()
                        .map(|(module, delta)| format!("`{module}` +{delta}"))
                        .collect();
                    let _ = writeln!(out, "- 本次新通过：{}", items.join("、"));
                }
                if !comparison.regressed.is_empty() {
                    let items: Vec<String> = comparison
                        .regressed
                        .iter()
                        .map(|(module, delta)| format!("`{module}` {delta}"))
                        .collect();
                    let _ = writeln!(out, "- 本次退步：{}", items.join("、"));
                }
            }
            let _ = writeln!(
                out,
                "- 通过知识点总数：{} → {}（{}{}）。",
                comparison.previous_passed,
                comparison.passed,
                sign(comparison.passed as i64 - comparison.previous_passed as i64),
                comparison.passed as i64 - comparison.previous_passed as i64
            );
        }
    }
    out
}

/// 正数前缀 `+`，负数自带 `-`。
fn sign(value: i64) -> &'static str {
    if value > 0 { "+" } else { "" }
}

/// 下一步行动清单（3~5 条，按当前状态生成，尽量可执行）。
fn next_actions(report: &Report) -> Vec<String> {
    let analysis = &report.analysis;
    let weak = weak_modules(report, 1);
    let mut actions = Vec::new();
    if !report.unrun_modules.is_empty() {
        let first = &report.unrun_modules[0];
        actions.push(format!(
            "先让所有考核目标都能跑起来：运行 `cargo test --no-run` 修编译错误，\
             再单独运行 `cargo test --test {}`；本轮完全没有记录的模块是 {}（共 {} 个模块、{} 个知识点）。",
            report.test_target(first),
            preview_list(&report.unrun_modules),
            report.unrun_modules.len(),
            analysis.not_run
        ));
    }
    if analysis.unimplemented > 0 {
        let focus = match weak.first() {
            Some(module) if module.unimplemented > 0 => format!(
                "（`{}` 里最多，共 {} 个）",
                module.module_id, module.unimplemented
            ),
            _ => String::new(),
        };
        actions.push(format!(
            "实现 {} 个还留着 `todo_exercise(...)` 的知识点{focus}：删掉占位那一行，\
             按函数上方「实现要求」写代码，再运行 `cargo test --test <目标名>` 验证。",
            analysis.unimplemented
        ));
    }
    if analysis.failed > 0 {
        actions.push(format!(
            "修 {} 个断言未通过的知识点：对照报告里「失败原因」的期望值与实际值，\
             先想清楚差在哪里再改代码（**不要**直接改期望值）。",
            analysis.failed
        ));
    }
    if analysis.panicked > 0 {
        actions.push(format!(
            "排查 {} 个运行期 panic：先看 panic 信息里的行号与提示，\
             重点检查 `unwrap`、索引越界、除零。",
            analysis.panicked
        ));
    }
    if let Some(module) = weak.first() {
        let first_kp = report
            .results
            .iter()
            .find(|result| result.module_id == module.module_id && result.outcome.needs_work())
            .map(|result| format!("从 `{}` 开始", result.kp_id))
            .unwrap_or_else(|| "按明细逐条推进".to_string());
        actions.push(format!(
            "把 `{}`（{}）作为本次重点：{first_kp}。",
            module.module_id, module.name
        ));
    }
    if analysis.total > 0 && analysis.passed == analysis.total {
        actions.push(
            "本课已全部通过：进入下一课，并在学完后运行 \
             `cargo run --bin assessment_report -- --emit-map` 更新知识点矩阵。"
                .to_string(),
        );
    }
    actions.push(
        "每次练习后重新运行 `cargo run --bin assessment_report`，\
         用「学习进度对比」确认没有退步。"
            .to_string(),
    );
    actions.truncate(5);
    actions
}

// ===========================================================================
// 十、知识矩阵（docs/04_knowledge_map.md）
// ===========================================================================

/// 矩阵里的一行（信息全部来自账本记录 + guide 字段解析）。
struct MapRow {
    module_id: String,
    module_name: String,
    kp_id: String,
    title: String,
    kind: String,
    location: String,
    /// 对应 README 出处（guide 里「说明见」之后的部分）。
    readme: String,
    /// 课程文件。
    course: String,
    /// 复习命令。
    command: String,
}

/// 解析 guide：`复习入口：<命令>（<课程文件>）；说明见 <README 出处>`。
fn parse_guide(guide: &str) -> (String, String, String) {
    let rest = match guide.split_once("复习入口：") {
        Some((_, rest)) => rest,
        None => return (String::new(), String::new(), String::new()),
    };
    let (command, rest) = match rest.split_once('（') {
        Some((command, rest)) => (command.trim().to_string(), rest),
        None => return (rest.trim().to_string(), String::new(), String::new()),
    };
    let (course, rest) = match rest.split_once('）') {
        Some((course, rest)) => (course.trim().to_string(), rest),
        None => return (command, String::new(), String::new()),
    };
    let readme = rest
        .trim()
        .trim_start_matches(['；', ';'])
        .trim_start()
        .strip_prefix("说明见")
        .map(|value| value.trim().to_string())
        .unwrap_or_default();
    (command, course, readme)
}

/// 从账本（全部运行的并集）收集矩阵行；同一知识点取最后一条记录的信息。
fn collect_map_rows(records: &[LedgerRecord]) -> Vec<MapRow> {
    let mut by_kp: BTreeMap<String, &LedgerRecord> = BTreeMap::new();
    for record in records {
        if record.kp_id.is_empty() {
            continue;
        }
        by_kp.insert(record.kp_id.clone(), record);
    }
    let mut rows: Vec<MapRow> = by_kp
        .into_values()
        .map(|record| {
            let (command, course, readme) = parse_guide(&record.guide);
            MapRow {
                module_id: record.module_id.clone(),
                module_name: record.module_name.clone(),
                kp_id: record.kp_id.clone(),
                title: record.kp_title.clone(),
                kind: record.kind.clone(),
                location: record.location.clone(),
                readme,
                course,
                command,
            }
        })
        .collect();
    rows.sort_by_key(|row| (row.module_id.clone(), kp_ordinal(&row.kp_id)));
    rows
}

/// 生成知识点覆盖矩阵 Markdown。
///
/// 格式必须能被本工具再次解析（`--check-map` 会读第 3 列作为 kp_id 集合）。
/// `targets` 是「模块 id → 测试目标名」，用来写出完整的 `cargo test --test <目标名>`。
fn render_map(rows: &[MapRow], targets: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# 04 · 知识点覆盖矩阵（考核 ↔ 课程 README 对应表）\n");
    let _ = writeln!(
        out,
        "> 本文件由 `cargo run --bin assessment_report -- --emit-map` 自动生成，请勿手工编辑；"
    );
    let _ = writeln!(out, "> 修改考核用例后重新运行该命令即可。\n");
    let _ = writeln!(
        out,
        "本矩阵说明考核体系「考什么、在哪考、对应课程哪一段」：\n"
    );
    let _ = writeln!(
        out,
        "- 每一行是一个知识点，`考核代码位置` 是 `assess(...)` 的调用点（`文件:行号`），\
         打开该文件即可看到考核标准与对应的练习函数；"
    );
    let _ = writeln!(
        out,
        "- `对应 README 出处` 取自考核文件里的复习指引，指向课程中讲这个知识点的段落，\
         卡住时按这一列去读原文；"
    );
    let _ = writeln!(
        out,
        "- 类型含义：**基础**（本课最先要求掌握的概念）/ **核心**（本课主要知识点）/ \
         **难点**（新人最容易卡住处，允许对照示例反复尝试）/ **边界**（空输入、极值、\
         多字节字符等容易被忽略的边界条件）；"
    );
    let _ = writeln!(
        out,
        "- 数据来源：账本 `assessments/report/data/ledger.tsv` 的**全部运行并集**，\
         因此新增知识点后要先跑一次对应考核，再重新生成本文件。\n"
    );

    // ---- 一、知识点总览 ----
    let _ = writeln!(out, "## 一、知识点总览\n");
    let _ = writeln!(
        out,
        "| 模块 | 模块名 | 知识点 | 知识点标题 | 类型 | 考核代码位置 | 对应 README 出处 |"
    );
    let _ = writeln!(out, "| --- | --- | --- | --- | --- | --- | --- |");
    for row in rows {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            cell(&row.module_id),
            cell(&row.module_name),
            cell(&row.kp_id),
            cell(&row.title),
            kind_label(&row.kind),
            cell(if row.location.is_empty() {
                "-"
            } else {
                row.location.as_str()
            }),
            cell(if row.readme.is_empty() {
                "-"
            } else {
                row.readme.as_str()
            })
        );
    }

    // ---- 二、课程 ↔ 考核文件 ↔ 运行命令对应表 ----
    let _ = writeln!(out, "\n## 二、课程 ↔ 考核文件 ↔ 运行命令对应表\n");
    let _ = writeln!(out, "| 课程文件 | 复习命令 | 考核目标（单独运行） |");
    let _ = writeln!(out, "| --- | --- | --- |");
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for row in rows {
        if row.module_id.is_empty() || !seen.insert(row.module_id.clone()) {
            continue;
        }
        let _ = writeln!(
            out,
            "| {} | {} | {} |",
            cell(if row.course.is_empty() {
                "（账本未记录课程文件）"
            } else {
                row.course.as_str()
            }),
            cell(if row.command.is_empty() {
                "（账本未记录复习命令）"
            } else {
                row.command.as_str()
            }),
            cell(&format!(
                "cargo test --test {}",
                targets
                    .get(&row.module_id)
                    .map(String::as_str)
                    .unwrap_or("（未在 Cargo.toml 找到 [[test]] 段）")
            ))
        );
    }

    // ---- 三、统计 ----
    let _ = writeln!(out, "\n## 三、统计\n");
    let mut modules: BTreeSet<String> = BTreeSet::new();
    let mut kind_counts: BTreeMap<String, usize> = BTreeMap::new();
    for row in rows {
        modules.insert(row.module_id.clone());
        *kind_counts.entry(row.kind.clone()).or_insert(0) += 1;
    }
    let count_of = |kind: &str| kind_counts.get(kind).copied().unwrap_or(0);
    let _ = writeln!(out, "- 模块数：{}；知识点数：{}", modules.len(), rows.len());
    let _ = writeln!(
        out,
        "- 类型分布：基础 {} / 核心 {} / 难点 {} / 边界 {}",
        count_of("basic"),
        count_of("core"),
        count_of("hard"),
        count_of("edge")
    );
    out
}

// ===========================================================================
// 十一、history.jsonl 解析与进度对比
// ===========================================================================

/// 历史对比结果（渲染「学习进度对比」用它）。
struct HistoryComparison {
    run_id: String,
    score: u32,
    passed: usize,
    /// 上一次运行的 run_id；None 表示这是首次记录。
    previous_run_id: Option<String>,
    previous_score: u32,
    previous_passed: usize,
    /// 本次进步的模块（模块 id, 通过数增量）。
    improved: Vec<(String, i64)>,
    /// 本次退步的模块（模块 id, 通过数增量，负数）。
    regressed: Vec<(String, i64)>,
}

/// 从 history.jsonl 里读到的一行摘要。
struct HistorySummary {
    run_id: String,
    score: u32,
    passed: usize,
    /// (module_id, passed)
    modules: Vec<(String, usize)>,
}

/// 从一段 JSON 文本里取字符串字段（只支持本工具自己写出的格式）。
fn json_get_string(text: &str, key: &str) -> String {
    let marker = format!("\"{key}\":\"");
    let Some(start) = text.find(&marker) else {
        return String::new();
    };
    let mut out = String::new();
    let mut chars = text[start + marker.len()..].chars();
    while let Some(character) = chars.next() {
        match character {
            '"' => break,
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(other) => out.push(other),
                None => break,
            },
            other => out.push(other),
        }
    }
    out
}

/// 从一段 JSON 文本里取无符号整数字段。
fn json_get_number(text: &str, key: &str) -> Option<u64> {
    let marker = format!("\"{key}\":");
    let start = text.find(&marker)? + marker.len();
    let digits: String = text[start..]
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    digits.parse::<u64>().ok()
}

/// 解析 history.jsonl 的一行；坏行返回 None（调用方跳过）。
fn parse_history_line(line: &str) -> Option<HistorySummary> {
    let run_id = json_get_string(line, "run_id");
    if run_id.is_empty() {
        return None;
    }
    let mut modules = Vec::new();
    if let Some(start) = line.find("\"modules\":[") {
        let rest = &line[start + "\"modules\":[".len()..];
        if let Some(end) = rest.find(']') {
            for fragment in rest[..end].split("},") {
                let module_id = json_get_string(fragment, "module_id");
                if module_id.is_empty() {
                    continue;
                }
                let passed = json_get_number(fragment, "passed").unwrap_or(0) as usize;
                modules.push((module_id, passed));
            }
        }
    }
    Some(HistorySummary {
        run_id,
        score: json_get_number(line, "score").unwrap_or(0) as u32,
        passed: json_get_number(line, "passed").unwrap_or(0) as usize,
        modules,
    })
}

/// 读取 history.jsonl 里**上一次**运行的摘要。
///
/// 必须排除本次运行的 run_id：报告是在写历史之前生成的，如果不过滤，最近一行
/// 很可能就是本次（重复运行时），会出现「上次和本次是同一个 run」的笑话。
/// 文件不存在、或里面只有本次这一条 → 返回 None（报告里显示「首次记录」）。
fn read_last_history(
    path: &Path,
    exclude_run_id: &str,
) -> Result<Option<HistorySummary>, AppError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(io_error("读取历史", path, &error)),
    };
    let mut last = None;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        if let Some(summary) = parse_history_line(line) {
            if summary.run_id != exclude_run_id {
                last = Some(summary);
            }
        }
    }
    Ok(last)
}

/// 与上一次运行对比（模块级通过数增量为准）。
fn compare_history(report: &Report, previous: &Option<HistorySummary>) -> HistoryComparison {
    let analysis = &report.analysis;
    let Some(previous) = previous else {
        return HistoryComparison {
            run_id: analysis.run_id.clone(),
            score: analysis.score,
            passed: analysis.passed,
            previous_run_id: None,
            previous_score: 0,
            previous_passed: 0,
            improved: Vec::new(),
            regressed: Vec::new(),
        };
    };
    let mut improved = Vec::new();
    let mut regressed = Vec::new();
    for module in &report.modules {
        let Some((_, before)) = previous
            .modules
            .iter()
            .find(|(module_id, _)| module_id == &module.module_id)
        else {
            continue;
        };
        let delta = module.passed as i64 - *before as i64;
        if delta > 0 {
            improved.push((module.module_id.clone(), delta));
        } else if delta < 0 {
            regressed.push((module.module_id.clone(), delta));
        }
    }
    improved.sort_by(|left, right| right.1.cmp(&left.1));
    regressed.sort_by_key(|entry| entry.1);
    HistoryComparison {
        run_id: analysis.run_id.clone(),
        score: analysis.score,
        passed: analysis.passed,
        previous_run_id: Some(previous.run_id.clone()),
        previous_score: previous.score,
        previous_passed: previous.passed,
        improved,
        regressed,
    }
}

// ===========================================================================
// 十二、控制台摘要（ANSI 颜色）
// ===========================================================================

/// 是否给控制台输出上色：标准输出必须是终端，且没有设置 `NO_COLOR`。
fn use_color() -> bool {
    io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none()
}

/// 绿色（通过 / 正常）。
fn green(enabled: bool, text: &str) -> String {
    if enabled {
        format!("\x1b[32m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// 红色（未通过 / 错误）。
fn red(enabled: bool, text: &str) -> String {
    if enabled {
        format!("\x1b[31m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// 黄色（未实现 / 警告）。
fn yellow(enabled: bool, text: &str) -> String {
    if enabled {
        format!("\x1b[33m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// 青色（标题）。
fn cyan(enabled: bool, text: &str) -> String {
    if enabled {
        format!("\x1b[36m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// 显示宽度：ASCII 字符算 1 列，其余（中文等）算 2 列。
fn display_width(text: &str) -> usize {
    text.chars()
        .map(|character| if character.is_ascii() { 1 } else { 2 })
        .sum()
}

/// 按显示宽度右侧补空格（让中文表格对齐）。
fn pad_end(text: &str, width: usize) -> String {
    let mut out = text.to_string();
    let mut current = display_width(text);
    while current < width {
        out.push(' ');
        current += 1;
    }
    out
}

/// 打印控制台摘要：运行 id、总分与等级、精简模块表、薄弱模块 Top 3、产物路径。
fn print_console_summary(report: &Report, context: &RenderContext) {
    let color = use_color();
    let analysis = &report.analysis;
    println!(
        "{}",
        cyan(color, "=== 学习评估报告（assessment_report） ===")
    );
    println!("运行 id：{}", analysis.run_id);
    println!(
        "总分：{} 分 · 等级：{}   （通过 {} / 预期 {}）",
        analysis.score, analysis.grade, analysis.passed, analysis.total
    );
    println!(
        "明细：{}｜{}｜{}｜运行期错误 {}｜未完成 {}｜本轮未运行 {}",
        green(color, &format!("通过 {}", analysis.passed)),
        yellow(color, &format!("未实现 {}", analysis.unimplemented)),
        red(color, &format!("未通过 {}", analysis.failed)),
        analysis.panicked,
        analysis.incomplete,
        analysis.not_run
    );
    if !report.unrun_modules.is_empty() {
        println!(
            "{}",
            yellow(
                color,
                &format!(
                    "⚠ 本轮未运行或编译失败的 {} 个模块（{} 个知识点计入分母，先修编译错误）：{}",
                    report.unrun_modules.len(),
                    analysis.not_run,
                    preview_list(&report.unrun_modules)
                )
            )
        );
    }

    // 精简模块表：只列本次真的跑过的模块（全没记录的模块已在上面提示）。
    println!("\n{}", cyan(color, "模块得分（本次运行）"));
    let rows: Vec<&ModuleStat> = report
        .modules
        .iter()
        .filter(|module| module.ran && module.total > module.not_run)
        .collect();
    if rows.is_empty() {
        println!("（本次运行没有任何知识点记录，请先运行 `cargo test`）");
    } else {
        println!(
            "{}",
            cyan(
                color,
                &format!(
                    "{} {} {} {} {} {} {}",
                    pad_end("模块", 14),
                    pad_end("模块名", 20),
                    pad_end("知识点", 8),
                    pad_end("通过", 6),
                    pad_end("未实现", 8),
                    pad_end("未通过", 8),
                    "得分"
                )
            )
        );
        for module in &rows {
            let score_text = module.score().to_string();
            let score_colored = if module.score() >= 90 {
                green(color, &score_text)
            } else if module.score() >= 60 {
                yellow(color, &score_text)
            } else {
                red(color, &score_text)
            };
            println!(
                "{} {} {} {} {} {} {}{}",
                pad_end(&module.module_id, 14),
                pad_end(&module.name, 20),
                pad_end(&module.total.to_string(), 8),
                pad_end(&module.passed.to_string(), 6),
                pad_end(&module.unimplemented.to_string(), 8),
                pad_end(&module.failed.to_string(), 8),
                score_colored,
                pad_end("", 4usize.saturating_sub(score_text.len()))
            );
        }
    }

    // 薄弱模块 Top 3。
    println!("\n{}", cyan(color, "薄弱模块 Top 3"));
    let weak = weak_modules(report, 3);
    if weak.is_empty() {
        println!("{}", green(color, "本次没有薄弱模块，继续保持。"));
    } else {
        for (index, module) in weak.iter().enumerate() {
            println!(
                "{}. {}（{}）：未通过 {} · 未实现 {} · 得分 {}",
                index + 1,
                module.module_id,
                module.name,
                module.failed + module.panicked + module.incomplete,
                module.unimplemented,
                module.score()
            );
        }
    }

    println!("\n报告：{}", context.report_path.display());
    println!(
        "摘要：{}",
        context.report_path.with_file_name("summary.json").display()
    );
    println!(
        "历史：{}（本次{}）",
        context.history_path.display(),
        if context.history_written {
            "已追加"
        } else {
            "未追加"
        }
    );
    println!(
        "对比：{}",
        match &context.comparison.previous_run_id {
            Some(previous) => {
                let delta = analysis.score as i64 - context.comparison.previous_score as i64;
                format!(
                    "上次 `{previous}` 得 {} 分，本次 {} 分（{}{}）",
                    context.comparison.previous_score,
                    analysis.score,
                    sign(delta),
                    delta
                )
            }
            None => "这是第一次运行，没有可比较的历史。".to_string(),
        }
    );
}

// ===========================================================================
// 十三、路径解析
// ===========================================================================

/// 工作区根目录：编译期由 cargo 注入的 `CARGO_MANIFEST_DIR`。
fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// 读取一个环境变量；空字符串视为未设置。
fn non_empty_env(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => Some(value),
        _ => None,
    }
}

/// 数据目录：`--data-dir` > `ASSESSMENT_DATA_DIR` > `{根}/assessments/report/data`。
fn resolve_data_dir(override_dir: Option<&Path>) -> PathBuf {
    if let Some(dir) = override_dir {
        return dir.to_path_buf();
    }
    if let Some(dir) = non_empty_env("ASSESSMENT_DATA_DIR") {
        return PathBuf::from(dir);
    }
    manifest_dir()
        .join("assessments")
        .join("report")
        .join("data")
}

/// 报告目录：`--report-dir` > `ASSESSMENT_REPORT_DIR` > `{根}/assessments/report`。
fn resolve_report_dir(override_dir: Option<&Path>) -> PathBuf {
    if let Some(dir) = override_dir {
        return dir.to_path_buf();
    }
    if let Some(dir) = non_empty_env("ASSESSMENT_REPORT_DIR") {
        return PathBuf::from(dir);
    }
    manifest_dir().join("assessments").join("report")
}

/// 知识点矩阵路径：`{根}/assessments/docs/04_knowledge_map.md`。
fn resolve_map_path() -> PathBuf {
    manifest_dir()
        .join("assessments")
        .join("docs")
        .join("04_knowledge_map.md")
}

/// 一次运行用到的全部路径（打包成一个结构体，避免主流程参数过多）。
struct Paths {
    /// 账本文件。
    ledger: PathBuf,
    /// 历史文件。
    history: PathBuf,
    /// 报告文件。
    report: PathBuf,
    /// 摘要文件。
    summary: PathBuf,
    /// 知识点矩阵。
    map: PathBuf,
}

impl Paths {
    /// 按「命令行覆盖 > 环境变量 > 项目内默认」的优先级解析全部路径。
    fn resolve(options: &Options) -> Paths {
        Paths {
            ledger: resolve_data_dir(options.data_dir.as_deref()).join("ledger.tsv"),
            history: resolve_data_dir(options.data_dir.as_deref()).join("history.jsonl"),
            report: resolve_report_dir(options.report_dir.as_deref()).join("assessment_report.md"),
            summary: resolve_report_dir(options.report_dir.as_deref()).join("summary.json"),
            map: resolve_map_path(),
        }
    }
}

// ===========================================================================
// 十四、子命令
// ===========================================================================

/// `--list`：列出账本里所有运行（id、开始时间、记录数、知识点数）。
fn run_list(records: &[LedgerRecord], zone: &TimeZone) {
    if records.is_empty() {
        println!("账本里还没有任何记录：先运行 `cargo test --test <模块名>`。");
        return;
    }
    println!(
        "账本里的运行记录（按开始时间升序；时间标注：{}）：\n",
        zone.label
    );
    println!(
        "{} {} {} {}",
        pad_end("运行 id", 26),
        pad_end("开始时间", 22),
        pad_end("记录数", 10),
        "知识点数"
    );
    for (run_id, first_ts) in list_runs(records) {
        let subset = records_for_run(records, &run_id);
        let kp_ids: BTreeSet<&str> = subset
            .iter()
            .map(|record| record.kp_id.as_str())
            .filter(|kp_id| !kp_id.is_empty())
            .collect();
        println!(
            "{} {} {} {}",
            pad_end(&run_id, 26),
            pad_end(&format_timestamp(first_ts, zone.offset_minutes), 22),
            pad_end(&subset.len().to_string(), 10),
            kp_ids.len()
        );
    }
}

/// `--emit-map`：从账本（全部运行的并集）生成知识点覆盖矩阵。
fn run_emit_map(records: &[LedgerRecord], map_path: &Path) -> Result<(), AppError> {
    let rows = collect_map_rows(records);
    if rows.is_empty() {
        eprintln!("账本里没有任何知识点记录，无法生成矩阵：请先运行 `cargo test`。");
        return Ok(());
    }
    // 测试目标名来自 Cargo.toml，这样矩阵里能写出可照抄的完整命令。
    let targets = load_test_targets();
    write_text(map_path, &render_map(&rows, &targets))?;
    println!(
        "已生成知识点覆盖矩阵：{}（{} 个知识点）",
        map_path.display(),
        rows.len()
    );
    Ok(())
}

/// 读取 `Cargo.toml` 并解析出「模块 id → 测试目标名」；读不到就返回空表。
fn load_test_targets() -> BTreeMap<String, String> {
    let cargo_path = manifest_dir().join("Cargo.toml");
    match fs::read_to_string(&cargo_path) {
        Ok(text) => parse_expected_modules(&text).1,
        Err(_) => BTreeMap::new(),
    }
}

/// `--check-map`：校验矩阵文件与账本并集是否一致；返回是否一致。
fn run_check_map(records: &[LedgerRecord], map_path: &Path) -> Result<bool, AppError> {
    let union: BTreeSet<String> = ledger_union(records).keys().cloned().collect();
    let text = match fs::read_to_string(map_path) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            println!(
                "矩阵文件不存在：{}；请先运行 `cargo run --bin assessment_report -- --emit-map`。",
                map_path.display()
            );
            return Ok(false);
        }
        Err(error) => return Err(io_error("读取矩阵", map_path, &error)),
    };
    let matrix = parse_map_kp_ids(&text);
    let missing = difference(&union, &matrix);
    let extra = difference(&matrix, &union);
    println!("矩阵文件：{}", map_path.display());
    println!(
        "账本并集知识点数：{}；矩阵知识点数：{}",
        union.len(),
        matrix.len()
    );
    if missing.is_empty() && extra.is_empty() {
        println!("结论：一致 ✅（矩阵与账本并集完全吻合）");
        return Ok(true);
    }
    if !missing.is_empty() {
        println!(
            "账本里有、矩阵里缺失的知识点（{} 个）：{}",
            missing.len(),
            preview_list(&missing)
        );
    }
    if !extra.is_empty() {
        println!(
            "矩阵里有、账本里没有的知识点（{} 个）：{}",
            extra.len(),
            preview_list(&extra)
        );
    }
    println!("结论：不一致 ❌（重新运行 `--emit-map` 更新矩阵）");
    Ok(false)
}

// ===========================================================================
// 十五、主流程
// ===========================================================================

/// 入口：解析参数 → 分派子命令 → 返回退出码。
fn run() -> Result<i32, AppError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let options = parse_args(&args)?;
    let zone = resolve_time_zone();
    let paths = Paths::resolve(&options);
    let records = load_ledger(&paths.ledger)?;

    match options.mode {
        Mode::List => {
            run_list(&records, &zone);
            Ok(0)
        }
        Mode::EmitMap => {
            run_emit_map(&records, &paths.map)?;
            Ok(0)
        }
        Mode::CheckMap => {
            let consistent = run_check_map(&records, &paths.map)?;
            Ok(if consistent { 0 } else { 1 })
        }
        Mode::Report => run_report(&records, &options, &zone, &paths),
    }
}

/// 默认模式：分析一次运行 → 写报告与摘要 → 追加历史 → 打印控制台摘要。
fn run_report(
    records: &[LedgerRecord],
    options: &Options,
    zone: &TimeZone,
    paths: &Paths,
) -> Result<i32, AppError> {
    // 1) 决定分析哪一次运行。
    let run_id = match &options.run_id {
        Some(run_id) => run_id.clone(),
        None => match latest_run_id(records) {
            Some(run_id) => run_id,
            None => {
                println!("账本里还没有任何运行记录（{}）。", paths.ledger.display());
                println!(
                    "请先运行一次考核，例如：cargo test --test lesson_01_variables_mutability"
                );
                return Ok(1);
            }
        },
    };
    if records_for_run(records, &run_id).is_empty() {
        println!(
            "账本里找不到运行 id `{run_id}`：用 `--list` 查看可用的运行 id。数据目录：{}",
            paths.ledger.display()
        );
        return Ok(1);
    }

    // 2) 预期模块集合：解析 Cargo.toml 的 [[test]] 段（同时拿到测试目标名）。
    let cargo_path = manifest_dir().join("Cargo.toml");
    let (mut all_modules, test_targets) = match fs::read_to_string(&cargo_path) {
        Ok(text) => parse_expected_modules(&text),
        Err(_) => (Vec::new(), BTreeMap::new()),
    };
    for record in records {
        if !record.module_id.is_empty() && !all_modules.contains(&record.module_id) {
            all_modules.push(record.module_id.clone());
        }
    }
    sort_modules_by_number(&mut all_modules);

    // 3) 预期知识点集合：矩阵存在则以矩阵为准，否则用账本并集。
    let union = ledger_union(records);
    let map_kp_ids = match fs::read_to_string(&paths.map) {
        Ok(text) => Some(parse_map_kp_ids(&text)),
        Err(_) => None,
    };
    let (expected, source_note, source_label) = expected_kp_ids(&union, map_kp_ids.as_ref());

    // 4) 历史对比：读 history.jsonl 里**上一次**（run_id ≠ 本次）的摘要。
    let previous = read_last_history(&paths.history, &run_id)?;

    // 5) 分析本次运行。
    let report = analyze(
        records,
        &run_id,
        &expected,
        &all_modules,
        &union,
        test_targets,
        source_note,
        source_label,
        &zone,
    );
    let comparison = compare_history(&report, &previous);
    let context = RenderContext {
        ledger_path: paths.ledger.clone(),
        report_path: paths.report.clone(),
        map_path: paths.map.clone(),
        time_label: zone.label,
        generated_at: format_timestamp(now_ms(), zone.offset_minutes),
        comparison,
        history_written: !options.no_history,
        history_path: paths.history.clone(),
    };

    // 6) 写报告 + 摘要 +（可选）历史。
    write_text(&paths.report, &render_markdown(&report, &context))?;
    write_text(
        &paths.summary,
        &build_summary_json(&report.analysis, &report.modules, true),
    )?;
    if !options.no_history {
        append_line(
            &paths.history,
            &build_summary_json(&report.analysis, &report.modules, false),
        )?;
    }

    // 7) 控制台摘要 + 退出码。
    print_console_summary(&report, &context);
    Ok(if report.analysis.passed == report.analysis.total {
        0
    } else {
        1
    })
}

/// 程序入口：打印错误并设置退出码。
fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(AppError::Usage(message)) => {
            eprintln!("参数错误：{message}");
            std::process::exit(2);
        }
        Err(AppError::Io(error)) => {
            eprintln!("错误：{error}");
            std::process::exit(1);
        }
    }
}

// ===========================================================================
// 十六、单元测试（纯函数，不产生 warning）
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use assessment_harness::escape;

    /// 把「公历年月日」换算成「1970-01-01 起的天数」（`civil_from_days` 的逆运算）。
    ///
    /// 只用于单元测试：验证 `civil_from_days` 真的算对了（避免手抄算法时出错）。
    fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
        let adjusted = if month <= 2 { year - 1 } else { year };
        let era = if adjusted >= 0 {
            adjusted
        } else {
            adjusted - 399
        } / 400;
        let yoe = adjusted - era * 400;
        let mp = if month > 2 { month - 3 } else { month + 9 } as i64;
        let doy = (153 * mp + 2) / 5 + day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    #[test]
    fn timestamp_matches_known_dates() {
        assert_eq!(format_timestamp(0, 0), "1970-01-01 00:00:00");
        // 2026-01-01 00:00:00 UTC = 1767225600 秒
        assert_eq!(
            format_timestamp(1_767_225_600_000, 0),
            "2026-01-01 00:00:00"
        );
        // 同一时刻在 UTC+8 是 08:00:00
        assert_eq!(
            format_timestamp(1_767_225_600_000, 8 * 60),
            "2026-01-01 08:00:00"
        );
        // 闰日：2024-02-29 00:00:00 UTC = 1709164800 秒
        assert_eq!(
            format_timestamp(1_709_164_800_000, 0),
            "2024-02-29 00:00:00"
        );
    }

    #[test]
    fn civil_conversion_round_trips() {
        for days in [-1_000_000i64, -1, 0, 1, 20_454, 1_000_000] {
            let (year, month, day) = civil_from_days(days);
            assert_eq!(days_from_civil(year, month, day), days);
        }
    }

    #[test]
    fn kp_module_id_extracts_lesson_prefix() {
        assert_eq!(kp_module_id("kp_01_08"), "lesson_01");
        assert_eq!(kp_module_id("kp_18_03"), "lesson_18");
        assert_eq!(kp_module_id("bad_id"), "");
    }

    #[test]
    fn percentage_rounds_to_nearest() {
        assert_eq!(percentage(0, 0), 0);
        assert_eq!(percentage(3, 4), 75);
        assert_eq!(percentage(1, 3), 33);
        assert_eq!(percentage(2, 3), 67);
        assert_eq!(percentage(8, 8), 100);
    }

    #[test]
    fn grade_boundaries() {
        assert_eq!(grade_label(100), "优秀");
        assert_eq!(grade_label(90), "优秀");
        assert_eq!(grade_label(89), "良好");
        assert_eq!(grade_label(75), "良好");
        assert_eq!(grade_label(74), "合格");
        assert_eq!(grade_label(60), "合格");
        assert_eq!(grade_label(59), "待加强");
        assert_eq!(grade_label(0), "待加强");
    }

    #[test]
    fn expected_modules_come_from_test_sections_only() {
        let text = "[[bin]]\nname = \"lesson_99_x\"\n\n[[test]]\n\
                    name = \"lesson_05_ownership_borrowing\"\n\n[[test]]\n\
                    name = \"lesson_01_variables_mutability\"\n";
        let (modules, targets) = parse_expected_modules(text);
        assert_eq!(
            modules,
            vec!["lesson_05".to_string(), "lesson_01".to_string()]
        );
        // 报告里要能写出可照抄的命令，所以测试目标名必须保留。
        assert_eq!(
            targets.get("lesson_05").map(String::as_str),
            Some("lesson_05_ownership_borrowing")
        );
    }

    #[test]
    fn module_sorting_is_numeric() {
        let mut modules = vec![
            "lesson_10".to_string(),
            "lesson_02".to_string(),
            "lesson_01".to_string(),
        ];
        sort_modules_by_number(&mut modules);
        assert_eq!(modules, vec!["lesson_01", "lesson_02", "lesson_10"]);
    }

    #[test]
    fn json_escaping_handles_quotes_and_controls() {
        assert_eq!(json_escape("a\"b\\c\nd\te"), "a\\\"b\\\\c\\nd\\te");
        assert_eq!(json_escape("\u{1}"), "\\u0001");
    }

    #[test]
    fn guide_parsing_extracts_command_course_and_readme() {
        let guide = "复习入口：cargo run --bin lesson_01_variables_mutability\
                     （src/tutorial/lesson_01_variables_mutability.rs）；\
                     说明见 src/tutorial/README.md 第一阶段「01 变量与可变性」";
        let (command, course, readme) = parse_guide(guide);
        assert_eq!(command, "cargo run --bin lesson_01_variables_mutability");
        assert_eq!(course, "src/tutorial/lesson_01_variables_mutability.rs");
        assert_eq!(readme, "src/tutorial/README.md 第一阶段「01 变量与可变性」");
    }

    #[test]
    fn ledger_line_parsing_keeps_tail_newlines() {
        // 第 10 列（detail）里含转义换行，解析后应还原成真实换行，且不破坏列数。
        let line = "run-1\t100\tfail\tlesson_01\t变量与可变性\tkp_01_01\t标题\tcore\t\
                    file.rs:42\t期望：42\\n  实际：41\t提示语\t复习入口：x（y）；说明见 z";
        let record = parse_ledger_line(line).expect("应能解析");
        assert_eq!(record.run_id, "run-1");
        assert_eq!(record.ts_ms, 100);
        assert_eq!(record.event, "fail");
        assert_eq!(record.location, "file.rs:42");
        assert_eq!(record.detail, "期望：42\n  实际：41");
        assert_eq!(record.hint, "提示语");
        assert_eq!(record.guide, "复习入口：x（y）；说明见 z");
    }

    #[test]
    fn outcome_prefers_pass_then_missing_then_fail() {
        let make = |event: &str| LedgerRecord {
            run_id: "r".to_string(),
            ts_ms: 0,
            event: event.to_string(),
            module_id: "lesson_01".to_string(),
            module_name: "变量与可变性".to_string(),
            kp_id: "kp_01_01".to_string(),
            kp_title: "标题".to_string(),
            kind: "basic".to_string(),
            location: "f.rs:1".to_string(),
            detail: String::new(),
            hint: String::new(),
            guide: String::new(),
        };
        let start = make("start");
        let pass = make("pass");
        let missing = make("missing");
        let fail = make("fail");
        let panic = make("panic");
        assert!(matches!(
            resolve_outcome(&[&start, &pass, &fail]),
            KpOutcome::Passed
        ));
        assert!(matches!(
            resolve_outcome(&[&start, &missing, &fail]),
            KpOutcome::Unimplemented
        ));
        assert!(matches!(
            resolve_outcome(&[&start, &fail, &panic]),
            KpOutcome::Failed
        ));
        assert!(matches!(
            resolve_outcome(&[&start, &panic]),
            KpOutcome::Panicked
        ));
        assert!(matches!(resolve_outcome(&[&start]), KpOutcome::Incomplete));
    }

    #[test]
    fn map_parsing_reads_third_column() {
        let text = "# 标题\n\n| 模块 | 模块名 | 知识点 | 标题 |\n| --- | --- | --- | --- |\n\
                    | lesson_01 | 变量与可变性 | kp_01_01 | let 绑定 | 基础 | f.rs:50 | 说明 |\n\
                    | lesson_01 | 变量与可变性 | kp_01_02 | 遮蔽 | 基础 | f.rs:104 | 说明 |\n\
                    | 其他行 | x | y |\n";
        let ids = parse_map_kp_ids(text);
        assert_eq!(ids.len(), 2);
        assert!(ids.contains("kp_01_01"));
        assert!(ids.contains("kp_01_02"));
    }

    #[test]
    fn merge_kp_takes_context_from_last_segment() {
        // 同一次运行里重复执行时，只统计最后一个 start 之后的事件；
        // 位置与复习指引要取自那条 start。
        let make = |event: &str, location: &str, detail: &str, hint: &str| LedgerRecord {
            run_id: "r".to_string(),
            ts_ms: 0,
            event: event.to_string(),
            module_id: "lesson_01".to_string(),
            module_name: "变量与可变性".to_string(),
            kp_id: "kp_01_01".to_string(),
            kp_title: "let 与 mut".to_string(),
            kind: "basic".to_string(),
            location: location.to_string(),
            detail: detail.to_string(),
            hint: hint.to_string(),
            guide: "复习入口：cargo run --bin x（y.rs）；说明见 z.md".to_string(),
        };
        let first = make("start", "old.rs:1", "旧的", "旧指引");
        let second = make("start", "new.rs:50", "新的", "新指引");
        let missing = make(
            "missing",
            "new.rs:91",
            "exercise_01_01_mut_counter",
            "实现要求",
        );
        let entries = [&first, &second, &missing];
        let merged = merge_kp(&entries);
        assert_eq!(merged.location, "new.rs:50");
        assert_eq!(merged.review, "新指引");
        assert_eq!(merged.missing_fn, "exercise_01_01_mut_counter");
        assert_eq!(merged.missing_hint, "实现要求");
        assert!(matches!(merged.outcome, KpOutcome::Unimplemented));
        assert_eq!(merged.title, "let 与 mut");
    }

    #[test]
    fn harness_escape_and_unescape_round_trip() {
        // 账本转义是 harness 的职责，本工具只负责还原；这里确认这一对函数确实互逆，
        // 也确认 `\t` / `\n` / `\\` 都不会破坏 TSV 的列结构。
        let raw = "期望：42\t实际：41\n第二行\\结尾";
        let encoded = escape(raw);
        assert!(!encoded.contains('\t'));
        assert!(!encoded.contains('\n'));
        assert_eq!(unescape(&encoded), raw);
    }
}
