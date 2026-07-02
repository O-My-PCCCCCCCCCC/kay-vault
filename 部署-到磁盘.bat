@echo off
chcp 65001 >nul
cd /d "%~dp0"
powershell -WindowStyle Hidden -NoProfile -ExecutionPolicy Bypass -Command "Start-Process PowerShell -Verb RunAs -WindowStyle Normal -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File \"%~dp0deploy.ps1\"'"
