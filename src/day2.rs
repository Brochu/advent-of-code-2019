fn main() {
    let mut mem: Vec<u64> = include_str!("../data/day2.input")
        .split(",")
        .map( |val| val.trim().parse::<u64>().unwrap() )
        .collect();

    //println!("Before run -> {:?}", mem);
    //run_sequence(&mut mem, 12, 2);
    //println!("After run -> {:?}", mem);

    println!("Part 1 -> {:?}", mem[0]);

    let (noun, verb) = find_part2(mem);
    println!("noun, verb -> {:?}; {:?}", noun, verb);
    println!("Part 2 -> {:?}", 100 * noun + verb);
}

fn run_sequence(mem: &mut [u64], noun: u64, verb: u64) {
    // Prep input
    mem[1] = noun;
    mem[2] = verb;

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

fn find_part2(mem: Vec<u64>) -> (u64, u64) {

    for i in 0..100 {
        for j in 0..100 {
            let mut current = mem.clone();
            run_sequence(&mut current, i, j);

            if current[0] == 19690720 {
                return (i, j);
            }
        }
    }
    return (0, 0);
}
