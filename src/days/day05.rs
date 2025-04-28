use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day5.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day5.input");

    let mut prog = intcode::create_program(input);
    println!("{}", prog);

    intcode::send_input(&mut prog, 1);
    intcode::run_program(&mut prog);
    intcode::print_output(&mut prog);

    let p1 = 0;
    let p2 = 0;
    println!("    Part 1 = {}", p1);
    println!("    Part 2 = {}", p2);

}
