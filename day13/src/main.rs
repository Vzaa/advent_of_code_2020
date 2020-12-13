fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let arrival: i64 = instr.lines().next().unwrap().parse().unwrap();
    let bus_str = instr.lines().nth(1).unwrap();

    let buses: Vec<i64> = bus_str
        .split(',')
        .filter_map(|b| match b {
            "x" => None,
            a => Some(a.parse().unwrap()),
        })
        .collect();

    let e = buses.iter().map(|b| (b - arrival % b, b)).min().unwrap();
    println!("Part 1: {}", e.0 * e.1);
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn solve_with_hint(blist: &[(i64, i64)], hint: (i64, i64)) -> (i64, i64) {
    let l = blist.iter().map(|(_, b)| b).fold(1, |acc, x| lcm(acc, *x));

    for i in 0.. {
        let n = (i * hint.0) + hint.1;
        let check = blist.iter().all(|(i, b)| (n + i) % b == 0);
        if check {
            return (l, n);
        }
    }
    unreachable!();
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let bus_str = instr.lines().nth(1).unwrap();

    let buses: Vec<(i64, i64)> = bus_str
        .split(',')
        .enumerate()
        .filter_map(|(idx, b)| match b {
            "x" => None,
            a => Some((idx as i64, a.parse().unwrap())),
        })
        .collect();

    let mut hint = (1, 0);
    for j in 2..=buses.len() {
        hint = solve_with_hint(&buses[0..j], hint);
    }

    println!("Part 2: {}", hint.1);
}

fn main() {
    p1();
    p2();
}
