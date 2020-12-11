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

static NLIST: [Pos; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbors<'a>(area: &'a HashMap<Pos, Tile>, p: Pos) -> impl Iterator<Item = &Tile> + 'a {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        area.get(&np)
    })
}

fn neighbors2<'a>(area: &'a HashMap<Pos, Tile>, p: Pos) -> impl Iterator<Item = &Tile> + 'a {
    NLIST.iter().filter_map(move |dir| {
        let mut np = p;

        iter::from_fn(move || {
            np = (np.0 + dir.0, np.1 + dir.1);
            area.get(&np)
        })
        .find(|&&t| t != Tile::Floor)
    })
}

fn p1(mut tilemap: HashMap<Pos, Tile>) {
    loop {
        let mut tilemap_new = HashMap::new();

        for (k, v) in tilemap.iter() {
            let t = match v {
                Tile::Empty => {
                    let ns = neighbors(&tilemap, *k);
                    let c = ns.filter(|&&a| a == Tile::Occupied).count();
                    if c == 0 {
                        Tile::Occupied
                    } else {
                        *v
                    }
                }
                Tile::Occupied => {
                    let ns = neighbors(&tilemap, *k);
                    let c = ns.filter(|&&a| a == Tile::Occupied).count();
                    if c >= 4 {
                        Tile::Empty
                    } else {
                        *v
                    }
                }
                Tile::Floor => *v,
            };
            tilemap_new.insert(*k, t);
        }

        if tilemap == tilemap_new {
            let cnt = tilemap.values().filter(|&&a| a == Tile::Occupied).count();
            println!("Part 1: {}", cnt);
            break;
        }

        tilemap = tilemap_new;
    }
}

fn p2(mut tilemap: HashMap<Pos, Tile>) {
    loop {
        let mut tilemap_new = HashMap::new();

        for (k, v) in tilemap.iter() {
            let t = match v {
                Tile::Empty => {
                    let ns = neighbors2(&tilemap, *k);
                    let c = ns.filter(|&&a| a == Tile::Occupied).count();
                    if c == 0 {
                        Tile::Occupied
                    } else {
                        *v
                    }
                }
                Tile::Occupied => {
                    let ns = neighbors2(&tilemap, *k);
                    let c = ns.filter(|&&a| a == Tile::Occupied).count();
                    if c >= 5 {
                        Tile::Empty
                    } else {
                        *v
                    }
                }
                Tile::Floor => *v,
            };
            tilemap_new.insert(*k, t);
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
    let instr = std::fs::read_to_string("input").unwrap();
    let mut tilemap: HashMap<Pos, Tile> = HashMap::new();

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x as i64, y as i64), c.try_into().unwrap());
        }
    }

    p1(tilemap.clone());
    p2(tilemap.clone());
}
