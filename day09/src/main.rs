fn p1(nums: &[i64]) -> i64{
    let preamble_size = 25;

    'outer: for win in nums.windows(preamble_size + 1) {
        let c = win[preamble_size];
        let preamble = &win[0..preamble_size];

        for (idx, a) in preamble.iter().enumerate() {
            let found = preamble.iter().skip(idx + 1).any(|b| b + a == c);
            if found {
                continue 'outer;
            }
        }
        return c;
    }
    unreachable!();
}

fn p2(nums: &[i64], tgt: i64) -> i64 {
    for w_size in 2.. {
        for win in nums.windows(w_size) {
            if win.iter().sum::<i64>() == tgt {
                let min = win.iter().min().unwrap();
                let max = win.iter().max().unwrap();
                return min + max;
            }
        }
    }
    unreachable!();
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    let nums: Vec<i64> = instr.lines().map(|l| l.parse().unwrap()).collect();

    let tgt = p1(&nums);
    println!("Part 1: {}", tgt);
    let tgt = p2(&nums, tgt);
    println!("Part 2: {}", tgt);
}
