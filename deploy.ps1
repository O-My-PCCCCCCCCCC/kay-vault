<#
 .SYNOPSIS
  凯伊密码管家 · 部署工具
  将编译好的程序部署到目标磁盘，并引导全盘加密。
#>

#Requires -RunAsAdministrator

$host.UI.RawUI.WindowTitle = "凯伊密码管家 - 部署工具"
$ErrorActionPreference = "Stop"

function Write-Step {
  param([string]$Msg, [string]$Color = "Cyan")
  Write-Host "`n>> $Msg" -ForegroundColor $Color
}

function Write-OK { Write-Host "  [✓] $args" -ForegroundColor Green }
function Write-Warn { Write-Host "  [!] $args" -ForegroundColor Yellow }
function Write-Err { Write-Host "  [✗] $args" -ForegroundColor Red }

# ── 标题 ──
Clear-Host
Write-Host "╔══════════════════════════════════════╗" -ForegroundColor DarkCyan
Write-Host "║    🔑 凯伊密码管家 · 部署工具       ║" -ForegroundColor DarkCyan
Write-Host "╠══════════════════════════════════════╣" -ForegroundColor DarkCyan
Write-Host "║  将程序部署到目标磁盘                ║" -ForegroundColor DarkCyan
Write-Host "╚══════════════════════════════════════╝" -ForegroundColor DarkCyan

# ── 检查构建产物 ──
Write-Step "检查构建产物..." "Yellow"
$buildDir = Split-Path -Parent $PSScriptRoot
$exePath = Join-Path $buildDir "src-tauri\target\release\key-vault.exe"

if (-not (Test-Path $exePath)) {
  Write-Err "未找到 key-vault.exe"
  Write-Host "  请先运行 npm run tauri build 构建" -ForegroundColor Gray
  pause
  exit 1
}

$exeSize = (Get-Item $exePath).Length / 1MB
Write-OK "key-vault.exe ($([math]::Round($exeSize,1)) MB)"

# ── 列出磁盘 ──
Write-Step "扫描可用磁盘..." "Yellow"
$drives = Get-PSDrive -PSProvider FileSystem | Where-Object { $_.Root -match '^[A-Z]:\\$' -and $_.Root -ne "$env:SystemDrive" } | Sort-Object Root

if ($drives.Count -eq 0) {
  Write-Err "未找到可用的磁盘"
  pause
  exit 1
}

Write-Host "`n  可用的磁盘:" -ForegroundColor Gray
$i = 1
$driveMenu = @{}
foreach ($d in $drives) {
  $freeGB = [math]::Round($d.Free / 1GB, 1)
  $usedGB = [math]::Round(($d.Used + $d.Free) / 1GB - $freeGB, 1)
  $totalGB = [math]::Round(($d.Used + $d.Free) / 1GB, 1)
  $pct = if ($totalGB -gt 0) { [math]::Round(($d.Used + $d.Free - $d.Free) / ($d.Used + $d.Free) * 100) } else { 0 }
  Write-Host "  [$i] $($d.Root)  $totalGB GB  (已用 $pct%)" -ForegroundColor Gray
  $driveMenu[$i.ToString()] = $d.Root
  $i++
}

# ── 选择目标 ──
$choice = Read-Host "`n  请选择目标磁盘 (1-$($drives.Count))"
$targetRoot = $driveMenu[$choice]
if (-not $targetRoot) {
  Write-Err "无效选择"
  pause
  exit 1
}

$targetDir = Join-Path $targetRoot "KayVault"
Write-OK "目标: $targetRoot → $targetDir"

# ── 确认 ──
Write-Warn "将部署到 $targetRoot，覆盖可能已存在的文件"
$confirm = Read-Host "继续？(Y/N)"
if ($confirm -ne 'Y' -and $confirm -ne 'y') {
  Write-Host "已取消" -ForegroundColor Gray
  pause
  exit
}

# ── 复制文件 ──
Write-Step "复制文件到 $targetDir ..." "Yellow"
try {
  New-Item -ItemType Directory -Force -Path $targetDir | Out-Null

  # 复制 exe
  Copy-Item $exePath (Join-Path $targetDir "key-vault.exe") -Force
  Write-OK "key-vault.exe"

  # 复制资源（如果有）
  $resDir = Join-Path $buildDir "src-tauri\target\release\resources"
  if (Test-Path $resDir) {
    Copy-Item "$resDir\*" (Join-Path $targetDir "resources\") -Recurse -Force
    Write-OK "resources/"
  }

  Write-OK "文件部署完成"
} catch {
  Write-Err "复制失败: $_"
  pause
  exit 1
}

# ── 加密引导 ──
Write-Step "磁盘加密建议" "Green"
Write-Host @"

  为了安全，建议对 $targetRoot 启用全盘加密：

  ┌────────────────────────────────────────┐
  │                                        │
  │  【推荐】VeraCrypt                     │
  │  1. 下载: https://veracrypt.fr         │
  │  2. 安装 → 创建加密卷 → 加密整个分区   │
  │  3. 解锁后运行 $($targetRoot)KayVault\key-vault.exe  │
  │                                        │
  │  【备选】BitLocker                     │
  │  右键 $($targetRoot) → 启用 BitLocker    │
  │                                        │
  └────────────────────────────────────────┘

  部署完成！运行 $($targetRoot)KayVault\key-vault.exe 启动程序。
  首次使用：输入任意主密码即可创建密码库。

"@ -ForegroundColor Gray

Read-Host "按 Enter 退出"
