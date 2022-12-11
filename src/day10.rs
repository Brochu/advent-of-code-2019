#[derive(Debug)]
enum State {
    Empty,
    Asteroid,
}

fn display_state(state: &State) -> char {
    match state {
        State::Empty => '.',
        State::Asteroid => '#',
    }
}

#[derive(Debug)]
struct Map {
    n: usize,
    states: Vec<State>,
}

fn print_map(map: &Map) {
    map.states[..]
        .chunks(map.n)
        .for_each(|st| {
            st.iter().for_each(|s| print!("{:?}", display_state(s)));
            println!();
        });

    println!();
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

            //match st {
            //    State::Empty => None,
            //    State::Asteroid => Some(st),
            //}
fn run_part1(map: &Map) -> u64 {
    print_map(&map);

    let asteroids = map.states.iter()
        .enumerate()
        .filter_map(|(idx, st)| {
            match st {
                State::Empty => None,
                State::Asteroid => Some((idx, idx_to_coords(&map, idx))),
            }
        })
        .collect::<Vec<(usize, (i64, i64))>>();

    let checks = asteroids.iter()
        .flat_map(|(i0, a0)| {
            asteroids.iter().filter_map(move |(i1, a1)| {
                if i0 != i1 {
                    Some((a0, a1))
                }
                else {
                    None
                }
            })
        });

    checks.for_each(|(a0, a1)| {
        println!("{:?} vs. {:?}", a0, a1);
    });
    return 0;
}

//fn run_part2(map: &Map) -> u64 {
//    return 0;
//}
