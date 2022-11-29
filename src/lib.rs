// Use this code file to store shared code between puzzles
use std::io::stdin;

#[derive(Debug)]
pub enum Arg {
    Pointer(usize),
    Immediate(i64),
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
    if *mode == 0 {
        return Arg::Pointer(*arg as usize);
    }
    else {
        return Arg::Immediate(*arg);
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
        3 => Op::Prompt(Arg::Pointer(mem[idx+1] as usize)),
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
            Arg::Pointer(mem[idx+3] as usize),
        ),
        8 => Op::Equal(
            intcode_parse_arg(&mem[idx+1], &modes[0]),
            intcode_parse_arg(&mem[idx+2], &modes[1]),
            Arg::Pointer(mem[idx+3] as usize),
        ),
        _ => panic!("[INTCODE] Could not parse op code"),
    };
}

pub fn intcode_solve_3param(mem: &mut [i64], arg1: Arg, arg2: Arg, dest: Arg) -> (i64, i64, usize) {
    let a = match arg1 {
        Arg::Pointer(addr) => mem[addr],
        Arg::Immediate(value) => value,
    };
    let b = match arg2 {
        Arg::Pointer(addr) => mem[addr],
        Arg::Immediate(value) => value,
    };

    //TODO: Is there a better way to handle this?
    let target = match dest {
        Arg::Pointer(addr) => addr,
        _ => panic!("[IntCode] Could not store result in immeidate"),
    };

    return (a, b, target as usize);
}

pub fn intcode_solve_2param(mem: &mut [i64], cond: Arg, jump: Arg) -> (i64, usize) {
    let cmp = match cond {
        Arg::Pointer(c) => mem[c],
        Arg::Immediate(c) => c,
    };

    let target = match jump {
        Arg::Pointer(addr) => mem[addr] as usize,
        Arg::Immediate(value) => value as usize,
    };

    return (cmp, target);
}

pub fn intcode_run(mem: &mut [i64]) {
    let mut idx: usize = 0;

    while mem[idx] != 99 {
        let op = intcode_parse_op(&mem, idx);

        match op {
            Op::Halt => return,
            Op::Add(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, arg1, arg2, dest);

                mem[target] = a + b;
                idx += 4;
            }
            Op::Multiply(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, arg1, arg2, dest);

                mem[target] = a * b;
                idx += 4;
            }
            Op::Prompt(dest) => {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer).expect("[IntCode] Could not read input for prompt op");

                let data = buffer.trim().parse::<i64>().unwrap();

                match dest {
                    Arg::Pointer(addr) => mem[addr] = data,
                    _ => panic!("[IntCode] Could not store result in immeidate"),
                };
                idx += 2;
            },
            Op::Output(src) => {
                match src {
                    Arg::Pointer(addr) => println!("[IntCode] {}", mem[addr]),
                    Arg::Immediate(value) => println!("[IntCode] {}", value),
                };
                idx += 2;
            },
            Op::JumpTrue(cond, jump) => {
                let (cmp, target) = intcode_solve_2param(mem, cond, jump);

                if cmp != 0 { idx = target; } else { idx += 3 }
            },
            Op::JumpFalse(cond, jump) => {
                let (cmp, target) = intcode_solve_2param(mem, cond, jump);

                if cmp == 0 { idx = target; } else { idx += 3 }
            },
            Op::Less(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, arg1, arg2, dest);

                mem[target] = if a < b { 1 } else { 0 };
                idx += 4;
            }
            Op::Equal(arg1, arg2, dest) => {
                let (a, b, target) = intcode_solve_3param(mem, arg1, arg2, dest);

                mem[target] = if a == b { 1 } else { 0 };
                idx += 4;
            }
        }
    }
}
