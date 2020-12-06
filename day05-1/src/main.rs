fn main() {
    use std::io::BufRead;
    let max_id = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut digit = 512;
            let mut id = 0;
            line.chars().for_each(|c| {
                if c == 'B' || c == 'R' {
                    id += digit;
                }
                digit /= 2;
            });
            id
        })
        .max()
        .unwrap();
    println!("{}", max_id);
}
