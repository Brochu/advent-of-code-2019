use std::collections::HashMap;

fn main() {
    println!("[Day6] part 1 = {}", run_part1());
    println!("[Day6] part 2 = {}", run_part2());
}

fn build_tree(data: &str) -> HashMap<String, Vec<String>> {
    let mut tree = HashMap::<String, Vec<String>>::new();

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

    return tree;
}

fn build_parents(tree: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut parents = HashMap::<String, String>::new();

    tree.iter()
        .for_each(|e| {
            e.1.iter().for_each(|c| {
                parents.insert(c.to_string(), e.0.to_string());
            });
        });

    parents.insert(String::from("COM"), String::from(""));
    return parents;
}

fn calc_orbit(tree: &HashMap<String, Vec<String>>) -> usize {
    return calc_orbit_internal(tree, String::from("COM"), 0);
}

fn calc_orbit_internal(tree: &HashMap<String, Vec<String>>, current: String, depth: usize) -> usize {
    return match tree.get(&current) {
        Some(children) => {
            let subtotal: usize = children.iter()
                .map(|c| calc_orbit_internal(tree, c.to_string(), depth+1))
                .sum();

            subtotal + depth
        },
        None => depth,
    };
}

fn run_part1() -> usize {
    let tree = build_tree(include_str!("../data/day6.input"));
    return calc_orbit(&tree);
}

fn run_part2() -> usize {
    let tree = build_tree(include_str!("../data/day6.input"));
    let parents = build_parents(&tree);

    //println!("{:?}", tree);
    //println!("{:?}", parents);

    let mut mybacktrack: Vec<String> = Vec::new();

    let mut current = String::from("YOU");
    while current.len() > 0 {
        current = parents.get(&current).unwrap().to_string();
        mybacktrack.push(current.to_string());
    }

    current = parents.get(&String::from("SAN")).unwrap().to_string();
    let mut count = 0;

    while current.len() > 0 {
        let pos = mybacktrack.iter().position(|n| *n == current);
        match pos {
            Some(i) => return i + count,
            None => {
                count += 1;
                current = parents.get(&current).unwrap().to_string();
            }
        }
    }

    return 0;
}
