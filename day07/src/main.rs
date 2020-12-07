use regex::Regex;
use std::collections::HashMap;

type Bag<'a> = HashMap<&'a str, usize>;

fn find_color(rules: &HashMap<&str, Bag>, cur_color: &str, tgt_color: &str) -> bool {
    let bag = rules.get(cur_color).unwrap();

    bag.contains_key(tgt_color) || bag.keys().any(|inner| find_color(rules, inner, tgt_color))
}

fn count_bags(rules: &HashMap<&str, Bag>, cur_color: &str) -> usize {
    let bag = rules.get(cur_color).unwrap();

    1 + bag
        .iter()
        .map(|(inner, cnt)| cnt * count_bags(rules, &inner))
        .sum::<usize>()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();

    let re_outer =
        Regex::new(r"^(\w+ \w+) bags contain((?:(?: \d+)? \w+ \w+ bags?,?)+).$").unwrap();
    let re_inner = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    let mut rules = HashMap::new();

    for line in instr.lines() {
        let caps = re_outer.captures(line).unwrap();

        let outer_color = caps.get(1).unwrap().as_str();
        let inner_txt = caps.get(2).unwrap().as_str();

        let mut contents = HashMap::new();
        for caps_inner in re_inner.captures_iter(inner_txt) {
            let cnt: usize = caps_inner[1].parse().unwrap();
            let color = caps_inner.get(2).unwrap().as_str();
            contents.insert(color, cnt);
        }
        rules.insert(outer_color, contents);
    }

    let cnt = rules
        .keys()
        .filter(|&b| find_color(&rules, b, "shiny gold"))
        .count();

    println!("Part 1: {}", cnt);
    println!("Part 2: {}", count_bags(&rules, "shiny gold") - 1);
}
