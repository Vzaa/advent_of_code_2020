use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

type Pos = (i32, i32);
type TileMap = HashMap<Pos, Pixel>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pixel {
    Dot,
    Pound,
}

impl TryFrom<char> for Pixel {
    type Error = ();
    fn try_from(c: char) -> Result<Pixel, ()> {
        let out = match c {
            '.' => Pixel::Dot,
            '#' => Pixel::Pound,
            _ => return Err(()),
        };

        Ok(out)
    }
}

#[derive(Debug, Clone)]
struct Tile {
    data: TileMap,
}

impl Tile {
    fn new(data: TileMap) -> Tile {
        Tile { data }
    }

    fn edges(&self) -> [Vec<Pixel>; 4] {
        let (max_x, max_y) = (
            self.data.keys().map(|p| p.0).max().unwrap(),
            self.data.keys().map(|p| p.1).max().unwrap(),
        );

        let a = (0..=max_x).map(|x| self.data[&(x, 0)]).collect();
        let b = (0..=max_y).map(|y| self.data[&(max_x, y)]).collect();
        let c = (0..=max_x).map(|x| self.data[&(x, max_y)]).collect();
        let d = (0..=max_y).map(|y| self.data[&(0, y)]).collect();

        [a, b, c, d]
    }

    fn matching_pos(&self, other: &Tile) -> Option<Pos> {
        let our = self.edges();
        let theirs = other.edges();

        if our[0] == theirs[2] {
            Some((0, -1))
        } else if our[1] == theirs[3] {
            Some((1, 0))
        } else if our[2] == theirs[0] {
            Some((0, 1))
        } else if our[3] == theirs[1] {
            Some((-1, 0))
        } else {
            None
        }
    }

    fn rotate(&mut self) {
        let (max_x, max_y) = (
            self.data.keys().map(|p| p.0).max().unwrap(),
            self.data.keys().map(|p| p.1).max().unwrap(),
        );

        let mut data = HashMap::new();

        for x in 0..=max_x {
            for y in 0..=max_y {
                data.insert((x, y), self.data[&(y, max_x - x)]);
            }
        }

        self.data = data;
    }

    fn flip_x(&mut self) {
        let (max_x, max_y) = (
            self.data.keys().map(|p| p.0).max().unwrap(),
            self.data.keys().map(|p| p.1).max().unwrap(),
        );

        let mut data = HashMap::new();

        for x in 0..=max_x {
            for y in 0..=max_y {
                data.insert((x, y), self.data[&(max_x - x, y)]);
            }
        }

        self.data = data;
    }

    fn flip_y(&mut self) {
        let (max_x, max_y) = (
            self.data.keys().map(|p| p.0).max().unwrap(),
            self.data.keys().map(|p| p.1).max().unwrap(),
        );

        let mut data = HashMap::new();

        for x in 0..=max_x {
            for y in 0..=max_y {
                data.insert((x, y), self.data[&(x, max_y - y)]);
            }
        }

        self.data = data;
    }

    fn erase_pattern(&mut self, p: &Tile) {
        let (max_x, max_y) = (
            self.data.keys().map(|p| p.0).max().unwrap(),
            self.data.keys().map(|p| p.1).max().unwrap(),
        );

        let points: Vec<_> = p
            .data
            .iter()
            .filter(|(_, v)| **v == Pixel::Pound)
            .map(|(k, _)| k)
            .collect();

        for _ in 0..4 {
            self.rotate();
            for _ in 0..2 {
                self.flip_y();
                for _ in 0..2 {
                    self.flip_x();

                    for x in 0..=max_x {
                        for y in 0..=max_y {
                            let found = points
                                .iter()
                                .all(|p| self.data.get(&(x + p.0, y + p.1)) == Some(&Pixel::Pound));
                            if found {
                                points.iter().for_each(|p| {
                                    if let Some(r) = self.data.get_mut(&(x + p.0, y + p.1)) {
                                        *r = Pixel::Dot;
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut tiles = HashMap::new();

    for tile_str in instr.trim().split("\n\n") {
        let tile_str = tile_str.trim();
        let mut line_iter = tile_str.lines();
        let id_line = line_iter.next().unwrap();

        let id: usize = id_line
            .split(|c| c == ' ' || c == ':')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let mut tilemap: TileMap = HashMap::new();

        for (y, line) in line_iter.enumerate() {
            for (x, c) in line.chars().enumerate() {
                tilemap.insert((x as i32, y as i32), c.try_into().unwrap());
            }
        }

        tiles.insert(id, Tile::new(tilemap));
    }

    let mut corners = vec![];

    for (id_a, tile_a) in &tiles {
        let edges_a = tile_a.edges();

        let mut cnts = vec![];

        for edge_a in &edges_a {
            let mut cnt = 0;
            let mut flip_a = edge_a.clone();
            flip_a.reverse();

            for (id_b, tile_b) in &tiles {
                if id_a == id_b {
                    continue;
                }
                let edges_b = tile_b.edges();

                if edges_b.contains(&edge_a) || edges_b.contains(&flip_a) {
                    cnt += 1;
                }
            }
            cnts.push(cnt);
        }

        let sum: usize = cnts.iter().sum();

        if sum == 2 {
            corners.push(id_a);
        }
    }

    println!("Part 1: {}", corners.iter().fold(1, |acc, x| acc * *x));
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut tiles = HashMap::new();

    for tile_str in instr.trim().split("\n\n") {
        let tile_str = tile_str.trim();
        let mut line_iter = tile_str.lines();
        let id_line = line_iter.next().unwrap();

        let id: usize = id_line
            .split(|c| c == ' ' || c == ':')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let mut tilemap: TileMap = HashMap::new();

        for (y, line) in line_iter.enumerate() {
            for (x, c) in line.chars().enumerate() {
                tilemap.insert((x as i32, y as i32), c.try_into().unwrap());
            }
        }

        tiles.insert(id, Tile::new(tilemap));
    }

    let mut corners = vec![];

    let mut matchings = HashMap::new();

    for (id_a, tile_a) in &tiles {
        let edges_a = tile_a.edges();

        let mut cnts = vec![];

        for edge_a in &edges_a {
            let mut cnt = 0;
            let mut flip_a = edge_a.clone();
            flip_a.reverse();

            for (id_b, tile_b) in &tiles {
                if id_a == id_b {
                    continue;
                }
                let edges_b = tile_b.edges();

                if edges_b.contains(&edge_a) || edges_b.contains(&flip_a) {
                    let c = matchings.entry(*id_a).or_insert(vec![]);
                    c.push(*id_b);
                    cnt += 1;
                }
            }
            cnts.push(cnt);
        }
        let sum: usize = cnts.iter().sum();

        if sum == 2 {
            corners.push(*id_a);
        }
    }

    let mut remaining = tiles;
    let mut mapped_tiles = HashMap::new();

    // Use first corner as an anchor
    let (id, anchor) = remaining.remove_entry(&corners[0]).unwrap();
    mapped_tiles.insert(id, ((0, 0), anchor));

    'outer: while !remaining.is_empty() {
        let (piece_id, neighbor_id) = remaining
            .keys()
            .filter_map(|id| {
                mapped_tiles
                    .keys()
                    .find(|o| matchings[id].contains(&o))
                    .map(|f| (*id, *f))
            })
            .next()
            .unwrap();

        let mut piece = remaining.remove(&piece_id).unwrap();

        let (neighbor_pos, neighbor) = &mapped_tiles[&neighbor_id].clone();

        for _ in 0..4 {
            piece.rotate();
            for _ in 0..2 {
                piece.flip_y();
                for _ in 0..2 {
                    piece.flip_x();
                    if let Some(p) = neighbor.matching_pos(&piece) {
                        let npos = (neighbor_pos.0 + p.0, neighbor_pos.1 + p.1);
                        mapped_tiles.insert(piece_id, (npos, piece.clone()));
                        continue 'outer;
                    }
                }
            }
        }
        panic!("invalid");
    }

    let mut tile_pos_map = HashMap::new();

    for (p, i) in mapped_tiles.values() {
        tile_pos_map.insert(*p, i.clone());
    }

    let (max_x, max_y) = (
        tile_pos_map.keys().map(|p| p.0).max().unwrap(),
        tile_pos_map.keys().map(|p| p.1).max().unwrap(),
    );

    let (min_x, min_y) = (
        tile_pos_map.keys().map(|p| p.0).min().unwrap(),
        tile_pos_map.keys().map(|p| p.1).min().unwrap(),
    );

    let mut img_borderless = HashMap::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let t = &tile_pos_map[&(x, y)];

            for (a, p) in &t.data {
                if a.0 == 0 || a.1 == 0 || a.0 == 9 || a.1 == 9 {
                    continue;
                }
                let cor = ((x - min_x) * 8 + a.0 - 1, (y - min_y) * 8 + a.1 - 1);
                if img_borderless.insert(cor, *p).is_some() {
                    panic!("a");
                }
            }
        }
    }
    let mut tile_borderless = Tile::new(img_borderless);

    let monster_str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let mut monster: TileMap = HashMap::new();

    for (y, line) in monster_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Ok(p) = c.try_into() {
                monster.insert((x as i32, y as i32), p);
            }
        }
    }

    let monster = Tile::new(monster);

    tile_borderless.erase_pattern(&monster);
    let count = tile_borderless
        .data
        .values()
        .filter(|p| **p == Pixel::Pound)
        .count();
    println!("Part 2: {}", count);
}

fn main() {
    p1();
    p2();
}
