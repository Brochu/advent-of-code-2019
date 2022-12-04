use shared::intcode_run;

fn main() {
    let mut program = include_str!("../data/day9.input")
        .split(",")
        .map( |val| val.trim().parse().unwrap() )
        .collect::<Vec<i64>>();
    program.resize(100_000, 0);

    intcode_run(&mut program);
}
