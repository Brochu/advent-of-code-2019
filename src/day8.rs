use std::str::from_utf8;

fn main() {
    //let ex0 = include_str!("../data/day8.example").trim();
    //let ex1 = include_str!("../data/day8.2.example").trim();
    let input = include_str!("../data/day8.input").trim();

    println!("[Day8] part 1 = {}", run_part1(25, 6, input));
    println!("[Day8] part 2 = {}", run_part2(25, 6, input));
}

fn run_part1(w: usize, h: usize, data: &str) -> u64 {
    println!("[Day8] Solving for {} x {}", w, h);

    let layers = data.as_bytes()
        .chunks(w*h)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let idx = layers.iter()
        .enumerate()
        .map(|e| (e.0, e.1.chars().filter(|c| *c == '0').count()))
        .min_by(|f, s| f.1.cmp(&s.1)).unwrap().0;

    let ones = layers[idx]
        .chars()
        .filter(|c| *c == '1')
        .count();

    let twos = layers[idx]
        .chars()
        .filter(|c| *c == '2')
        .count();

    println!("[Day8] Found layer at index {:?} ({:?} ones and {:?} twos)", idx, ones, twos);
    return (ones * twos) as u64;
}

fn run_part2(w: usize, h: usize, data: &str) -> u64 {
    println!("[Day8] Solving for {} x {}", w, h);

    let layers = data.as_bytes()
        .chunks(w*h)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let mut start = Vec::<&str>::new();
    start.resize(w*h, "2");

    let mut image = layers.iter()
        .fold(start, |img, layer| {
            img.iter()
                .enumerate()
                .map(|p| {
                    if *p.1 == "2" {
                        &layer[p.0..p.0+1]
                    }
                    else {
                        *p.1
                    }
                })
                .collect()
        })
        .iter()
        .fold(String::new(), |out, pix| out + pix);

    image = image.replace("0", ".");
    image = image.replace("1", "O");

    let lines = image.as_bytes()
        .chunks(w)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    println!("");
    lines.iter()
        .for_each(|l| println!("{}", l));
    println!("");

    return 0;
}
