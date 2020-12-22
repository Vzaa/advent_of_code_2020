use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
enum Winner {
    P1,
    P2,
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut iter = instr.split("\n\n");

    let player1_str = iter.next().unwrap();

    let mut p1_cards: VecDeque<usize> = player1_str
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    let player2_str = iter.next().unwrap();

    let mut p2_cards: VecDeque<usize> = player2_str
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    while !p1_cards.is_empty() && !p2_cards.is_empty() {
        let c1 = p1_cards.pop_front().unwrap();
        let c2 = p2_cards.pop_front().unwrap();

        if c1 > c2 {
            p1_cards.push_back(c1);
            p1_cards.push_back(c2);
        } else if c2 > c1 {
            p2_cards.push_back(c2);
            p2_cards.push_back(c1);
        } else {
            panic!("noo");
        }
    }

    let winner = if !p1_cards.is_empty() {
        &p1_cards
    } else {
        &p2_cards
    };

    let s: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(a, b)| (a + 1) * b)
        .sum();

    println!("Part 1: {}", s);
}

fn calc_score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(a, b)| (a + 1) * b)
        .sum()
}

fn geemu(mut p1_cards: VecDeque<usize>, mut p2_cards: VecDeque<usize>) -> (Winner, usize) {
    let mut past = HashSet::new();

    while !p1_cards.is_empty() && !p2_cards.is_empty() {
        if !past.insert((p1_cards.clone(), p2_cards.clone())) {
            return (Winner::P1, calc_score(&p1_cards));
        }

        let c1 = p1_cards.pop_front().unwrap();
        let c2 = p2_cards.pop_front().unwrap();

        let winner = if c1 <= p1_cards.len() && c2 <= p2_cards.len() {
            let p1 = p1_cards.iter().take(c1).cloned().collect();
            let p2 = p2_cards.iter().take(c2).cloned().collect();
            let (w, _) = geemu(p1, p2);
            w
        } else if c1 > c2 {
            Winner::P1
        } else if c2 > c1 {
            Winner::P2
        } else {
            panic!("noo");
        };

        match winner {
            Winner::P1 => {
                p1_cards.push_back(c1);
                p1_cards.push_back(c2);
            }
            Winner::P2 => {
                p2_cards.push_back(c2);
                p2_cards.push_back(c1);
            }
        }
    }

    if !p1_cards.is_empty() {
        (Winner::P1, calc_score(&p1_cards))
    } else {
        (Winner::P2, calc_score(&p2_cards))
    }
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut iter = instr.split("\n\n");

    let player1_str = iter.next().unwrap();

    let p1_cards: VecDeque<usize> = player1_str
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    let player2_str = iter.next().unwrap();

    let p2_cards: VecDeque<usize> = player2_str
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    let (_, s) = geemu(p1_cards, p2_cards);
    println!("Part 2: {}", s);
}

fn main() {
    p1();
    p2();
}
