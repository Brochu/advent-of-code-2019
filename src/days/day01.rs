use std::cmp::max;

pub fn solve(example: bool) {
    let input = if example {
        include_str!("../../data/day1.example")
    } else {
        include_str!("../../data/day1.input")
    };

    let nums: Vec<_> = input.lines()
        .map(|s| s.parse::<i32>().unwrap() )
        .collect();

    let p1: i32 = nums.iter()
        .map(|i| (i/3) - 2 )
        .sum();
    let p2 : i32 = nums.iter()
        .fold(0, |acc, n| {
            let mut temp = *n;
            let mut total = 0;

            while temp > 0 {
                temp = max((temp/3) - 2, 0);
                total += temp;
            }
            acc + total
        });

    println!("    Part 1 -> {}", p1);
    println!("    Part 2 -> {}", p2);
}
