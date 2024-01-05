// Use this code file to store shared code between puzzles
use std::io::stdin;

#[derive(Debug)]
pub enum Arg {
    Pointer(usize),
    Immediate(i64),
    Relative(i64),
}

#[derive(Debug)]
pub enum Op {
    Halt,

    Add(Arg, Arg, Arg), //Arg1, Arg2, Dest -> Dest = Arg1 + Arg2
    Multiply(Arg, Arg, Arg), //Arg1, Arg2, Dest -> Dest = Arg1 * Arg2

    Prompt(Arg), //stdin -> Dest
    Output(Arg), //Src -> stdout

    JumpTrue(Arg, Arg), //Cond, Jump -> if cond idx = Jump
    JumpFalse(Arg, Arg), //Cond, Jump -> if !cond idx = Jump

    Less(Arg, Arg, Arg), //Arg1, Arg2, Dest -> Dest = Arg1 < Arg2
    Equal(Arg, Arg, Arg), //Arg1, Arg2, Dest -> Dest = Arg1 == Arg2

    RelativeOffset(Arg), //base_addr += Arg
}

fn intcode_parse_opcode(opcode: &i64) -> (u8, Vec<u32>) {
    let data = format!("{:0>5}", opcode.to_string());
    let op_pos = data.len() - 2;

    return (
        data[op_pos..].parse::<u8>().unwrap(),
        data[0..op_pos].chars().rev().map(|c| c.to_digit(10).unwrap()).collect(),
    );

}

pub fn intcode_parse_arg(arg: &i64, mode: &u32) -> Arg {
    match *mode {
        0 => Arg::Pointer(*arg as usize),
        1 => Arg::Immediate(*arg),
        2 => Arg::Relative(*arg),
        _ => panic!("[IntCode] Could not parse arg mode"),
    }
}

pub fn intcode_parse_op(mem: &[i64], idx: usize) -> Op {
    let (opcode, modes) = intcode_parse_opcode(&mem[idx]);

    return match opcode {
        99 => Op::Halt,
        1 => Op::Add(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
            intcode_parse_arg(&mem[idx+3], &modes[2]), 
        ),
        2 => Op::Multiply(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
            intcode_parse_arg(&mem[idx+3], &modes[2]),
        ),
        3 => Op::Prompt(intcode_parse_arg(&mem[idx+1], &modes[0])),
        4 => Op::Output(intcode_parse_arg(&mem[idx+1], &modes[0])),
        5 => Op::JumpTrue(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
        ),
        6 => Op::JumpFalse(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
        ),
        7 => Op::Less(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
            intcode_parse_arg(&mem[idx+3], &modes[2]),
        ),
        8 => Op::Equal(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
            intcode_parse_arg(&mem[idx+3], &modes[2]),
        ),
        9 => Op::RelativeOffset(
            intcode_parse_arg(&mem[idx+1], &modes[0])
        ),
        _ => panic!("[INTCODE] Could not parse op code"),
    };
}

pub fn intcode_solve_3param(mem: &mut [i64], base: i64, arg1: Arg, arg2: Arg, dest: Arg) -> (i64, i64, usize) {
    let a = match arg1 {
        Arg::Pointer(addr) => mem[addr],
        Arg::Immediate(value) => value,
        Arg::Relative(offset) => mem[(base+offset) as usize],
    };
    let b = match arg2 {
        Arg::Pointer(addr) => mem[addr],
        Arg::Immediate(value) => value,
        Arg::Relative(offset) => mem[(base+offset) as usize],
    };

    //TODO: Is there a better way to handle this?
    let target = match dest {
        Arg::Pointer(addr) => addr,
        Arg::Relative(offset) => (base + offset) as usize,
        _ => panic!("[IntCode] Could not store result in immeidate"),
    };

    return (a, b, target as usize);
}

pub fn intcode_solve_2param(mem: &mut [i64], base: i64, cond: Arg, jump: Arg) -> (i64, usize) {
    let cmp = match cond {
        Arg::Pointer(addr) => mem[addr],
        Arg::Immediate(value) => value,
        Arg::Relative(offset) => mem[(base + offset) as usize],
    };

    let target = match jump {
        Arg::Pointer(addr) => mem[addr] as usize,
        Arg::Immediate(value) => value as usize,
        Arg::Relative(offset) => mem[(base + offset) as usize] as usize,
    };

    return (cmp, target);
}

pub fn intcode_run(mem: &mut [i64], inputs: &[String]) {
    let mut pc: usize = 0;
    let mut ic: usize = 0;
    let mut base_addr: i64 = 0;

    while mem[pc] != 99 {
        let op = intcode_parse_op(&mem, pc);
        //print!("({}) {:?} : ", pc, op);

        match op {
            Op::Halt => return,
            Op::Add(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, base_addr, arg1, arg2, dest);

                mem[target] = a + b;
                //print!("[{}] = {}", target, mem[target]);
                pc += 4;
            }
            Op::Multiply(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, base_addr, arg1, arg2, dest);

                mem[target] = a * b;
                //print!("[{}] = {}", target, mem[target]);
                pc += 4;
            }
            Op::Prompt(dest) => {
                if let Some(buffer) = inputs.get(ic) {
                    ic += 1;
                    let data = buffer.trim().parse::<i64>().unwrap();

                    match dest {
                        Arg::Pointer(addr) => mem[addr] = data,
                        Arg::Relative(offset) => mem[(base_addr + offset) as usize] = data,
                        _ => panic!("[IntCode] Could not store result in immeidate"),
                    };
                    pc += 2;
                } else {
                    println!("[ERROR] Ran out of inputs !");
                    return;
                }
            },
            Op::Output(src) => {
                //TODO: Find a way to send a stream for output instead of stdout or stderr
                match src {
                    Arg::Pointer(addr) => println!("[IntCode] {}", mem[addr]),
                    Arg::Immediate(value) => println!("[IntCode] {}", value),
                    Arg::Relative(offset) => println!("[IntCode] {}", mem[(base_addr + offset) as usize]),
                };
                pc += 2;
            },
            Op::JumpTrue(cond, jump) => {
                let (cmp, target) = intcode_solve_2param(mem, base_addr, cond, jump);

                //print!("[{}] = {}", target, cmp);
                if cmp != 0 { pc = target; } else { pc += 3 }
            },
            Op::JumpFalse(cond, jump) => {
                let (cmp, target) = intcode_solve_2param(mem, base_addr, cond, jump);

                //print!("[{}] = {}", target, cmp);
                if cmp == 0 { pc = target; } else { pc += 3 }
            },
            Op::Less(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, base_addr, arg1, arg2, dest);

                //print!("[{}] = {}", target, mem[target]);
                mem[target] = if a < b { 1 } else { 0 };
                pc += 4;
            },
            Op::Equal(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, base_addr, arg1, arg2, dest);

                //print!("[{}] = {}", target, mem[target]);
                mem[target] = if a == b { 1 } else { 0 };
                pc += 4;
            },
            Op::RelativeOffset(arg) => {
                match arg {
                    Arg::Pointer(addr) => base_addr += mem[addr],
                    Arg::Immediate(value) => base_addr += value,
                    Arg::Relative(offset) => base_addr += mem[(base_addr + offset) as usize],
                };
                //print!("[{}]", base_addr);
                pc += 2;
            },
        }
        //println!();
    }
}
