use std::collections::BTreeMap;

const SIZE: usize = 10;

#[derive(Debug)]
struct Tile {
    id: u16,
    sides: BTreeMap<Side, u16>,
    cells: Vec<bool>,
}

impl Tile {
    fn all_sides<'a>(&'a self) -> impl 'a + Iterator<Item = u16> {
        self.sides
            .values()
            .cloned()
            .chain(self.sides.values().cloned().map(flip_side))
    }

    fn get_side(&self, orientation: Orientation, side: Side) -> u16 {
        let oriented_side = side.rotate_anticlockwise(orientation.rotate);
        match orientation.flip {
            Flip::Original => self.sides[&oriented_side],
            Flip::Flipped => match side {
                Side::Left | Side::Right => flip_side(self.sides[&oriented_side]),
                Side::Top | Side::Bottom => {
                    flip_side(self.sides[&oriented_side.rotate_clockwise(Rotate::D180)])
                }
            },
        }
    }

    fn orientation_for_side(&self, side_pattern: u16, side: Side) -> Option<Orientation> {
        for &rotate in &[Rotate::D0, Rotate::D90, Rotate::D180, Rotate::D270] {
            for &flip in &[Flip::Flipped, Flip::Original] {
                let orientation = Orientation { rotate, flip };
                if self.get_side(orientation, side) == side_pattern {
                    return Some(orientation);
                }
            }
        }
        None
    }

    fn transform_cells(&self, orientation: Orientation) -> Vec<bool> {
        let mut out = self.cells.clone();
        for i in 0..SIZE {
            for j in 0..SIZE {
                let index = i * SIZE + j;
                let (dest_x, dest_y) = orientation.rotate.apply(j, i);
                let dest_index = dest_y * SIZE + dest_x;
                out[dest_index] = self.cells[index];
            }
        }
        if let Flip::Flipped = orientation.flip {
            for i in 0..(SIZE / 2) {
                for j in 0..SIZE {
                    let index = i * SIZE + j;
                    let dest_index = (SIZE - i - 1) * SIZE + j;
                    out.swap(index, dest_index);
                }
            }
        }
        out
    }
}

fn print_cells(cells: &[bool]) {
    assert_eq!(cells.len(), SIZE * SIZE);
    cells.chunks(SIZE).for_each(|c| {
        c.iter().for_each(|&x| {
            if x {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!("");
    });
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
        let mut cells = Vec::new();
        for i in 0..SIZE {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let line = buf.trim_end();
            let mut row = 0;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << (SIZE - j - 1);
                    cells.push(true);
                } else {
                    cells.push(false);
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
        bottom = flip_side(bottom);
        left = flip_side(left);
        let tile = Tile {
            id,
            sides: vec![
                (Side::Top, top),
                (Side::Bottom, bottom),
                (Side::Left, left),
                (Side::Right, right),
            ]
            .into_iter()
            .collect(),
            cells,
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

fn side_pattern_to_tiles_ids(tiles: &BTreeMap<u16, Tile>) -> BTreeMap<u16, Vec<u16>> {
    let mut out = BTreeMap::new();
    for tile in tiles.values() {
        for side in tile.all_sides() {
            out.entry(side).or_insert_with(Vec::new).push(tile.id);
        }
    }
    out
}

fn edge_patterns(tiles: &BTreeMap<u16, Tile>) -> Vec<u16> {
    let side_counts = side_pattern_to_tiles_ids(tiles);
    let mut edges = BTreeMap::new();
    for (side, ids) in side_counts {
        if ids.len() == 1 {
            edges.entry(ids[0]).or_insert_with(Vec::new).push(side);
        }
    }
    let mut out = Vec::new();
    for (_, sides) in edges {
        if sides.len() == 4 {
            for side in sides {
                out.push(side);
            }
        }
    }

    out
}

fn corner_ids(tiles: &BTreeMap<u16, Tile>) -> Vec<u16> {
    let side_counts = side_pattern_to_tiles_ids(tiles);
    let mut edges = BTreeMap::new();
    for (side, ids) in side_counts {
        if ids.len() == 1 {
            edges.entry(ids[0]).or_insert_with(Vec::new).push(side);
        }
    }
    let mut out = Vec::new();
    for (id, sides) in edges {
        if sides.len() == 4 {
            out.push(id);
        }
    }
    out
}

#[derive(Debug, Clone, Copy)]
enum Rotate {
    D0,
    D90,
    D180,
    D270,
}

impl Rotate {
    fn to_i8(self) -> i8 {
        match self {
            Self::D0 => 0,
            Self::D90 => 1,
            Self::D180 => 2,
            Self::D270 => 3,
        }
    }

    fn apply(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::D0 => (x, y),
            Self::D90 => (SIZE - y - 1, x),
            Self::D180 => (SIZE - x - 1, SIZE - y - 1),
            Self::D270 => (y, SIZE - x - 1),
        }
    }

    fn apply2(self, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        match self {
            Self::D0 => (x, y),
            Self::D90 => (height - y - 1, x),
            Self::D180 => (width - x - 1, height - y - 1),
            Self::D270 => (y, width - x - 1),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    fn from_i8(u: i8) -> Self {
        match u {
            0 => Side::Top,
            1 => Side::Right,
            2 => Side::Bottom,
            3 => Side::Left,
            _ => panic!(),
        }
    }
    fn to_i8(self) -> i8 {
        match self {
            Side::Top => 0,
            Side::Right => 1,
            Side::Bottom => 2,
            Side::Left => 3,
        }
    }
    fn rotate_clockwise(self, rotate: Rotate) -> Self {
        Self::from_i8((self.to_i8() + rotate.to_i8()) % 4)
    }
    fn rotate_anticlockwise(self, rotate: Rotate) -> Self {
        Self::from_i8((self.to_i8() - rotate.to_i8() + 4) % 4)
    }
}

#[derive(Debug, Clone, Copy)]
enum Flip {
    Original,
    Flipped,
}

#[derive(Debug, Clone, Copy)]
struct Orientation {
    rotate: Rotate,
    flip: Flip,
}

struct TileTable {
    tiles: BTreeMap<u16, Tile>,
    side_pattern_to_tiles_ids: BTreeMap<u16, Vec<u16>>,
}

impl TileTable {
    fn new(tiles: BTreeMap<u16, Tile>) -> Self {
        let side_pattern_to_tiles_ids = side_pattern_to_tiles_ids(&tiles);
        Self {
            tiles,
            side_pattern_to_tiles_ids,
        }
    }

    fn get_possible_neighbour(
        &self,
        tile_id: u16,
        orientation: Orientation,
        side: Side,
    ) -> Option<(&Tile, Orientation, u16)> {
        let side_pattern = self.tiles[&tile_id].get_side(orientation, side);
        self.side_pattern_to_tiles_ids
            .get(&side_pattern)
            .and_then(|tiles| {
                assert!(tiles.len() <= 2);
                tiles.iter().cloned().filter(|&i| i != tile_id).next()
            })
            .map(|id| {
                let tile = &self.tiles[&id];
                (
                    tile,
                    tile.orientation_for_side(
                        flip_side(side_pattern),
                        side.rotate_clockwise(Rotate::D180),
                    )
                    .unwrap(),
                    side_pattern,
                )
            })
    }
}

struct Grid {
    height: usize,
    width: usize,
    cells: Vec<bool>,
}

impl Grid {
    fn sea_moster() -> Self {
        let s = &[
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ];
        let height = s.len();
        let width = s[0].len();
        let mut cells = Vec::new();
        for r in s {
            for c in r.chars() {
                if c == '#' {
                    cells.push(true);
                } else {
                    cells.push(false);
                }
            }
        }
        Self {
            width,
            height,
            cells,
        }
    }
    fn transform(&self, orientation: Orientation) -> Self {
        let (width, height) = match orientation.rotate {
            Rotate::D0 | Rotate::D180 => (self.width, self.height),
            Rotate::D90 | Rotate::D270 => (self.height, self.width),
        };
        let mut out = Self {
            width,
            height,
            cells: self.cells.clone(),
        };
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * out.width + j;
                let (dest_x, dest_y) = orientation.rotate.apply2(j, i, self.width, self.height);
                let dest_index = dest_y * out.width + dest_x;
                out.cells[dest_index] = self.cells[index];
            }
        }
        if let Flip::Flipped = orientation.flip {
            for i in 0..(out.height / 2) {
                for j in 0..out.width {
                    let index = i * out.width + j;
                    let dest_index = (out.height - i - 1) * out.width + j;
                    out.cells.swap(index, dest_index);
                }
            }
        }
        out
    }
    fn from_tiles(tiles: BTreeMap<u16, Tile>) -> Self {
        let side_length = (0..).find(|i| i * i == tiles.len()).unwrap();
        let corner_ids = corner_ids(&tiles);
        let edge_patterns = edge_patterns(&tiles);
        let (top_left_id, top_left_orientation) = {
            let top_left_id = corner_ids[0];
            let top_left_top_candidate = tiles[&top_left_id]
                .all_sides()
                .find(|s| edge_patterns.contains(s))
                .unwrap();
            let top_left_orientation_candidate = tiles[&top_left_id]
                .orientation_for_side(top_left_top_candidate, Side::Top)
                .unwrap();
            let top_left_orientation = if edge_patterns
                .contains(&tiles[&top_left_id].get_side(top_left_orientation_candidate, Side::Left))
            {
                top_left_orientation_candidate
            } else {
                assert!(edge_patterns.contains(
                    &tiles[&top_left_id].get_side(top_left_orientation_candidate, Side::Right)
                ));
                let top_left_left = top_left_top_candidate;
                tiles[&top_left_id]
                    .orientation_for_side(top_left_left, Side::Left)
                    .unwrap()
            };
            assert!(edge_patterns
                .contains(&tiles[&top_left_id].get_side(top_left_orientation, Side::Top)));
            assert!(edge_patterns
                .contains(&tiles[&top_left_id].get_side(top_left_orientation, Side::Left)));
            (top_left_id, top_left_orientation)
        };
        let tile_table = TileTable::new(tiles);
        let mut rows: Vec<Vec<(u16, Orientation)>> = Vec::new(); //vec![vec![(top_left_id, top_left_orientation)]];
        for i in 0..side_length {
            let (tile_id, orientation) = if i == 0 {
                (top_left_id, top_left_orientation)
            } else {
                let (above_id, above_orientation) = rows[i - 1][0];
                let (tile, orientation, _pattern) = tile_table
                    .get_possible_neighbour(above_id, above_orientation, Side::Bottom)
                    .unwrap();
                (tile.id, orientation)
            };
            let mut row = vec![(tile_id, orientation)];
            for j in 1..side_length {
                let (left_id, left_orientation) = row[j - 1];
                let (tile, orientation, _pattern) = tile_table
                    .get_possible_neighbour(left_id, left_orientation, Side::Right)
                    .unwrap();
                row.push((tile.id, orientation));
            }
            rows.push(row);
        }
        let full_side_length = side_length * (SIZE - 2);
        let mut full_cells = vec![false; full_side_length * full_side_length];
        for i in 0..side_length {
            for j in 0..side_length {
                let off_x = j * (SIZE - 2);
                let off_y = i * (SIZE - 2);
                let (tile_id, orientation) = rows[i][j];
                let cells = &tile_table.tiles[&tile_id].transform_cells(orientation);
                for dy in 0..(SIZE - 2) {
                    for dx in 0..(SIZE - 2) {
                        let dest_x = off_x + dx;
                        let dest_y = off_y + dy;
                        let dest_index = dest_y * full_side_length + dest_x;
                        let x = dx + 1;
                        let y = dy + 1;
                        let index = y * SIZE + x;
                        full_cells[dest_index] = cells[index]
                    }
                }
            }
        }
        Grid {
            cells: full_cells,
            width: full_side_length,
            height: full_side_length,
        }
    }

    fn print(&self) {
        for c in self.cells.chunks(self.width) {
            for &x in c {
                if x {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn count_overlaps(&self, other: &Self) -> usize {
        use std::collections::HashSet;
        let mut full_set = HashSet::new();
        for y_off in 0..=(self.height - other.height) {
            for x_off in 0..=(self.width - other.width) {
                let mut overlap = true;
                let mut set = HashSet::new();
                'outer: for (dy, chunk) in other.cells.chunks(other.width).enumerate() {
                    for (dx, &other_cell) in chunk.into_iter().enumerate() {
                        if other_cell {
                            let index = (y_off + dy) * self.width + (x_off + dx);
                            if !self.cells[index] {
                                overlap = false;
                                break 'outer;
                            }
                            set.insert(index);
                        }
                    }
                }
                if overlap {
                    full_set = full_set.union(&set).cloned().collect();
                }
            }
        }
        self.true_count() - full_set.len()
    }

    fn true_count(&self) -> usize {
        self.cells.iter().filter(|&&b| b).count()
    }
}

fn main() {
    let tiles = parse_stdin();
    let tiles = tiles
        .into_iter()
        .map(|t| (t.id, t))
        .collect::<BTreeMap<_, _>>();
    let grid = Grid::from_tiles(tiles);
    let sea_moster = Grid::sea_moster();
    for &rotate in &[Rotate::D0, Rotate::D90, Rotate::D180, Rotate::D270] {
        for &flip in &[Flip::Flipped, Flip::Original] {
            let orientation = Orientation { rotate, flip };
            let transformed_grid = grid.transform(orientation);
            if transformed_grid.count_overlaps(&sea_moster) != grid.true_count() {
                println!("{}", transformed_grid.count_overlaps(&sea_moster));
            }
        }
    }
}
