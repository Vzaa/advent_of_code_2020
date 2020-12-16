use std::collections::HashMap;

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut sect_iter = instr.split("\n\n");

    let fields_str = sect_iter.next().unwrap().trim();
    let others_str = sect_iter.skip(1).next().unwrap().trim();

    let mut fields = HashMap::new();

    for line in fields_str.lines() {
        let mut iter = line.split(':');
        let field = iter.next().unwrap().trim();
        let ranges_txt = iter.next().unwrap().trim();

        let mut range_iter = ranges_txt.split("or");

        let range_1_str = range_iter.next().unwrap().trim();
        let range_2_str = range_iter.next().unwrap().trim();

        let mut iter = range_1_str.split('-');
        let range_1: (i64, i64) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );

        let mut iter = range_2_str.split('-');
        let range_2: (i64, i64) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );

        fields.insert(field, (range_1.0..=range_1.1, range_2.0..=range_2.1));
    }

    let mut others = vec![];

    for line in others_str.lines().skip(1) {
        others.extend(line.split(',').map(|t| t.parse::<i64>().unwrap()));
    }

    let s: i64 = others
        .iter()
        .filter(|&&o| {
            !fields
                .values()
                .any(|r| r.0.contains(&o) || r.1.contains(&o))
        })
        .sum();

    println!("Part 1: {}", s);
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut sect_iter = instr.split("\n\n");

    let fields_str = sect_iter.next().unwrap().trim();
    let ticket_str = sect_iter.next().unwrap().trim();
    let others_str = sect_iter.next().unwrap().trim();

    let mut fields = HashMap::new();

    for line in fields_str.lines() {
        let mut iter = line.split(':');
        let field = iter.next().unwrap().trim();
        let ranges_txt = iter.next().unwrap().trim();

        let mut range_iter = ranges_txt.split("or");

        let range_1_str = range_iter.next().unwrap().trim();
        let range_2_str = range_iter.next().unwrap().trim();

        let mut iter = range_1_str.split('-');
        let range_1: (i64, i64) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );

        let mut iter = range_2_str.split('-');
        let range_2: (i64, i64) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );

        fields.insert(field, (range_1.0..=range_1.1, range_2.0..=range_2.1));
    }

    let ticket: Vec<i64> = ticket_str
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|t| t.parse().unwrap())
        .collect();

    let mut others = vec![];

    for line in others_str.lines().skip(1) {
        others.push(
            line.split(',')
                .map(|t| t.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }

    let valids: Vec<Vec<i64>> = others
        .iter()
        .filter(|&t| {
            // check every value against all ranges in all fields
            t.iter().all(|f| {
                fields
                    .values()
                    .any(|r| r.0.contains(&f) || r.1.contains(&f))
            })
        })
        .cloned()
        .collect();

    let mut field_map = HashMap::new();

    while field_map.len() == ticket.len() {
        for (field, vals) in fields.iter() {
            let candidates: Vec<_> = (0..ticket.len())
                // Skip columns already mapped
                .filter(|&idx| !field_map.values().any(|&i| i == idx))
                // Get columns that match ranges for all
                .filter(|&idx| {
                    valids
                        .iter()
                        .map(|v| v[idx])
                        .all(|f| vals.0.contains(&f) || vals.1.contains(&f))
                })
                .collect();

            if candidates.len() == 1 {
                field_map.insert(field, candidates[0]);
            }
        }
    }

    let mult = fields
        .keys()
        .filter(|f| f.starts_with("departure"))
        .map(|f| ticket[field_map[f]])
        .fold(1, |acc, x| x * acc);

    println!("Part 2: {}", mult);
}

fn main() {
    p1();
    p2();
}
