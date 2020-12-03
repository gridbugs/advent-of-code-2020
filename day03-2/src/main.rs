#[derive(Debug)]
struct Grid {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse_stdin() -> Self {
        use std::io::BufRead;
        let mut width = 0;
        let mut height = 0;
        let cells = std::io::stdin()
            .lock()
            .lines()
            .flat_map(|l| {
                let line = l.unwrap();
                width = line.len();
                height += 1;
                line.chars().map(|c| c == '.').collect::<Vec<_>>()
            })
            .collect();
        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(y < self.height);
        let index = (y * self.width) + (x % self.width);
        self.cells[index]
    }

    fn count_slope(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
        while y < self.height {
            count += !self.get(x, y) as usize;
            x += dx;
            y += dy;
        }
        count
    }
}

fn main() {
    let grid = Grid::parse_stdin();
    let result = grid.count_slope(1, 1)
        * grid.count_slope(3, 1)
        * grid.count_slope(5, 1)
        * grid.count_slope(7, 1)
        * grid.count_slope(1, 2);
    println!("{}", result);
}
