@echo off
setlocal enabledelayedexpansion
chcp 65001 >nul
title 凯伊密码管家 - 调试模式
echo ============================================
echo   * 凯伊密码管家 - 开发调试模式
echo ============================================
echo.
echo  前端端口: http://localhost:5173
echo  按 Ctrl+C 停止调试
echo.
echo ============================================

cd /d "%~dp0"

:: ── 清理残留的旧进程 ──────────────────────────
echo [检查] 检测残留的旧进程...

:: 查端口 5173 (Vite 开发服务器)
for /f "tokens=5" %%a in ('
    netstat -ano ^| findstr /C:":5173 " ^| findstr LISTENING
') do (
    echo [清理] 发现旧 Vite 进程 (PID: %%a)，正在关闭...
    taskkill /F /PID %%a >nul 2>&1
)

:: 查 key-vault 程序窗口
taskkill /F /IM "key-vault.exe" >nul 2>&1
if !errorlevel! equ 0 (
    echo [清理] 已关闭旧的 key-vault 窗口
)

:: 等端口释放
timeout /t 1 /nobreak >nul

:: ── 检查依赖 ──────────────────────────────────
if not exist "node_modules\" (
    echo [安装] 正在安装前端依赖...
    npm install
    if !errorlevel! neq 0 (
        echo.
        echo [错误] npm install 失败，请检查 Node.js 是否已安装
        pause
        exit /b 1
    )
    echo.
)

:: ── 启动 Tauri 开发模式 ──────────────────────
echo [启动] 正在运行 npx tauri dev...
echo.
npx tauri dev

if !errorlevel! neq 0 (
    echo.
    echo [错误] 程序异常退出 (错误码: !errorlevel!)
) else (
    echo.
    echo [正常] 调试程序已退出
)

echo.
echo 按任意键关闭此窗口...
pause >nul
