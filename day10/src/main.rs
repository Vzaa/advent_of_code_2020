use std::collections::HashMap;

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mut adapters: Vec<i32> = instr.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();

    let mut res: HashMap<i32, usize> = HashMap::new();
    for sl in adapters.windows(2) {
        let diff = sl[1] - sl[0];

        let cnt = res.entry(diff).or_insert(1_usize);
        *cnt += 1;
    }

    println!("Part 1: {}", res[&3] * res[&1]);
}

fn arrs_until_zero(adapters: &[i32], memo: &mut HashMap<usize, usize>, cur_idx: usize) -> usize {
    let cur = adapters[cur_idx];

    let end = if cur <= 3 { 1 } else { 0 };

    let children = adapters
        .iter()
        .enumerate()
        .skip(cur_idx + 1)
        .take_while(|&(_, a)| *a >= cur - 3)
        .map(|(idx, _)| {
            if let Some(x) = memo.get(&idx) {
                *x
            } else {
                let tmp = arrs_until_zero(adapters, memo, idx);
                memo.insert(idx, tmp);
                tmp
            }
        })
        .sum::<usize>();

    children + end
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mut adapters: Vec<i32> = instr.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    adapters.reverse();

    let mut memo = HashMap::new();

    let arrs = arrs_until_zero(&adapters, &mut memo, 0);
    println!("Part 2: {}", arrs);
}

fn main() {
    p1();
    p2();
}
