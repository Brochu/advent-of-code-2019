use shared::intcode_run;

fn main() {
    let mut mem: Vec<i64> = include_str!("../data/day2.input")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    prep_input(&mut mem, 12, 2);
    let inputs: &[String; 0] = &[];
    let mut outputs: Vec<String> = vec![];
    intcode_run(&mut mem, inputs, &mut outputs);

    println!("Part 1 -> {:?}", mem[0]);

    mem = include_str!("../data/day2.input")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    let (noun, verb) = find_part2(&mut mem);
    println!("noun, verb -> {:?}; {:?}", noun, verb);
    println!("Part 2 -> {:?}", 100 * noun + verb);
}

fn prep_input(mem: &mut [i64], noun: i64, verb: i64) {
    mem[1] = noun;
    mem[2] = verb;
}

fn find_part2(mem: &mut Vec<i64>) -> (i64, i64) {
    let inputs: &[String; 0] = &[];
    let mut outputs: Vec<String> = vec![];

    for i in 0..100 {
        for j in 0..100 {
            let mut current = mem.clone();
            prep_input(&mut current, i, j);
            intcode_run(&mut current, inputs, &mut outputs);

            if current[0] == 19690720 {
                return (i, j);
            }
        }
    }
    return (0, 0);
}
