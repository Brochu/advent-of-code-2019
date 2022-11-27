fn main() {
    let part1: i64 = include_str!("../data/day1.input")
        .lines()
        .map( |l| l.parse::<i64>().unwrap() )
        .map( |v| (v / 3) - 2 )
        .sum();

    println!("Part 1 -> {:?}", part1);

    let part2: i64 = include_str!("../data/day1.input")
        .lines()
        .map( |l| l.parse::<i64>().unwrap() )
        .map( calc_fuel )
        .sum();

    println!("Part 2 -> {:?}", part2);
}

fn calc_fuel(w: i64) -> i64 {
    let mut total = 0;
    let mut acc: i64 = (w / 3) - 2;

    while acc > 0 {
        total += acc;
        acc = (acc / 3) - 2;
    }

    return total;
}
