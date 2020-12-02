use std::{num::ParseIntError, ops::RangeInclusive};
use std::str::FromStr;

#[derive(Debug)]
struct PasswdRule {
    c: char,
    range: RangeInclusive<usize>,
    txt: String,
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

impl FromStr for PasswdRule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let range_str = iter.next().ok_or(ParseError::Empty)?;
        let c_str = iter.next().ok_or(ParseError::Empty)?;
        let txt = iter.next().ok_or(ParseError::Empty)?;

        let mut iter = range_str.split('-');
        let s = iter.next().ok_or(ParseError::Empty)?.parse()?;
        let e = iter.next().ok_or(ParseError::Empty)?.parse()?;

        let range = s..=e;

        Ok(PasswdRule {
            c: c_str.chars().next().ok_or(ParseError::Empty)?,
            range,
            txt: txt.into(),
        })
    }
}

impl PasswdRule {
    fn is_valid_p1(&self) -> bool {
        let cnt = self.txt.matches(self.c).count();
        self.range.contains(&cnt)
    }

    fn is_valid_p2(&self) -> bool {
        let c1 = self.txt.chars().nth(self.range.start() - 1).unwrap();
        let c2 = self.txt.chars().nth(self.range.end() - 1).unwrap();

        (c1 == self.c) ^ (c2 == self.c)
    }
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();
    let rules: Vec<PasswdRule> = instr.lines().map(|l| l.parse().unwrap()).collect();

    let cnt = rules.iter().filter(|r| r.is_valid_p1()).count();
    println!("Part 1: {}", cnt);
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();
    let rules: Vec<PasswdRule> = instr.lines().map(|l| l.parse().unwrap()).collect();

    let cnt = rules.iter().filter(|r| r.is_valid_p2()).count();
    println!("Part 2: {}", cnt);
}

fn main() {
    p1();
    p2();
}
