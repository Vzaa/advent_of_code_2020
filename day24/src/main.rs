use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

type TileMap = HashMap<Pos, Color>;

type Pos = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    B,
    W,
}

impl Color {
    fn flip(self) -> Color {
        match self {
            Color::B => Color::W,
            Color::W => Color::B,
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = match s {
            "e" => Dir::E,
            "se" => Dir::SE,
            "sw" => Dir::SW,
            "w" => Dir::W,
            "nw" => Dir::NW,
            "ne" => Dir::NE,
            _ => return Err(()),
        };

        Ok(d)
    }
}

impl From<Dir> for Pos {
    fn from(d: Dir) -> Pos {
        match d {
            Dir::E => (2, 0),
            Dir::SE => (1, -1),
            Dir::SW => (-1, -1),
            Dir::W => (-2, 0),
            Dir::NW => (-1, 1),
            Dir::NE => (1, 1),
        }
    }
}

static NLIST: [Pos; 6] = [(2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1), (1, 1)];

fn neighbors(area: &TileMap, p: Pos) -> impl Iterator<Item = &Color> + '_ {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        area.get(&np)
    })
}

fn neighbors_p(p: Pos) -> impl Iterator<Item = Pos> {
    NLIST.iter().filter_map(move |n| {
        let np = (p.0 + n.0, p.1 + n.1);
        Some(np)
    })
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut tile_dirs = vec![];

    for line in instr.lines() {
        let mut i = 0;
        let mut dirs = vec![];

        while i < line.len() {
            let a = &line[i..(i + 1)];
            let b = if i + 2 <= line.len() {
                &line[i..(i + 2)]
            } else {
                ""
            };

            if let Ok(d) = a.parse::<Dir>() {
                dirs.push(d);
                i += 1;
            } else if let Ok(d) = b.parse::<Dir>() {
                dirs.push(d);
                i += 2;
            } else {
                panic!("no");
            }
        }

        tile_dirs.push(dirs);
    }

    let mut tilemap = HashMap::new();

    for dirs in &tile_dirs {
        let mut pos = (0, 0);
        for d in dirs {
            let j: Pos = (*d).into();
            pos = (pos.0 + j.0, pos.1 + j.1);
        }

        let color = *tilemap.get(&pos).unwrap_or(&Color::W);
        tilemap.insert(pos, color.flip());
    }

    let cnt = tilemap.values().filter(|c| matches!(c, Color::B)).count();
    println!("Part 1: {}", cnt);

    let mut tilemap_new = HashMap::new();
    for _ in 0..100 {
        let mut to_visit = HashSet::new();

        for t in tilemap.keys() {
            to_visit.insert(*t);
            for n in neighbors_p(*t) {
                to_visit.insert(n);
            }
        }

        for p in to_visit {
            let v = *tilemap.get(&p).unwrap_or(&Color::W);

            let c = neighbors(&tilemap, p)
                .filter(|c| matches!(c, Color::B))
                .count();

            match v {
                Color::B => {
                    if c == 0 || c > 2 {
                        tilemap_new.insert(p, Color::W);
                    } else {
                        tilemap_new.insert(p, Color::B);
                    }
                }
                Color::W => {
                    if c == 2 {
                        tilemap_new.insert(p, Color::B);
                    } else {
                        tilemap_new.insert(p, Color::W);
                    }
                }
            }
        }

        std::mem::swap(&mut tilemap, &mut tilemap_new);
    }

    let cnt = tilemap.values().filter(|c| matches!(c, Color::B)).count();
    println!("Part 2: {}", cnt);
}
