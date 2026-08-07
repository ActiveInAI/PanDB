@echo off
setlocal
set "DBX_STATIC_DIR=%~dp0dist"
set "DBX_DATA_DIR=%~dp0data"
if not defined DBX_PORT set "DBX_PORT=4224"
if not defined DBX_PASSWORD set "DBX_PASSWORD=change-this-password"
cd /d "%~dp0"
bin\dbx-web.exe
