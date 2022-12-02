fn main() {
    let part1 = run_part1();
    println!("[Day7] Part 1 = {}", part1);
}

fn run_part1() -> i64 {
    let mem: Vec<i64> = include_str!("../data/day7.example")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    //TODO: I DO NOT KNOW HOW TO QUEUE SOME DATA IN STDIN
    // HOW DO I PROGRAMMATICALLY SEND DATA TO STDIN

    return 0;
}
