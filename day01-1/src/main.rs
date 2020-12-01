fn main() {
    use std::io::BufRead;
    let entries = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    for (i, a) in entries.iter().enumerate() {
        for b in &entries[i..] {
            if a + b == 2020 {
                println!("{}", a * b);
                return;
            }
        }
    }
}
