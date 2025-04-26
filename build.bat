@echo off
rustc --cfg day%1 --cfg %2 -o AoC2019.exe src/main.rs
