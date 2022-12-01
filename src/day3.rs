use std::collections::HashSet;

fn main() {
    println!("Part 1 -> {}", run_part1());
    //println!("Part 2 -> {}", run_part2());
}

const SIZE:i64 = 1_000_000;

#[derive(Debug)]
enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

fn create_move(elem: &str) -> Move {
    let num: i64 = elem[1..].parse().unwrap();

    return match &elem[0..1] {
        "U" => Move::Up(num),
        "D" => Move::Down(num),
        "L" => Move::Left(num),
        "R" => Move::Right(num),
        _ => panic!(),
    }
}

fn build_wire(moves: &[Move]) -> HashSet<i64> {
    let mut coords: HashSet<i64> = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for m in moves {
        let (vec, dist) = match m {
            Move::Up(i) => ((0, 1), *i),
            Move::Down(i) => ((0, -1), *i),
            Move::Left(i) => ((-1, 0), *i),
            Move::Right(i) => ((1, 0), *i),
        };

        for _ in 1..=dist {
            x += vec.0;
            y += vec.1;

            coords.insert(coords_to_idx(x, y));
        }
    }

    return coords;
}

fn idx_to_coords(num: &i64) -> (i64, i64) {
    return (num%SIZE, num/SIZE);
}

fn coords_to_idx(x: i64, y: i64) -> i64 {
    return (y * SIZE) + x;
}

fn run_part1() -> i64 {
    let mut file = include_str!("../data/day3.input").lines();
    let wire1: Vec<Move> = file.next()
        .unwrap()
        .split(",")
        .map(create_move).collect();

    let wire2: Vec<Move> = file.next()
        .unwrap()
        .split(",")
        .map(create_move).collect();

    let coords1 = build_wire(&wire1);
    let coords2 = build_wire(&wire2);

    let res = coords1.intersection(&coords2)
        .map(|i| idx_to_coords(i))
        .map(|c| c.0.abs() + c.1.abs())
        .min().unwrap();

    return res;
}

//fn run_part2() -> i64 {
//    return 0;
//}
