use std::collections::HashMap;

fn id(r: i32, c: i32) -> i32 {
    r * 8 + c
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let seats: HashMap<_, _> = instr
        .lines()
        .map(|line| {
            let row_str = line[0..7].replace("F", "0").replace("B", "1");
            let col_str = line[7..].replace("L", "0").replace("R", "1");

            let row = i32::from_str_radix(&row_str, 2).unwrap();
            let col = i32::from_str_radix(&col_str, 2).unwrap();

            (id(row, col), (row, col))
        })
        .collect();

    println!("Part 1: {}", seats.keys().max().unwrap());
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let seats: HashMap<_, _> = instr
        .lines()
        .map(|line| {
            let row_str = line[0..7].replace("F", "0").replace("B", "1");
            let col_str = line[7..].replace("L", "0").replace("R", "1");

            let row = i32::from_str_radix(&row_str, 2).unwrap();
            let col = i32::from_str_radix(&col_str, 2).unwrap();

            (id(row, col), (row, col))
        })
        .collect();

    for row in 0..=127 {
        for col in 0..=7 {
            if !seats.contains_key(&id(row, col))
                && seats.contains_key(&(id(row, col) + 1))
                && seats.contains_key(&(id(row, col) - 1))
            {
                println!("Part 2: {}", id(row, col));
                return;
            }
        }
    }
}
fn main() {
    p1();
    p2();
}
