@echo off
setlocal enabledelayedexpansion
chcp 65001 >nul
title 凯伊密码管家 - 调试模式
echo ============================================
echo   * 凯伊密码管家 - 开发调试模式
echo ============================================
echo.
echo  正在启动 Tauri 开发服务器...
echo  将打开前端热重载 + Rust 后端
echo  按 Ctrl+C 停止
echo.
echo  前端端口: http://localhost:5173
echo.
echo ============================================
echo.

cd /d "%~dp0"

:: 检查 node_modules 是否存在
if not exist "node_modules\" (
    echo [首次运行] 正在安装前端依赖...
    npm install
    if !errorlevel! neq 0 (
        echo.
        echo [错误] npm install 失败，请检查 Node.js 是否已安装
        pause
        exit /b 1
    )
    echo.
)

:: 检查 Tauri CLI 是否可用
npx tauri --version >nul 2>&1
if !errorlevel! neq 0 (
    echo [错误] Tauri CLI 不可用，尝试安装...
    npm install
)

echo [启动] 正在运行 npm run tauri dev...
echo.
npx tauri dev

if !errorlevel! neq 0 (
    echo.
    echo [错误] 程序异常退出 (错误码: !errorlevel!)
    pause
)
