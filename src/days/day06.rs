use std::{collections::HashMap, vec};

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day6.example");
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

    println!("{:?}", tree);
}
