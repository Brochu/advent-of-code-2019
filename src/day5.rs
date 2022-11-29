use shared::intcode_run;

fn main() {
    //let mut example = vec![1002, 4, 3, 4, 33];

    //println!("{:?}", example);
    //intcode_run(&mut example);
    //println!("{:?}", example);

    let mut mem: Vec<i64> = include_str!("../data/day5.input")
        .split(",")
        .map( |val| val.trim().parse::<i64>().unwrap() )
        .collect();

    intcode_run(&mut mem);
}
