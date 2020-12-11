use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::iter;

type Pos = (i64, i64);

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Tile, ()> {
        let out = match c {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            _ => return Err(()),
        };

        Ok(out)
    }
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

fn neighbors(area: &HashMap<Pos, Tile>, p: Pos) -> Vec<Tile> {
    let nlist = [
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    nlist
        .iter()
        .map(|&s| (s.0 + p.0, s.1 + p.1))
        .map(|n| area.get(&n).copied().unwrap_or(Tile::Floor))
        .collect()
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mut tilemap: HashMap<Pos, Tile> = HashMap::new();

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x as i64, y as i64), c.try_into().unwrap());
        }
    }

    loop {
        let mut tilemap_new = HashMap::new();

        for (k, v) in tilemap.iter() {
            let n = neighbors(&tilemap, *k);

            let n = match v {
                Tile::Empty => {
                    let c = n.iter().filter(|&&a| a == Tile::Occupied).count();
                    if c == 0 {
                        Tile::Occupied
                    } else {
                        *v
                    }
                }
                Tile::Occupied => {
                    let c = n.iter().filter(|&&a| a == Tile::Occupied).count();
                    if c >= 4 {
                        Tile::Empty
                    } else {
                        *v
                    }
                }
                Tile::Floor => *v,
            };
            tilemap_new.insert(*k, n);
        }

        if tilemap == tilemap_new {
            let cnt = tilemap.values().filter(|&&a| a == Tile::Occupied).count();
            println!("Part 1: {}", cnt);
            break;
        }

        tilemap = tilemap_new;
    }
}

fn neighbors2(area: &HashMap<Pos, Tile>, p: Pos) -> Vec<Tile> {
    let nlist = [
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    let mut out = vec![];

    for dir in nlist.iter() {
        let mut np = p;

        let found = iter::from_fn(move || {
            np = (np.0 + dir.0, np.1 + dir.1);
            area.get(&np)
        })
        .find(|&&a| a != Tile::Floor);

        if let Some(t) = found {
            out.push(*t);
        }
    }

    out
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mut tilemap: HashMap<Pos, Tile> = HashMap::new();

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x as i64, y as i64), c.try_into().unwrap());
        }
    }

    loop {
        let mut tilemap_new = HashMap::new();

        for (k, v) in tilemap.iter() {
            let n = neighbors2(&tilemap, *k);

            let n = match v {
                Tile::Empty => {
                    let c = n.iter().filter(|&&a| a == Tile::Occupied).count();
                    if c == 0 {
                        Tile::Occupied
                    } else {
                        *v
                    }
                }
                Tile::Occupied => {
                    let c = n.iter().filter(|&&a| a == Tile::Occupied).count();
                    if c >= 5 {
                        Tile::Empty
                    } else {
                        *v
                    }
                }
                Tile::Floor => *v,
            };
            tilemap_new.insert(*k, n);
        }

        if tilemap == tilemap_new {
            let cnt = tilemap.values().filter(|&&a| a == Tile::Occupied).count();
            println!("Part 2: {}", cnt);
            break;
        }

        tilemap = tilemap_new;
    }
}

#[allow(dead_code)]
fn draw_map(area: &HashMap<Pos, Tile>) {
    'outer: for y in 0.. {
        for x in 0.. {
            if let Some(c) = area.get(&(x, y)) {
                let x = match c {
                    Tile::Floor => '.',
                    Tile::Empty => 'L',
                    Tile::Occupied => '#',
                };
                print!("{}", x);
            } else if x == 0 {
                break 'outer;
            } else {
                break;
            };
        }
        println!();
    }
}

fn main() {
    p1();
    p2();
}
