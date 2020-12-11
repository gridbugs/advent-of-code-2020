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
                            let nindex = ni * self.width + nj;
                            if nindex != index {
                                if original_grid.cells[nindex] == Some(Seat::Occupied) {
                                    neighbour_count += 1;
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
                            if neighbour_count >= 4 {
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
