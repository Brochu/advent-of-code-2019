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

pub enum Status {
    Halted,
    Output,
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
pub fn pop_output(prog: &mut Program) -> i64 {
    return prog.stdout.pop_back().unwrap();
}

pub fn create_program(code: &str) -> Program {
    let mem: Vec<_> = code.split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    return Program { mem, pc: 0, stdin: VecDeque::new(), stdout: VecDeque::new() };
}
pub fn fork_program(memory: &Vec<i64>) -> Program {
    let mem = memory.clone();
    return Program { mem, pc: 0, stdin: VecDeque::new(), stdout: VecDeque::new() }
}

pub fn run_program(prog: &mut Program) -> Status {
    loop {
        let op = parse_op(prog);
        println!("    {}", op);

        match op.code {
            1 => {
                // ADD
                let sum = resolve_arg(prog, &op, 0) + resolve_arg(prog, &op, 1);
                prog.mem[op.target as usize] = sum;
            },
            2 => {
                // MULT
                let prod = resolve_arg(prog, &op, 0) * resolve_arg(prog, &op, 1);
                prog.mem[op.target as usize] = prod;
            },
            3 => {
                //INPUT
                prog.mem[op.target as usize] = prog.stdin.pop_back().unwrap();
            },
            4 => {
                //OUTPUT
                let val = resolve_arg(prog, &op, 0);
                prog.stdout.push_front(val);
                return Status::Output;
            },
            5 => {
                // JUMP_IF_TRUE
                if resolve_arg(prog, &op, 0) != 0 {
                    prog.pc = resolve_arg(prog, &op, 1) as usize;
                }
            }
            6 => {
                // JUMP_IF_FALSE
                if resolve_arg(prog, &op, 0) == 0 {
                    prog.pc = resolve_arg(prog, &op, 1) as usize;
                }
            }
            7 => {
                // LESS_THAN
                let res = if resolve_arg(prog, &op, 0) < resolve_arg(prog, &op, 1) {
                    1
                } else {
                    0
                };
                prog.mem[op.target as usize] = res;
            },
            8 => {
                // EQUALS
                let res = if resolve_arg(prog, &op, 0) == resolve_arg(prog, &op, 1) {
                    1
                } else {
                    0
                };
                prog.mem[op.target as usize] = res;
            },
            99 => {
                return Status::Halted;
            }
            _ => unimplemented!(),
        }
    }
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
    let mut target = 0;
    match code {
        1 | 2 | 7 | 8 => {
            args[0] = prog.mem[prog.pc];
            prog.pc += 1;
            args[1] = prog.mem[prog.pc];
            prog.pc += 1;
            target = prog.mem[prog.pc];
            prog.pc += 1;
        },
        3 => {
            target = prog.mem[prog.pc];
            prog.pc += 1;
        },
        4 => {
            args[0] = prog.mem[prog.pc];
            prog.pc += 1;
        },
        5 | 6 => {
            args[0] = prog.mem[prog.pc];
            prog.pc += 1;
            args[1] = prog.mem[prog.pc];
            prog.pc += 1;
        }
        99 => {
        }
        _ => unimplemented!(),
    };


    return Op { code, args, target, modes };
}

fn resolve_arg(prog: &mut Program, op: &Op, index: usize) -> i64 {
    match op.modes[index] {
        Mode::Pos => prog.mem[op.args[index] as usize],
        Mode::Imm => op.args[index],
    }
}
