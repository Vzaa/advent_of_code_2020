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

impl Default for Tile {
    fn default() -> Self {
        Tile::Floor
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TileMap<T> {
    buf: Vec<T>,
    w: usize,
    h: usize,
}

impl<T: Default + Clone> TileMap<T> {
    fn new(h: usize, w: usize) -> TileMap<T> {
        TileMap {
            buf: vec![Default::default(); w * h],
            w,
            h,
        }
    }

    fn get(&self, p: &Pos) -> Option<&T> {
        if p.0 < 0 || p.1 < 0 || p.0 >= self.w as i64 {
            return None;
        }
        self.buf.get(p.1 as usize * self.w + p.0 as usize)
    }

    fn insert(&mut self, p: Pos, a: T) {
        self.buf[p.1 as usize * self.w + p.0 as usize] = a;
    }
    // TODO: add iterators like HashMap
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

fn neighbors(area: &TileMap<Tile>, p: Pos) -> impl Iterator<Item = &Tile> + '_ {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        area.get(&np)
    })
}

fn neighbors2(area: &TileMap<Tile>, p: Pos) -> impl Iterator<Item = &Tile> + '_ {
    NLIST.iter().filter_map(move |dir| {
        let mut np = p;

        iter::from_fn(move || {
            np = (np.0 + dir.0, np.1 + dir.1);
            area.get(&np)
        })
        .find(|&&t| t != Tile::Floor)
    })
}

fn p1(mut tilemap: TileMap<Tile>) {
    let mut tilemap_new = TileMap::new(tilemap.h, tilemap.w);
    loop {
        for y in 0_i64..(tilemap.h as i64) {
            for x in 0_i64..(tilemap.w as i64) {
                let k = &(x, y);
                let v = tilemap.get(k).unwrap();

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
        }

        if tilemap == tilemap_new {
            let cnt = tilemap.buf.iter().filter(|&&a| a == Tile::Occupied).count();
            println!("Part 1: {}", cnt);
            break;
        }

        std::mem::swap(&mut tilemap, &mut tilemap_new);
    }
}

fn p2(mut tilemap: TileMap<Tile>) {
    let mut tilemap_new = TileMap::new(tilemap.h, tilemap.w);
    loop {
        for y in 0_i64..(tilemap.h as i64) {
            for x in 0_i64..(tilemap.w as i64) {
                let k = &(x, y);
                let v = tilemap.get(k).unwrap();

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
        }

        if tilemap == tilemap_new {
            let cnt = tilemap.buf.iter().filter(|&&a| a == Tile::Occupied).count();
            println!("Part 2: {}", cnt);
            break;
        }

        std::mem::swap(&mut tilemap, &mut tilemap_new);
    }
}

#[allow(dead_code)]
fn draw_map(area: &TileMap<Tile>, h: i64, w: i64) {
    'outer: for y in 0..h {
        for x in 0..w {
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
    let w = instr.lines().next().unwrap().chars().count();
    let h = instr.lines().count();

    let mut tilemap = TileMap::new(h, w);

    for (y, line) in instr.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tilemap.insert((x as i64, y as i64), c.try_into().unwrap());
        }
    }

    p1(tilemap.clone());
    p2(tilemap.clone());
}
