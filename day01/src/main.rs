use std::fs::File;
use std::io::{BufRead, BufReader};

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let nums: Vec<i32> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    for a in &nums {
        for b in &nums {
            if a + b == 2020 {
                println!("Part 1: {}", a * b);
                return;
            }
        }
    }
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let nums: Vec<i32> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    for a in &nums {
        for b in &nums {
            for c in &nums {
                if a + b + c == 2020 {
                    println!("Part 2: {}", a * b * c);
                    return;
                }
            }
        }
    }
}

fn main() {
    p1();
    p2();
}
