use shared::intcode_run;

fn main() {
    let part1 = run_part1();
    println!("[Day7] Part 1 = {}", part1);
}

fn run_part1() -> i64 {
    let mem: Vec<i64> = include_str!("../../data/day7.example")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    let inputs: &[String; 2] = &["4".to_string(), "0".to_string()];
    let mut outputs: Vec<String> = vec![];

    //for phase in 0..5 {
    {
        let mut state = mem.clone();
        intcode_run(&mut state, inputs, &mut outputs);

        outputs.iter().for_each(|str| println!("[OUT] {}", str));
    }

    return 0;
}
