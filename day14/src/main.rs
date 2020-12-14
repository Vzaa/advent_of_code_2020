use std::collections::HashMap;

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut mask_a: i64 = 0;
    let mut mask_o: i64 = 0;
    let mut mem = HashMap::new();
    for line in instr.lines() {
        let val_str = line.split(' ').nth(2).unwrap();

        if line.starts_with("mask") {
            mask_a = val_str
                .chars()
                .fold(0, |acc, c| acc << 1 | ((c == 'X') as i64));

            mask_o = val_str
                .chars()
                .fold(0, |acc, c| acc << 1 | ((c == '1') as i64));
        } else if line.starts_with("mem") {
            let mem_str = line.split(' ').nth(0).unwrap();
            let addr_str = mem_str.split(|c| c == '[' || c == ']').nth(1).unwrap();

            let addr: usize = addr_str.parse().unwrap();
            let val: i64 = val_str.parse().unwrap();

            mem.insert(addr, (val & mask_a) | mask_o);
        } else {
            panic!()
        }
    }

    println!("Part 1: {}", mem.values().sum::<i64>());
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut mask_a: usize = 0;
    let mut mask_o: usize = 0;
    let mut mem = HashMap::new();
    for line in instr.lines() {
        let val_str = line.split(' ').nth(2).unwrap();

        if line.starts_with("mask") {
            mask_a = val_str
                .chars()
                .fold(0, |acc, c| acc << 1 | ((c == 'X') as usize));

            mask_o = val_str
                .chars()
                .fold(0, |acc, c| acc << 1 | ((c == '1') as usize));
        } else if line.starts_with("mem") {
            let mem_str = line.split(' ').nth(0).unwrap();
            let addr_str = mem_str.split(|c| c == '[' || c == ']').nth(1).unwrap();

            let addr: usize = addr_str.parse().unwrap();
            let val: i64 = val_str.parse().unwrap();

            let addr_base = addr | mask_o;

            let mut bits = vec![];

            // Separate each X
            for b in 0..=36 {
                let bit = 1 << b;
                if (bit & mask_a) != 0 {
                    bits.push(bit);
                }
            }

            // For each possible 2^n combination of XXs
            for m in 0..(1 << bits.len()) {
                let mut mask = 0;

                // Map the combination to the actual positions in 36 bit
                for (i, bit) in bits.iter().enumerate() {
                    if ((1 << i) & m) != 0 {
                        mask = mask | bit;
                    }
                }
                let addr = ((!mask_a) & addr_base) | mask;
                mem.insert(addr, val);
            }
        } else {
            panic!()
        }
    }

    println!("Part 2: {}", mem.values().sum::<i64>());
}

fn main() {
    p1();
    p2();
}
