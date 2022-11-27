fn main() {
    let mut mem: Vec<u64> = include_str!("../data/day2.input")
        .split(",")
        .map( |val| val.trim().parse::<u64>().unwrap() )
        .collect();

    // Prep input
    mem[1] = 12;
    mem[2] = 2;

    //println!("Before run -> {:?}", mem);
    run_sequence(&mut mem);
    //println!("After run -> {:?}", mem);

    println!("Part 1 -> {:?}", mem[0]);
}

fn run_sequence(mem: &mut [u64]) {
    for i in (0..mem.len()).step_by(4) {
        if mem[i] == 99 {
            return;
        }

        let fpos: usize = mem[i+1] as usize;
        let spos: usize = mem[i+2] as usize;
        let output = mem[i+3] as usize;

        if mem[i] == 1 {
            mem[output] = mem[fpos] + mem[spos];
        }

        if mem[i] == 2 {
            mem[output] = mem[fpos] * mem[spos];
        }
    }
}
