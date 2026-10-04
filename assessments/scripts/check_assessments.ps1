<#
.SYNOPSIS
    考核体系自检：骨架完整性 + 零警告编译 + 知识点登记数校验。

.DESCRIPTION
    面向维护者。对 Cargo.toml 里注册的每个考核目标（[[test]]）依次做三件事：
      1. --no-run 编译，确认**零 warning**（MinGW 的 linker 噪声会被过滤掉）；
      2. 在**临时数据目录**里运行测试，确认每个知识点都报【未实现】
         （说明练习函数都是占位、没有残留的参考实现，骨架是"干净"的）；
      3. 校验知识点登记数：文件里 `#[test]` 的个数应等于输出里【未实现】的条数。

    自检数据写到临时目录，不会污染学员的账本与历史（report/data/）。

.PARAMETER Lesson
    只检查指定的课（课号或目标名，规则同 run_assessments.ps1）。省略则检查全部。

.PARAMETER SkipRun
    只做编译检查，不运行测试（更快）。

.EXAMPLE
    .\check_assessments.ps1
    检查全部 18 个考核目标。

.EXAMPLE
    .\check_assessments.ps1 -Lesson 05,06
    只检查第 5、6 课。
#>
[CmdletBinding()]
param(
    [string[]] $Lesson,
    [switch] $SkipRun
)

$ErrorActionPreference = 'Continue'
$ProgressPreference = 'SilentlyContinue'

# 定位仓库根目录（兼容 -File 与 scriptblock 两种加载方式）
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
    Write-Host '[错误] 找不到项目根目录（含 Cargo.toml）。' -ForegroundColor Red
    exit 2
}
Set-Location -LiteralPath $root

$cargoToml = Join-Path $root 'Cargo.toml'
if (-not (Test-Path -LiteralPath $cargoToml)) {
    Write-Host "[错误] 找不到 $cargoToml" -ForegroundColor Red
    exit 2
}

# 解析 [[test]] 目标
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
$allTargets = $allTargets | Sort-Object

# 解析 -Lesson
$targets = @()
if ($Lesson) {
    foreach ($token in $Lesson) {
        $raw = $token.Trim()
        if ($raw -match '^\d+$') {
            $prefix = 'lesson_{0:d2}' -f [int]$raw
            $targets += ($allTargets | Where-Object { $_ -like "$prefix*" })
        }
        else {
            $exact = $allTargets | Where-Object { $_ -eq $raw }
            if ($exact) { $targets += $exact } else { $targets += ($allTargets | Where-Object { $_ -like "$raw*" }) }
        }
    }
    $targets = $targets | Sort-Object -Unique
}
else {
    $targets = $allTargets
}

if (-not $targets -or $targets.Count -eq 0) {
    Write-Host '[错误] 没有匹配到任何考核目标。' -ForegroundColor Red
    exit 2
}

$verifyDataDir = Join-Path $env:TEMP ('assess-check-' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
New-Item -ItemType Directory -Force -Path $verifyDataDir | Out-Null

Write-Host ''
Write-Host '=============== 考核体系自检（骨架完整性） ===============' -ForegroundColor Cyan
Write-Host ("检查目标：{0} 个" -f $targets.Count)
Write-Host ("自检数据目录（不污染学员进度）：{0}" -f $verifyDataDir) -ForegroundColor DarkGray
Write-Host ''

$failures = New-Object System.Collections.Generic.List[string]
$summary = New-Object System.Collections.Generic.List[object]

foreach ($target in $targets) {
    $file = Join-Path $root ("assessments\{0}.rs" -f $target)
    if (-not (Test-Path -LiteralPath $file)) {
        Write-Host ("[缺失] {0}（文件不存在）" -f $target) -ForegroundColor Red
        $failures.Add("$target : 文件不存在")
        continue
    }

    $testCount = (Select-String -LiteralPath $file -Pattern '^\s*#\[test\]' -AllMatches).Count
    # 只统计"真正的占位调用"（行首的 todo_exercise，可能带 assessment_harness:: 前缀；排除文档注释里的示例文字）
    $placeholderCount = (Select-String -LiteralPath $file -Pattern '^\s*(assessment_harness::)?todo_exercise\(' -AllMatches).Count

    # --- 1) 编译检查（零 warning） ---
    # 输出重定向到文件而不是走管道：PowerShell 5.1 在大段多线程输出下用管道捕获会丢行，
    # 而「【未实现】条数 == 知识点条数」这条校验必须逐行可靠。
    # 并行跑其它 cargo 任务时可能撞上构建锁，导致偶发失败，因此失败后重试一次。
    $buildLog = Join-Path $verifyDataDir ("build-{0}.log" -f $target)
    $runLog = Join-Path $verifyDataDir ("run-{0}.log" -f $target)
    $buildOutput = ''
    $compiled = $false
    for ($attempt = 1; $attempt -le 2; $attempt++) {
        & cargo test --test $target --no-run *> $buildLog
        $buildOutput = if (Test-Path -LiteralPath $buildLog) {
            [System.IO.File]::ReadAllText($buildLog, [System.Text.Encoding]::UTF8)
        }
        else { '' }
        $compiled = ($buildOutput -notmatch 'error\[E\d+\]' -and
                     $buildOutput -notmatch '(?m)^error' -and
                     $buildOutput -notmatch 'could not compile')
        if ($compiled) { break }
        Start-Sleep -Seconds 2
    }

    $noisy = ($buildOutput -split "`n") |
        Where-Object { $_ -match 'warning' } |
        Where-Object { $_ -notmatch 'corrupt \.drectve' } |
        Where-Object { $_ -notmatch 'linker stderr' } |
        Where-Object { $_ -notmatch 'generated \d+ warning' } |
        Where-Object { $_ -notmatch 'warn\(linker_messages\)' } |
        Where-Object { $_ -match [regex]::Escape($target) -or $_ -match 'assessment_harness' -or $_ -match 'assessments\\' }

    if (-not $compiled) {
        Write-Host ("[编译失败] {0}" -f $target) -ForegroundColor Red
        ($buildOutput -split "`n") |
            Where-Object { $_ -match '(?m)^error|error\[E\d+\]|could not compile|^ *-->' } |
            Select-Object -First 12 |
            ForEach-Object { Write-Host ("        " + $_.TrimEnd()) -ForegroundColor DarkRed }
        $failures.Add("$target : 编译失败")
    }
    elseif ($noisy) {
        Write-Host ("[有警告] {0}" -f $target) -ForegroundColor Yellow
        foreach ($w in $noisy) { Write-Host ("        " + $w.Trim()) -ForegroundColor DarkYellow }
        $failures.Add("$target : 编译有 warning")
    }
    else {
        Write-Host ("[编译 OK] {0}（知识点 {1} 个，占位 {2} 处）" -f $target, $testCount, $placeholderCount) -ForegroundColor Green
    }

    if ($SkipRun -or -not $compiled) {
        $summary.Add([pscustomobject]@{ Target = $target; Kp = $testCount; Placeholder = $placeholderCount; Unimplemented = -1 })
        continue
    }

    # --- 2) 骨架态运行：每个知识点都必须报【未实现】 ---
    # `--test-threads=1`：串行执行，输出顺序固定，便于逐条计数（也避免多线程输出交错）
    $env:ASSESSMENT_DATA_DIR = $verifyDataDir
    $env:ASSESSMENT_RUN_ID = "selfcheck-$target"
    & cargo test --test $target -- --test-threads=1 *> $runLog
    $runOutput = if (Test-Path -LiteralPath $runLog) {
        [System.IO.File]::ReadAllText($runLog, [System.Text.Encoding]::UTF8)
    }
    else { '' }
    $unimplemented = ([regex]::Matches($runOutput, '【未实现')).Count

    if ($unimplemented -ne $testCount) {
        Write-Host ("[骨架不干净] {0}：应 {1} 个【未实现】，实际 {2} 个" -f $target, $testCount, $unimplemented) -ForegroundColor Red
        Write-Host '        说明有练习函数已被实现（或测试函数被改动），请检查该文件。' -ForegroundColor DarkYellow
        $failures.Add("$target : 骨架态【未实现】数量不符（期望 $testCount，实际 $unimplemented）")
    }
    else {
        Write-Host ("           骨架态运行：{0} 个知识点全部报【未实现】" -f $unimplemented) -ForegroundColor Green
    }

    $summary.Add([pscustomobject]@{ Target = $target; Kp = $testCount; Placeholder = $placeholderCount; Unimplemented = $unimplemented })
}

Write-Host ''
Write-Host '-------------------------- 汇总 --------------------------' -ForegroundColor Cyan
$summary | Format-Table -AutoSize | Out-String | Write-Host

$totalKp = ($summary | Measure-Object -Property Kp -Sum).Sum
Write-Host ("知识点合计：{0} 个" -f $totalKp) -ForegroundColor Cyan

if ($failures.Count -eq 0) {
    Write-Host '自检结论：全部通过（编译零警告 + 骨架干净）。' -ForegroundColor Green
    exit 0
}

Write-Host ("自检结论：发现 {0} 个问题：" -f $failures.Count) -ForegroundColor Red
foreach ($item in $failures) { Write-Host ("  - " + $item) -ForegroundColor Red }
exit 1
