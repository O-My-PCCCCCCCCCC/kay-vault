@echo off
chcp 65001 >nul
title 凯伊密码管家 - 部署工具
cd /d "%~dp0"
echo ============================================
echo   🔑 凯伊密码管家 · 部署工具
echo ============================================
echo.
echo  正在请求管理员权限...
echo.
powershell -Command "Start-Process PowerShell -Verb RunAs -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File \"%~dp0deploy.ps1\"'"
pause
