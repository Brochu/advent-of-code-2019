use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day9.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day9.input");

    let mut prog = intcode::create_program(input, 150);
    intcode::send_input(&mut prog, 1);
    println!("{}", prog);

    loop {
        if let intcode::Status::Halted = intcode::run_program(&mut prog) {
            break;
        }
    }

    println!("{}", prog);
}
