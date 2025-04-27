use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let _input = include_str!("../../data/day5.example");
    #[cfg(not(ex))] let _input = include_str!("../../data/day5.input");
    let input = "1101,100,-1,4";

    let mut prog = intcode::create_program(input);
    println!("{}", prog);

    intcode::run_program(&mut prog);

    let p1 = 0;
    let p2 = 0;
    println!("    Part 1 = {}", p1);
    println!("    Part 2 = {}", p2);

}
