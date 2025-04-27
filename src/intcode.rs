use std::fmt::Display;

pub struct Program {
    mem: Vec<i64>,
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROG MEM: {:?}", self.mem)
    }
}

pub fn read_mem(prog: &Program, addr: usize) -> i64 {
    return prog.mem[addr];
}
pub fn write_mem(prog: &mut Program, addr: usize, new_val: i64) {
    return prog.mem[addr] = new_val;
}

pub fn create_program(code: &str) -> Program {
    let mem: Vec<_> = code.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    return Program { mem };
}

pub fn run_program(prog: &mut Program) {
    //TODO: Rework this because we need to forward the program counter based off of opcode
    for i in (0..prog.mem.len()).step_by(4) {
        let op = prog.mem[i + 0];
        if op == 99 { break; }

        let arg0_addr = prog.mem[i + 1];
        let arg1_addr = prog.mem[i + 2];
        let tar_addr = prog.mem[i + 3];

        let arg0 = prog.mem[arg0_addr as usize];
        let arg1 = prog.mem[arg1_addr as usize];

        //println!("{}: ({}, {}) , {}", op, arg0, arg1, tar_addr);
        let result = match op {
            1 => { arg0 + arg1 }
            2 => { arg0 * arg1 }
            _ => { unimplemented!( )}
        };

        prog.mem[tar_addr as usize] = result;
    }
}
