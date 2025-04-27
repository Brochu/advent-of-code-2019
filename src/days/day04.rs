pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day4.input");
    #[cfg(not(ex))] let input = include_str!("../../data/day4.input");

    let (lo, hi) = input.trim().split_once('-').unwrap();
    let lo: i32 = lo.parse().unwrap();
    let hi: i32 = hi.parse().unwrap();

    let mut p1 = 0;
    let mut p2 = 0;
    for pass in lo..=hi {
        let p = pass.to_string();

        if match_p1(&p) { p1 += 1; }
        if match_p2(&p) { p2 += 1; }
    }

    println!("    Part 1 = {}", p1);
    println!("    Part 2 = {}", p2);
}

fn match_p1(password: &str) -> bool {
    let mut prev = '0';
    let mut found_dupe = false;

    for ele in password.chars() {
        if ele < prev { return false; }
        if ele == prev { found_dupe = true; }

        prev = ele;
    }

    return found_dupe;
}

fn match_p2(password: &str) -> bool {
    let mut prev = '\0';
    let mut streak = 1;
    let mut valid = false;

    for ele in password.chars() {
        if ele < prev { return false; }
        if ele == prev {
            streak += 1;
        } else {
            if streak == 2 { valid = true; }
            streak = 1;
        }

        prev = ele;
    }

    return valid || streak == 2;
}
