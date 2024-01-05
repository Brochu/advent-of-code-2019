use shared::intcode_run;

fn main() {
    let mut mem: Vec<i64> = include_str!("../data/day5.input")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    let inputs: &[String; 1] = &["5".to_string()];
    let mut outputs: Vec<String> = vec![];
    intcode_run(&mut mem, inputs, &mut outputs);

    outputs.iter().for_each(|str| println!("{}", str));
}
