@echo off
REM Config block: single build variant ? self-contained wallbang.exe with the DLL embedded.
setlocal EnableExtensions
cd /d "%~dp0"
set "TMP=%~dp0target\tmp"
set "TEMP=%TMP%"
if not exist "%TEMP%" mkdir "%TEMP%" >nul
if exist dist rmdir /s /q dist
mkdir dist >nul

echo [1/2] Building wallbang.dll...
cargo build --release -p wallbang || exit /b 1

echo [2/2] Building wallbang.exe (DLL embedded)...
cargo build --release -p wallbang_loader --bin wallbang_loader || exit /b 1

copy /y target\release\wallbang.dll dist\wallbang.dll >nul
copy /y target\release\wallbang_loader.exe dist\wallbang.exe >nul
if not exist dist\wallbang.exe (
  echo BUILD FAILED: dist\wallbang.exe missing
  exit /b 1
)
echo Built: dist\wallbang.exe

set "DESK=%USERPROFILE%\OneDrive\Desktop"
if not exist "%DESK%" set "DESK=%USERPROFILE%\Desktop"
if exist "%DESK%" (
  REM Old variants are obsolete: only wallbang.exe remains.
  del /q "%DESK%\wallbang.dll" 2>nul
  del /q "%DESK%\wallbang_gui.exe" 2>nul
  del /q "%DESK%\wallbang_loader.exe" 2>nul
  del /q "%DESK%\wallbang_packed.exe" 2>nul
  del /q "%DESK%\wallbang_loader_packed.exe" 2>nul
  del /q "%DESK%\RUN_WALLBANG.bat" 2>nul
  copy /y dist\wallbang.exe "%DESK%\wallbang.exe" >nul
  echo Copied to %DESK%\wallbang.exe
)
endlocal
exit /b 0
