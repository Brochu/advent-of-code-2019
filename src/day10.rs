#[derive(Debug)]
enum State {
    Empty,
    Asteroid,
}

fn to_short(state: &State) -> char {
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

impl Map {
    fn debug_print(&self) {
        self.states[..].chunks(self.n)
            .for_each(|c| {
                c.iter().for_each(|s| print!("{:?}", to_short(s)));
                println!();
            });
    }
}

fn idx_to_coords(map: &Map, idx: usize) -> (usize, usize) {
    return (idx / map.n, idx % map.n);
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

fn main() {
    let map = parse_input();

    println!("[Day10] part 1 = {}", run_part1(&map));
    //println!("[Day10] part 2 = {}", run_part2(&map));
}

fn run_part1(map: &Map) -> u64 {
    map.debug_print();

    println!("TEST: {}", f32::INFINITY == f32::NEG_INFINITY);

    let coords = map.states.iter()
        .enumerate()
        .filter_map(|st| {
            match st.1 {
                State::Empty => None,
                State::Asteroid => Some(st),
            }
        })
        .map(|st| idx_to_coords(&map, st.0))
        .collect::<Vec<(usize, usize)>>();

    let cross = coords.iter().flat_map(|&c0| {
        coords.iter().filter_map(move |&c1| {
            if c0 == c1 { 
                None
            }
            else {
                Some((c0, c1))
            }
        })
    });

    cross.for_each(|(coord0, coord1)| println!("{:?} -> {:?}", coord0, coord1));

    //TODO: Need to implement a find_gcd function
    return 0;
}

//fn run_part2(map: &Map) -> u64 {
//    return 0;
//}
