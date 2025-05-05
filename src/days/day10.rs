use std::collections::{ HashSet, HashMap };
use std::cmp::Eq;

#[derive(Debug, Hash, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Eq for Asteroid { }

#[derive(Debug)]
struct IVec {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    ast: Asteroid,
    quad: i32,
    div: f64,
}

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day10.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day10.input");
    //println!("{}", input);

    let mut set = HashSet::<Asteroid>::new();
    for (i, line) in input.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c == '#' { set.insert(Asteroid{ x: j as i32, y: i as i32 }); }
        }
    }
    /*
    for ast in set.iter() {
        println!("    {:?}", ast);
    }
    */

    let mut visible = HashMap::<&Asteroid, Vec<&Asteroid>>::new();
    for (i, a) in set.iter().enumerate() {
        //println!("[{}] {:?}", i, a);
        for (j, other) in set.iter().enumerate() {
            if i == j { continue; }
            //println!("    - [{}] {:?}", j, other);

            let vec = get_direction(a, other);
            if check_los(&set, &a, &other, vec) {
                if let Some(list) = visible.get_mut(a) {
                    list.push(other);
                } else {
                    visible.insert(a, vec![other]);
                }
            }
        }
    }

    /*
    for (k, v) in visible.iter() {
        println!("    {:?} -> {}", k, v);
    }
    */
    let (station, p1) = visible.iter()
        .fold((None, 0), |res, (&ast, list)| {
            if list.len() > res.1 {
                (Some(ast), list.len())
            } else {
                res
            }
        });
    println!("    Part 1 = {}", p1);

    let station = station.unwrap();
    //println!("    station {:?}", station);

    let others = visible.get(station).unwrap();
    let mut sorted = Vec::<Node>::new();
    for other in others.iter() {
        //TODO: Make sure to verify I understand this
        let (dx, dy) = (other.x - station.x, other.y - station.y);
        let quad = get_quadrant(dx, -dy);

        let div = dy as f64 / dx as f64;
        sorted.push(Node { ast: Asteroid { x: other.x, y: other.y }, quad, div });
    }
    sorted.sort_unstable_by(|a, b| {
        a.quad.cmp(&b.quad).then(a.div.total_cmp(&b.div))
    });
    /*
    for (i, elem) in sorted.iter().enumerate() {
        println!("    - [{}] {:?}", i+1, elem);
    }
    */
    let target = &sorted[199];
    let (tx, ty) = (target.ast.x, target.ast.y);
    println!("    Part 2 = {}", tx * 100 + ty);
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a.abs();
}

fn get_direction(this: &Asteroid, other: &Asteroid) -> IVec {
    let diffx = other.x - this.x;
    let diffy = other.y - this.y;
    //println!("    [DIR] ({}, {})", diffx, diffy);

    let div = gcd(diffx, diffy);
    //println!("    [DIR] [{}]({}, {})", div, diffx/div, diffy/div);
    return IVec { x: diffx / div, y: diffy / div };
}

fn check_los(set: &HashSet<Asteroid>, from: &Asteroid, to: &Asteroid, dir: IVec) -> bool {
    //println!("    [LOS] {:?} -> {:?} : {:?}", from, to, dir);
    let (x, y) = (dir.x, dir.y);
    let (mut cx, mut cy) = (from.x + x, from.y + y);

    let (tx, ty) = (to.x, to.y);
    while (cx, cy) != (tx, ty) {
        let cast = Asteroid { x: cx, y: cy };
        if set.contains(&cast) {
            return false
        }

        (cx, cy) = (cx + x, cy + y);
    }

    return true;
}

fn get_quadrant(dx: i32, dy: i32) -> i32 {
    if dx >= 0 && dy >= 0 {
        return 0
    }
    else if dx >= 0 && dy < 0 {
        return 1;
    }
    else if dx < 0 && dy < 0 {
        return 2;
    }
    else if dx < 0 && dy >= 0 {
        return 3;
    }
    else {
        return -1;
    }
}
