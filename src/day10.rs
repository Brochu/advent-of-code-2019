use std::collections::HashSet;

#[derive(Debug)]
enum State {
    Empty,
    Asteroid,
}

#[derive(Debug)]
struct Map {
    n: usize,
    states: Vec<State>,
}

fn idx_to_coords(map: &Map, idx: usize) -> (i64, i64) {
    return ((idx / map.n) as i64, (idx % map.n) as i64);
}

fn parse_input() -> Map {
    let mut size = 0;
    let map_states = include_str!("../data/day10.example")
        .lines()
        .flat_map(|l| {
            size = l.len(); // Store the length of one line, to switch between idx and coords later
            l.chars()
        })
        .map(|c| {
            match c {
                '.' => State::Empty,
                '#' => State::Asteroid,
                _ => panic!("[Day10] Could not parse map state"),
            }
        })
        .collect::<Vec<State>>();

    return Map { n: size, states: map_states };
}

fn find_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn main() {
    let map = parse_input();

    println!("[Day10] part 1 = {}", run_part1(&map));
    //println!("[Day10] part 2 = {}", run_part2(&map));
}

fn run_part1(map: &Map) -> u64 {
    let coords = map.states.iter()
        .enumerate()
        .filter_map(|st| {
            match st.1 {
                State::Empty => None,
                State::Asteroid => Some(st),
            }
        })
        .map(|st| idx_to_coords(&map, st.0))
        .collect::<Vec<(i64, i64)>>();

    println!("{:?}", coords);
    return 0;
}

//fn run_part2(map: &Map) -> u64 {
//    return 0;
//}
