use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Rule {
    Char(char),
    Other(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('"') {
            Ok(Rule::Char(s.chars().nth(1).unwrap()))
        } else {
            let mut out = vec![];

            for p in s.split('|') {
                let r = p.split_whitespace().map(|n| n.parse().unwrap()).collect();
                out.push(r);
            }

            Ok(Rule::Other(out))
        }
    }
}

// Return matches with length
fn check(rule_list: &HashMap<usize, Rule>, id: usize, txt: &str) -> Vec<usize> {
    let r = &rule_list[&id];

    let mut match_list = vec![];

    match r {
        Rule::Char(c) => {
            if let Some(ch) = txt.chars().next() {
                if ch == *c {
                    match_list.push(1);
                }
            }
        }
        Rule::Other(alters) => {
            for rules in alters {
                let mut start_idxs = vec![0];

                for rule in rules {
                    let mut next = vec![];
                    for i in &start_idxs {
                        let mats = check(rule_list, *rule, &txt[*i..]);

                        for m in mats {
                            next.push(m + i);
                        }
                    }
                    start_idxs = next;
                }
                match_list.extend_from_slice(&start_idxs);
            }
        }
    }

    match_list
}

fn count_matches(rule_list: &HashMap<usize, Rule>, msg_str: &str) -> usize {
    msg_str
        .lines()
        .filter(|msg| {
            let mats = check(&rule_list, 0, msg);
            mats.iter().any(|m| *m == msg.len())
        })
        .count()
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut it = instr.split("\n\n");

    let rules_str = it.next().unwrap().trim();
    let msg_str = it.next().unwrap().trim();

    let mut rule_list = HashMap::new();

    for line in rules_str.lines() {
        let mut iter = line.split(':');
        let id: usize = iter.next().unwrap().parse().unwrap();
        let txt = iter.next().unwrap().trim();
        let rule: Rule = txt.parse().unwrap();
        rule_list.insert(id, rule);
    }

    let p1 = count_matches(&rule_list, msg_str);
    println!("Part 1: {}", p1);

    // Replace new rules
    rule_list.insert(8, Rule::Other(vec![vec![42], vec![42, 8]]));
    rule_list.insert(11, Rule::Other(vec![vec![42, 31], vec![42, 11, 31]]));
    let p2 = count_matches(&rule_list, msg_str);
    println!("Part 2: {}", p2);
}
