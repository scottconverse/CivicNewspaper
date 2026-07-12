@echo off
powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%~dp0sign-windows-artifact.ps1" -File "%~1"
exit /b %ERRORLEVEL%
