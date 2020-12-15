use std::collections::HashMap;

fn p1() {
    let instr = "2,0,6,12,1,3";
    let nums: Vec<i32> = instr.split(',').map(|n| n.parse().unwrap()).collect();

    let mut past = HashMap::new();
    let mut last_num = 0;
    for turn in 1..=2020 {
        let num = if let Some(v) = nums.get((turn - 1) as usize) {
            *v
        } else if let Some(v) = past.get(&last_num) {
            turn - 1 - *v
        } else {
            0
        };

        past.insert(last_num, turn - 1);
        last_num = num;
    }
    println!("Part 1: {}", last_num);
}

fn p2() {
    let instr = "2,0,6,12,1,3";
    let nums: Vec<i32> = instr.split(',').map(|n| n.parse().unwrap()).collect();

    let mut past = HashMap::new();

    let mut last_num = 0;
    for turn in 1..=30000000 {
        let num = if let Some(v) = nums.get((turn - 1) as usize) {
            *v
        } else if let Some(v) = past.get(&last_num) {
            turn - 1 - *v
        } else {
            0
        };

        past.insert(last_num, turn - 1);
        last_num = num;
    }
    println!("Part 2: {}", last_num);
}

fn main() {
    p1();
    p2();
}
