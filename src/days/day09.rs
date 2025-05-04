use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day9.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day9.input");
    //let input = "109, 1, 203, 2, 204, 2, 99";

    let memory: Vec<_> = input.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut prog = intcode::fork_program(&memory, 1500);
    intcode::send_input(&mut prog, 2);
    println!("{}", prog);

    loop {
        if let intcode::Status::Halted = intcode::run_program(&mut prog) {
            break;
        }
    }

    println!("{}", prog);
}
