use std::{collections::HashMap, collections::VecDeque, cmp::min};

pub fn solve() {
    //#[cfg(ex)] let input = include_str!("../../data/day6.example");
    #[cfg(ex)] let input = include_str!("../../data/day6.2.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day6.input");

    let tree = input.lines()
        .fold(HashMap::<&str, Vec<&str>>::new(), |mut tree, l| {
            let (l, r) = l.split_once(")").unwrap();
            match tree.get_mut(l) {
                Some(list) => list.push(r),
                None => {
                    tree.insert(l, vec![r]);
                },
            };
            tree
        });

    fn dfs(tree: &HashMap<&str, Vec<&str>>, node: Option<&str>, depth: usize) -> usize {
        match node {
            Some(n) => {
                //println!("    [dfs] Process {} ( depth = {})", n, depth);
                let mut sum = 0;
                if let Some(list) = tree.get(n) {
                    for &ele in list {
                        sum += dfs(tree, Some(ele), depth + 1);
                    }
                } 
                return sum + depth;
            },
            None => { return depth; },
        }
    }

    let p1 = dfs(&tree, Some("COM"), 0);
    println!("    Part 1 = {}", p1);

    let mut youp = VecDeque::<&str>::new();
    let mut sanp = VecDeque::<&str>::new();
    fn find<'a>(tree: &HashMap<&str, Vec<&'a str>>, out: &mut VecDeque<&'a str>, target: &str, node: Option<&'a str>) -> bool {
        match node {
            Some(n) => {
                if n == target {
                    out.push_front(n);
                    return true;
                }

                //println!("    [dfs] Process {} ( depth = {})", n, depth);
                let mut found = false;
                if let Some(list) = tree.get(n) {
                    for &ele in list {
                        found |= find(tree, out, target, Some(ele));
                    }
                } 
                if found {
                    out.push_front(n);
                }
                return found;
            },
            None => { return false; },
        }
    }

    find(&tree, &mut youp, "YOU", Some("COM"));
    //println!("    YOU PATH: {:?}", youp);
    find(&tree, &mut sanp, "SAN", Some("COM"));
    //println!("    SAN PATH: {:?}", sanp);

    let mut p1 = 0;
    for i in 0..min(youp.len(), sanp.len()) {
        if youp[i] != sanp[i] {
            p1 = youp.len() - i - 1 + sanp.len() - i - 1;
            break;
        }
    }
    println!("    Part 1 = {}", p1);
}
