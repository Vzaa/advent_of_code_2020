use std::num::ParseIntError;
use std::str::FromStr;

type Pos = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    L,
    R,
}

impl Turn {
    fn turn_pos(self, p: Pos, deg: i64) -> Pos {
        let left = |p: Pos| (-p.1, p.0);
        let right = |p: Pos| (p.1, -p.0);

        match (self, deg) {
            (Turn::L, 90) | (Turn::R, 270) => left(p),
            (Turn::R, 90) | (Turn::L, 270) => right(p),
            (_, 180) => (-p.0, -p.1),
            (_, _) => unreachable!(),
        }
    }
}

impl Dir {
    fn turn_dir(self, t: Turn, deg: i64) -> Dir {
        let left = |d| match d {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::E => Dir::N,
        };

        let right = |d| match d {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::E => Dir::S,
        };

        match (t, deg) {
            (Turn::L, 90) | (Turn::R, 270) => left(self),
            (Turn::R, 90) | (Turn::L, 270) => right(self),
            (_, 180) => left(left(self)),
            (_, _) => unreachable!(),
        }
    }

    fn mv(self, p: Pos, mag: i64) -> Pos {
        match self {
            Dir::N => (p.0, p.1 + mag),
            Dir::S => (p.0, p.1 - mag),
            Dir::W => (p.0 - mag, p.1),
            Dir::E => (p.0 + mag, p.1),
        }
    }
}

#[derive(Debug)]
struct Ship {
    dir: Dir,
    p: Pos,
    wp: Pos,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            dir: Dir::E,
            p: (0, 0),
            wp: (10, 1),
        }
    }

    fn act(&mut self, action: &Action) {
        match *action {
            Action::D(d, n) => self.p = d.mv(self.p, n),
            Action::T(t, d) => self.dir = self.dir.turn_dir(t, d),
            Action::F(n) => self.p = self.dir.mv(self.p, n),
        }
    }

    fn act2(&mut self, action: &Action) {
        match *action {
            Action::D(d, n) => self.wp = d.mv(self.wp, n),
            Action::T(t, d) => self.wp = t.turn_pos(self.wp, d),
            Action::F(n) => self.p = forward(self.p, self.wp, n),
        }
    }
}

fn forward(p: Pos, wp: Pos, mag: i64) -> Pos {
    (p.0 + mag * wp.0, p.1 + mag * wp.1)
}

fn m_dist(a: Pos, b: Pos) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug)]
enum Action {
    D(Dir, i64),
    T(Turn, i64),
    F(i64),
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Empty,
    Invalid,
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().nth(0).ok_or(ParseError::Empty)?;
        let num: i64 = s[1..].parse()?;

        let a = match c {
            'N' => (Action::D(Dir::N, num)),
            'S' => (Action::D(Dir::S, num)),
            'E' => (Action::D(Dir::E, num)),
            'W' => (Action::D(Dir::W, num)),
            'L' => (Action::T(Turn::L, num)),
            'R' => (Action::T(Turn::R, num)),
            'F' => (Action::F(num)),
            _ => return Err(ParseError::Invalid),
        };

        Ok(a)
    }
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();
    let actions: Vec<Action> = instr.lines().map(|l| l.parse().unwrap()).collect();
    let mut ship = Ship::new();

    for a in &actions {
        ship.act(&a);
    }
    println!("Part 1: {}", m_dist((0, 0), ship.p));
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();
    let actions: Vec<Action> = instr.lines().map(|l| l.parse().unwrap()).collect();
    let mut ship = Ship::new();

    for a in &actions {
        ship.act2(&a);
    }

    println!("Part 2: {}", m_dist((0, 0), ship.p));
}

fn main() {
    p1();
    p2();
}
