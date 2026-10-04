<#
.SYNOPSIS
    Rust 教程考核环节：一键运行考核（libtest）+ 生成学习评估报告。

.DESCRIPTION
    做三件事：
      1. 为本次运行生成一个运行 ID（ASSESSMENT_RUN_ID），让 18 个测试目标的结果归入同一次考核；
      2. 运行 cargo test --no-fail-fast（保证即使前面有目标失败，也会跑完全部考核，一次拿到完整结果），
         并把完整输出留档到 assessments/report/data/last_run.log；
      3. 调用报告生成器 cargo run --bin assessment_report，输出总分、薄弱环节与学习进度对比。

    考核是可选环节：本脚本不会修改任何课程文件或考核文件。

.PARAMETER Lesson
    只考指定的课。可以写课号（05、5、lesson_05）或考核目标名（lesson_05_ownership_borrowing）。
    省略则考全部 18 课。

.PARAMETER Open
    跑完直接打开生成的评估报告（assessments/report/assessment_report.md）。

.PARAMETER Quiet
    不在控制台回显 cargo test 的过程输出（仍然会写入 last_run.log）。

.PARAMETER NoHistory
    本次运行不写入历史记录（report/data/history.jsonl），适合维护者自检时使用。

.EXAMPLE
    .\run_assessments.ps1
    考核全部 18 课并生成报告。

.EXAMPLE
    .\run_assessments.ps1 -Lesson 05,07 -Open
    只考第 5、7 课，跑完打开报告。

.NOTES
    如果 PowerShell 因执行策略拒绝运行本脚本，可用：
      powershell -ExecutionPolicy Bypass -File .\assessments\scripts\run_assessments.ps1
    或按根 README 第六节的方式用 [scriptblock]::Create(...) 加载执行。
#>
[CmdletBinding()]
param(
    [string[]] $Lesson,
    [switch] $Open,
    [switch] $Quiet,
    [switch] $NoHistory
)

$ErrorActionPreference = 'Continue'
$ProgressPreference = 'SilentlyContinue'

# ---------------------------------------------------------------------------
# 1. 定位仓库根目录（本脚本位于 <root>\assessments\scripts\）
#    兼容两种运行方式：
#      a) powershell -ExecutionPolicy Bypass -File .\run_assessments.ps1
#      b) $code = [IO.File]::ReadAllText(路径, [Text.Encoding]::UTF8); & ([scriptblock]::Create($code))
#        （b 方式下 $PSScriptRoot 可能为空，因此再从当前目录向上找 Cargo.toml）
# ---------------------------------------------------------------------------
function Find-RepoRoot {
    param([string] $StartDir)

    $dir = $null
    try { $dir = (Resolve-Path -LiteralPath $StartDir).Path } catch { return $null }
    for ($depth = 0; $depth -lt 6 -and $dir; $depth++) {
        if (Test-Path -LiteralPath (Join-Path $dir 'Cargo.toml')) { return $dir }
        $parent = Split-Path -Parent $dir
        if (-not $parent -or $parent -eq $dir) { break }
        $dir = $parent
    }
    return $null
}

$startDir = $PSScriptRoot
if (-not $startDir) {
    $commandPath = $MyInvocation.MyCommand.Path
    if ($commandPath) { $startDir = Split-Path -Parent $commandPath }
}
if (-not $startDir) { $startDir = (Get-Location).Path }

$root = Find-RepoRoot -StartDir $startDir
if (-not $root) { $root = Find-RepoRoot -StartDir (Get-Location).Path }
if (-not $root) {
    Write-Host '[错误] 找不到项目根目录（含 Cargo.toml）：请在仓库内运行本脚本。' -ForegroundColor Red
    exit 2
}
Set-Location -LiteralPath $root

# ---------------------------------------------------------------------------
# 2. 从 Cargo.toml 解析全部考核测试目标（只取 [[test]] 段，避免混入同名 bin）
# ---------------------------------------------------------------------------
$cargoToml = Join-Path $root 'Cargo.toml'
$section = ''
$allTargets = New-Object System.Collections.Generic.List[string]
foreach ($line in (Get-Content -LiteralPath $cargoToml)) {
    $trimmed = $line.Trim()
    if ($trimmed -match '^\[\[([A-Za-z0-9_]+)\]\]') { $section = $Matches[1]; continue }
    if ($trimmed -match '^\[([A-Za-z0-9_]+)') { $section = $Matches[1]; continue }
    if ($section -eq 'test' -and $trimmed -match '^name\s*=\s*"([^"]+)"') {
        if (-not $allTargets.Contains($Matches[1])) { $allTargets.Add($Matches[1]) }
    }
}

if ($allTargets.Count -eq 0) {
    Write-Host '[错误] Cargo.toml 里没有解析到任何 [[test]] 考核目标，考核体系可能未安装完整。' -ForegroundColor Red
    exit 2
}

# 依据课堂顺序排序（lesson_01 → lesson_18）
$sorted = $allTargets | Sort-Object

# ---------------------------------------------------------------------------
# 3. 解析 -Lesson 参数
# ---------------------------------------------------------------------------
function Resolve-Targets {
    param([string[]] $Tokens, [string[]] $Candidates)

    $picked = New-Object System.Collections.Generic.List[string]
    foreach ($token in $Tokens) {
        $raw = $token.Trim()
        if ([string]::IsNullOrWhiteSpace($raw)) { continue }

        $matched = @()
        if ($raw -match '^\d+$') {
            # 纯数字：5 / 05 → lesson_05
            $prefix = 'lesson_{0:d2}' -f [int]$raw
            $matched = $Candidates | Where-Object { $_ -like "$prefix*" }
        }
        else {
            $matched = $Candidates | Where-Object { $_ -eq $raw }
            if ($matched.Count -eq 0) {
                $matched = $Candidates | Where-Object { $_ -like "$raw*" }
            }
        }

        if ($matched.Count -eq 0) {
            Write-Host "[警告] 没有匹配到考核目标：$raw（可用目标见 Cargo.toml 的 [[test]] 段）" -ForegroundColor Yellow
            continue
        }
        foreach ($item in $matched) {
            if (-not $picked.Contains($item)) { $picked.Add($item) }
        }
    }
    return $picked
}

if ($Lesson) {
    $targets = Resolve-Targets -Tokens $Lesson -Candidates $sorted
    if ($targets.Count -eq 0) {
        Write-Host '[错误] -Lesson 没有匹配到任何考核目标，已终止。' -ForegroundColor Red
        exit 2
    }
}
else {
    $targets = $sorted
}

# ---------------------------------------------------------------------------
# 4. 准备本次运行
# ---------------------------------------------------------------------------
$runId = 'run-' + (Get-Date -Format 'yyyyMMdd-HHmmss')
$env:ASSESSMENT_RUN_ID = $runId

# 报告生成器只会用 UTC 计算时间；这里把本机时区偏移（分钟，东为正）传给它，
# 报告里就能显示成本地时间。缺了它是允许的（报告会明确标注 UTC）。
$env:ASSESSMENT_LOCAL_OFFSET_MINUTES = [string][int]((Get-Date) - (Get-Date).ToUniversalTime()).TotalMinutes

$dataDir = Join-Path $root 'assessments\report\data'
$reportDir = Join-Path $root 'assessments\report'
$logPath = Join-Path $dataDir 'last_run.log'
New-Item -ItemType Directory -Force -Path $dataDir | Out-Null

Write-Host ''
Write-Host '==================== Rust 教程考核环节 ====================' -ForegroundColor Cyan
Write-Host ("运行 ID   : {0}" -f $runId)
Write-Host ("考核目标  : {0} 个" -f $targets.Count)
Write-Host ("本次课程  : {0}" -f (($targets | ForEach-Object { $_ }) -join ', '))
Write-Host '----------------------------------------------------------'
Write-Host '第 1 步：运行考核（cargo test --no-fail-fast）' -ForegroundColor Cyan
Write-Host ''

$testArgs = @('test', '--no-fail-fast')
foreach ($target in $targets) {
    $testArgs += '--test'
    $testArgs += $target
}

if ($Quiet) {
    & cargo @testArgs 2>&1 | Out-File -LiteralPath $logPath -Encoding utf8
}
else {
    & cargo @testArgs 2>&1 | Tee-Object -FilePath $logPath
}
$testExit = $LASTEXITCODE
Write-Host ''
Write-Host ("完整测试输出已留档：{0}" -f $logPath) -ForegroundColor DarkGray
if ($testExit -ne 0) {
    Write-Host ("提示：cargo test 退出码为 {0}（骨架态或有未通过的知识点时是正常的，属预期）。" -f $testExit) -ForegroundColor Yellow
}

# ---------------------------------------------------------------------------
# 5. 生成学习评估报告
# ---------------------------------------------------------------------------
Write-Host ''
Write-Host '第 2 步：生成学习评估报告（cargo run --bin assessment_report）' -ForegroundColor Cyan
Write-Host ''

$reportArgs = @('run', '--quiet', '--bin', 'assessment_report')
if ($NoHistory) {
    $reportArgs += @('--', '--no-history')
}
# 显式捕获再逐行 Write-Host：保证无论调用环境如何（-File / scriptblock），
# 报告生成器的控制台摘要都能显示出来。
$reportOutput = & cargo @reportArgs 2>&1
$reportExit = $LASTEXITCODE
foreach ($line in $reportOutput) { Write-Host $line }

$reportPath = Join-Path $reportDir 'assessment_report.md'
Write-Host ''
if (Test-Path -LiteralPath $reportPath) {
    Write-Host ("评估报告：{0}" -f $reportPath) -ForegroundColor Green
    Write-Host ("机器可读摘要：{0}" -f (Join-Path $reportDir 'summary.json')) -ForegroundColor DarkGray
    Write-Host '报告解读见：assessments/docs/03_reading_reports.md' -ForegroundColor DarkGray
}
else {
    Write-Host '[警告] 没有找到生成的报告文件，请检查上方报告生成器的输出。' -ForegroundColor Yellow
}

if ($Open -and (Test-Path -LiteralPath $reportPath)) {
    Invoke-Item -LiteralPath $reportPath
}
Write-Host '==========================================================' -ForegroundColor Cyan

# 报告生成器的退出码：全部通过为 0，否则为 1（便于脚本/CI 判断）
if ($null -ne $reportExit) { exit $reportExit }
exit $testExit
