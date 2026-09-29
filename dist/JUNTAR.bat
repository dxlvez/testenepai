@echo off
rem Junta as partes do jogo num unico RedThread.exe (o GitHub limita arquivos a 100 MB)
cd /d "%~dp0"
copy /b RedThread.exe.part1+RedThread.exe.part2+RedThread.exe.part3+RedThread.exe.part4 RedThread.exe >nul
echo Pronto! Agora abra RedThread.exe
pause
