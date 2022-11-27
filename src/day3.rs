use std::collections::HashMap;

fn main() {
    println!("Part 1 -> {}", run_part1());
    //println!("Part 2 -> {}", run_part2());
}

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
    println!("{:?}", moves);
    const SIZE:i64 = 500;
    let mut pos = 0;

    for m in moves {
        let (min, max, step) = match m {
            Move::Up(i) => (SIZE, i*SIZE, SIZE),
            Move::Down(i) => (-i*SIZE, -SIZE, SIZE),
            Move::Left(i) => (-i, -1, 1),
            Move::Right(i) => (1, i*1, 1),
        };

        for i in (min..max).step_by(step as usize) {

            let mut num = 0;
            if coords.contains_key(&(pos + i)) {
                num = *coords.get(&(pos + i)).unwrap();
            }

            coords.insert(pos+i, num+1);
        }

        if min.abs() > max.abs() {
            pos += min;
        }
        if max.abs() > min.abs() {
            pos += max;
        }
    }
}

fn to_coords(num: i64) -> (i64, i64) {
    return (num/500, num%500);
}

fn run_part1() -> i64 {
    let mut coords: HashMap<i64, i64> = HashMap::new();

    let wires: Vec<Vec<Move>> = include_str!("../data/day3.example")
        .lines()
        .map(|line| line.split(",").map(create_move).collect())
        .collect();

    for w in wires {
        draw_wire(&mut coords, &w);
    }

    let cross: Vec<(i64, i64)> = coords.into_iter()
        .filter(|e| e.1 > 1)
        .map(|e| to_coords(e.0.abs()))
        .collect();
    println!("{:?}", cross);

    //TODO:  Calculate manathan distances

    return 0;
}

//fn run_part2() -> i64 {
//    return 0;
//}
