use std::collections::HashSet;
use std::cmp::Eq;

#[derive(Debug, Hash, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Eq for Asteroid { }

pub fn solve() {
    //#[cfg(ex)] let input = include_str!("../../data/day10.example");
    //#[cfg(not(ex))] let input = include_str!("../../data/day10.input");
    let input = ".#..#
.....
#####
....#
...##
";
    let dim = input.trim().split('\n').count() as i32;
    println!("[{}]\n{}", dim, input);

    let set = input.trim().replace("\n", "").chars()
        .enumerate()
        .filter(|(_, n)| *n == '#')
        .fold(HashSet::<Asteroid>::new(), |mut acc, (i, _)| {
            let i = i as i32;
            acc.insert(Asteroid{ x: i % dim, y: i / dim });
            acc
        });
    println!("{:?}", set);

    let target = Asteroid{ x: 1, y: 0 };
    let check = set.contains(&target);
    println!("{:?} -> {}", target, check);
}

fn _gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a.abs();
}
