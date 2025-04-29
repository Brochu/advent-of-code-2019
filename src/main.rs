mod intcode;
mod days;

use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("[AoC2019] usage: AoC2019 <day_num> <example_data>");
        process::exit(1);
    }
    let day: i32 = args.get(1).map_or(0, |s| s.parse().unwrap_or_default());

    println!("[AoC2019] Solving puzzle for day {}", day);
    match day {
        1 => days::day01::solve(),
        2 => days::day02::solve(),
        3 => days::day03::solve(),
        4 => days::day04::solve(),
        5 => days::day05::solve(),
        6 => days::day06::solve(),
        7 => days::day07::solve(),
        _ => {
            println!("[AoC2019] invalid day_num: {}", args.get(1).unwrap())
        },
    };

    process::exit(0);
}
