@echo off
set SRC=src\tsp.rs
set OUT=bin\tsp.exe

echo Compiling %SRC%...
if not exist bin mkdir bin
rustc %SRC% -o %OUT%
echo Compilation successful: %OUT%

echo Running program...
%OUT%

