use crate::intcode;

use std::{collections::HashSet, fmt::Display};

enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Eq for Pos {}

struct Robot {
    pos: Pos,
    dir: Dir,
}
impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self.dir {
            Dir::Up => '^',
            Dir::Left => '<',
            Dir::Down => 'v',
            Dir::Right => '>',
        };
        write!(f, "    [{}] - {}", d, self.pos)
    }
}

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day11.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day11.input");

    let mut robot = Robot { pos: Pos { x: 0, y: 0}, dir: Dir::Up };
    let mut painted = HashSet::<Pos>::new();
    let mut uniques = HashSet::<Pos>::new();

    let memory: Vec<_> = input.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut prog = intcode::fork_program(&memory, 1109);
    intcode::send_input(&mut prog, 1);

    loop {

        if let intcode::Status::Halted = intcode::run_program(&mut prog) {
            break;
        }
        if let intcode::Status::Halted = intcode::run_program(&mut prog) {
            break;
        }
        let color = intcode::pop_output(&mut prog);
        let direction = intcode::pop_output(&mut prog);
        //println!("    color: {}; dir: {}", color, direction);

        match (color, painted.contains(&robot.pos)) {
            (0, true) => {
                painted.remove(&robot.pos);
            },
            (1, false) => {
                painted.insert(Pos { x: robot.pos.x, y: robot.pos.y });
            },
            _ => {}
        }
        uniques.insert(Pos { x: robot.pos.x, y: robot.pos.y });

        turn_robot(&mut robot, direction as i32);
        move_robot(&mut robot);

        let under = if painted.contains(&robot.pos) {
            1
        } else {
            0
        };
        //println!("    sent: {}", under);
        intcode::send_input(&mut prog, under);
    }

    for y in -5..=10 {
        for x in -10..=50 {
            let pos = Pos { x, y };
            match painted.contains(&pos) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!("");
    }
}

fn turn_robot(robot: &mut Robot, input: i32) {
    robot.dir = match (input, &robot.dir) {
        (0, Dir::Up) | (1, Dir::Down) => {
            Dir::Left
        },
        (0, Dir::Left) | (1, Dir::Right) => {
            Dir::Down
        },
        (0, Dir::Down) | (1, Dir::Up) => {
            Dir::Right
        },
        (0, Dir::Right) | (1, Dir::Left) => {
            Dir::Up
        },
        _ => panic!("    [TURN] Cannot handle turn input")
    };
}

fn move_robot(robot: &mut Robot) {
    match robot.dir {
        Dir::Up => robot.pos.y -= 1,
        Dir::Left => robot.pos.x -= 1,
        Dir::Down => robot.pos.y += 1,
        Dir::Right => robot.pos.x += 1,
    };
}
