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

    //println!("    1st wire: {:?}", first_wire);
    //println!("    2nd wire: {:?}", second_wire);


    let fv = visit_wire(&first_wire);
    let sv = visit_wire(&second_wire);
    let inter: Vec<_> = fv.intersection(&sv).collect();
    let p1 = inter.iter().fold(i32::MAX, |acc, (x, y)| {
        min(x.abs() + y.abs(), acc)
    });

    let mut p2 = i32::MAX;
    for coord in inter {
        let len_f = reach_coord(&first_wire, coord);
        let len_s = reach_coord(&second_wire, coord);

        p2 = min(p2, len_f + len_s);
    }

    println!("    Part 1 = {}", p1);
    println!("    Part 2 = {}", p2);
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

fn reach_coord(wire: &Vec<Step>, target: &(i32, i32)) -> i32 {
    let mut pos = (0, 0);
    let mut dist = 0;

    for step in wire {
        match &step {
            Step::Up(val) => {
                let diff = (target.1 - pos.1).abs();
                if pos.0 == target.0 && target.1 < pos.1 && diff <= *val {
                    dist += diff;
                    return dist;
                } else {
                    pos.1 -= val;
                    dist += val;
                }
            },
            Step::Left(val) => {
                let diff = (target.0 - pos.0).abs();
                if pos.1 == target.1 && target.0 < pos.0 && diff <= *val {
                    dist += diff;
                    return dist;
                } else {
                    pos.0 -= val;
                    dist += val;
                }
            },
            Step::Right(val) => {
                let diff = (target.0 - pos.0).abs();
                if pos.1 == target.1 && target.0 > pos.0 && diff <= *val {
                    dist += diff;
                    return dist;
                } else {
                    pos.0 += val;
                    dist += val;
                }
            },
            Step::Down(val) => {
                let diff = (target.1 - pos.1).abs();
                if pos.0 == target.0 && target.1 > pos.1 && diff <= *val {
                    dist += diff;
                    return dist;
                } else {
                    pos.1 += val;
                    dist += val;
                }
            },
        }
    }
    return dist;
}
