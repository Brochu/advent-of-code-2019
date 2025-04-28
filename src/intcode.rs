use std::collections::VecDeque;
use std::fmt::Display;

pub struct Program {
    mem: Vec<i64>,
    pc: usize,
    stdin: VecDeque<i64>,
    stdout: VecDeque<i64>,
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    PROG [{}]", self.pc).unwrap();
        writeln!(f, "    MEM: {:?}", self.mem).unwrap();
        writeln!(f, "    stdin: {:?}", self.stdin).unwrap();
        writeln!(f, "    stdout: {:?}", self.stdout).unwrap();
        write!(f, "    -------------------------")
    }
}

#[derive(Debug)]
enum Mode {
    Pos,
    Imm,
}

struct Op {
    code: i64,
    args: [i64; 2],
    target: i64,
    modes: [Mode; 3],
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    [{}] - ([{:?}]{}, [{:?}]{}) ; [{:?}]{}",
            self.code,
            self.modes[0], self.args[0],
            self.modes[1], self.args[1],
            self.modes[2], self.target)
    }
}

const PROG_END: i64 = 99;

pub fn read_mem(prog: &Program, addr: usize) -> i64 {
    return prog.mem[addr];
}
pub fn write_mem(prog: &mut Program, addr: usize, new_val: i64) {
    return prog.mem[addr] = new_val;
}

pub fn send_input(prog: &mut Program, value: i64) {
    prog.stdin.push_front(value);
}
pub fn print_output(prog: &mut Program) {
    println!("    PROGRAM OUTPUT:");
    while let Some(val) = prog.stdout.pop_back() {
        println!("    - `{}`", val);
    }
    println!("    -------------------------");
}

pub fn create_program(code: &str) -> Program {
    let mem: Vec<_> = code.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    return Program { mem, pc: 0, stdin: VecDeque::new(), stdout: VecDeque::new() };
}

pub fn run_program(prog: &mut Program) {
    while prog.pc < prog.mem.len() && prog.mem[prog.pc] != PROG_END {
        let op = parse_op(prog);

        match op.code {
            1 => {
                let sum = resolve_arg(prog, &op.modes[0], op.args[0]) + resolve_arg(prog, &op.modes[1], op.args[1]);
                prog.mem[op.target as usize] = sum;
            },
            2 => {
                let prod = resolve_arg(prog, &op.modes[0], op.args[0]) * resolve_arg(prog, &op.modes[1], op.args[1]);
                prog.mem[op.target as usize] = prod;
            },
            3 => {
                prog.mem[op.target as usize] = prog.stdin.pop_back().unwrap();
            },
            4 => {
                let val = prog.mem[op.target as usize];
                prog.stdout.push_front(val);
            },
            _ => unimplemented!(),
        }
    }
    /*
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
    */
}

fn parse_op(prog: &mut Program) -> Op {
    let mut opcode = prog.mem[prog.pc];
    prog.pc += 1;

    let mut code = opcode % 10;
    opcode /= 10;
    code += (opcode % 10) * 10;

    let mut modes = [Mode::Pos, Mode::Pos, Mode::Pos];
    for i in 0..modes.len() {
        opcode /= 10;
        modes[i] = match opcode % 2 {
            0 => Mode::Pos,
            1 => Mode::Imm,
            _ => Mode::Pos,
        };
    }

    let mut args = [0, 0];
    match code {
        1 | 2 => {
            args[0] = prog.mem[prog.pc];
            prog.pc += 1;
            args[1] = prog.mem[prog.pc];
            prog.pc += 1;
        },
        3 | 4 => {
        },
        _ => unimplemented!(),
    };

    let target = prog.mem[prog.pc];
    prog.pc += 1;

    return Op { code, args, target, modes };
}

fn resolve_arg(prog: &mut Program, mode: &Mode, value: i64) -> i64 {
    match mode {
        Mode::Pos => prog.mem[value as usize],
        Mode::Imm => value,
    }
}
