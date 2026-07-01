@echo off
chcp 65001 >nul
title 凯伊密码管家 - 调试模式

echo ============================================
echo   * 凯伊密码管家 - 开发调试模式
echo ============================================
echo.

cd /d "%~dp0"

:: ── 第 1 步：清理残留进程 ────────────────────
echo [1/3] 清理残留进程...

:: 用 powershell 查端口 5173 上的进程并关掉
powershell -NoProfile -Command "try{$p=Get-NetTCPConnection -LocalPort 5173 -ErrorAction Stop;taskkill /F /PID $p.OwningProcess /PID 2>$null}catch{}" >nul 2>&1

:: 关掉旧的程序窗口
taskkill /F /IM "key-vault.exe" >nul 2>&1

:: ── 第 2 步：检查依赖 ────────────────────────
echo [2/3] 检查依赖...

if not exist "node_modules\" (
    echo.
    echo   首次运行，安装依赖...
    call npm install
    if errorlevel 1 (
        echo.
        echo [错误] npm install 失败，请检查 Node.js 是否已安装
        pause
        exit /b 1
    )
)

:: ── 第 3 步：启动 Tauri ──────────────────────
echo [3/3] 启动 Tauri 开发服务器...
echo.
echo   前端窗口: http://localhost:5173
echo.
echo ============================================
echo.

npx tauri dev

echo.
echo ============================================
if errorlevel 1 (
    echo [错误] 程序异常退出
) else (
    echo [正常] 调试程序已退出
)
echo.
pause
