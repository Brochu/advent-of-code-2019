pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day8.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day8.input");
    #[cfg(ex)] let img_dims = (3, 2);
    #[cfg(not(ex))] let img_dims = (25, 6);

    let (x, y) = img_dims;
    let len = x * y;

    let vals: Vec<_> = input.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let layers: Vec<_> = vals.chunks(len).collect();
    let mut min_zeros = usize::MAX;
    let mut target_idx = 0;

    for (i, layer) in layers.iter().enumerate() {
        let zero_cnt = layer.iter().filter(|&val| *val == 0).count();
        if zero_cnt < min_zeros {
            min_zeros = zero_cnt;
            target_idx = i;
        }
    }

    let ones_cnt = layers[target_idx].iter().filter(|&val| *val == 1).count();
    let twos_cnt = layers[target_idx].iter().filter(|&val| *val == 2).count();

    println!("    Part 1 = {}", ones_cnt * twos_cnt);
    println!("    Part 2 = {}", 0);
}
