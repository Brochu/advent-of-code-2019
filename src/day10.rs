use std::fmt::Display;

enum Node {
    Empty,
    Asteroid,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Node::Empty => ".",
            Node::Asteroid => "#",
        };
        write!(f, "{}", char)
    }
}

struct Map {
    n: usize,
    nodes: Vec<Node>,
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.nodes[..].chunks(self.n)
            .fold(String::new(), |acc, c| {
                let line = c.iter().fold(String::new(), |l, n| {
                    let char = match n {
                        Node::Empty => ".",
                        Node::Asteroid => "#",
                    };
                    format!("{}{}", l, char)
                });
                format!("{}\n{}", acc, line)
            });
        writeln!(f, "{}", output)
    }
}

fn parse_input() -> Map {
    let input = include_str!("../data/day10.example");
    return Map { n: 0, nodes: vec![] };
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
