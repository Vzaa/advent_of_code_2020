use std::collections::{HashMap, HashSet};

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let sum: usize = instr
        .split("\n\n")
        .map(|line| {
            let line = line.trim();
            let group: HashSet<_> = line.chars().filter(|c| c.is_alphabetic()).collect();
            group.len()
        })
        .sum();

    println!("{}", sum);
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let sum: usize = instr
        .split("\n\n")
        .map(|line| {
            let line = line.trim();
            let ppl = line.matches('\n').count() + 1;

            let mut group = HashMap::new();
            for c in line.chars().filter(|c| c.is_alphabetic()) {
                let cnt = group.entry(c).or_insert(0_usize);
                *cnt += 1;
            }

            group.values().filter(|&&a| a == ppl).count()
        })
        .sum();

    println!("{}", sum);
}

fn main() {
    p1();
    p2();
}
