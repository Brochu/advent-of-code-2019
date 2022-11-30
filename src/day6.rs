use std::collections::HashMap;

fn main() {
    println!("[Day6] part 1 = {}", run_part1());
    println!("[Day6] part 2 = {}", run_part2());
}

fn build_tree(data: &str) -> HashMap<String, Vec<String>> {
    let mut tree = HashMap::new();
    tree.insert(String::from(""), vec![]);

    data
        .lines()
        .for_each( |l| {
            let (from, to) = l.split_once(")").unwrap();

            match tree.get_mut(from) {
                Some(v) => v.push(to.to_string()),
                None => {
                    let mut newvec = Vec::new();
                    newvec.push(to.to_string());
                    tree.insert(from.to_string(), newvec);
                }
            };
        });

    tree.remove_entry(&String::from(""));
    return tree;
}

fn calc_orbit(tree: &HashMap<String, Vec<String>>) -> usize {
    let mut depth: usize = 0;
    return calc_orbit_internal(tree, String::from("COM"), &mut depth);
}

fn calc_orbit_internal(tree: &HashMap<String, Vec<String>>, current: String, depth: &mut usize) -> usize {
    //TODO: Need to debug this
    let count = match tree.get(&current) {
        Some(children) => {
            *depth += 1;
            for c in children {
                calc_orbit_internal(tree, c.to_string(), depth);
            }
            *depth
        },
        None => *depth,
    };

    println!("[ORBIT_CALC] [{:?}]={:?}", current, count);
    return count;
}

fn run_part1() -> usize {
    let tree = build_tree(include_str!("../data/day6.example"));

    return calc_orbit(&tree);
}

fn run_part2() -> usize {
    return 0;
}
