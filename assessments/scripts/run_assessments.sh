#!/usr/bin/env bash
#
# Rust 教程考核环节：一键运行考核（libtest）+ 生成学习评估报告。
#
# 用法：
#   ./run_assessments.sh              # 考全部 18 课
#   ./run_assessments.sh 05 07        # 只考第 5、7 课（可写 05 / 5 / lesson_05）
#   ./run_assessments.sh --open       # 跑完打开报告（macOS: open / Linux: xdg-open）
#   ./run_assessments.sh --quiet      # 不回显 cargo test 过程输出（仍写入日志）
#   ./run_assessments.sh --no-history # 不写入历史记录（维护者自检用）
#
# 考核是可选环节：本脚本不会修改任何课程文件或考核文件。

set -uo pipefail

# ---------------------------------------------------------------------------
# 1. 定位仓库根目录（本脚本位于 <root>/assessments/scripts/）
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$ROOT" || exit 2

if [ ! -f "$ROOT/Cargo.toml" ]; then
    echo "[错误] 在 $ROOT 找不到 Cargo.toml，请把本脚本放在 <仓库根>/assessments/scripts/ 下。" >&2
    exit 2
fi

# ---------------------------------------------------------------------------
# 2. 解析参数
# ---------------------------------------------------------------------------
OPEN_REPORT=0
QUIET=0
NO_HISTORY=0
LESSON_TOKENS=()

for arg in "$@"; do
    case "$arg" in
        --open)       OPEN_REPORT=1 ;;
        --quiet)      QUIET=1 ;;
        --no-history) NO_HISTORY=1 ;;
        -h|--help)
            sed -n '2,16p' "$0"
            exit 0
            ;;
        *)            LESSON_TOKENS+=("$arg") ;;
    esac
done

# ---------------------------------------------------------------------------
# 3. 从 Cargo.toml 解析全部考核测试目标（只取 [[test]] 段，避免混入同名 bin）
# ---------------------------------------------------------------------------
ALL_TARGETS=()
section=""
while IFS= read -r line; do
    trimmed="${line#"${line%%[![:space:]]*}"}"
    case "$trimmed" in
        "[[test]]") section="test"; continue ;;
        "[[bin]]")  section="bin";  continue ;;
        "[["*)      section="other"; continue ;;
        "[dependencies]"|"[package]"|"[lib]") section="other"; continue ;;
    esac
    if [ "$section" = "test" ] && [[ "$trimmed" =~ ^name[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
        ALL_TARGETS+=("${BASH_REMATCH[1]}")
    fi
done < "$ROOT/Cargo.toml"

if [ "${#ALL_TARGETS[@]}" -eq 0 ]; then
    echo "[错误] Cargo.toml 里没有解析到任何 [[test]] 考核目标，考核体系可能未安装完整。" >&2
    exit 2
fi

# ---------------------------------------------------------------------------
# 4. 计算本次要跑的考核目标
# ---------------------------------------------------------------------------
TARGETS=()
if [ "${#LESSON_TOKENS[@]}" -eq 0 ]; then
    TARGETS=("${ALL_TARGETS[@]}")
else
    for token in "${LESSON_TOKENS[@]}"; do
        matched=0
        if [[ "$token" =~ ^[0-9]+$ ]]; then
            printf -v prefix 'lesson_%02d' "$token" 2>/dev/null || prefix="lesson_$(printf '%02d' "$token")"
            for candidate in "${ALL_TARGETS[@]}"; do
                if [[ "$candidate" == "$prefix"* ]]; then
                    TARGETS+=("$candidate"); matched=1
                fi
            done
        else
            for candidate in "${ALL_TARGETS[@]}"; do
                if [ "$candidate" = "$token" ]; then
                    TARGETS+=("$candidate"); matched=1
                fi
            done
            if [ "$matched" -eq 0 ]; then
                for candidate in "${ALL_TARGETS[@]}"; do
                    if [[ "$candidate" == "$token"* ]]; then
                        TARGETS+=("$candidate"); matched=1
                    fi
                done
            fi
        fi
        if [ "$matched" -eq 0 ]; then
            echo "[警告] 没有匹配到考核目标：$token" >&2
        fi
    done
fi

if [ "${#TARGETS[@]}" -eq 0 ]; then
    echo "[错误] 没有匹配到任何考核目标，已终止。" >&2
    exit 2
fi

# ---------------------------------------------------------------------------
# 5. 准备本次运行
# ---------------------------------------------------------------------------
RUN_ID="run-$(date +%Y%m%d-%H%M%S)"
export ASSESSMENT_RUN_ID="$RUN_ID"

# 报告生成器只用 UTC 计算时间；这里把本机时区偏移（分钟，东为正）传给它，
# 报告里就能显示成本地时间。缺了它是允许的（报告会明确标注 UTC）。
OFFSET_RAW="$(date +%z)"
if [[ "$OFFSET_RAW" =~ ^([+-])([0-9]{2})([0-9]{2})$ ]]; then
    OFFSET=$((10#${BASH_REMATCH[2]} * 60 + 10#${BASH_REMATCH[3]}))
    if [ "${BASH_REMATCH[1]}" = "-" ]; then OFFSET=$((-OFFSET)); fi
    export ASSESSMENT_LOCAL_OFFSET_MINUTES="$OFFSET"
fi

DATA_DIR="$ROOT/assessments/report/data"
REPORT_DIR="$ROOT/assessments/report"
LOG_PATH="$DATA_DIR/last_run.log"
mkdir -p "$DATA_DIR"

echo ""
echo "==================== Rust 教程考核环节 ===================="
echo "运行 ID   : $RUN_ID"
echo "考核目标  : ${#TARGETS[@]} 个"
echo "本次课程  : ${TARGETS[*]}"
echo "----------------------------------------------------------"
echo "第 1 步：运行考核（cargo test --no-fail-fast）"
echo ""

TEST_ARGS=(test --no-fail-fast)
for target in "${TARGETS[@]}"; do
    TEST_ARGS+=(--test "$target")
done

if [ "$QUIET" -eq 1 ]; then
    cargo "${TEST_ARGS[@]}" > "$LOG_PATH" 2>&1
    test_exit=$?
else
    cargo "${TEST_ARGS[@]}" 2>&1 | tee "$LOG_PATH"
    test_exit=${PIPESTATUS[0]}
fi

echo ""
echo "完整测试输出已留档：$LOG_PATH"
if [ "$test_exit" -ne 0 ]; then
    echo "提示：cargo test 退出码为 $test_exit（骨架态或有未通过的知识点时属预期）。"
fi

# ---------------------------------------------------------------------------
# 6. 生成学习评估报告
# ---------------------------------------------------------------------------
echo ""
echo "第 2 步：生成学习评估报告（cargo run --bin assessment_report）"
echo ""

REPORT_ARGS=(run --quiet --bin assessment_report)
if [ "$NO_HISTORY" -eq 1 ]; then
    REPORT_ARGS+=(-- --no-history)
fi

cargo "${REPORT_ARGS[@]}"
report_exit=$?

REPORT_PATH="$REPORT_DIR/assessment_report.md"
echo ""
if [ -f "$REPORT_PATH" ]; then
    echo "评估报告：$REPORT_PATH"
    echo "机器可读摘要：$REPORT_DIR/summary.json"
    echo "报告解读见：assessments/docs/03_reading_reports.md"
else
    echo "[警告] 没有找到生成的报告文件，请检查上方报告生成器的输出。"
fi

if [ "$OPEN_REPORT" -eq 1 ] && [ -f "$REPORT_PATH" ]; then
    if command -v open >/dev/null 2>&1; then
        open "$REPORT_PATH"
    elif command -v xdg-open >/dev/null 2>&1; then
        xdg-open "$REPORT_PATH"
    else
        echo "[提示] 未找到 open / xdg-open，请手动打开报告。"
    fi
fi
echo "=========================================================="

exit "$report_exit"
