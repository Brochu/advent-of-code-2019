use std::fmt::Display;

enum Node {
    Empty,
    Asteroid,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Node::Empty => '.',
            Node::Asteroid => '#',
        };
        write!(f, "{}", c)
    }
}

struct Map {
    n: usize,
    nodes: Vec<Node>,
    asteroids: Vec<usize>,
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
        write!(f, "{}{:?}", output, self.asteroids)
    }
}
impl Map {
    fn get(&self, x: usize, y: usize) -> &Node {
        return &self.nodes[(y * self.n) + x];
    }
    fn coords(&self, idx: usize) -> (usize, usize) {
        return (idx % self.n, idx / self.n);
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
    let asteroids = nodes.iter().enumerate()
        .filter_map(|(i, n)| {
            match n {
                Node::Asteroid => Some(i),
                Node::Empty => None,
            }
        })
        .collect();
    return Map { n, nodes, asteroids };
}

fn main() {
    let map = parse_input();

    println!("[Day10] part 1 = {}", run_part1(&map));
    //println!("[Day10] part 2 = {}", run_part2(&map));
}

fn run_part1(map: &Map) -> u64 {
    //for y in 0..5 {
    //    for x in 0..5 {
    //        println!(" - ({}, {}) -> {}", x, y, map.get(x, y));
    //    }
    //}
    //for i in 0..map.n * map.n {
    //    let (x, y) = map.coords(i);
    //    println!(" - {} -> ({}, {})", i, x, y);
    //}

    for i0 in &map.asteroids[0..1] {
        for i1 in &map.asteroids[..] {
            if i0 == i1 { continue; }
            println!("{} vs. {}", i0, i1);
        }
    }
    return 0;
}

//fn run_part2(map: &Map) -> u64 {
//    return 0;
//}
