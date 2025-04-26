#[cfg(day1)] mod day1; #[cfg(day1)] use day1::solve;
#[cfg(day2)] mod day2; #[cfg(day2)] use day2::solve;
#[cfg(day3)] mod day3; #[cfg(day3)] use day3::solve;
#[cfg(day4)] mod day4; #[cfg(day4)] use day4::solve;
#[cfg(day5)] mod day5; #[cfg(day5)] use day5::solve;
#[cfg(day6)] mod day6; #[cfg(day6)] use day6::solve;
#[cfg(day7)] mod day7; #[cfg(day7)] use day7::solve;
#[cfg(day8)] mod day8; #[cfg(day8)] use day8::solve;
#[cfg(day9)] mod day9; #[cfg(day9)] use day9::solve;
#[cfg(day10)] mod day10; #[cfg(day10)] use day10::solve;
#[cfg(day11)] mod day11; #[cfg(day11)] use day11::solve;
#[cfg(day12)] mod day12; #[cfg(day12)] use day12::solve;
#[cfg(day13)] mod day13; #[cfg(day13)] use day13::solve;
#[cfg(day14)] mod day14; #[cfg(day14)] use day14::solve;
#[cfg(day15)] mod day15; #[cfg(day15)] use day15::solve;
#[cfg(day16)] mod day16; #[cfg(day16)] use day16::solve;
#[cfg(day17)] mod day17; #[cfg(day17)] use day17::solve;
#[cfg(day18)] mod day18; #[cfg(day18)] use day18::solve;
#[cfg(day19)] mod day19; #[cfg(day19)] use day19::solve;
#[cfg(day20)] mod day20; #[cfg(day20)] use day20::solve;
#[cfg(day21)] mod day21; #[cfg(day21)] use day21::solve;
#[cfg(day22)] mod day22; #[cfg(day22)] use day22::solve;
#[cfg(day23)] mod day23; #[cfg(day23)] use day23::solve;
#[cfg(day24)] mod day24; #[cfg(day24)] use day24::solve;
#[cfg(day25)] mod day25; #[cfg(day25)] use day25::solve;

mod intcode;

fn main() {
    println!("Trying something {}", 10);
    intcode::init();
    solve();
}
