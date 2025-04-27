use std::fmt::Debug;
use std::collections::HashSet;
use std::cmp::min;


#[derive(Debug)]
enum Step {
    Up(i32),
    Left(i32),
    Right(i32),
    Down(i32),
}

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day3.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day3.input");

    let lines: Vec<_> = input.lines().collect();
    let first_wire: Vec<_> = lines[0].split(",")
        .map(|s| parse_steps(s))
        .collect();
    let second_wire: Vec<_> = lines[1].split(",")
        .map(|s| parse_steps(s))
        .collect();

    println!("    1st wire: {:?}", first_wire);
    println!("    2nd wire: {:?}", second_wire);


    let fv = visit_wire(&first_wire);
    let sv = visit_wire(&second_wire);
    let uni = fv.intersection(&sv);
    let res = uni.fold(i32::MAX, |acc, (x, y)| {
        min(x.abs() + y.abs(), acc)
    });

    println!("    MIN: {}", res);
}

fn parse_steps(desc: &str) -> Step {
    let val = desc[1..].parse::<i32>().unwrap();
    match desc.chars().nth(0).unwrap() {
        'U' => Step::Up(val),
        'L' => Step::Left(val),
        'R' => Step::Right(val),
        'D' => Step::Down(val),
        _ => unimplemented!()
    }
}

fn visit_wire(wire: &Vec<Step>) -> HashSet<(i32, i32)> {
    let mut first_visit = HashSet::<(i32, i32)>::new();
    let mut pos = (0, 0);
    for step in wire {
        match &step {
            Step::Up(val) => {
                for y in 1..=*val {
                    first_visit.insert((pos.0, pos.1 - y));
                }
                pos.1 -= val;
            },
            Step::Left(val) => {
                for x in 1..=*val {
                    first_visit.insert((pos.0 - x, pos.1));
                }
                pos.0 -= val;
            },
            Step::Right(val) => {
                for x in 1..=*val {
                    first_visit.insert((pos.0 + x, pos.1));
                }
                pos.0 += val;
            },
            Step::Down(val) => {
                for y in 1..=*val {
                    first_visit.insert((pos.0, pos.1 + y));
                }
                pos.1 += val;
            },
        }
    }

    return first_visit;
}
