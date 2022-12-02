use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    println!("Part 1 -> {}", run_part1());
    println!("Part 2 -> {}", run_part2());
}

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

fn build_wire(moves: &[Move]) -> (HashSet<(i64, i64)>, HashMap<(i64, i64), i64>) {
    let mut coords: HashSet<(i64, i64)> = HashSet::new();
    let mut maps: HashMap<(i64, i64), i64> = HashMap::new();

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut travel: i64 = 0;

    for m in moves {
        let (vec, dist) = match m {
            Move::Up(i) => ((0, 1), *i),
            Move::Down(i) => ((0, -1), *i),
            Move::Left(i) => ((-1, 0), *i),
            Move::Right(i) => ((1, 0), *i),
        };

        for _ in 1..=dist {
            travel += 1; // Keep track of the distance for each coords reached

            x += vec.0;
            y += vec.1;

            coords.insert((x, y));
            maps.insert((x, y), travel);
        }
    }

    return (coords, maps);
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

    let (coords1, _) = build_wire(&wire1);
    let (coords2, _) = build_wire(&wire2);

    let res = coords1.intersection(&coords2)
        .map(|c| c.0.abs() + c.1.abs())
        .min().unwrap();

    return res;
}

fn run_part2() -> i64 {
    let mut file = include_str!("../data/day3.input").lines();
    let wire1: Vec<Move> = file.next()
        .unwrap()
        .split(",")
        .map(create_move).collect();

    let wire2: Vec<Move> = file.next()
        .unwrap()
        .split(",")
        .map(create_move).collect();

    let (coords1, map1) = build_wire(&wire1);
    let (coords2, map2) = build_wire(&wire2);

    let best = coords1.intersection(&coords2)
        .map(|i| (i, map1.get(i).unwrap(), map2.get(i).unwrap()))
        .min_by(|f, s| (f.1 + f.2).cmp(&(s.1 + s.2))).unwrap();

    return best.1 + best.2;
}
