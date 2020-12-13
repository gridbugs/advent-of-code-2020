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
    let mut waypoint_dxy = (10, 1);
    std::io::stdin().lock().lines().for_each(|l| {
        let line = l.unwrap();
        let command = line.chars().next().unwrap();
        let argument = line.split_at(1).1.parse::<i32>().unwrap();
        match command {
            'N' => waypoint_dxy.1 += argument,
            'S' => waypoint_dxy.1 -= argument,
            'E' => waypoint_dxy.0 += argument,
            'W' => waypoint_dxy.0 -= argument,
            'L' => {
                let mut rem = argument;
                while rem > 0 {
                    rem -= 90;
                    waypoint_dxy = left90(waypoint_dxy);
                }
            }
            'R' => {
                let mut rem = argument;
                while rem > 0 {
                    rem -= 90;
                    waypoint_dxy = right90(waypoint_dxy);
                }
            }
            'F' => {
                x += waypoint_dxy.0 * argument;
                y += waypoint_dxy.1 * argument;
            }
            _ => panic!(),
        }
    });
    println!("{}", x.abs() + y.abs());
}
