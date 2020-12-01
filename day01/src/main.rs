use std::io::{BufRead, BufReader};
use std::{collections::HashSet, fs::File};

fn find_pair(nums: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    nums.iter()
        .find_map(|&a| nums.get(&(target - a)).map(|&b| (a, b)))
}

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let nums: HashSet<i32> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let (a, b) = find_pair(&nums, 2020).unwrap();
    println!("Part 1: {}", a * b);
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let nums: HashSet<i32> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let (a, b, c) = nums
        .iter()
        .find_map(|a| find_pair(&nums, 2020 - a).map(|(b, c)| (*a, b, c)))
        .unwrap();
    println!("Part 2: {}", a * b * c);
}

fn main() {
    p1();
    p2();
}
