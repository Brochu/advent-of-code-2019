@echo off

if "%1"=="ex" (
    rustc --cfg ex -o AoC2019.exe src/main.rs
) else (
    rustc -o AoC2019.exe src/main.rs
)

