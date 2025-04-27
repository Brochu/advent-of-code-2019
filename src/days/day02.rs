use crate::intcode;

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day2.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day2.input");

    let mut prog = intcode::create_program(input);
    intcode::write_mem(&mut prog, 1, 12);
    intcode::write_mem(&mut prog, 2, 2);

    intcode::run_program(&mut prog);
    let p1 = intcode::read_mem(&prog, 0);
    println!("    Part 1 = {}", p1);

    let mut p2 = 0;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut check_prg = intcode::create_program(input);
            intcode::write_mem(&mut check_prg, 1, noun);
            intcode::write_mem(&mut check_prg, 2, verb);

            intcode::run_program(&mut check_prg);
            let res = intcode::read_mem(&check_prg, 0);
            if res == 19690720 {
                p2 = (100 * noun) + verb;
                break;
            }
        }
    }
    println!("    Part 2 = {}", p2);
}
