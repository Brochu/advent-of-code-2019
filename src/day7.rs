fn main() {
    let part1 = run_part1();
    println!("[Day7] Part 1 = {}", part1);
}

fn run_part1() -> i64 {
    let mem: Vec<i64> = include_str!("../data/day7.input")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    println!("{:?}", mem);
    return 0;
}
