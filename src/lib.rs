// Use this code file to store shared code between puzzles

#[derive(Debug)]
enum Op {
    Halt,
    Add(i64, i64, i64), //Arg1, Arg2, Dest -> Dest = Arg1 + Arg2
    Multiply(i64, i64, i64), //Arg1, Arg2, Dest -> Dest = Arg1 * Arg2
}

pub fn intcode_run(mem: &mut [i64]) {
    for idx in (0..mem.len()).step_by(4) {

        let op = match mem[idx] {
            99 => Op::Halt,
            1 => Op::Add(mem[idx+1], mem[idx+2], mem[idx+3]),
            2 => Op::Multiply(mem[idx+1], mem[idx+2], mem[idx+3]),
            _ => panic!("[INTCODE] Could not parse op code"),
        };

        if let Op::Halt = op {

            return;

        } else if let Op::Add(arg1, arg2, dest) = op {

            mem[dest as usize] = mem[arg1 as usize] + mem[arg2 as usize];

        } else if let Op::Multiply(arg1, arg2, dest) = op {

            mem[dest as usize] = mem[arg1 as usize] * mem[arg2 as usize];

        }
    }
}
