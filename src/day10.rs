use std::{fmt::{Display, Pointer}, cmp::min, cmp::max};

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
    fn get(&self, x: i32, y: i32) -> &Node {
        let size: i32 = self.n.try_into().unwrap();
        let idx: usize = ((y * size) + x).try_into().unwrap();
        return &self.nodes[idx];
    }
    fn coords(&self, idx: usize) -> (i32, i32) {
        let x: i32 = (idx % self.n).try_into().unwrap();
        let y: i32 = (idx / self.n).try_into().unwrap();
        return (x, y);
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

fn gcd(a: i32, b: i32) -> i32 {
    let mut res = min(a.abs(), b.abs());
    if res == 0 {
        // Handle if one value is 0
        return max(a.abs(), b.abs());
    }

    while res > 0 {
        if a % res == 0 && b % res == 0 { break; }
        res -= 1;
    }

    return res.abs();
}

fn check_los(map: &Map, start: (i32, i32), end: (i32, i32)) -> bool {
    let (sx, sy) = start;
    let (ex, ey) = end;
    //println!("[LOS] from ({},{}), end ({},{})", sx, sy, ex, ey);
    let (dx, dy) = (ex - sx, ey - sy);
    //println!(" - dx/dy ({},{})", dx, dy);
    let div = gcd(dx, dy);
    //println!(" - div {}", div);
    let (xd, yd) = (dx/div, dy/div);
    //println!(" - xd,yd ({},{})", xd, yd);

    if div <= 1 { return true; }
    for i in 1..div {
        let (xp, yp) = ((sx + (i * xd)), (sy + (i * yd)));
        let c = map.get(xp, yp);
        //println!(" - ({},{}), {}", xp, yp, c);

        if let Node::Asteroid = c {
            return false;
        }
    }

    return true;
}

fn run_part1(map: &Map) -> usize {
    //for y in 0..5 {
    //    for x in 0..5 {
    //        println!(" - ({}, {}) -> {}", x, y, map.get(x, y));
    //    }
    //}
    //for i in 0..map.n * map.n {
    //    let (x, y) = map.coords(i);
    //    println!(" - {} -> ({}, {})", i, x, y);
    //}

    map.asteroids[..].iter().map(|a| {
        map.asteroids.iter().filter(|&a1| {
            if a == a1 { return false; }

            let res = check_los(map, map.coords(*a), map.coords(*a1));
            //println!("[LOS] res = {}", res);
            res
        })
        .count()
    })
    .inspect(|c| println!(" - {}", c))
    .fold(0, |m, count| {
            max(m, count)
        })
}

//fn run_part2(map: &Map) -> u64 {
//    return 0;
//}
