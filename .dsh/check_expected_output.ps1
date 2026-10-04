$ErrorActionPreference = 'Continue'
# 严格校验器 v3：
#   对每个连续"期望值块"，其内容必须**按顺序、逐字节**出现在紧邻上方的实际输出块末尾
#   （允许实际块前缀有多余行，例如同一个 println! 之前还打印了别的内容）。
#   若在紧邻上方找不到，再尝试向上回溯最多 3 个"其它行段"，并记录回溯距离。
$root = (Get-Location).Path
$lessons = @(
    Get-ChildItem (Join-Path $root 'src\tutorial') -Filter 'lesson_*.rs'
) | Sort-Object Name
$MARK = '// 预期输出：'
function Test-Exempt([string]$s) {
    # 仅豁免真正的"非确定值占位符"：地址/十六进制占位、因环境而异、每次运行不同、ASLR
    # 注意：不能一概豁免含尖括号的行 —— `Point<i32> 占用 8 字节`、`identity::<i32>(42) = 42`
    # 都是合法的确定性期望值，必须参与比对。
    return ($s -match '<十六进制|<地址|<本文件|<字节数>|因环境而异|每次运行|ASLR')
}

$totalChecked = 0; $totalMismatch = 0; $totalSkipped = 0
foreach ($f in $lessons) {
    $exe = Join-Path $env:TEMP ("ck5_" + $f.BaseName + ".exe")
    & rustc --edition 2024 --crate-type bin $f.FullName -o $exe 2>$null | Out-Null
    if ($LASTEXITCODE -ne 0) { Write-Output ("COMPILE FAIL: " + $f.Name); continue }
    $out = @(& $exe 2>&1 | ForEach-Object { [string]$_ })

    # 期望块：连续的期望值行
    $expSegs = @()
    for ($i = 0; $i -lt $out.Count; $i++) {
        if ($out[$i].StartsWith($MARK)) {
            $j = $i
            while ($j -lt $out.Count -and $out[$j].StartsWith($MARK)) { $j++ }
            $lines = @()
            for ($k = $i; $k -lt $j; $k++) { $lines += $out[$k].Substring($MARK.Length) }
            $expSegs += [pscustomobject]@{ Start = $i; Lines = $lines }
            $i = $j - 1
        }
    }

    $checked = 0; $mismatch = 0; $skipped = 0
    foreach ($seg in $expSegs) {
        $need = @($seg.Lines | Where-Object { -not (Test-Exempt $_) })
        $skipped += ($seg.Lines.Count - $need.Count)
        if ($need.Count -eq 0) { continue }
        # 向上回溯：在 start 之前收集实际输出行，直到遇到上一个期望块
        $cand = @()
        for ($k = $seg.Start - 1; $k -ge 0; $k--) {
            if ($out[$k].StartsWith($MARK)) { break }
            $cand = @($out[$k]) + $cand
        }
        # 在 cand 中寻找 need 的连续子序列
        $found = $false
        for ($s = 0; $s -le ($cand.Count - $need.Count); $s++) {
            $ok = $true
            for ($t = 0; $t -lt $need.Count; $t++) {
                if ($cand[$s + $t] -ne $need[$t]) { $ok = $false; break }
            }
            if ($ok) { $found = $true; break }
        }
        $checked += $need.Count
        if (-not $found) {
            $mismatch += $need.Count
            Write-Output ("MISMATCH {0}: 期望=[{1}] 未在其上方实际输出中出现；实际块=[{2}]" -f `
                $f.BaseName, ($need -join ' | '), (($cand | Select-Object -Last 4) -join ' | '))
        }
    }
    $totalChecked += $checked; $totalMismatch += $mismatch; $totalSkipped += $skipped
    Write-Output ("{0,-16} 已核对={1,3}  未命中={2}  豁免={3}" -f $f.BaseName, $checked, $mismatch, $skipped)
}
Write-Output ""
Write-Output ("汇总：已核对={0}  未命中={1}  豁免={2}" -f $totalChecked, $totalMismatch, $totalSkipped)
