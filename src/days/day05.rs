use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day5.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day5.input");

    let mut p1_prog = intcode::create_program(input, 100);
    intcode::send_input(&mut p1_prog, 1);
    intcode::run_program(&mut p1_prog);
    intcode::print_output(&mut p1_prog);

    let mut p2_prog = intcode::create_program(input, 100);
    intcode::send_input(&mut p2_prog, 5);
    intcode::run_program(&mut p2_prog);
    intcode::print_output(&mut p2_prog);

}
