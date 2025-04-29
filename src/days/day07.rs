use std::cmp::max;
use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day7.example");
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
    let mut phases = vec![4, 3, 2, 1, 0];
    let mut all_checks: Vec<Vec<i64>> = vec![];
    permutations(&mut phases, 0, &mut all_checks);

    let mut max_signal = 0;
    for phase in all_checks {
        let mut signal = 0;
        for i in 0..phases.len() {
            let mut prog = intcode::fork_program(&memory);
            intcode::send_input(&mut prog, phase[i]);
            intcode::send_input(&mut prog, signal);

            intcode::run_program(&mut prog);
            signal = intcode::pop_output(&mut prog);
        }
        max_signal = max(max_signal, signal);
    }

    println!("    final signal = {}", max_signal);
}
