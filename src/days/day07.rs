use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day7.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day7.input");

    let memory: Vec<_> = input.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    println!("{:?}", memory);
    let mut a_prog = intcode::fork_program(&memory);
    let b_prog = intcode::fork_program(&memory);

    println!("{}", a_prog);
    println!("{}", b_prog);

    intcode::send_input(&mut a_prog, 0);
    intcode::send_input(&mut a_prog, 1);
    intcode::run_program(&mut a_prog);
    let res = intcode::pop_output(&mut a_prog);
    println!("result = {}", res);

    println!("{}", a_prog);
    println!("{}", b_prog);
}
