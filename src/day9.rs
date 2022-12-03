use shared::intcode_run;

fn main() {
    let mut program = include_str!("../data/day9.input")
        .split(",")
        .map( |val| val.trim().parse().unwrap() )
        .collect::<Vec<i64>>();
    program.resize(100_000, 0);

    intcode_run(&mut program);

    println!("[Day9] part 1 = {}", run_part1(&program));
    println!("[Day9] part 2 = {}", run_part2(&program));
}

fn run_part1(program: &Vec<i64>) -> u64 {
    return program.len() as u64;
}

fn run_part2(program: &Vec<i64>) -> u64 {
    return program.len() as u64;
}
