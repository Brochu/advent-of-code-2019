use std::cmp::max;
use crate::intcode;

pub fn solve() {
    //#[cfg(ex)] let input = include_str!("../../data/day7.example");
    #[cfg(ex)] let input = include_str!("../../data/day7.2.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day7.input");

    let memory: Vec<_> = input.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    fn permutations<T: Clone>(items: &mut Vec<T>, start: usize, result: &mut Vec<Vec<T>>) {
        if start == items.len() {
            result.push(items.clone());
        } else {
            for i in start..items.len() {
                items.swap(start, i);
                permutations(items, start + 1, result);
                items.swap(start, i); // backtrack
            }
        }
    }
    /*
    let mut phases = vec![4, 3, 2, 1, 0];
    let mut all_checks: Vec<Vec<i64>> = vec![];
    permutations(&mut phases, 0, &mut all_checks);

    let mut p1_max = 0;
    for phase in all_checks.iter() {
        let mut signal = 0;
        for i in 0..phases.len() {
            let mut prog = intcode::fork_program(&memory, 100);
            intcode::send_input(&mut prog, phase[i]);
            intcode::send_input(&mut prog, signal);

            intcode::run_program(&mut prog);
            signal = intcode::pop_output(&mut prog);
        }
        p1_max = max(p1_max, signal);
    }

    println!("    Part 1 = {}", p1_max);
    */

    let mut phases = vec![9, 8, 7, 6, 5];
    let mut all_checks: Vec<Vec<i64>> = vec![];
    permutations(&mut phases, 0, &mut all_checks);

    /*
    let mut ap = intcode::fork_program(&memory, 100);
    intcode::send_input(&mut ap, 5);

    let mut signal = 0;
    loop {
        intcode::send_input(&mut ap, signal);
        match intcode::run_program(&mut ap) {
            intcode::Status::Halted => {
                println!("    HALTED");
                break;
            },
            intcode::Status::Output => {
                signal = intcode::pop_output(&mut ap);
                println!("    New Output: {}", signal);
            },
        }
    }
    */

    let mut p2_max = 0;
    for phase in all_checks.iter() {
        let mut programs = Vec::<intcode::Program>::new();
        for &val in phase {
            let mut p = intcode::fork_program(&memory, 0);
            intcode::send_input(&mut p, val);
            programs.push(p);
        }

        let mut signal = 0;
        'main: loop {
            for p in &mut programs {
                intcode::send_input(p, signal);
                if let intcode::Status::Output = intcode::run_program(p) {
                    signal = intcode::pop_output(p);
                } else {
                    break 'main;
                }
            }
        }
        p2_max = max(p2_max, signal);
    }

    println!("    Part 2 = {}", p2_max);
}
