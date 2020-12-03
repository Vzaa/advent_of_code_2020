use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

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

fn count_trees(tilemap: &HashMap<Pos, Tile>, jump: Pos) -> usize {
    let max_x = tilemap.keys().map(|p| p.0).max().unwrap();
    let mut pos = (0, 0);

    iter::from_fn(move || {
        pos = ((pos.0 + jump.0) % (max_x + 1), pos.1 + jump.1);
        tilemap.get(&pos)
    })
    .filter(|&&p| p == Tile::Tree)
    .count()
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

    println!("Part 1: {}", count_trees(&tilemap, (3, 1)));
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

    let list = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mult = list
        .iter()
        .fold(1, |acc, &j| acc * count_trees(&tilemap, j));

    println!("Part 2: {}", mult);
}

fn main() {
    p1();
    p2();
}
