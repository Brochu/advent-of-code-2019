pub fn solve() {
    #[cfg(EXAMPLE)] let input = include_str!("../data/day1.example");
    #[cfg(FULL)] let input = include_str!("../data/day1.input");

    let p1: i32 = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap() )
        .map(|i| (i/3) - 2 )
        .sum();
    println!("[D1] Part 1 -> {}", p1);
}
