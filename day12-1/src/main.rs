fn left90((x, y): (i32, i32)) -> (i32, i32) {
    (-y, x)
}
fn right90((x, y): (i32, i32)) -> (i32, i32) {
    (y, -x)
}

fn main() {
    use std::io::BufRead;
    let mut x = 0i32;
    let mut y = 0i32;
    let mut facing_dxy = (1, 0);
    std::io::stdin().lock().lines().for_each(|l| {
        let line = l.unwrap();
        let command = line.chars().next().unwrap();
        let argument = line.split_at(1).1.parse::<i32>().unwrap();
        match command {
            'N' => y += argument,
            'S' => y -= argument,
            'E' => x += argument,
            'W' => x -= argument,
            'L' => {
                let mut rem = argument;
                while rem > 0 {
                    rem -= 90;
                    facing_dxy = left90(facing_dxy);
                }
            }
            'R' => {
                let mut rem = argument;
                while rem > 0 {
                    rem -= 90;
                    facing_dxy = right90(facing_dxy);
                }
            }
            'F' => {
                x += facing_dxy.0 * argument;
                y += facing_dxy.1 * argument;
            }
            _ => panic!(),
        }
    });
    println!("{}", x.abs() + y.abs());
}
