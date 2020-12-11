#[derive(Clone, Copy, PartialEq, Eq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Option<Seat>>,
}

impl Grid {
    fn parse_stdin() -> Self {
        use std::io::BufRead;
        let mut width = 0;
        let cells = std::io::stdin()
            .lock()
            .lines()
            .map(|l| {
                let line = l.unwrap();
                width = line.len();
                line.chars()
                    .map(|c| match c {
                        '.' => None,
                        'L' => Some(Seat::Empty),
                        '#' => Some(Seat::Occupied),
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        let height = cells.len() / width;
        Self {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<Option<Seat>> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(self.cells[(y * self.width as i32 + x) as usize])
        }
    }

    fn update(&mut self) -> usize {
        let mut count = 0;
        let original_grid = self.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * self.width + j;
                if let Some(ref seat) = original_grid.cells[index] {
                    let mut neighbour_count = 0;
                    for ni in i.saturating_sub(1)..=(i + 1).min(self.height - 1) {
                        for nj in j.saturating_sub(1)..=(j + 1).min(self.width - 1) {
                            let dy = ni as i32 - i as i32;
                            let dx = nj as i32 - j as i32;
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let mut x = nj as i32;
                            let mut y = ni as i32;
                            while let Some(maybe_seat) = original_grid.get(x, y) {
                                if let Some(seat) = maybe_seat {
                                    match seat {
                                        Seat::Empty => (),
                                        Seat::Occupied => neighbour_count += 1,
                                    }
                                    break;
                                } else {
                                    x += dx;
                                    y += dy;
                                }
                            }
                        }
                    }
                    match seat {
                        Seat::Empty => {
                            if neighbour_count == 0 {
                                self.cells[index] = Some(Seat::Occupied);
                                count += 1;
                            }
                        }
                        Seat::Occupied => {
                            if neighbour_count >= 5 {
                                self.cells[index] = Some(Seat::Empty);
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let mut grid = Grid::parse_stdin();
    while grid.update() > 0 {}
    let occupied_count = grid
        .cells
        .iter()
        .cloned()
        .filter(|&s| s == Some(Seat::Occupied))
        .count();
    println!("{}", occupied_count);
}
