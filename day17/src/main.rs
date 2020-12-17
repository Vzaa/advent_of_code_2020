use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};

type Pos = (i8, i8, i8);
type TileMap = HashSet<Pos>;

type Pos4 = (i8, i8, i8, i8);
type TileMap4 = HashSet<Pos4>;

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Tile, ()> {
        let out = match c {
            '.' => Tile::Inactive,
            '#' => Tile::Active,
            _ => return Err(()),
        };

        Ok(out)
    }
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Inactive,
    Active,
}

static NLIST: [Pos; 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

static NLIST2: [Pos4; 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

fn neighbors(area: &TileMap, p: Pos) -> impl Iterator<Item = &Pos> + '_ {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1, p.2 + n.2);
        area.get(&np)
    })
}

fn neighbors2(area: &TileMap4, p: Pos4) -> impl Iterator<Item = &Pos4> + '_ {
    NLIST2.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1, p.2 + n.2, p.3 + n.3);
        area.get(&np)
    })
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut tilemap: TileMap = HashSet::new();

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let t: Tile = c.try_into().unwrap();
            if t == Tile::Active {
                tilemap.insert((x as i8, y as i8, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut tilemap_new = HashSet::new();
        let max_x = tilemap.iter().map(|p| p.0).max().unwrap();
        let max_y = tilemap.iter().map(|p| p.1).max().unwrap();
        let max_z = tilemap.iter().map(|p| p.2).max().unwrap();

        let min_x = tilemap.iter().map(|p| p.0).min().unwrap();
        let min_y = tilemap.iter().map(|p| p.1).min().unwrap();
        let min_z = tilemap.iter().map(|p| p.2).min().unwrap();

        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    let k = (x, y, z);
                    let v = if tilemap.contains(&k) {
                        Tile::Active
                    } else {
                        Tile::Inactive
                    };
                    let c = neighbors(&tilemap, k).count();

                    match v {
                        Tile::Active => {
                            if c == 2 || c == 3 {
                                tilemap_new.insert(k);
                            }
                        }
                        Tile::Inactive => {
                            if c == 3 {
                                tilemap_new.insert(k);
                            }
                        }
                    }
                }
            }
        }

        tilemap = tilemap_new;
    }
    let c = tilemap.len();
    println!("Part 1: {}", c);
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut tilemap: TileMap4 = HashSet::new();

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let t: Tile = c.try_into().unwrap();
            if t == Tile::Active {
                tilemap.insert((x as i8, y as i8, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut tilemap_new = HashSet::new();
        let max_x = tilemap.iter().map(|p| p.0).max().unwrap();
        let max_y = tilemap.iter().map(|p| p.1).max().unwrap();
        let max_z = tilemap.iter().map(|p| p.2).max().unwrap();
        let max_w = tilemap.iter().map(|p| p.3).max().unwrap();

        let min_x = tilemap.iter().map(|p| p.0).min().unwrap();
        let min_y = tilemap.iter().map(|p| p.1).min().unwrap();
        let min_z = tilemap.iter().map(|p| p.2).min().unwrap();
        let min_w = tilemap.iter().map(|p| p.3).min().unwrap();

        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    for w in (min_w - 1)..=(max_w + 1) {
                        let k = (x, y, z, w);
                        let v = if tilemap.contains(&k) {
                            Tile::Active
                        } else {
                            Tile::Inactive
                        };
                        let c = neighbors2(&tilemap, k).count();

                        match v {
                            Tile::Active => {
                                if c == 2 || c == 3 {
                                    tilemap_new.insert(k);
                                }
                            }
                            Tile::Inactive => {
                                if c == 3 {
                                    tilemap_new.insert(k);
                                }
                            }
                        }
                    }
                }
            }
        }

        tilemap = tilemap_new;
    }
    let c = tilemap.len();
    println!("Part 2: {}", c);
}

fn main() {
    p1();
    p2();
}
