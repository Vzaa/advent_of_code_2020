use std::collections::HashSet;

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let max = instr
        .lines()
        .map(|line| {
            let id = line
                .chars()
                .fold(0, |acc, c| acc << 1 | ((c == 'B' || c == 'R') as i32));
            id
        })
        .max()
        .unwrap();

    println!("Part 1: {}", max);
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let seats: HashSet<_> = instr
        .lines()
        .map(|line| {
            let id = line
                .chars()
                .fold(0, |acc, c| acc << 1 | ((c == 'B' || c == 'R') as i32));
            id
        })
        .collect();

    for id in 0.. {
        if !seats.contains(&id) && seats.contains(&(id + 1)) && seats.contains(&(id - 1)) {
            println!("Part 2: {}", id);
            return;
        }
    }
}

fn main() {
    p1();
    p2();
}
