use std::collections::HashMap;

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
    return match &elem[0..1] {
        "U" => Move::Up(elem[1..].parse().unwrap()),
        "D" => Move::Down(elem[1..].parse().unwrap()),
        "L" => Move::Left(elem[1..].parse().unwrap()),
        "R" => Move::Right(elem[1..].parse().unwrap()),
        _ => panic!(),
    }
}

fn draw_wire(coords: &mut HashMap<i64, i64>, moves: &[Move]) {
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

            let i = coords_to_idx(x, y);

            let mut num = 0;
            if coords.contains_key(&i) {
                num = *coords.get(&i).unwrap();
            }
            num += 1;
            coords.insert(i, num);
        }
        //println!("({}, {}) : {}", x, y, coords_to_idx(x, y));
    }
}

fn idx_to_coords(num: i64) -> (i64, i64) {
    return (num%SIZE, num/SIZE);
}

fn coords_to_idx(x: i64, y: i64) -> i64 {
    return (y * SIZE) + x;
}

fn run_part1() -> i64 {
    let mut coords: HashMap<i64, i64> = HashMap::new();
    let wires: Vec<Vec<Move>> = include_str!("../data/day3.input")
        .lines()
        .map(|line| line.split(",").map(create_move).collect())
        .collect();

    for w in wires {
        println!("{:?}", w);
        draw_wire(&mut coords, &w);
    }

    let closest = coords.into_iter()
        .filter(|e| e.1 > 1)
        .map(|e| e.0.abs())
        .min().unwrap();

    let c = idx_to_coords(closest);
    return c.0 + c.1;
}

//fn run_part2() -> i64 {
//    return 0;
//}
