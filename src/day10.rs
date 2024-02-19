use std::fmt::Display;

enum Node {
    Empty,
    Asteroid,
}

struct Map {
    n: usize,
    nodes: Vec<Node>,
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for chunk in self.nodes[..].chunks(self.n) {
            for e in chunk {
                let c = match e {
                    Node::Empty => '.',
                    Node::Asteroid => '#',
                };
                output.push(c);
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

fn parse_input() -> Map {
    let input = include_str!("../data/day10.example");
    let n = input.split_once("\n").unwrap().0.len() - 1;
    let nodes: Vec<Node> = input.chars()
        .filter_map(|c| {
            match c {
                '#' => Some(Node::Asteroid),
                '.' => Some(Node::Empty),
                _   => None,
            }
        })
        .collect();
    return Map { n, nodes };
}

fn main() {
    let map = parse_input();

    println!("[Day10] part 1 = {}", run_part1(&map));
    //println!("[Day10] part 2 = {}", run_part2(&map));
}

fn run_part1(map: &Map) -> u64 {
    println!("{}", map);
    return 0;
}

//fn run_part2(map: &Map) -> u64 {
//    return 0;
//}
