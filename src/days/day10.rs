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

    let mut visible = HashMap::<&Asteroid, i32>::new();
    for (i, a) in set.iter().enumerate() {
        //println!("[{}] {:?}", i, a);
        let mut count = 0;

        for (j, other) in set.iter().enumerate() {
            if i == j { continue; }
            //println!("    - [{}] {:?}", j, other);

            let vec = get_direction(a, other);
            if check_los(&set, &a, &other, vec) {
                count += 1;
            }
        }

        visible.insert(a, count);
    }

    /*
    for (k, v) in visible.iter() {
        println!("    {:?} -> {}", k, v);
    }
    */
    let p1 = visible.values().max().unwrap();
    println!("    Part 1 = {}", p1);
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
