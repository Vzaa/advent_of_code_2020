use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i64, i64);

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Tile, ()> {
        let out = match c {
            '.' => Tile::Empty,
            '#' => Tile::Tree,
            _ => return Err(()),
        };

        Ok(out)
    }
}

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut tilemap: HashMap<Pos, Tile> = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x as i64, y as i64), c.try_into().unwrap());
        }
    }

    let max_x = tilemap.keys().map(|p| p.0).max().unwrap();

    let mut cnt = 0;
    let mut pos = (0, 0);
    loop {
        if let Some(t) = tilemap.get(&pos) {
            if *t == Tile::Tree {
                cnt += 1;
            }
        } else {
            break;
        }
        pos = ((pos.0 + 3) % (max_x + 1), pos.1 + 1);
    }

    println!("Part 1: {}", cnt);
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut tilemap: HashMap<Pos, Tile> = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x as i64, y as i64), c.try_into().unwrap());
        }
    }

    let max_x = tilemap.keys().map(|p| p.0).max().unwrap();

    let list = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut out = vec![];

    for step in &list {
        let mut cnt: i64 = 0;
        let mut pos = (0, 0);

        loop {
            if let Some(t) = tilemap.get(&pos) {
                if *t == Tile::Tree {
                    cnt += 1;
                }
            } else {
                break;
            }
            pos = ((pos.0 + step.0) % (max_x + 1), pos.1 + step.1);
        }
        out.push(cnt);
    }

    println!("Part 2: {}", out.iter().fold(1, |x, y| x * y));
}

fn main() {
    p1();
    p2();
}
