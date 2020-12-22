const SIZE: usize = 10;

#[derive(Debug)]
struct Tile {
    id: u16,
    sides: [u16; 4],
}

impl Tile {
    fn all_sides<'a>(&'a self) -> impl 'a + Iterator<Item = u16> {
        self.sides
            .iter()
            .cloned()
            .chain(self.sides.iter().cloned().map(flip_side))
    }
}

fn parse_stdin() -> Vec<Tile> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut out = Vec::new();
    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let line = buf.trim_end();
        if line.is_empty() {
            break;
        }
        let id = line
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let (mut top, mut bottom, mut left, mut right) = (0, 0, 0, 0);
        for i in 0..SIZE {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let line = buf.trim_start();
            let mut row = 0;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << (SIZE - j - 1);
                }
            }
            if row & 1 != 0 {
                right |= 1 << (SIZE - i - 1);
            }
            if row & (1 << (SIZE - 1)) != 0 {
                left |= 1 << (SIZE - i - 1);
            }
            if i == 0 {
                top = row;
            }
            if i == SIZE - 1 {
                bottom = row;
            }
        }
        let tile = Tile {
            id,
            sides: [top, bottom, left, right],
        };
        out.push(tile);
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
    }
    out
}

fn flip_side(side: u16) -> u16 {
    let mut out = 0u16;
    for i in 0..SIZE {
        if side & (1 << i) != 0 {
            out |= 1 << (SIZE - i - 1);
        }
    }
    out
}

fn main() {
    use std::collections::HashMap;
    let tiles = parse_stdin();
    let mut side_counts = HashMap::new();
    for tile in &tiles {
        for side in tile.all_sides() {
            let entry = side_counts.entry(side).or_insert_with(Vec::new);
            entry.push(tile.id);
        }
    }
    println!("{:#?}", side_counts);
    let mut edges = HashMap::new();
    for (side, ids) in side_counts {
        if ids.len() == 1 {
            edges.entry(ids[0]).or_insert_with(Vec::new).push(side);
        }
    }
    let mut product = 1u64;
    for (id, sides) in edges {
        if sides.len() == 4 {
            println!("{}: {:?}", id, sides);
            product *= id as u64;
        }
    }
    println!("{}", product);
}
