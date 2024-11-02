fn main() {
    //let passwd = String::from("111122");
    //let valid = check_passwd(&passwd);
    //println!("password {} is {}", passwd, if valid { "valid" } else { "invalid" });

    println!("Part 1 -> {}", run_part1());
}

fn check_passwd(pw: &str) -> bool {
    if pw.len() > 6 {
        return false;
    }

    let letters: Vec<char> = pw
        .chars()
        .collect();

    let double = letters
        .windows(2)
        .any(|w| { 
            w[0] == w[1] &&
            letters.iter().filter(|c| *c == &w[0]).count() == 2
        });

    let nodown = letters
        .windows(2)
        .all( |w| w[0].to_digit(10) <= w[1].to_digit(10) );

    return double && nodown;
}

fn run_part1() -> usize {
    let (min, max) = include_str!("../../data/day4.input")
        .split("-")
        .fold((0, 0), |res, num| {
            // Not sure if this is smart of ugly, probably the latter
            if res.0 == 0 {
                return (num.trim().parse::<u64>().unwrap(), 0);
            }
            else {
                return (res.0, num.trim().parse::<u64>().unwrap());
            }
        });

    return (min..=max)
        .filter(|p| check_passwd(&p.to_string()))
        .count();
}
